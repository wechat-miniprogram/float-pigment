#![cfg(feature = "ffi")]

use alloc::{
    ffi::CString,
    string::{String, ToString},
};
use core::{
    ffi::{c_char, CStr},
    ptr::null_mut,
};

use cssparser::{ParseError, ParseErrorKind, Parser, ParserInput, ToCss, Token};
use hashbrown::HashSet;

use crate::parser::CustomError;

pub type CustomPropertyGetter =
    unsafe extern "C" fn(map: *mut (), name: *const c_char) -> *const c_char;

pub type CustomPropertySetter =
    unsafe extern "C" fn(map: *mut (), name: *const c_char, value: *const c_char);

#[derive(Debug, Clone)]
pub(crate) struct CustomPropertyContext {
    map: *mut (),
    getter: CustomPropertyGetter,
    setter: CustomPropertySetter,
}

impl CustomPropertyContext {
    pub(crate) fn create(
        map: *mut (),
        getter: CustomPropertyGetter,
        setter: CustomPropertySetter,
    ) -> Self {
        Self {
            map,
            getter,
            setter,
        }
    }
    fn custom_property(&self, name: &str) -> Option<String> {
        unsafe {
            let name_ptr = CString::new(name).expect("CString new error").into_raw();
            let value_ptr = (self.getter)(self.map, name_ptr);
            drop(CString::from_raw(name_ptr));
            if value_ptr.is_null() {
                return None;
            }
            let value = CStr::from_ptr(value_ptr).to_string_lossy();
            Some(value.to_string())
        }
    }

    fn set_custom_property(&self, name: &str, value: Option<String>) {
        let name_ptr = CString::new(name).expect("CString new error").into_raw();
        let value_ptr = if let Some(value) = value {
            CString::new(value).expect("CString new error").into_raw()
        } else {
            null_mut()
        };
        unsafe {
            (self.setter)(self.map, name_ptr, value_ptr);
            drop(CString::from_raw(name_ptr));
            if !value_ptr.is_null() {
                drop(CString::from_raw(value_ptr));
            }
        }
    }
}

fn is_exhausted_with_whitespace<'a, 't: 'a, 'i: 't>(parser: &'a mut Parser<'i, 't>) -> bool {
    let start = parser.state();
    let result = parser.next_including_whitespace().is_err();
    parser.reset(&start);
    result
}

/// Variable Substitute
pub(crate) fn substitute_variable(expr: &str, context: &CustomPropertyContext) -> Option<String> {
    let mut parser_input = ParserInput::new(expr);
    let mut parser = Parser::new(&mut parser_input);
    let mut substituted_expr = String::new();
    let mut variable_visited: HashSet<String> = HashSet::default();
    parse_and_substitute_var(
        &mut parser,
        context,
        &mut substituted_expr,
        &mut variable_visited,
        true,
    )
    .map(|_| substituted_expr.trim().to_string())
    .ok()
}

pub(crate) fn parse_and_substitute_var<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    context: &CustomPropertyContext,
    substituted_expr: &mut String,
    variable_visited: &mut HashSet<String>,
    is_entrance: bool,
) -> Result<(), ParseError<'i, CustomError>> {
    while !is_exhausted_with_whitespace(parser) {
        let next_token: Token = parser.next_including_whitespace()?.clone();
        match &next_token {
            Token::Function(func) => {
                let func: &str = func;
                if func == "var" {
                    parser.parse_nested_block::<_, (), CustomError>(|parser| {
                        let next: &str = &parser.expect_ident()?.clone();
                        if next.starts_with("--") && next.len() > 2 {
                            let maybe_has_value = context.custom_property(next);
                            if let Some(value) = maybe_has_value {
                                let name = next.to_string();
                                if variable_visited.contains(&name) {
                                    context.set_custom_property(&name, None);
                                    return Err(parser.new_custom_error(
                                        CustomError::VariableCycle(name, false),
                                    ));
                                }
                                variable_visited.insert(next.to_string());
                                let value: &str = &value;
                                let mut parser_input: ParserInput = ParserInput::new(value);
                                let mut inner_parser = Parser::new(&mut parser_input);
                                let res = parse_and_substitute_var(
                                    &mut inner_parser,
                                    context,
                                    substituted_expr,
                                    variable_visited,
                                    false,
                                )
                                .map_err(|err| {
                                    if let ParseErrorKind::Custom(CustomError::VariableCycle(
                                        cycle_head,
                                        _,
                                    )) = err.kind
                                    {
                                        let fallback = cycle_head == name;
                                        context.set_custom_property(&name, None);
                                        parser.new_custom_error(CustomError::VariableCycle(
                                            cycle_head, fallback,
                                        ))
                                    } else {
                                        parser.new_custom_error(CustomError::Unmatched)
                                    }
                                });
                                if res.is_ok() {
                                    while !parser.is_exhausted() {
                                        parser.next()?;
                                    }
                                    return res;
                                } else {
                                    if let ParseErrorKind::Custom(CustomError::VariableCycle(
                                        _,
                                        fallback,
                                    )) = res.as_ref().err().unwrap().kind
                                    {
                                        if !fallback {
                                            return res;
                                        }
                                    }
                                }
                            }
                            variable_visited.clear();
                            parser.expect_comma()?;
                            let ret = parse_and_substitute_var(
                                parser,
                                context,
                                substituted_expr,
                                variable_visited,
                                true,
                            );
                            return ret;
                        }
                        Err(parser.new_custom_error(CustomError::Unsupported))
                    })?;
                } else {
                    let next_token_string = next_token.to_css_string();
                    substituted_expr.push_str(next_token_string.as_str());
                    parser.parse_nested_block::<_, (), CustomError>(|parser| {
                        parse_and_substitute_var(
                            parser,
                            context,
                            substituted_expr,
                            variable_visited,
                            is_entrance,
                        )
                    })?;
                    substituted_expr.push(')');
                }
            }
            Token::ParenthesisBlock => {
                let next_token_string = next_token.to_css_string();
                substituted_expr.push_str(next_token_string.as_str());
                parser.parse_nested_block::<_, (), CustomError>(|parser| {
                    parse_and_substitute_var(
                        parser,
                        context,
                        substituted_expr,
                        variable_visited,
                        is_entrance,
                    )
                })?;
                substituted_expr.push(')');
            }
            _ => substituted_expr.push_str(&next_token.to_css_string()),
        }
        if is_entrance {
            variable_visited.clear()
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use alloc::{
        borrow::ToOwned,
        boxed::Box,
        ffi::CString,
        string::{String, ToString},
    };
    use core::{
        ffi::{c_char, CStr},
        ptr::null,
    };
    use hashbrown::HashMap;

    use crate::parser::property_value::var::{substitute_variable, CustomPropertyContext};

    extern "C" fn variable_getter(map: *mut (), name: *const c_char) -> *const c_char {
        unsafe {
            let map = &*(map as *mut HashMap<&str, &str>);
            let name: &str = CStr::from_ptr(name).to_str().unwrap();
            if let Some(ret) = map.get(name) {
                CString::new(ret.to_string()).unwrap().into_raw()
            } else {
                null()
            }
        }
    }

    extern "C" fn variable_setter(map: *mut (), name: *const c_char, value: *const c_char) {
        unsafe {
            let map = &mut *(map as *mut HashMap<&str, &str>);
            let name = CStr::from_ptr(name).to_str().unwrap().to_owned();
            if value.is_null() {
                map.remove(name.as_str());
            } else {
                let value = CStr::from_ptr(value).to_str().unwrap().to_owned();
                map.insert(&name, &value);
            }
        }
    }
    #[test]
    fn illegal_custom_property_name() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "10px");
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let tmp = "var(a)";
        let ret: Option<String> = substitute_variable(tmp, &context);
        assert!(ret.is_none());
    }
    #[test]
    fn defined_variable() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "10px");
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let tmp = "var(--a)";
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("10px", ret);
        }
    }

    #[test]
    fn defined_variable_2() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "10px");
        let tmp = "var(--a, 20px)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("10px", ret);
        }
    }

    #[test]
    fn undefined_variable() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let tmp = "var(--a)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_none());
    }

    #[test]
    fn undefined_variable_fallback_value() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let tmp = "var(--a, 10px)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("10px", ret);
        }
    }

    #[test]
    fn undefined_variable_fallback_value_2() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let tmp = "var(--a, var(--a, 10px))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("10px", ret);
        }
    }

    #[test]
    fn undefined_variable_fallback_value_3() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--b", "10px");
        let tmp = "var(--a, var(--b))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("10px", ret);
        }
    }

    #[test]
    fn self_cycle() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--a)");
        let tmp = "var(--a)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_none());
    }

    #[test]
    fn self_cycle_2() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--a, 10px)");
        let tmp = "var(--a, 20px)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("20px", ret);
        }
    }

    #[test]
    fn self_cycle_3() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--a, 10px)");
        let tmp = "var(--a, var(--a, 30px))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("30px", ret);
        }
    }

    #[test]
    fn cycle() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--b, 10px)");
        map_mut.insert("--b", "var(--c, 20px)");
        map_mut.insert("--c", "var(--a, 30px)");

        let tmp = "var(--a, 40px)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("40px", ret);
        }
    }

    #[test]
    fn cycle_2() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--b, 10px)");
        map_mut.insert("--b", "var(--c, 20px)");
        map_mut.insert("--c", "var(--b, 30px)");
        let tmp = "var(--a, 40px)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("10px", ret);
        }
    }

    #[test]
    fn cycle_3() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--b, 10px)");
        map_mut.insert("--b", "var(--c, 20px)");
        map_mut.insert("--c", "var(--a, 30px)");
        let tmp = "var(--a, var(--a, var(--b)))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_none());
    }

    #[test]
    fn cycle_4() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--b, 10px)");
        map_mut.insert("--b", "var(--c, 20px)");
        map_mut.insert("--c", "var(--a, 30px)");
        let tmp = "var(--a, var(--a, var(--b, 40px)))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("40px", ret);
        }
    }

    #[test]
    fn cycle_5() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--b)");
        map_mut.insert("--b", "var(--c)");
        map_mut.insert("--c", "var(--a, 10px)");
        let tmp = "var(--c, 20px)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("20px", ret);
        }
    }

    #[test]
    fn custom_property_cycle() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "var(--b)");
        map_mut.insert("--b", "var(--c)");
        map_mut.insert("--c", "var(--a, 10px)");
        let tmp = "var(--b)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_none());
    }

    #[test]
    fn multiple_var() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "50px");
        let tmp = "var(--a,10px) var(--b,20px) var(--a) var(--b,var(--a))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("50px 20px 50px 50px", ret);
        }
    }

    #[test]
    fn calc() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "50px");

        let tmp = "calc(10px + var(--a))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("calc(10px + 50px)", ret);
        }
    }

    #[test]
    fn calc_2() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "50px + 20%");
        let tmp = "calc(var(--a))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("calc(50px + 20%)", ret);
        }
    }

    #[test]
    fn calc_3() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "50px + 20%");
        let tmp = "calc(var(--a))";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("calc(50px + 20%)", ret);
        }
    }

    // maybe this case should be remove
    #[test]
    fn parenthesis() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let tmp: &str = "(20px + 30px)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("(20px + 30px)", ret);
        }
    }

    #[test]
    fn empty_custom_property() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let map_mut = unsafe { &mut *map };
        map_mut.insert("--a", "");
        let tmp = "var(--a)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_some());
        if let Some(ret) = ret {
            assert_eq!("", ret);
        }
    }

    #[test]
    fn null_custom_property() {
        let map = Box::into_raw(Box::new(HashMap::<&str, &str>::default()));
        let tmp = "var(--a)";
        let context =
            CustomPropertyContext::create(map as *mut (), variable_getter, variable_setter);
        let ret = substitute_variable(tmp, &context);
        assert!(ret.is_none());
    }
}
