#![doc(hidden)]
#![cfg(feature = "ffi")]

use alloc::{
    boxed::Box,
    ffi::CString,
    string::{String, ToString},
    vec::Vec,
};
use bit_set::BitSet;
use core::{
    ffi::{c_char, CStr},
    ptr::null,
};
use hashbrown::HashMap;

use crate::parser::parse_color_to_rgba;
use crate::property::{Property, PropertyMeta};
use crate::sheet::borrow::{InlineRule, Selector};
use crate::typing::ImportantBitSet;

use crate::parser::property_value::var::{
    CustomPropertyContext, CustomPropertyGetter, CustomPropertySetter,
};

use super::{group::drop_css_extension, *};
use parser::Warning;
use sheet::borrow::{Array, StyleSheet};
use sheet::str_store::StrRef;

#[cfg(feature = "deserialize")]
use sheet::borrow::de_static_ref_zero_copy_env;

#[repr(C)]
pub struct StyleSheetResourcePtr {
    ptr: *mut (),
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_new() -> StyleSheetResourcePtr {
    let res = Box::into_raw(Box::new(group::StyleSheetResource::new()));
    StyleSheetResourcePtr {
        ptr: res as *mut (),
    }
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_free(this: &mut StyleSheetResourcePtr) {
    drop(Box::from_raw(this.ptr as *mut StyleSheetResource));
    this.ptr = core::ptr::null_mut();
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_add_tag_name_prefix(
    this: &mut StyleSheetResourcePtr,
    path: *const c_char,
    prefix: *const c_char,
) {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let prefix = CStr::from_ptr(prefix).to_string_lossy();
    res.add_tag_name_prefix(&path, &prefix);
}
/// # Safety
///
#[cfg(all(feature = "serialize", feature = "serialize_json"))]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_serialize_json(
    this: &mut StyleSheetResourcePtr,
    path: *const c_char,
    ret_buffer_len: &mut usize,
) -> *mut u8 {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let serial = res.serialize_json(&path).unwrap_or_default();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_str());
    ret as *mut u8
}
/// # Safety
///
#[cfg(feature = "serialize")]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_serialize_bincode(
    this: &mut StyleSheetResourcePtr,
    path: *const c_char,
    ret_buffer_len: &mut usize,
) -> *mut u8 {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let serial = res.serialize_bincode(&path).unwrap_or_default();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_slice());
    ret as *mut u8
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_add_source(
    this: &mut StyleSheetResourcePtr,
    path: *const c_char,
    source: *const c_char,
    warnings: *mut *mut Array<Warning>,
) {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let source = CStr::from_ptr(source).to_string_lossy();
    let w = res.add_source(&path, &source);
    if !warnings.is_null() {
        *warnings = Box::into_raw(Box::new(w.into()));
    }
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_add_source_with_hooks(
    this: &mut StyleSheetResourcePtr,
    hooks: parser::hooks::CParserHooks,
    path: *const c_char,
    source: *const c_char,
    warnings: *mut *mut Array<Warning>,
) {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let source = CStr::from_ptr(source).to_string_lossy();
    let w = res.add_source_with_hooks(&path, &source, Some(Box::new(hooks)));
    if !warnings.is_null() {
        *warnings = Box::into_raw(Box::new(w.into()));
    }
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_add_bincode(
    this: &mut StyleSheetResourcePtr,
    path: *const c_char,
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(*mut ())>,
    drop_args: *mut (),
    warnings: *mut *mut Array<Warning>,
) {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let bincode: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    let path = CStr::from_ptr(path).to_string_lossy();
    let w = res.add_bincode_zero_copy(&path, bincode, move || {
        if let Some(drop_fn) = drop_fn {
            drop_fn(drop_args);
        }
    });
    if !warnings.is_null() {
        *warnings = Box::into_raw(Box::new(w.into()));
    }
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_direct_dependencies(
    this: &mut StyleSheetResourcePtr,
    path: *const c_char,
) -> *mut Array<StrRef> {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = res.direct_dependencies(&path);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    Box::into_raw(Box::new(deps.into()))
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_resource_generate_import_index(
    this: &mut StyleSheetResourcePtr,
) -> StyleSheetImportIndexPtr {
    let res = &mut *(this.ptr as *mut StyleSheetResource);
    let ii = Box::into_raw(Box::new(res.generate_import_indexes()));
    let style_sheet_map: Box<StyleSheetMap> = Box::default();
    StyleSheetImportIndexPtr {
        ptr: ii as *mut (),
        map: Box::into_raw(style_sheet_map) as *mut _,
    }
}

type StyleSheetMap = HashMap<String, StyleSheet>;

#[repr(C)]
pub struct StyleSheetImportIndexPtr {
    ptr: *mut (),
    map: *mut (),
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_new() -> StyleSheetImportIndexPtr {
    let ii = Box::into_raw(Box::new(StyleSheetImportIndex::new()));
    let style_sheet_map: Box<StyleSheetMap> = Box::default();
    StyleSheetImportIndexPtr {
        ptr: ii as *mut (),
        map: Box::into_raw(style_sheet_map) as *mut _,
    }
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_free(this: &mut StyleSheetImportIndexPtr) {
    drop(Box::from_raw(this.ptr as *mut StyleSheetImportIndex));
    this.ptr = core::ptr::null_mut();
    drop(Box::from_raw(this.map as *mut StyleSheetMap));
    this.map = core::ptr::null_mut();
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_query_and_mark_dependencies(
    this: &mut StyleSheetImportIndexPtr,
    path: *const c_char,
) -> *mut Array<StrRef> {
    let ii = &mut *(this.ptr as *mut StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = ii.query_and_mark_dependencies(&path);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    Box::into_raw(Box::new(deps.into()))
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_list_dependencies(
    this: &mut StyleSheetImportIndexPtr,
    path: *const c_char,
) -> *mut Array<StrRef> {
    let ii = &mut *(this.ptr as *mut StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = ii.list_dependencies(&path, true);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    Box::into_raw(Box::new(deps.into()))
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_list_dependency(
    this: &mut StyleSheetImportIndexPtr,
    path: *const c_char,
) -> *mut Array<StrRef> {
    if path.is_null() {
        panic!("style_sheet_import_index_list_dependency: path is null!")
    }
    let ii = &mut *(this.ptr as *mut StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = ii.list_dependencies(&path, false);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    Box::into_raw(Box::new(deps.into()))
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_add_bincode(
    this: &mut StyleSheetImportIndexPtr,
    path: *const c_char,
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(*mut ())>,
    drop_args: *mut (),
    warnings: *mut *mut Array<Warning>,
) {
    use float_pigment_consistent_bincode::Options;
    use parser::WarningKind;
    let path = CStr::from_ptr(path).to_string_lossy();
    let path: &str = &path;
    let sheet = de_static_ref_zero_copy_env(
        core::slice::from_raw_parts_mut(buffer_ptr, buffer_len),
        |s| {
            let s: Result<StyleSheet, _> = float_pigment_consistent_bincode::DefaultOptions::new()
                .allow_trailing_bytes()
                .deserialize(s);
            match s {
                Ok(ss) => ss,
                Err(err) => {
                    let w = vec![Warning {
                        kind: WarningKind::DeserializationFailed,
                        message: format!(
                            "failed to deserialize bincode formatted style sheet: {}",
                            err
                        )
                        .into(),
                        start_line: 0,
                        start_col: 0,
                        end_line: 0,
                        end_col: 0,
                    }];
                    if !warnings.is_null() {
                        *warnings = Box::into_raw(Box::new(w.into()));
                    }
                    StyleSheet::from_sheet(&sheet::CompiledStyleSheet::new())
                }
            }
        },
        move || {
            if let Some(drop_fn) = drop_fn {
                drop_fn(drop_args);
            }
        },
    );
    let path = drop_css_extension(path).into();
    let map = &mut *(this.map as *mut StyleSheetMap);
    map.insert(path, sheet);
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_remove_bincode(
    this: &mut StyleSheetImportIndexPtr,
    path: *const c_char,
) {
    let path = CStr::from_ptr(path).to_string_lossy();
    let path: &str = &path;
    let path = drop_css_extension(path);
    let map = &mut *(this.map as *mut StyleSheetMap);
    map.remove(path);
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_get_style_sheet(
    this: &mut StyleSheetImportIndexPtr,
    path: *const StrRef,
) -> *mut StyleSheet {
    let path = (*path).as_str();
    let path = drop_css_extension(path);
    let map = &mut *(this.map as *mut StyleSheetMap);
    match map.get_mut(path) {
        None => core::ptr::null_mut(),
        Some(x) => x as *mut _,
    }
}
/// # Safety
///
#[cfg(all(feature = "serialize", feature = "serialize_json"))]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_serialize_json(
    this: &mut StyleSheetImportIndexPtr,
    ret_buffer_len: &mut usize,
) -> *mut u8 {
    let ii = &mut *(this.ptr as *mut StyleSheetImportIndex);
    let serial = ii.serialize_json();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_str());
    ret as *mut u8
}
/// # Safety
///
#[cfg(feature = "serialize")]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_serialize_bincode(
    this: &mut StyleSheetImportIndexPtr,
    ret_buffer_len: &mut usize,
) -> *mut u8 {
    let ii = &mut *(this.ptr as *mut StyleSheetImportIndex);
    let serial = ii.serialize_bincode();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_slice());
    ret as *mut u8
}
/// # Safety
///
#[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_deserialize_json(
    json: *const c_char,
) -> StyleSheetImportIndexPtr {
    let json = CStr::from_ptr(json).to_string_lossy();
    let ii = StyleSheetImportIndex::deserialize_json(&json);
    let ii = Box::into_raw(Box::new(ii));
    StyleSheetImportIndexPtr {
        ptr: ii as *mut (),
        map: Box::into_raw(Box::new(StyleSheetMap::default())) as *mut _,
    }
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_deserialize_bincode(
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(*mut ())>,
    drop_args: *mut (),
) -> StyleSheetImportIndexPtr {
    let bincode: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    let ii = StyleSheetImportIndex::deserialize_bincode_zero_copy(bincode, move || {
        if let Some(drop_fn) = drop_fn {
            drop_fn(drop_args);
        }
    });
    let ii = Box::into_raw(Box::new(ii));
    let style_sheet_map: Box<StyleSheetMap> = Box::default();
    StyleSheetImportIndexPtr {
        ptr: ii as *mut (),
        map: Box::into_raw(style_sheet_map) as *mut _,
    }
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_import_index_merge_bincode(
    this: &mut StyleSheetImportIndexPtr,
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(*mut ())>,
    drop_args: *mut (),
) {
    let ii = &mut *(this.ptr as *mut StyleSheetImportIndex);
    let bincode: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    ii.merge_bincode_zero_copy(bincode, move || {
        if let Some(drop_fn) = drop_fn {
            drop_fn(drop_args);
        }
    });
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn buffer_free(buffer_ptr: *mut u8, buffer_len: usize) {
    let x: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    drop(Box::from_raw(x));
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn array_str_ref_free(x: *mut Array<StrRef>) {
    drop(Box::from_raw(x));
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn array_warning_free(warnings: *mut Array<parser::Warning>) {
    drop(Box::from_raw(warnings));
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn parse_inline_style(
    inline_style_text_ptr: *const c_char,
    warnings: *mut *mut Array<parser::Warning>,
) -> *mut InlineRule {
    let inline_style_text = CStr::from_ptr(inline_style_text_ptr).to_string_lossy();
    let (prop, w) =
        parser::parse_inline_style(&inline_style_text, parser::StyleParsingDebugMode::None);
    if !warnings.is_null() {
        *warnings = Box::into_raw(Box::new(w.into()));
    }
    let mut important = BitSet::new();
    let mut bs_empty = true;
    let properties = prop
        .into_iter()
        .enumerate()
        .map(|(index, x)| match x {
            PropertyMeta::Normal { property } => property,
            PropertyMeta::Important { property } => {
                important.insert(index);
                bs_empty = false;
                property
            }
            PropertyMeta::DebugGroup { .. } => Property::Unknown,
        })
        .collect();
    let important = if bs_empty {
        ImportantBitSet::None
    } else {
        ImportantBitSet::Array(important.into_bit_vec().to_bytes().into())
    };
    let inline_rule = InlineRule::new(properties, important);
    Box::into_raw(Box::new(inline_rule))
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn inline_style_free(inline_rule: *mut InlineRule) {
    drop(Box::from_raw(inline_rule));
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn parse_style_sheet_from_string(
    style_text_ptr: *const c_char,
) -> *mut StyleSheet {
    let style_text = CStr::from_ptr(style_text_ptr).to_string_lossy();
    let (compiled_style_sheet, _) = parser::parse_style_sheet("string", &style_text);
    let style_sheet = StyleSheet::from_sheet(&compiled_style_sheet);
    Box::into_raw(Box::new(style_sheet))
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn parse_selector_from_string(
    selector_text_ptr: *const c_char,
) -> *mut Selector {
    let selector_text = CStr::from_ptr(selector_text_ptr).to_string_lossy();
    let selector = Selector::from_string(&selector_text);
    Box::into_raw(Box::new(selector))
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn selector_free(selector: *mut Selector) {
    drop(Box::from_raw(selector));
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn style_sheet_free(style_sheet: *mut StyleSheet) {
    drop(Box::from_raw(style_sheet));
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[no_mangle]
pub unsafe extern "C" fn style_sheet_bincode_version(
    buffer_ptr: *mut u8,
    buffer_len: usize,
) -> *mut StrRef {
    use float_pigment_consistent_bincode::Options;
    let sheet = de_static_ref_zero_copy_env(
        core::slice::from_raw_parts_mut(buffer_ptr, buffer_len),
        |s| {
            let s: Result<StyleSheet, _> = float_pigment_consistent_bincode::DefaultOptions::new()
                .allow_trailing_bytes()
                .deserialize(s);
            match s {
                Ok(ss) => ss,
                Err(err) => {
                    let mut ss = StyleSheet::from_sheet(&sheet::CompiledStyleSheet::new());
                    if let StyleSheet::V1(ssv) = &mut ss {
                        ssv.version = Box::new(
                            format!(
                                "Failed to deserialize bincode formatted style sheet: {}",
                                err
                            )
                            .into(),
                        );
                    }
                    ss
                }
            }
        },
        move || {},
    );
    let version = match sheet {
        StyleSheet::V1(v1) => v1.version,
        _ => Box::new("unknown version".into()),
    };
    Box::into_raw(version)
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn css_parser_version() -> *mut StrRef {
    let version = env!("CARGO_PKG_VERSION").to_string().into();
    Box::into_raw(Box::new(version))
}

#[repr(C)]
#[derive(Debug)]
pub struct ColorValue {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn parse_color_from_string(source: *const c_char) -> ColorValue {
    let source = CStr::from_ptr(source).to_string_lossy();
    let ret = parse_color_to_rgba(&source);
    ColorValue {
        red: ret.0,
        green: ret.1,
        blue: ret.2,
        alpha: ret.3,
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn substitute_variable(
    expr_ptr: *const c_char,
    map: *mut (),
    getter: CustomPropertyGetter,
    setter: CustomPropertySetter,
) -> *const c_char {
    let expr = CStr::from_ptr(expr_ptr).to_string_lossy();
    let context = CustomPropertyContext::create(map, getter, setter);
    if let Some(ret) = parser::property_value::var::substitute_variable(&expr, &context) {
        CString::new(ret).expect("CString new error").into_raw()
    } else {
        null()
    }
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn str_free(ptr: *const c_char) {
    drop(CString::from_raw(ptr as *mut _));
}
