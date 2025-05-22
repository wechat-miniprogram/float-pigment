use crate::ffi::{FfiErrorCode, FfiResult, NullPtr, RawMutPtr};
use alloc::{
    boxed::Box,
    ffi::CString,
    string::{String, ToString},
    vec::Vec,
};

use bit_set::BitSet;
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
use core::ffi::{c_char, CStr};
use core::ptr::{null, null_mut};
use group::drop_css_extension;
use group::StyleSheetImportIndex as StyleSheetImportIndexImpl;
use parser::Warning;
use sheet::borrow::{Array, StyleSheet};
use sheet::str_store::StrRef;

#[cfg(feature = "deserialize")]
use sheet::borrow::de_static_ref_zero_copy_env;

#[macro_export]
macro_rules! check_null {
    ($arg:expr, $error:expr, $default:expr) => {
        if $arg.is_null() {
            return $crate::ffi::FfiResult::error($error, $default);
        }
    };
}

#[macro_export]
macro_rules! raw_ptr_as_mut_ref {
    ($from:expr, $type:ty) => {
        &mut *($from as *mut $type)
    };
}

/// # Safety
///
/// Create a new style sheet resource.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetResource`] instance
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
/// * `source` - C string pointer to the CSS source content (UTF-8 encoded)
/// * `warnings` - Optional output parameter to receive warnings array pointer
///
/// # Examples
///
/// ```c
/// FfiResult result = FPStyleSheetResourceNew();
/// if (result.err != FfiErrorCode::None) {
///     // handle error
/// }
/// RawMutPtr resource = result.value;
/// ```
///
#[export_name = "FPStyleSheetResourceNew"]
pub unsafe extern "C" fn style_sheet_resource_new() -> FfiResult<RawMutPtr> {
    FfiResult::ok(Box::into_raw(Box::new(group::StyleSheetResource::new())) as RawMutPtr)
}

/// # Safety
///
/// Free the style sheet resource.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetResource`] instance
///
/// # Examples
///
/// ```c
/// FfiResult result = FPStyleSheetResourceFree(resource);
/// if (result.err != FfiErrorCode::None) {
///     // handle error
/// }
/// ```
///
#[export_name = "FPStyleSheetResourceFree"]
pub unsafe extern "C" fn style_sheet_resource_free(this: RawMutPtr) -> FfiResult<NullPtr> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null());
    drop(Box::from_raw(this as *mut group::StyleSheetResource));
    FfiResult::ok(null())
}

/// # Safety
/// Add a tag name prefix to the resource.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetResource`] instance
/// * `path` - C string pointer to the stylesheet path (UTF-8 encoded)
/// * `prefix` - C string pointer to the prefix to add to the tag name (UTF-8 encoded)
///
/// # Examples
///
/// ```c
/// FfiResult result = FPStyleSheetResourceAddTagNamePrefix(resource, path, prefix);
/// if (result.err != FfiErrorCode::None) {
///     // handle error
/// }
/// ```
///
#[export_name = "FPStyleSheetResourceAddTagNamePrefix"]
pub unsafe extern "C" fn style_sheet_resource_add_tag_name_prefix(
    this: RawMutPtr,
    path: *const c_char,
    prefix: *const c_char,
) -> FfiResult<NullPtr> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null());
    check_null!(path, FfiErrorCode::PathNullPointer, null());
    check_null!(prefix, FfiErrorCode::PrefixNullPointer, null());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let prefix = CStr::from_ptr(prefix).to_string_lossy();
    res.add_tag_name_prefix(&path, &prefix);
    FfiResult::ok(null())
}

#[cfg(feature = "serialize")]
pub mod serialize {
    use super::*;
    #[cfg(all(feature = "serialize", feature = "serialize_json"))]

    pub mod json {
        use super::*;
        /// # Safety
        /// Serialize the specified style sheet to the JSON format.
        ///
        /// # Arguments
        /// * `this` - A raw pointer to a [`StyleSheetResource`] instance
        /// * `path` - C string pointer to the stylesheet path (UTF-8 encoded)
        /// * `ret_buffer_len` - Pointer to a variable to store the length of the serialized data
        ///
        /// # Examples
        ///
        /// ```c
        /// FfiResult result = FPStyleSheetResourceSerializeJson(resource, path, &mut buffer_len);
        /// if (result.err != FfiErrorCode::None) {
        ///     // handle error
        /// }
        /// ```
        ///
        #[export_name = "FPStyleSheetResourceSerializeJson"]
        pub unsafe extern "C" fn style_sheet_resource_serialize_json(
            this: RawMutPtr,
            path: *const c_char,
            ret_buffer_len: &mut usize,
        ) -> FfiResult<*mut u8> {
            check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
            check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
            let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
            let path = CStr::from_ptr(path).to_string_lossy();
            let serial = res.serialize_json(&path).unwrap_or_default();
            *ret_buffer_len = serial.len();
            let ret = Box::into_raw(serial.into_boxed_str());
            FfiResult::ok(ret as *mut u8)
        }

        /// # Safety
        /// Serialize the style sheet import index to the JSON format.
        ///
        /// # Arguments
        /// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
        /// * `ret_buffer_len` - Pointer to a variable to store the length of the serialized data
        ///
        /// # Examples
        ///
        /// ```c
        /// FPStyleSheetImportIndexSerializeJson(import_index, &buffer_len);
        /// ```
        ///
        #[export_name = "FPStyleSheetImportIndexSerializeJson"]
        pub unsafe extern "C" fn style_sheet_import_index_serialize_json(
            this: RawMutPtr,
            ret_buffer_len: &mut usize,
        ) -> FfiResult<*mut u8> {
            check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
            let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
            let serial = style_sheet_import_index.inner.serialize_json();
            *ret_buffer_len = serial.len();
            let ret = Box::into_raw(serial.into_boxed_str());
            FfiResult::ok(ret as *mut u8)
        }
    }

    pub mod bincode {
        use super::*;

        /// # Safety
        ///
        /// Serialize the specified style sheet to the binary format.
        ///
        /// # Arguments
        /// * `this` - A raw pointer to a [`StyleSheetResource`] instance
        /// * `path` - C string pointer to the stylesheet path (UTF-8 encoded)
        /// * `ret_buffer_len` - Pointer to a variable to store the length of the serialized data
        ///
        /// # Examples
        ///
        /// ```c
        /// FfiResult result = FPStyleSheetResourceSerializeBincode(resource, path, &mut buffer_len);
        /// if (result.err != FfiErrorCode::None) {
        ///     // handle error
        /// }
        /// ```
        ///
        #[export_name = "FPStyleSheetResourceSerializeBincode"]
        pub unsafe extern "C" fn style_sheet_resource_serialize_bincode(
            this: RawMutPtr,
            path: *const c_char,
            ret_buffer_len: &mut usize,
        ) -> FfiResult<*mut u8> {
            check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
            check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
            let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
            let path = CStr::from_ptr(path).to_string_lossy();
            let serial = res.serialize_bincode(&path).unwrap_or_default();
            *ret_buffer_len = serial.len();
            let ret = Box::into_raw(serial.into_boxed_slice());
            FfiResult::ok(ret as *mut u8)
        }

        /// # Safety
        /// Serialize the style sheet import index to the binary format.
        ///
        /// # Arguments
        /// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
        /// * `ret_buffer_len` - Pointer to a variable to store the length of the serialized data
        ///
        /// # Examples
        ///
        /// ```c
        /// FPStyleSheetImportIndexSerializeBincode(import_index, &buffer_len);
        /// ```
        ///
        #[export_name = "FPStyleSheetImportIndexSerializeBincode"]
        pub unsafe extern "C" fn style_sheet_import_index_serialize_bincode(
            this: RawMutPtr,
            ret_buffer_len: &mut usize,
        ) -> FfiResult<*mut u8> {
            check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
            let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
            let serial = style_sheet_import_index.inner.serialize_bincode();
            *ret_buffer_len = serial.len();
            let ret = Box::into_raw(serial.into_boxed_slice());
            FfiResult::ok(ret as *mut u8)
        }
    }
}

#[cfg(feature = "deserialize")]
pub mod deserialize {
    use super::*;
    #[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
    pub mod json {
        use super::*;
        /// # Safety
        /// Deserialize the style sheet import index from the JSON format.
        ///
        /// # Arguments
        /// * `json` - C string pointer to the JSON data
        ///
        /// # Examples
        ///
        /// ```c
        /// FPStyleSheetImportIndexDeserializeJson(json, &import_index);
        /// ```
        ///
        #[export_name = "FPStyleSheetImportIndexDeserializeJson"]
        pub unsafe extern "C" fn style_sheet_import_index_deserialize_json(
            json: *const c_char,
        ) -> FfiResult<RawMutPtr> {
            check_null!(json, FfiErrorCode::JsonNullPointer, null_mut());
            let json = CStr::from_ptr(json).to_string_lossy();
            FfiResult::ok(
                StyleSheetImportIndex {
                    inner: StyleSheetImportIndexImpl::deserialize_json(&json),
                    map: StyleSheetMap::default(),
                }
                .into_raw(),
            )
        }
    }
    pub mod bincode {
        use super::*;
        /// # Safety
        /// Add a style sheet to the resource manager from binary format.
        ///
        /// # Arguments
        /// * `this` - A raw pointer to a [`StyleSheetResource`] instance
        /// * `path` - C string pointer to the stylesheet path (UTF-8 encoded)
        /// * `buffer_ptr` - Pointer to the buffer to store the serialized data
        /// * `buffer_len` - Length of the buffer
        /// * `drop_fn` - Optional drop function
        /// * `drop_args` - Pointer to the drop argument
        /// * `warnings` - Optional output parameter to receive warnings array pointer
        ///
        /// # Examples
        ///
        /// ```c
        /// FPStyleSheetResourceAddBincode(resource, path, buffer_ptr, buffer_len, drop_fn, drop_args, &mut warnings);
        /// ```
        ///
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
            check_null!(this, FfiErrorCode::ThisNullPointer, null());
            check_null!(path, FfiErrorCode::PathNullPointer, null());
            check_null!(buffer_ptr, FfiErrorCode::BufferNullPointer, null());
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
        /// Deserialize the style sheet import index from the binary format.
        ///
        /// # Arguments
        /// * `buffer_ptr` - Pointer to the binary data
        /// * `buffer_len` - Length of the binary data
        /// * `drop_fn` - Optional drop function
        /// * `drop_args` - Pointer to the drop argument
        ///
        /// # Examples
        ///
        /// ```c
        /// FPStyleSheetImportIndexDeserializeBincode(buffer_ptr, buffer_len, drop_fn, drop_args);
        /// ```
        ///
        #[export_name = "FPStyleSheetImportIndexDeserializeBincode"]
        pub unsafe extern "C" fn style_sheet_import_index_deserialize_bincode(
            buffer_ptr: *mut u8,
            buffer_len: usize,
            drop_fn: Option<unsafe extern "C" fn(RawMutPtr)>,
            drop_args: RawMutPtr,
        ) -> FfiResult<RawMutPtr> {
            check_null!(buffer_ptr, FfiErrorCode::BufferNullPointer, null_mut());
            let bincode: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
            FfiResult::ok(
                StyleSheetImportIndex {
                    inner: StyleSheetImportIndexImpl::deserialize_bincode_zero_copy(
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
        /// Merge the style sheet import index from binary format.
        ///
        /// # Arguments
        /// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
        /// * `buffer_ptr` - Pointer to the binary data
        /// * `buffer_len` - Length of the binary data
        /// * `drop_fn` - Optional drop function
        /// * `drop_args` - Pointer to the drop argument
        ///
        /// # Examples
        ///
        /// ```c
        /// FPStyleSheetImportIndexMergeBincode(import_index, buffer_ptr, buffer_len, drop_fn, drop_args);
        /// ```
        ///
        #[export_name = "FPStyleSheetImportIndexMergeBincode"]
        pub unsafe extern "C" fn style_sheet_import_index_merge_bincode(
            this: RawMutPtr,
            buffer_ptr: *mut u8,
            buffer_len: usize,
            drop_fn: Option<unsafe extern "C" fn(*mut ())>,
            drop_args: *mut (),
        ) -> FfiResult<NullPtr> {
            check_null!(this, FfiErrorCode::ThisNullPointer, null());
            check_null!(buffer_ptr, FfiErrorCode::BufferNullPointer, null());
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
        /// Add a style sheet to the import index from binary format.
        ///
        /// # Arguments
        /// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
        /// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
        /// * `buffer_ptr` - Pointer to the buffer to store the serialized data
        /// * `buffer_len` - Length of the buffer
        /// * `drop_fn` - Optional drop function
        /// * `drop_args` - Pointer to the drop argument
        /// * `warnings` - Optional output parameter to receive warnings array pointer
        ///
        /// # Examples
        ///
        /// ```c
        /// FPStyleSheetImportIndexAddBincode(import_index, path, buffer_ptr, buffer_len, drop_fn, drop_args, &mut warnings);
        /// ```
        ///
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
            check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
            check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
            check_null!(buffer_ptr, FfiErrorCode::BufferNullPointer, null_mut());
            let path = CStr::from_ptr(path).to_string_lossy();
            let sheet = de_static_ref_zero_copy_env(
                core::slice::from_raw_parts_mut(buffer_ptr, buffer_len),
                |s| {
                    let s: Result<StyleSheet, _> =
                        float_pigment_consistent_bincode::DefaultOptions::new()
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
    }

    /// # Safety
    /// Get the version of the style sheet in the binary format.
    ///
    /// # Arguments
    /// * `buffer_ptr` - Pointer to the buffer
    /// * `buffer_len` - Length of the buffer
    ///
    /// # Examples
    ///
    /// ```c
    /// FPStyleSheetBincodeVersion(buffer_ptr, buffer_len);
    /// ```
    ///
    #[export_name = "FPStyleSheetBincodeVersion"]
    pub unsafe extern "C" fn style_sheet_bincode_version(
        buffer_ptr: *mut u8,
        buffer_len: usize,
    ) -> FfiResult<*mut StrRef> {
        use float_pigment_consistent_bincode::Options;
        check_null!(buffer_ptr, FfiErrorCode::BufferNullPointer, null_mut());
        let sheet = de_static_ref_zero_copy_env(
            core::slice::from_raw_parts_mut(buffer_ptr, buffer_len),
            |s| {
                let s: Result<StyleSheet, _> =
                    float_pigment_consistent_bincode::DefaultOptions::new()
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
}

/// # Safety
///
/// Add a style sheet to the resource manager.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetResource`] instance
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
/// * `source` - C string pointer to the CSS source content (UTF-8 encoded)
/// * `warnings` - Optional output parameter to receive warnings array pointer
///
/// # Examples
///
/// ```c
/// FPStyleSheetResourceAddSource(resource, path, source, &mut warnings);
/// ```
///
#[export_name = "FPStyleSheetResourceAddSource"]
pub unsafe extern "C" fn style_sheet_resource_add_source(
    this: RawMutPtr,
    path: *const c_char,
    source: *const c_char,
    warnings: *mut *mut Array<Warning>,
) -> FfiResult<NullPtr> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null());
    check_null!(path, FfiErrorCode::PathNullPointer, null());
    check_null!(source, FfiErrorCode::SourceNullPointer, null());
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
/// Add a style sheet to the resource manager with hooks.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetResource`] instance
/// * `hooks` - A parser hooks
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
/// * `source` - C string pointer to the CSS source content (UTF-8 encoded)
/// * `warnings` - Optional output parameter to receive warnings array pointer
///
/// # Examples
///
/// ```c
/// FPStyleSheetResourceAddSourceWithHooks(resource, hooks, path, source, &mut warnings);
/// ```
///
#[export_name = "FPStyleSheetResourceAddSourceWithHooks"]
pub unsafe extern "C" fn style_sheet_resource_add_source_with_hooks(
    this: RawMutPtr,
    hooks: parser::hooks::CParserHooks,
    path: *const c_char,
    source: *const c_char,
    warnings: *mut *mut Array<Warning>,
) -> FfiResult<NullPtr> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null());
    check_null!(path, FfiErrorCode::PathNullPointer, null());
    check_null!(source, FfiErrorCode::SourceNullPointer, null());
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
/// Get the direct dependencies of the specified style sheet.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetResource`] instance
/// * `path` - C string pointer to the stylesheet path (UTF-8 encoded)
///
/// # Examples
///
/// ```c
/// FPStyleSheetResourceDirectDependencies(resource, path);
/// ```
///
#[export_name = "FPStyleSheetResourceDirectDependencies"]
pub unsafe extern "C" fn style_sheet_resource_direct_dependencies(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
    check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
    let res = raw_ptr_as_mut_ref!(this, group::StyleSheetResource);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = res.direct_dependencies(&path);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}

/// # Safety
///
/// Generate the import index of the resource.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetResource`] instance
///
/// # Examples
///
/// ```c
/// FPStyleSheetResourceGenerateImportIndex(resource);
/// ```
///
#[export_name = "FPStyleSheetResourceGenerateImportIndex"]
pub unsafe extern "C" fn style_sheet_resource_generate_import_index(
    this: RawMutPtr,
) -> FfiResult<RawMutPtr> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
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
    inner: StyleSheetImportIndexImpl,
    map: StyleSheetMap,
}

impl StyleSheetImportIndex {
    fn into_raw(self) -> RawMutPtr {
        Box::into_raw(Box::new(self)) as RawMutPtr
    }
}

/// # Safety
/// Create a new style sheet import index.
///
/// # Examples
///
/// ```c
/// RawMutPtr import_index = FPStyleSheetImportIndexNew();
/// ```
///
#[export_name = "FPStyleSheetImportIndexNew"]
pub unsafe extern "C" fn style_sheet_import_index_new() -> FfiResult<RawMutPtr> {
    FfiResult::ok(
        StyleSheetImportIndex {
            inner: StyleSheetImportIndexImpl::new(),
            map: StyleSheetMap::default(),
        }
        .into_raw(),
    )
}

/// # Safety
/// Free the style sheet import index.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
///
/// # Examples
///
/// ```c
/// FPStyleSheetImportIndexFree(import_index);
/// ```
///
#[export_name = "FPStyleSheetImportIndexFree"]
pub unsafe extern "C" fn style_sheet_import_index_free(this: RawMutPtr) -> FfiResult<NullPtr> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null());
    drop(Box::from_raw(this as *mut StyleSheetImportIndex));
    FfiResult::ok(null())
}

/// # Safety
/// Query and mark the dependencies of the specified style sheet.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
///
/// # Examples
///
/// ```c
/// FPStyleSheetImportIndexQueryAndMarkDependencies(import_index, path);
/// ```
///
#[export_name = "FPStyleSheetImportIndexQueryAndMarkDependencies"]
pub unsafe extern "C" fn style_sheet_import_index_query_and_mark_dependencies(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
    check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = style_sheet_import_index
        .inner
        .query_and_mark_dependencies(&path);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}

/// # Safety
/// List the dependencies of the specified style sheet.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
///
/// # Examples
///
/// ```c
/// FPStyleSheetImportIndexListDependencies(import_index, path);
/// ```
///
#[export_name = "FPStyleSheetImportIndexListDependencies"]
pub unsafe extern "C" fn style_sheet_import_index_list_dependencies(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
    check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = style_sheet_import_index
        .inner
        .list_dependencies(&path, true);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}

/// # Safety
/// List the dependency of the specified style sheet.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
///
/// # Examples
///
/// ```c
/// FPStyleSheetImportIndexListDependency(import_index, path);
/// ```
///
#[export_name = "FPStyleSheetImportIndexListDependency"]
pub unsafe extern "C" fn style_sheet_import_index_list_dependency(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut Array<StrRef>> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
    check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    let path = CStr::from_ptr(path).to_string_lossy();
    let deps = style_sheet_import_index
        .inner
        .list_dependencies(&path, false);
    let deps: Vec<_> = deps.into_iter().map(StrRef::from).collect();
    FfiResult::ok(Box::into_raw(Box::new(deps.into())))
}

/// # Safety
/// Remove a style sheet from the style sheet import index.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
///
/// # Examples
///
/// ```c
/// FPStyleSheetImportIndexRemoveBincode(import_index, path);
/// ```
///
#[export_name = "FPStyleSheetImportIndexRemoveBincode"]
pub unsafe extern "C" fn style_sheet_import_index_remove_bincode(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<NullPtr> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null());
    check_null!(path, FfiErrorCode::PathNullPointer, null());
    let path = CStr::from_ptr(path).to_string_lossy();
    let path = drop_css_extension(&path);
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    style_sheet_import_index.map.remove(path);
    FfiResult::ok(null())
}

/// # Safety
/// Get the style sheet from the style sheet import index.
///
/// # Arguments
/// * `this` - A raw pointer to a [`StyleSheetImportIndex`] instance
/// * `path` - C string pointer to the style sheet path (UTF-8 encoded)
///
/// # Examples
///
/// ```c
/// FPStyleSheetImportIndexGetStyleSheet(import_index, path);
/// ```
///
#[export_name = "FPStyleSheetImportIndexGetStyleSheet"]
pub unsafe extern "C" fn style_sheet_import_index_get_style_sheet(
    this: RawMutPtr,
    path: *const c_char,
) -> FfiResult<*mut StyleSheet> {
    check_null!(this, FfiErrorCode::ThisNullPointer, null_mut());
    check_null!(path, FfiErrorCode::PathNullPointer, null_mut());
    let path = CStr::from_ptr(path).to_string_lossy();
    let path = drop_css_extension(&path);
    let style_sheet_import_index = raw_ptr_as_mut_ref!(this, StyleSheetImportIndex);
    match style_sheet_import_index.map.get_mut(path) {
        None => FfiResult::error(FfiErrorCode::InvalidPath, null_mut()),
        Some(x) => FfiResult::ok(x as *mut StyleSheet),
    }
}

/// # Safety
/// Free the buffer.
///
/// # Arguments
/// * `buffer_ptr` - Pointer to the buffer
/// * `buffer_len` - Length of the buffer
///
/// # Examples
///
/// ```c
/// FPBufferFree(buffer_ptr, buffer_len);
/// ```
///
#[export_name = "FPBufferFree"]
pub unsafe extern "C" fn buffer_free(buffer_ptr: *mut u8, buffer_len: usize) -> FfiResult<NullPtr> {
    check_null!(buffer_ptr, FfiErrorCode::BufferNullPointer, null());
    let x: *mut [u8] = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
    drop(Box::from_raw(x));
    FfiResult::ok(null())
}

/// # Safety
/// Free the array of string references.
///
/// # Arguments
/// * `x` - Pointer to the array of string references
///
/// # Examples
///
/// ```c
/// FPArrayStrRefFree(x);
/// ```
///
#[export_name = "FPArrayStrRefFree"]
pub unsafe extern "C" fn array_str_ref_free(x: *mut Array<StrRef>) -> FfiResult<NullPtr> {
    check_null!(x, FfiErrorCode::ArrayNullPointer, null());
    drop(Box::from_raw(x));
    FfiResult::ok(null())
}

/// # Safety
/// Free the array of warnings.
///
/// # Arguments
/// * `warnings` - Pointer to the array of warnings
///
/// # Examples
///
/// ```c
/// FPArrayWarningFree(warnings);
/// ```
///
#[export_name = "FPArrayWarningFree"]
pub unsafe extern "C" fn array_warning_free(
    warnings: *mut Array<parser::Warning>,
) -> FfiResult<NullPtr> {
    check_null!(warnings, FfiErrorCode::ArrayNullPointer, null());
    drop(Box::from_raw(warnings));
    FfiResult::ok(null())
}

/// # Safety
/// Parse the inline style from the string.
///
/// # Arguments
/// * `inline_style_text_ptr` - Pointer to the inline style text
/// * `warnings` - Optional output parameter to receive warnings array pointer
///
/// # Examples
///
/// ```c
/// FPParseInlineStyle(inline_style_text_ptr, warnings);
/// ```
///
#[export_name = "FPParseInlineStyle"]
pub unsafe extern "C" fn parse_inline_style(
    inline_style_text_ptr: *const c_char,
    warnings: *mut *mut Array<parser::Warning>,
) -> FfiResult<*mut InlineRule> {
    check_null!(
        inline_style_text_ptr,
        FfiErrorCode::InlineStyleTextNullPointer,
        null_mut()
    );
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
/// Free the inline style.
///
/// # Arguments
/// * `inline_rule` - Pointer to the inline style
///
/// # Examples
///
/// ```c
/// FPInlineStyleFree(inline_rule);
/// ```
///
#[export_name = "FPInlineStyleFree"]
pub unsafe extern "C" fn inline_style_free(inline_rule: *mut InlineRule) -> FfiResult<NullPtr> {
    check_null!(inline_rule, FfiErrorCode::InlineRuleNullPointer, null());
    drop(Box::from_raw(inline_rule));
    FfiResult::ok(null())
}

/// # Safety
/// Parse the style sheet from the string.
///
/// # Arguments
/// * `style_text_ptr` - Pointer to the style sheet text
///
/// # Examples
///
/// ```c
/// FPStyleSheetFromString(style_text_ptr);
/// ```
///
#[export_name = "FPParseStyleSheetFromString"]
pub unsafe extern "C" fn parse_style_sheet_from_string(
    style_text_ptr: *const c_char,
) -> FfiResult<*mut StyleSheet> {
    check_null!(
        style_text_ptr,
        FfiErrorCode::StyleTextNullPointer,
        null_mut()
    );
    let style_text = CStr::from_ptr(style_text_ptr).to_string_lossy();
    let (compiled_style_sheet, _) = parser::parse_style_sheet("string", &style_text);
    let style_sheet = StyleSheet::from_sheet(&compiled_style_sheet);
    FfiResult::ok(Box::into_raw(Box::new(style_sheet)))
}

/// # Safety
/// Parse the selector from the string.
///
/// # Arguments
/// * `selector_text_ptr` - Pointer to the selector text
///
/// # Examples
///
/// ```c
/// FPParseSelectorFromString(selector_text_ptr);
/// ```
///
#[export_name = "FPParseSelectorFromString"]
pub unsafe extern "C" fn parse_selector_from_string(
    selector_text_ptr: *const c_char,
) -> FfiResult<*mut Selector> {
    check_null!(
        selector_text_ptr,
        FfiErrorCode::SelectorTextNullPointer,
        null_mut()
    );
    let selector_text = CStr::from_ptr(selector_text_ptr).to_string_lossy();
    let selector = Selector::from_string(&selector_text);
    FfiResult::ok(Box::into_raw(Box::new(selector)))
}

/// # Safety
/// Free the selector.
///
/// # Arguments
/// * `selector` - Pointer to the selector
///
/// # Examples
///
/// ```c
/// FPSelectorFree(selector);
/// ```
///
#[export_name = "FPSelectorFree"]
pub unsafe extern "C" fn selector_free(selector: *mut Selector) -> FfiResult<NullPtr> {
    check_null!(selector, FfiErrorCode::SelectorNullPointer, null());
    drop(Box::from_raw(selector));
    FfiResult::ok(null())
}

/// # Safety
/// Free the style sheet.
///
/// # Arguments
/// * `style_sheet` - Pointer to the style sheet
///
/// # Examples
///
/// ```c
/// FPStyleSheetFree(style_sheet);
/// ```
///
#[export_name = "FPStyleSheetFree"]
pub unsafe extern "C" fn style_sheet_free(style_sheet: *mut StyleSheet) -> FfiResult<NullPtr> {
    check_null!(style_sheet, FfiErrorCode::StyleSheetNullPointer, null());
    drop(Box::from_raw(style_sheet));
    FfiResult::ok(null())
}

/// # Safety
/// Get the version of the CSS parser.
///
/// # Examples
///
/// ```c
/// FPCssParserVersion();
/// ```
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
/// Parse the color from the string.
///
/// # Arguments
/// * `source` - Pointer to the source string
///
/// # Examples
///
/// ```c
/// FPParseColorFromString(source);
/// ```
///
#[export_name = "FPParseColorFromString"]
pub unsafe extern "C" fn parse_color_from_string(source: *const c_char) -> FfiResult<ColorValue> {
    check_null!(
        source,
        FfiErrorCode::SourceNullPointer,
        ColorValue::default()
    );
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
/// Substitute the variable in the expression.
///
/// # Arguments
/// * `expr_ptr` - Pointer to the expression
/// * `map` - Pointer to the map
/// * `getter` - Custom property getter
/// * `setter` - Custom property setter
///
/// # Examples
///
/// ```c
/// FPSubstituteVariable(expr_ptr, map, getter, setter);
/// ```
///
#[export_name = "FPSubstituteVariable"]
pub unsafe extern "C" fn substitute_variable(
    expr_ptr: *const c_char,
    map: RawMutPtr,
    getter: CustomPropertyGetter,
    setter: CustomPropertySetter,
) -> FfiResult<*const c_char> {
    check_null!(expr_ptr, FfiErrorCode::ExprPtrNullPointer, null());
    check_null!(map, FfiErrorCode::MapNullPointer, null());
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
/// Free the string.
///
/// # Arguments
/// * `ptr` - Pointer to the string
///
/// # Examples
///
/// ```c
/// FPStrFree(ptr);
/// ```
///
#[export_name = "FPStrFree"]
pub unsafe extern "C" fn str_free(ptr: *const c_char) -> FfiResult<NullPtr> {
    check_null!(ptr, FfiErrorCode::StrNullPointer, null());
    drop(CString::from_raw(ptr as *mut c_char));
    FfiResult::ok(null())
}
