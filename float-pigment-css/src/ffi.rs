#![doc(hidden)]
#![cfg(feature = "ffi")]

use alloc::{
    boxed::Box,
    ffi::CString,
    string::{String, ToString},
    vec::Vec,
};
use bit_set::BitSet;
use core::ffi::{c_char, CStr};
use core::ptr::{null, null_mut};
use hashbrown::HashMap;

use crate::parser::parse_color_to_rgba;
use crate::property::{Property, PropertyMeta};
use crate::sheet::borrow::{InlineRule, Selector};
use crate::typing::ImportantBitSet;

use crate::group;
use crate::parser;
use crate::parser::property_value::var::{
    CustomPropertyContext, CustomPropertyGetter, CustomPropertySetter,
};
use crate::sheet;
use group::drop_css_extension;
use parser::Warning;
use sheet::borrow::{Array, StyleSheet};
use sheet::str_store::StrRef;

#[cfg(feature = "deserialize")]
use sheet::borrow::de_static_ref_zero_copy_env;

#[macro_export]
macro_rules! check_null {
    (($($arg:expr),+ $(,)?), $default: expr) => {
        if $( $arg.is_null() )||+ {
            return FfiResult::error(FfiErrorCode::NullPointer, $default);
        }
    };
}

#[macro_export]
macro_rules! raw_ptr_as_mut_ref {
    ($from:expr, $type:ty) => {
        &mut *($from as *mut $type)
    };
}

pub type RawMutPtr = *mut ();

pub type NullPtr = *const ();

#[repr(C)]
pub enum FfiErrorCode {
    None,
    NullPointer,
    InvalidPath,
    Unknown,
}

#[repr(C)]
pub struct FfiResult<T> {
    pub value: T,
    pub err: FfiErrorCode,
}

impl<T> FfiResult<T> {
    pub fn ok(value: T) -> Self {
        Self {
            err: FfiErrorCode::None,
            value,
        }
    }
    pub fn error(err: FfiErrorCode, default: T) -> Self {
        Self {
            err,
            value: default,
        }
    }
}

/// # Safety
///
#[export_name = "FPStyleSheetResourceNew"]
pub unsafe extern "C" fn style_sheet_resource_new() -> FfiResult<RawMutPtr> {
    FfiResult::ok(Box::into_raw(Box::new(group::StyleSheetResource::new())) as RawMutPtr)
}
/// # Safety
///
#[export_name = "FPStyleSheetResourceFree"]
pub unsafe extern "C" fn style_sheet_resource_free(this: RawMutPtr) -> FfiResult<NullPtr> {
    check_null!((this), null());
    drop(Box::from_raw(this as *mut group::StyleSheetResource));
    FfiResult::ok(null())
}
/// # Safety
///
#[export_name = "FPStyleSheetResourceAddTagNamePrefix"]
pub unsafe extern "C" fn style_sheet_resource_add_tag_name_prefix(
    this: RawMutPtr,
    path: *const c_char,
    prefix: *const c_char,
) -> FfiResult<NullPtr> {
    check_null!((this, path, prefix), null());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let prefix = CStr::from_ptr(prefix).to_string_lossy();
    res.add_tag_name_prefix(&path, &prefix);
    FfiResult::ok(null())
}
/// # Safety
///
#[cfg(all(feature = "serialize", feature = "serialize_json"))]
#[export_name = "FPStyleSheetResourceSerializeJson"]
pub unsafe extern "C" fn style_sheet_resource_serialize_json(
    this: RawMutPtr,
    path: *const c_char,
    ret_buffer_len: &mut usize,
) -> FfiResult<*mut u8> {
    check_null!((this, path), null_mut());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let serial = res.serialize_json(&path).unwrap_or_default();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_str());
    FfiResult::ok(ret as *mut u8)
}
/// # Safety
///
#[cfg(feature = "serialize")]
#[export_name = "FPStyleSheetResourceSerializeBincode"]
pub unsafe extern "C" fn style_sheet_resource_serialize_bincode(
    this: RawMutPtr,
    path: *const c_char,
    ret_buffer_len: &mut usize,
) -> FfiResult<*mut u8> {
    check_null!((this, path), null_mut());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let serial = res.serialize_bincode(&path).unwrap_or_default();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_slice());
    FfiResult::ok(ret as *mut u8)
}
/// # Safety
///
#[export_name = "FPStyleSheetResourceAddSource"]
pub unsafe extern "C" fn style_sheet_resource_add_source(
    this: RawMutPtr,
    path: *const c_char,
    source: *const c_char,
    warnings: *mut *mut Array<Warning>,
) -> FfiResult<NullPtr> {
    check_null!((this, path, source), null());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let source = CStr::from_ptr(source).to_string_lossy();
    let w = res.add_source(&path, &source);
    if !warnings.is_null() {
        *warnings = Box::into_raw(Box::new(w.into()));
    }
    FfiResult::ok(null())
}
/// # Safety
///
#[export_name = "FPStyleSheetResourceAddSourceWithHooks"]
pub unsafe extern "C" fn style_sheet_resource_add_source_with_hooks(
    this: RawMutPtr,
    hooks: parser::hooks::CParserHooks,
    path: *const c_char,
    source: *const c_char,
    warnings: *mut *mut Array<Warning>,
) -> FfiResult<NullPtr> {
    check_null!((this, path, source), null());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let source = CStr::from_ptr(source).to_string_lossy();
    let w = res.add_source_with_hooks(&path, &source, Some(Box::new(hooks)));
    if !warnings.is_null() {
        *warnings = Box::into_raw(Box::new(w.into()));
    }
    FfiResult::ok(null())
}

/// # Safety
///
#[cfg(feature = "deserialize")]
#[export_name = "FPStyleSheetResourceAddBincode"]
pub unsafe extern "C" fn style_sheet_resource_add_bincode(
    this: RawMutPtr,
    path: *const c_char,
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(RawMutPtr)>,
    drop_args: RawMutPtr,
    warnings: *mut *mut Array<Warning>,
) -> FfiResult<NullPtr> {
    check_null!((this, path, buffer_ptr), null());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
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
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPStyleSheetResourceDirectDependencies"]
pub unsafe extern "C" fn style_sheet_resource_direct_dependencies(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!((this, path), null_mut());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = res.direct_dependencies(&path);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}

/// # Safety
///
#[export_name = "FPStyleSheetResourceGenerateImportIndex"]
pub unsafe extern "C" fn style_sheet_resource_generate_import_index(
    this: RawMutPtr,
) -> FfiResult<RawMutPtr> {
    check_null!((this), null_mut());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    FfiResult::ok(
        StyleSheetImportIndex {
            inner: res.generate_import_indexes(),
            map: StyleSheetMap::default(),
        }
        .into_raw(),
    )
}

type StyleSheetMap = HashMap<String, StyleSheet>;

struct StyleSheetImportIndex {
    inner: group::StyleSheetImportIndex,
    map: StyleSheetMap,
}

impl StyleSheetImportIndex {
    fn into_raw(self) -> RawMutPtr {
        Box::into_raw(Box::new(self)) as RawMutPtr
    }
}

/// # Safety
///
#[export_name = "FPStyleSheetImportIndexNew"]
pub unsafe extern "C" fn style_sheet_import_index_new() -> FfiResult<RawMutPtr> {
    FfiResult::ok(
        StyleSheetImportIndex {
            inner: group::StyleSheetImportIndex::new(),
            map: StyleSheetMap::default(),
        }
        .into_raw(),
    )
}

/// # Safety
///
#[export_name = "FPStyleSheetImportIndexFree"]
pub unsafe extern "C" fn style_sheet_import_index_free(this: RawMutPtr) -> FfiResult<NullPtr> {
    check_null!((this), null());
    drop(Box::from_raw(this as *mut StyleSheetImportIndex));
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPStyleSheetImportIndexQueryAndMarkDependencies"]
pub unsafe extern "C" fn style_sheet_import_index_query_and_mark_dependencies(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!((this, path), null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = style_sheet_import_index
        .inner
        .query_and_mark_dependencies(&path);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}

/// # Safety
///
#[export_name = "FPStyleSheetImportIndexListDependencies"]
pub unsafe extern "C" fn style_sheet_import_index_list_dependencies(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!((this, path), null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = style_sheet_import_index
        .inner
        .list_dependencies(&path, true);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}

/// # Safety
///
#[export_name = "FPStyleSheetImportIndexListDependency"]
pub unsafe extern "C" fn style_sheet_import_index_list_dependency(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!((this, path), null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = style_sheet_import_index
        .inner
        .list_dependencies(&path, false);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[export_name = "FPStyleSheetImportIndexAddBincode"]
pub unsafe extern "C" fn style_sheet_import_index_add_bincode(
    this: RawMutPtr,
    path: *const c_char,
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(RawMutPtr)>,
    drop_args: RawMutPtr,
    warnings: *mut *mut Array<Warning>,
) -> FfiResult<NullPtr> {
    use float_pigment_consistent_bincode::Options;
    use parser::WarningKind;
    check_null!((this, path, buffer_ptr), null());
    let path = CStr::from_ptr(path).to_string_lossy();
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
    let path = drop_css_extension(&path).into();
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    style_sheet_import_index.map.insert(path, sheet);
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPStyleSheetImportIndexRemoveBincode"]
pub unsafe extern "C" fn style_sheet_import_index_remove_bincode(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<NullPtr> {
    check_null!((this, path), null());
    let path = CStr::from_ptr(path).to_string_lossy();
    let path = drop_css_extension(&path);
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    style_sheet_import_index.map.remove(path);
    FfiResult::ok(null())
}
/// # Safety
///
#[export_name = "FPStyleSheetImportIndexGetStyleSheet"]
pub unsafe extern "C" fn style_sheet_import_index_get_style_sheet(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut StyleSheet> {
    check_null!((this, path), null_mut());
    let path = CStr::from_ptr(path).to_string_lossy();
    let path = drop_css_extension(&path);
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    match style_sheet_import_index.map.get_mut(path) {
        None => FfiResult::error(FfiErrorCode::InvalidPath, null_mut()),
        Some(x) => FfiResult::ok(x as *mut StyleSheet),
    }
}
/// # Safety
///
#[cfg(all(feature = "serialize", feature = "serialize_json"))]
#[export_name = "FPStyleSheetImportIndexSerializeJson"]
pub unsafe extern "C" fn style_sheet_import_index_serialize_json(
    this: RawMutPtr,
    ret_buffer_len: &mut usize,
) -> FfiResult<*mut u8> {
    check_null!((this), null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let serial = style_sheet_import_index.inner.serialize_json();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_str());
    FfiResult::ok(ret as *mut u8)
}
/// # Safety
///
#[cfg(feature = "serialize")]
#[export_name = "FPStyleSheetImportIndexSerializeBincode"]
pub unsafe extern "C" fn style_sheet_import_index_serialize_bincode(
    this: RawMutPtr,
    ret_buffer_len: &mut usize,
) -> FfiResult<*mut u8> {
    check_null!((this), null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let serial = style_sheet_import_index.inner.serialize_bincode();
    *ret_buffer_len = serial.len();
    let ret = Box::into_raw(serial.into_boxed_slice());
    FfiResult::ok(ret as *mut u8)
}
/// # Safety
///
#[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
#[export_name = "FPStyleSheetImportIndexDeserializeJson"]
pub unsafe extern "C" fn style_sheet_import_index_deserialize_json(
    json: *const c_char,
) -> FfiResult<RawMutPtr> {
    check_null!((json), null_mut());
    let json = CStr::from_ptr(json).to_string_lossy();
    FfiResult::ok(
        StyleSheetImportIndex {
            inner: group::StyleSheetImportIndex::deserialize_json(&json),
            map: StyleSheetMap::default(),
        }
        .into_raw(),
    )
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[export_name = "FPStyleSheetImportIndexDeserializeBincode"]
pub unsafe extern "C" fn style_sheet_import_index_deserialize_bincode(
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(RawMutPtr)>,
    drop_args: RawMutPtr,
) -> FfiResult<RawMutPtr> {
    check_null!((buffer_ptr), null_mut());
    let bincode: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    FfiResult::ok(
        StyleSheetImportIndex {
            inner: group::StyleSheetImportIndex::deserialize_bincode_zero_copy(
                bincode,
                move || {
                    if let Some(drop_fn) = drop_fn {
                        drop_fn(drop_args);
                    }
                },
            ),
            map: StyleSheetMap::default(),
        }
        .into_raw(),
    )
}
/// # Safety
///
#[cfg(feature = "deserialize")]
#[export_name = "FPStyleSheetImportIndexMergeBincode"]
pub unsafe extern "C" fn style_sheet_import_index_merge_bincode(
    this: RawMutPtr,
    buffer_ptr: *mut u8,
    buffer_len: usize,
    drop_fn: Option<unsafe extern "C" fn(*mut ())>,
    drop_args: *mut (),
) -> FfiResult<NullPtr> {
    check_null!((this, buffer_ptr), null());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let bincode: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    style_sheet_import_index
        .inner
        .merge_bincode_zero_copy(bincode, move || {
            if let Some(drop_fn) = drop_fn {
                drop_fn(drop_args);
            }
        });
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPBufferFree"]
pub unsafe extern "C" fn buffer_free(buffer_ptr: *mut u8, buffer_len: usize) -> FfiResult<NullPtr> {
    check_null!((buffer_ptr), null());
    let x: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    drop(Box::from_raw(x));
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPArrayStrRefFree"]
pub unsafe extern "C" fn array_str_ref_free(x: *mut Array<StrRef>) -> FfiResult<NullPtr> {
    check_null!((x), null());
    drop(Box::from_raw(x));
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPArrayWarningFree"]
pub unsafe extern "C" fn array_warning_free(
    warnings: *mut Array<parser::Warning>,
) -> FfiResult<NullPtr> {
    check_null!((warnings), null());
    drop(Box::from_raw(warnings));
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPParseInlineStyle"]
pub unsafe extern "C" fn parse_inline_style(
    inline_style_text_ptr: *const c_char,
    warnings: *mut *mut Array<parser::Warning>,
) -> FfiResult<*mut InlineRule> {
    check_null!((inline_style_text_ptr), null_mut());
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
    FfiResult::ok(Box::into_raw(Box::new(inline_rule)))
}
/// # Safety
///
#[export_name = "FPInlineStyleFree"]
pub unsafe extern "C" fn inline_style_free(inline_rule: *mut InlineRule) -> FfiResult<NullPtr> {
    check_null!((inline_rule), null());
    drop(Box::from_raw(inline_rule));
    FfiResult::ok(null())
}
/// # Safety
///
#[export_name = "FPParseStyleSheetFromString"]
pub unsafe extern "C" fn parse_style_sheet_from_string(
    style_text_ptr: *const c_char,
) -> FfiResult<*mut StyleSheet> {
    check_null!((style_text_ptr), null_mut());
    let style_text = CStr::from_ptr(style_text_ptr).to_string_lossy();
    let (compiled_style_sheet, _) = parser::parse_style_sheet("string", &style_text);
    let style_sheet = StyleSheet::from_sheet(&compiled_style_sheet);
    FfiResult::ok(Box::into_raw(Box::new(style_sheet)))
}
/// # Safety
///
#[export_name = "FPParseSelectorFromString"]
pub unsafe extern "C" fn parse_selector_from_string(
    selector_text_ptr: *const c_char,
) -> FfiResult<*mut Selector> {
    check_null!((selector_text_ptr), null_mut());
    let selector_text = CStr::from_ptr(selector_text_ptr).to_string_lossy();
    let selector = Selector::from_string(&selector_text);
    FfiResult::ok(Box::into_raw(Box::new(selector)))
}
/// # Safety
///
#[export_name = "FPSelectorFree"]
pub unsafe extern "C" fn selector_free(selector: *mut Selector) -> FfiResult<NullPtr> {
    check_null!((selector), null());
    drop(Box::from_raw(selector));
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPStyleSheetFree"]
pub unsafe extern "C" fn style_sheet_free(style_sheet: *mut StyleSheet) -> FfiResult<NullPtr> {
    check_null!((style_sheet), null());
    drop(Box::from_raw(style_sheet));
    FfiResult::ok(null())
}

/// # Safety
///
#[cfg(feature = "deserialize")]
#[export_name = "FPStyleSheetBincodeVersion"]
pub unsafe extern "C" fn style_sheet_bincode_version(
    buffer_ptr: *mut u8,
    buffer_len: usize,
) -> FfiResult<*mut StrRef> {
    use float_pigment_consistent_bincode::Options;
    check_null!((buffer_ptr), null_mut());
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
    FfiResult::ok(Box::into_raw(version))
}

/// # Safety
///
#[export_name = "FPCssParserVersion"]
pub unsafe extern "C" fn css_parser_version() -> FfiResult<*mut StrRef> {
    let version = env!("CARGO_PKG_VERSION").to_string().into();
    FfiResult::ok(Box::into_raw(Box::new(version)))
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct ColorValue {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

/// # Safety
///
#[export_name = "FPParseColorFromString"]
pub unsafe extern "C" fn parse_color_from_string(source: *const c_char) -> FfiResult<ColorValue> {
    check_null!((source), ColorValue::default());
    let source = CStr::from_ptr(source).to_string_lossy();
    let ret = parse_color_to_rgba(&source);
    FfiResult::ok(ColorValue {
        red: ret.0,
        green: ret.1,
        blue: ret.2,
        alpha: ret.3,
    })
}

/// # Safety
///
#[export_name = "FPSubstituteVariable"]
pub unsafe extern "C" fn substitute_variable(
    expr_ptr: *const c_char,
    map: RawMutPtr,
    getter: CustomPropertyGetter,
    setter: CustomPropertySetter,
) -> FfiResult<*const c_char> {
    check_null!((expr_ptr, map), null());
    let expr = CStr::from_ptr(expr_ptr).to_string_lossy();
    let context = CustomPropertyContext::create(map, getter, setter);
    if let Some(ret) = parser::property_value::var::substitute_variable(&expr, &context) {
        if let Ok(r) = CString::new(ret) {
            return FfiResult::ok(r.into_raw());
        }
    }
    FfiResult::ok(null())
}

/// # Safety
///
#[export_name = "FPStrFree"]
pub unsafe extern "C" fn str_free(ptr: *const c_char) -> FfiResult<NullPtr> {
    check_null!((ptr), null());
    drop(CString::from_raw(ptr as *mut _));
    FfiResult::ok(null())
}
