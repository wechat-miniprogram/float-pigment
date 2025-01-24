use proc_macro2::TokenStream;
use quote::*;
use rustc_hash::FxHashMap;
use std::fmt::Write;
use std::ops::RangeInclusive;
use syn::parse::*;
use syn::punctuated::Punctuated;
use syn::*;

// style syntax parsing
#[derive(Clone)]
struct StyleSyntaxItem {
    name: Option<String>,
    fn_name: Ident,
    ret_ty: Option<Path>,
    value: StyleSyntaxValueItem,
    related_prop_names: Vec<Ident>,
    is_property: bool,
}

#[derive(Clone)]
enum StyleSyntaxValueItem {
    Series(Vec<StyleSyntaxValueItem>),
    AnyOrder(Punctuated<StyleSyntaxValueItem, Token![&&]>),
    MultiSelect(Punctuated<StyleSyntaxValueItem, Token![||]>),
    Branch(Punctuated<StyleSyntaxValueItem, Token![|]>),
    RepeatTimes(Box<StyleSyntaxValueItem>, RangeInclusive<usize>),
    Optional(Box<StyleSyntaxValueItem>),
    RepeatCommaSep(Box<StyleSyntaxValueItem>),
    MustMatchOnce(Vec<StyleSyntaxValueItem>),
    MatchFn(String, Box<StyleSyntaxValueItem>),
    MatchIdent(String),
    MatchDelim(char),
    Call(Path),
    FilterFn(Box<StyleSyntaxValueItem>, Path),
    FilterClosure(Box<StyleSyntaxValueItem>, ExprClosure),
    Convert(Box<StyleSyntaxValueItem>, Path),
    Assign(Ident, Box<StyleSyntaxValueItem>),
    MultiAssign(Punctuated<Ident, Token![,]>, Box<StyleSyntaxValueItem>),
    ResultFilterClosure(Box<StyleSyntaxValueItem>, ExprClosure),
}

impl StyleSyntaxValueItem {
    fn parse_sub(input: ParseStream) -> Result<Self> {
        let ret = Punctuated::parse_separated_nonempty_with(input, Self::parse_branch)?;
        if ret.len() == 1 {
            Ok(ret[0].clone())
        } else {
            Ok(Self::Branch(ret))
        }
    }

    fn parse_branch(input: ParseStream) -> Result<Self> {
        let ret = Punctuated::parse_separated_nonempty_with(input, Self::parse_any_order)?;
        if ret.len() == 1 {
            Ok(ret[0].clone())
        } else {
            Ok(Self::MultiSelect(ret))
        }
    }

    fn parse_any_order(input: ParseStream) -> Result<Self> {
        let ret = Punctuated::parse_separated_nonempty_with(input, Self::parse_series)?;
        if ret.len() == 1 {
            Ok(ret[0].clone())
        } else {
            Ok(Self::AnyOrder(ret))
        }
    }

    fn parse_series(input: ParseStream) -> Result<Self> {
        let mut ret = vec![];
        let mut last_lookahead = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(token::Bracket) {
                // [ ... ] sub pattern
                let content;
                bracketed!(content in input);
                let input = content;
                ret.push(Self::parse_sub(&input)?);
                if !input.is_empty() {
                    Err(input.lookahead1().error())?;
                }
            } else if lookahead.peek(token::Brace) {
                // {M, N} or {{ ... = ... }}
                let content;
                braced!(content in input);
                let input = content;
                let lookahead2 = input.lookahead1();
                if lookahead2.peek(token::Brace) {
                    let content;
                    braced!(content in input);
                    let input = content;
                    let lookahead3 = input.lookahead1();
                    if lookahead3.peek(token::Paren) {
                        // {{ (..., ..., ...) = ... }}
                        let idents = {
                            let content;
                            parenthesized!(content in input);
                            let input = content;
                            Punctuated::parse_terminated_with(&input, |input| input.parse())?
                        };
                        input.parse::<Token![=]>()?;
                        ret.push(Self::MultiAssign(
                            idents,
                            Box::new(Self::parse_sub(&input)?),
                        ));
                    } else {
                        // {{ ... = ... }}
                        let ident = input.parse::<Ident>()?;
                        input.parse::<Token![=]>()?;
                        ret.push(Self::Assign(ident, Box::new(Self::parse_sub(&input)?)));
                    }
                    if !input.is_empty() {
                        Err(input.lookahead1().error())?;
                    }
                } else {
                    // {M, N}
                    if let Some(x) = ret.last_mut() {
                        let start = input.parse::<LitInt>()?.base10_parse()?;
                        input.parse::<Token![,]>()?;
                        let end = input.parse::<LitInt>()?.base10_parse()?;
                        *x = Self::RepeatTimes(Box::new(x.clone()), start..=end);
                    } else {
                        Err(lookahead2.error())?;
                    }
                }
                if !input.is_empty() {
                    Err(input.lookahead1().error())?;
                }
            } else if lookahead.peek(Token![<]) {
                // <...> parse function
                input.parse::<Token![<]>()?;
                let p = input.parse::<Path>()?;
                input.parse::<Token![>]>()?;
                ret.push(Self::Call(p));
            } else if lookahead.peek(Token![*]) {
                // [ ... ]*
                input.parse::<Token![*]>()?;
                if let Some(x) = ret.last_mut() {
                    *x = Self::RepeatTimes(Box::new(x.clone()), 0..=usize::MAX);
                } else {
                    Err(lookahead.error())?;
                }
            } else if lookahead.peek(Token![+]) {
                // [ ... ]+
                input.parse::<Token![+]>()?;
                if let Some(x) = ret.last_mut() {
                    *x = Self::RepeatTimes(Box::new(x.clone()), 1..=usize::MAX);
                } else {
                    Err(lookahead.error())?;
                }
            } else if lookahead.peek(Token![?]) {
                // [ ... ]?
                input.parse::<Token![?]>()?;
                if let Some(x) = ret.last_mut() {
                    *x = Self::Optional(Box::new(x.clone()));
                } else {
                    Err(lookahead.error())?;
                }
            } else if lookahead.peek(Token![#]) {
                // [ ... ]#
                input.parse::<Token![#]>()?;
                if let Some(x) = ret.last_mut() {
                    *x = Self::RepeatCommaSep(Box::new(x.clone()));
                } else {
                    Err(lookahead.error())?;
                }
            } else if lookahead.peek(Token![!]) {
                // [ ...?, ...* ]!
                input.parse::<Token![!]>()?;
                if let Some(Self::Series(x)) = ret.pop() {
                    let mut illegal = false;
                    for x in x.iter() {
                        match x {
                            Self::Optional(..) | Self::RepeatTimes(..) => {}
                            _ => {
                                illegal = true;
                            }
                        }
                    }
                    if illegal {
                        Err(lookahead.error())?;
                    } else {
                        ret.push(Self::MustMatchOnce(x));
                    }
                } else {
                    Err(lookahead.error())?;
                }
            } else if lookahead.peek(Token![=>]) {
                // ... => ... value convertion
                input.parse::<Token![=>]>()?;
                if let Some(x) = ret.last_mut() {
                    *x = Self::Convert(Box::new(x.clone()), input.parse()?);
                } else {
                    Err(lookahead.error())?;
                }
            } else if lookahead.peek(Token![->]) {
                // ... -> ... value convertion function or closure
                input.parse::<Token![->]>()?;
                if let Some(x) = ret.last_mut() {
                    let lookahead2 = input.lookahead1();
                    if lookahead2.peek(Token![|]) {
                        *x = Self::FilterClosure(Box::new(x.clone()), input.parse()?);
                        input.parse::<Token![;]>()?;
                    } else if lookahead2.peek(Ident) {
                        // -> ResultClosure |...| {...}
                        let ret = input.parse::<Path>()?;
                        if ret.to_token_stream().to_string() == "ResultClosure"
                            && input.lookahead1().peek(Token![|])
                        {
                            *x = Self::ResultFilterClosure(Box::new(x.clone()), input.parse()?);
                            input.parse::<Token![;]>()?;
                        } else {
                            *x = Self::FilterFn(Box::new(x.clone()), ret);
                        }
                    }
                } else {
                    Err(lookahead.error())?;
                }
            } else if lookahead.peek(LitChar) {
                // '...' static delimiter (such as '/')
                let c = input.parse::<LitChar>()?;
                ret.push(Self::MatchDelim(c.value()));
            } else if lookahead.peek(LitStr) {
                // "..." static identifier
                let s = input.parse::<LitStr>()?;
                ret.push(Self::MatchIdent(s.value().to_lowercase()));
            } else if lookahead.peek(Ident) {
                // ...( ... ) function parsing (such as cubic-bezier( ... ) )
                let ident = input.parse::<Ident>()?;
                let lookahead2 = input.lookahead1();
                if lookahead2.peek(token::Paren) {
                    let content;
                    parenthesized!(content in input);
                    let input = content;
                    ret.push(Self::MatchFn(
                        ident.to_string().replace('_', "-"),
                        Box::new(Self::parse_sub(&input)?),
                    ));
                    if !input.is_empty() {
                        Err(input.lookahead1().error())?;
                    }
                } else {
                    Err(lookahead.error())?;
                }
            } else {
                last_lookahead = Some(lookahead);
                break;
            }
        }
        if ret.is_empty() {
            Err(last_lookahead.unwrap_or_else(|| input.lookahead1()).error())
        } else if ret.len() == 1 {
            Ok(ret[0].clone())
        } else {
            Ok(Self::Series(ret))
        }
    }

    fn collect_prop_names(&self, related_prop_names: &mut Vec<Ident>) {
        match self {
            Self::Series(x) => {
                for item in x.iter() {
                    item.collect_prop_names(related_prop_names);
                }
            }
            Self::AnyOrder(x) => {
                for item in x.iter() {
                    item.collect_prop_names(related_prop_names);
                }
            }
            Self::MultiSelect(x) => {
                for item in x.iter() {
                    item.collect_prop_names(related_prop_names);
                }
            }
            Self::Branch(x) => {
                for item in x.iter() {
                    item.collect_prop_names(related_prop_names);
                }
            }
            Self::RepeatTimes(x, _range) => {
                x.collect_prop_names(related_prop_names);
            }
            Self::Optional(x) => {
                x.collect_prop_names(related_prop_names);
            }
            Self::RepeatCommaSep(x) => {
                x.collect_prop_names(related_prop_names);
            }
            Self::MustMatchOnce(x) => {
                for item in x.iter() {
                    item.collect_prop_names(related_prop_names);
                }
            }
            Self::MatchFn(_name, x) => {
                x.collect_prop_names(related_prop_names);
            }
            Self::MatchIdent(_s) => {
                // empty
            }
            Self::MatchDelim(_c) => {
                // empty
            }
            Self::Call(_f) => {
                // empty
            }
            Self::FilterFn(x, _f) => {
                x.collect_prop_names(related_prop_names);
            }
            Self::FilterClosure(x, _f) => {
                x.collect_prop_names(related_prop_names);
            }
            Self::Convert(x, _expr) => {
                x.collect_prop_names(related_prop_names);
            }
            Self::Assign(prop_name, x) => {
                related_prop_names.push(prop_name.clone());
                x.collect_prop_names(related_prop_names);
            }
            Self::MultiAssign(prop_names, x) => {
                related_prop_names.append(&mut prop_names.iter().cloned().collect());
                x.collect_prop_names(related_prop_names);
            }
            Self::ResultFilterClosure(x, _f) => {
                x.collect_prop_names(related_prop_names);
            }
        }
    }

    fn generate(&self, tokens: &mut TokenStream) {
        let t = match self {
            Self::Series(x) => {
                let target_op: Vec<_> = x
                    .iter()
                    .map(|target| {
                        quote! {
                            { #target }
                        }
                    })
                    .collect();
                quote! {
                    (
                        #(#target_op),*
                    )
                }
            }
            Self::AnyOrder(x) => {
                let count = x.len();
                let ret_group: Vec<_> = x
                    .iter()
                    .map(|_| {
                        quote! {
                            None
                        }
                    })
                    .collect();
                let target_op: Vec<_> = x.iter().enumerate().map(|(i, target)| {
                    let i = syn::Index::from(i);
                    quote! {
                        if __ret.#i.is_none() {
                            let r = parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                                Ok(#target)
                            });
                            if let Ok(x) = r {
                                __ret.#i = Some(x);
                                continue;
                            }
                        }
                    }
                }).collect();
                quote! {
                    {
                        let mut __ret = (#(#ret_group),*);
                        let mut __i = #count;
                        loop {
                            if __i == 0 {
                                break Ok(__ret);
                            }
                            __i -= 1;
                            #(#target_op)*
                            break Err(parser.new_custom_error(CustomError::Unmatched));
                        }?
                    }
                }
            }
            Self::MultiSelect(x) => {
                let count = x.len();
                let ret_group: Vec<_> = x
                    .iter()
                    .map(|_| {
                        quote! {
                            None
                        }
                    })
                    .collect();
                let target_op: Vec<_> = x.iter().enumerate().map(|(i, target)| {
                    let i = syn::Index::from(i);
                    quote! {
                        if __ret.#i.is_none() {
                            let r = parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                                Ok(#target)
                            });
                            if let Ok(x) = r {
                                __ret.#i = Some(x);
                                __changed = true;
                                continue;
                            }
                        }
                    }
                }).collect();
                quote! {
                    {
                        let mut __ret = (#(#ret_group),*);
                        let mut __i = #count;
                        let mut __changed = false;
                        loop {
                            if __i == 0 {
                                break Ok(__ret);
                            }
                            __i -= 1;
                            #(#target_op)*
                            if !__changed {
                                break Err(parser.new_custom_error(CustomError::Unmatched));
                            }
                        }?
                    }
                }
            }
            Self::Branch(x) => {
                let target_op: Vec<_> = x.iter().map(|target| quote! {
                    let __r = parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                        Ok(#target)
                    });
                    if let Ok(x) = __r {
                        break Ok(x);
                    }
                }).collect();
                quote! {
                    {
                        loop {
                            #(#target_op)*
                            break Err(parser.new_custom_error(CustomError::Unmatched));
                        }?
                    }
                }
            }
            Self::RepeatTimes(x, range) => {
                let min = *range.start();
                let max = *range.end();
                let initial_len = max.min(16);
                quote! {
                    {
                        let mut __ret = Vec::with_capacity(#initial_len);
                        while __ret.len() < #max {
                            let r = parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                                Ok(#x)
                            });
                            if let Ok(x) = r {
                                __ret.push(x);
                            } else {
                                break;
                            }
                        }
                        if __ret.len() >= #min {
                            Ok(__ret)
                        } else {
                            Err(parser.new_custom_error(CustomError::Unmatched))
                        }?
                    }
                }
            }
            Self::Optional(x) => {
                quote! {
                    {
                        let r = parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                            Ok(#x)
                        });
                        if let Ok(x) = r {
                            Some(x)
                        } else {
                            None
                        }
                    }
                }
            }
            Self::RepeatCommaSep(x) => {
                quote! {
                    {
                        let mut __ret = vec![];
                        loop {
                            __ret.push(#x);
                            let __r = parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                                parser.expect_comma().map_err(|x| x.into())
                            });
                            if !__r.is_ok() {
                                break __ret;
                            }
                        }
                    }
                }
            }
            Self::MustMatchOnce(x) => {
                let target_op: Vec<_> = x
                    .iter()
                    .map(|target| {
                        quote! {
                            {
                                let ret = (#target);
                                if ret.iter().next().is_some() {
                                    __matched = true;
                                }
                                ret
                            }
                        }
                    })
                    .collect();
                quote! {
                    (
                        let mut __matched = false;
                        let ret = (#(#target_op),*);
                        if __matched {
                            Err(parser.new_custom_error(CustomError::Unmatched))
                        } else {
                            Ok(ret)
                        }?
                    )
                }
            }
            Self::MatchFn(name, x) => {
                quote! {
                    {
                        parser.expect_function_matching(#name)?;
                        parser.parse_nested_block(|parser| {
                            Ok(#x)
                        })?
                    }
                }
            }
            Self::MatchIdent(s) => {
                quote! {
                    parser.expect_ident_matching(#s)?
                }
            }
            Self::MatchDelim(c) => match c {
                ',' => quote! {
                    parser.expect_comma()?
                },
                x => quote! {
                    parser.expect_delim(#x)?
                },
            },
            Self::Call(f) => {
                quote! {
                    (#f(parser, properties, st)?)
                }
            }
            Self::FilterFn(x, f) => {
                quote! {
                    (#f(#x))
                }
            }
            Self::FilterClosure(x, f) => {
                quote! {
                    {
                        let __r = (#x);
                        let __f = #f;
                        __f(__r)
                    }
                }
            }
            Self::ResultFilterClosure(x, f) => {
                quote! {
                    {
                        let __r = (#x);
                        let __f = #f;
                        __f(__r, parser)?
                    }
                }
            }
            Self::Convert(x, expr) => {
                quote! {
                    {
                        #x;
                        #expr
                    }
                }
            }
            Self::Assign(prop_name, x) => {
                quote! {
                    {
                        let __p = PropertyMeta::Normal {
                            property: Property::#prop_name((#x).into()),
                        };
                        properties.push(__p);
                        ()
                    }
                }
            }
            Self::MultiAssign(prop_names, x) => {
                let target_op: Vec<_> = prop_names
                    .iter()
                    .enumerate()
                    .map(|(i, prop_name)| {
                        let i = syn::Index::from(i);
                        quote! {
                            let __p = PropertyMeta::Normal {
                                property: Property::#prop_name(__r.#i.into()),
                            };
                            properties.push(__p);
                        }
                    })
                    .collect();
                quote! {
                    {
                        let __r = #x;
                        #(#target_op)*
                        ()
                    }
                }
            }
        };
        tokens.append_all(t);
    }
}

impl Parse for StyleSyntaxItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let fn_name: Ident;
        let name: Option<String>;
        let ret_ty: Option<Path>;
        let mut is_property = false;
        if lookahead.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            fn_name = input.parse()?;
            input.parse::<Token![:]>()?;
            ret_ty = Some(input.parse()?);
            input.parse::<Token![>]>()?;
            name = None;
        } else {
            fn_name = input.parse()?;
            name = Some(fn_name.to_string().replace('_', "-"));
            ret_ty = None;
            is_property = true;
        }

        input.parse::<Token![:]>()?;
        // input.parse::<Token![:]>()?;
        let value = input.parse()?;

        let mut ret = Self {
            name,
            fn_name,
            ret_ty,
            value,
            related_prop_names: vec![],
            is_property,
        };
        let mut related_prop_names = vec![];
        ret.value.collect_prop_names(&mut related_prop_names);
        ret.related_prop_names = related_prop_names;
        // if is_property {
        //     related_prop_map.insert(name.unwrap(), ret.related_prop_names.clone());
        // }
        Ok(ret)
    }
}

impl Parse for StyleSyntaxValueItem {
    fn parse(input: ParseStream) -> Result<Self> {
        Self::parse_sub(input)
    }
}

impl ToTokens for StyleSyntaxValueItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generate(tokens)
    }
}

impl ToTokens for StyleSyntaxItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            fn_name,
            value,
            ret_ty,
            ..
        } = self;
        let ret_map = match ret_ty {
            Some(_) => quote! { __ret },
            None => quote! { __ret.map(|_| ()) },
        };
        let ret_ty = match ret_ty {
            Some(x) => quote! { #x },
            None => quote! { () },
        };
        tokens.append_all(quote! {
            #[inline(never)]
            pub(crate) fn #fn_name<'a, 't: 'a, 'i: 't>(
                parser: &'a mut Parser<'i, 't>,
                properties: &mut Vec<PropertyMeta>,
                st: &mut ParseState,
            ) -> Result<#ret_ty, ParseError<'i, CustomError>> {
                let __ori_len = properties.len();
                let __ret = parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                    Ok(#value)
                });
                if __ret.is_err() {
                    properties.truncate(__ori_len);
                }
                #ret_map
            }
        });
    }
}

#[derive(Clone)]
struct StyleSyntaxDefinition {
    trait_name: Path,
    items: Punctuated<StyleSyntaxItem, Token![;]>,
}

impl Parse for StyleSyntaxDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);
        let input = content;
        let items = input.parse_terminated(StyleSyntaxItem::parse)?;
        Ok(Self { trait_name, items })
    }
}

impl ToTokens for StyleSyntaxDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { trait_name, items } = self;

        // the parser functions
        let item_fn_list: Vec<_> = items.iter().map(|x| quote! { #x }).collect();
        tokens.append_all(quote! {
            #(#item_fn_list)*
        });

        // the value parser mapping
        let map: Vec<_> = items
            .iter()
            .map(|item| {
                if let Some(name) = &item.name {
                    let fn_name = &item.fn_name;
                    let related_prop_names = &item.related_prop_names;
                    let global_value_parser = quote! {
                        let ident = parser.expect_ident()?.clone();
                        let ident: &str = &ident;
                        #(
                            properties.push(PropertyMeta::Normal {
                                property: Property::#related_prop_names(match ident {
                                    "initial" => #trait_name::initial(),
                                    "inherit" => #trait_name::inherit(),
                                    "unset" => #trait_name::unset(),
                                    _ => Err(parser.new_custom_error(CustomError::Unmatched))?
                                }),
                            });
                        )*
                        Ok(false)
                    };
                    let handle_var_properties = {
                        let prop = if related_prop_names.len() == 1 {
                            quote! {
                                #trait_name::var(expr.clone())
                            }
                        } else {
                            quote! {
                                #trait_name::var_in_shorthand(name.into(), expr.clone())
                            }
                        };
                        quote! {
                                let mut end_with_important = false;
                                if expr.trim().ends_with("!important") {
                                    expr = expr.trim().trim_end_matches("!important").to_string();
                                    if expr.trim().ends_with("!important") {
                                        return Err(parser.new_custom_error(CustomError::Unmatched))?;
                                    }
                                    end_with_important = true;
                                }
                                if expr.len() == 0 {
                                    expr.push_str(" ");
                                }
                                if end_with_important {
                                    #(properties.push(PropertyMeta::Important {
                                        property: Property::#related_prop_names(#prop),
                                    });)*
                                } else {
                                    #(properties.push(PropertyMeta::Normal {
                                        property: Property::#related_prop_names(#prop),
                                    });)*
                                }
                        }
                    };
                    let var_parser = quote! {
                        let start_position = parser.position();
                        if let Some(end_position) = rule_end_position{
                            let expr = parser.slice(start_position..end_position).to_string();
                            if let Some(_) = expr.find("var(") {
                                while !parser.is_exhausted() {
                                    parser.next()?;
                                }
                                let mut expr = expr.trim_end_matches(&['\n', '}', ';']).to_string();
                                #handle_var_properties;
                                return Ok(true);
                            }
                        }
                        Err(parser.new_custom_error(CustomError::Unmatched))?
                    };
                    let value_parser = quote! {
                        #fn_name(parser, properties, st).map(|_| false)
                    };
                    quote! {
                        #name => {
                            parser.try_parse::<_, bool, ParseError<'i, CustomError>>(|parser| {
                                #global_value_parser
                            })
                            .or_else(|_: ParseError<'i, CustomError>| {
                                #var_parser
                            })
                            .or_else(|_: ParseError<'i, CustomError>| {
                                #value_parser
                            })
                        },
                    }
                } else {
                    quote! {}
                }
            })
            .collect();
        tokens.append_all(quote! {
            pub(crate) fn parse_property_value<'a, 't: 'a, 'i: 't>(
                parser: &'a mut Parser<'i, 't>,
                name: &'a str,
                properties: &'a mut Vec<PropertyMeta>,
                st: &'a mut ParseState,
                rule_end_position: Option<SourcePosition>
            ) -> Result<bool, ParseError<'i, CustomError>> {
                match name {
                    #(#map)*
                    _ => return Err(parser.new_custom_error(CustomError::UnsupportedProperty))
                }?;
                Ok(false)
            }
        });

        // the supported property list
        let mut supported_properties: Vec<_> = items.iter().filter(|x| x.name.is_some()).collect();
        supported_properties.sort_by(|a, b| {
            let a = a.name.as_ref().unwrap();
            let b = b.name.as_ref().unwrap();
            a.cmp(b)
        });
        let supported_property_count = supported_properties.len();
        let supported_property_names = supported_properties
            .iter()
            .map(|x| x.name.as_ref().unwrap());
        let mut style_doc = String::new();
        writeln!(&mut style_doc, "The supported CSS property names.\n").unwrap();
        writeln!(
            &mut style_doc,
            "This list is sorted, so it is safe to do binary search on it.\n"
        )
        .unwrap();
        writeln!(
            &mut style_doc,
            "Note that this is just a simple list of basic parsing rules.\n"
        )
        .unwrap();
        writeln!(&mut style_doc, "* Some properties in this list are shorthand properties that cannot be found in the [Property] enum.").unwrap();
        writeln!(
            &mut style_doc,
            "* Parsing rules of some properties are slightly different from the web standard."
        )
        .unwrap();
        writeln!(
            &mut style_doc,
            "\nSee the table below for more information about all supported properties.\n"
        )
        .unwrap();
        writeln!(
            &mut style_doc,
            "| Property Name | Related Property Variant | Major Value Options |"
        )
        .unwrap();
        writeln!(&mut style_doc, "| ---- | ---- | ---- |").unwrap();
        let table_list_a = supported_properties
            .iter()
            .filter(|x| !x.name.as_ref().unwrap().starts_with("-"));
        let table_list_b = supported_properties
            .iter()
            .filter(|x| x.name.as_ref().unwrap().starts_with("-"));
        for x in table_list_a.chain(table_list_b) {
            let name = x.name.as_ref().unwrap();
            let non_standard = name.starts_with("-");
            let name_col = if non_standard {
                format!("*`{}`*", name)
            } else {
                format!("`{}`", name)
            };
            let mut doc_col = String::new();
            let mut options_col = vec![];
            if let StyleSyntaxValueItem::Assign(variant, v) = &x.value {
                doc_col = format!("[Property::{}]", variant);
                if let StyleSyntaxValueItem::Branch(branches) = &**v {
                    for item in branches {
                        if let StyleSyntaxValueItem::Convert(v, _) = item {
                            if let StyleSyntaxValueItem::MatchIdent(s) = &**v {
                                options_col.push(format!("`{}`", s));
                            }
                        }
                    }
                }
            }
            options_col.sort();
            writeln!(
                &mut style_doc,
                "| {} | {} | {} |",
                name_col,
                doc_col,
                options_col.join("<br>")
            )
            .unwrap();
        }
        tokens.append_all(quote! {
            #[doc = #style_doc]
            pub const SUPPORTED_CSS_PROPERTY_NAMES: [&'static str; #supported_property_count] = [
                #(
                    #supported_property_names,
                )*
            ];
        });
    }
}

pub(crate) fn property_value_format(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut style_syntax: StyleSyntaxDefinition =
        parse_macro_input!(tokens as StyleSyntaxDefinition);
    let items: &mut Punctuated<StyleSyntaxItem, token::Semi> = &mut style_syntax.items;
    let mut related_prop_map: FxHashMap<String, Vec<Ident>> = FxHashMap::default();
    {
        for ssi in items.iter() {
            if ssi.is_property {
                related_prop_map.insert(ssi.name.clone().unwrap(), ssi.related_prop_names.clone());
            }
        }
    }
    for ssi in items.iter_mut() {
        let v = &mut ssi.value;
        let mut related_prop_names: Vec<Ident> = vec![];
        if let StyleSyntaxValueItem::MultiSelect(x) = v {
            for m in x.iter_mut() {
                if let StyleSyntaxValueItem::Call(p) = m {
                    let ident = p.segments.first().unwrap().ident.clone().to_string();
                    let ident = related_prop_map.get(&ident.replace('_', "-"));
                    ident.unwrap().iter().for_each(|i| {
                        related_prop_names.push(i.clone());
                    });
                }
            }
        }
        if !related_prop_names.is_empty() {
            ssi.related_prop_names = related_prop_names;
        }
    }
    let ret = quote! {
        #style_syntax
    };
    // eprintln!("{}", proc_macro::TokenStream::from(ret.clone()));
    proc_macro::TokenStream::from(ret)
}
