use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use core::{
    cell::{Cell, RefCell},
    num::NonZeroUsize,
};

use hashbrown::HashMap;

#[cfg(feature = "wasm-entrance")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm-entrance")]
use wasm_bindgen::JsCast;

use crate::{
    length_num::LengthNum,
    parser::{WarningKind, DEFAULT_INPUT_CSS_EXTENSION},
    sheet::{FontFace, KeyFrames},
};

use super::parser::Warning;
use super::property::*;
use super::query::*;
#[cfg(any(feature = "serialize", feature = "deserialize"))]
use super::sheet::borrow_resource;
use super::sheet::{CompiledStyleSheet, LinkedStyleSheet, Rule};

#[cfg(feature = "deserialize")]
use super::sheet::borrow;

pub(crate) fn drop_css_extension(path: &str) -> &str {
    path.strip_suffix(DEFAULT_INPUT_CSS_EXTENSION)
        .unwrap_or(path)
}

/// Resource manager to store style sheet files.
#[cfg_attr(feature = "wasm-entrance", wasm_bindgen)]
#[derive(Default)]
pub struct StyleSheetResource {
    pub(crate) refs: HashMap<String, RefCell<CompiledStyleSheet>>,
    panic_on_warning: bool,
}

#[cfg_attr(feature = "wasm-entrance", wasm_bindgen)]
impl StyleSheetResource {
    /// Create a new resource manager.
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(constructor))]
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a `StyleSheetImportIndex`.
    #[cfg_attr(
        feature = "wasm-entrance",
        wasm_bindgen(js_name = "generateImportIndexes")
    )]
    pub fn generate_import_indexes(&self) -> StyleSheetImportIndex {
        let deps = self
            .refs
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    (
                        v.borrow().list_deps().into_iter().collect::<Vec<String>>(),
                        false,
                        Cell::new(false),
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        StyleSheetImportIndex { deps }
    }

    /// Add a prefix to all tag names.
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(js_name = "addTagNamePrefix"))]
    pub fn js_add_tag_name_prefix(&mut self, path: &str, prefix: &str) {
        self.add_tag_name_prefix(path, prefix)
    }

    /// Get `@import` source paths (for JavaScript).
    #[cfg(feature = "wasm-entrance")]
    #[cfg_attr(
        feature = "wasm-entrance",
        wasm_bindgen(js_name = "directDependencies")
    )]
    pub fn js_direct_dependencies(&self, path: &str) -> Vec<JsValue> {
        self.direct_dependencies(drop_css_extension(path))
            .into_iter()
            .map(|x| JsValue::from_str(&x))
            .collect()
    }

    #[doc(hidden)]
    #[cfg(all(feature = "serialize", feature = "serialize_json"))]
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(js_name = "serializeJson"))]
    pub fn serialize_json(&self, path: &str) -> Option<String> {
        let path = drop_css_extension(path);
        self.refs.get(path).map(|ss| ss.borrow().serialize_json())
    }

    /// Serialize the specified style sheet to the binary format.
    #[cfg(feature = "serialize")]
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(js_name = "serializeBincode"))]
    pub fn serialize_bincode(&self, path: &str) -> Option<Vec<u8>> {
        let path = drop_css_extension(path);
        self.refs
            .get(path)
            .map(|ss| ss.borrow().serialize_bincode())
    }

    #[cfg(feature = "wasm-entrance")]
    unsafe fn convert_warnings_to_js_unsafe(warnings: Vec<Warning>) -> JsValue {
        let ret = js_sys::Array::new();
        for warning in warnings {
            let item = js_sys::Object::new();
            js_sys::Reflect::set(
                &item,
                &JsValue::from("message"),
                &JsValue::from(warning.message.as_str()),
            )
            .unwrap();
            js_sys::Reflect::set(
                &item,
                &JsValue::from("startLine"),
                &JsValue::from(warning.start_line),
            )
            .unwrap();
            js_sys::Reflect::set(
                &item,
                &JsValue::from("startCol"),
                &JsValue::from(warning.start_col),
            )
            .unwrap();
            js_sys::Reflect::set(
                &item,
                &JsValue::from("endLine"),
                &JsValue::from(warning.end_line),
            )
            .unwrap();
            js_sys::Reflect::set(
                &item,
                &JsValue::from("endCol"),
                &JsValue::from(warning.end_col),
            )
            .unwrap();

            ret.push(&item);
        }
        ret.dyn_into().unwrap()
    }

    #[cfg(feature = "wasm-entrance")]
    fn convert_warnings_to_js(warnings: Vec<Warning>) -> JsValue {
        unsafe { Self::convert_warnings_to_js_unsafe(warnings) }
    }

    /// Add a style sheet file (for JavaScript).
    #[cfg(feature = "wasm-entrance")]
    #[wasm_bindgen(js_name = "addSource")]
    pub fn js_add_source(&mut self, path: &str, source: &str) -> JsValue {
        let ret = self.add_source(path, source);
        Self::convert_warnings_to_js(ret)
    }

    #[doc(hidden)]
    #[cfg(all(
        feature = "wasm-entrance",
        feature = "deserialize",
        feature = "deserialize_json"
    ))]
    #[wasm_bindgen(js_name = "addJson")]
    pub fn js_add_json(&mut self, path: &str, json: &str) -> JsValue {
        let ret = self.add_json(path, json);
        Self::convert_warnings_to_js(ret)
    }

    /// Add a style sheet file in the binary format (for JavaScript).
    #[cfg(all(feature = "wasm-entrance", feature = "deserialize"))]
    #[wasm_bindgen(js_name = "addBincode")]
    pub fn js_add_bincode(&mut self, path: &str, bincode: Vec<u8>) -> JsValue {
        let ret = self.add_bincode(path, bincode);
        Self::convert_warnings_to_js(ret)
    }
}

impl StyleSheetResource {
    /// Get `@import` source paths.
    pub fn direct_dependencies(&self, path: &str) -> Vec<String> {
        match self.refs.get(path) {
            None => vec![],
            Some(v) => v.borrow().list_deps(),
        }
    }

    fn add(&mut self, path: &str, sheet: CompiledStyleSheet) {
        let path = drop_css_extension(path).into();
        self.refs.insert(path, RefCell::new(sheet));
    }

    /// Add a prefix to all tag names.
    pub fn add_tag_name_prefix(&mut self, path: &str, prefix: &str) {
        let path = drop_css_extension(path);
        if let Some(ss) = self.refs.get(path) {
            ss.borrow_mut().add_tag_name_prefix(prefix);
        }
    }

    /// Enable or disable `panic_on_warning`, i,e, panics on compilation warnings.
    pub fn set_panic_on_warning(&mut self, panic_on_warning: bool) {
        self.panic_on_warning = panic_on_warning;
    }

    pub(crate) fn link(
        &self,
        path: &str,
        scope: Option<NonZeroUsize>,
    ) -> (LinkedStyleSheet, Vec<Warning>) {
        let (ss, warnings) = self
            .refs
            .get(path)
            .map(|ss| ss.borrow_mut().link(self, scope))
            .unwrap_or_else(|| {
                let warnings = vec![Warning {
                    kind: WarningKind::MissingImportTarget,
                    message: format!("Target style sheet {:?} is not found.", path).into(),
                    start_line: 0,
                    start_col: 0,
                    end_line: 0,
                    end_col: 0,
                }];
                (LinkedStyleSheet::new_empty(), warnings)
            });
        if self.panic_on_warning {
            if let Some(w) = warnings.last() {
                panic!("{:?}", w);
            }
        }
        (ss, warnings)
    }

    /// Add a style sheet.
    pub fn add_source(&mut self, path: &str, source: &str) -> Vec<Warning> {
        self.add_source_with_hooks(path, source, None)
    }

    /// Add a style sheet with compilation hooks.
    pub fn add_source_with_hooks(
        &mut self,
        path: &str,
        source: &str,
        hooks: Option<Box<dyn crate::parser::hooks::Hooks>>,
    ) -> Vec<Warning> {
        // drop .wxss
        let path = drop_css_extension(path);
        let (sheet, warning) = crate::parser::parse_style_sheet_with_hooks(path, source, hooks);
        if self.panic_on_warning {
            if let Some(w) = warning.last() {
                panic!("{:?}", w);
            }
        }
        self.add(path, sheet);
        warning
    }

    #[cfg(feature = "deserialize")]
    fn deserialize_failed_warning(msg: String) -> Vec<Warning> {
        vec![Warning {
            kind: WarningKind::DeserializationFailed,
            message: format!(
                "failed to deserialize bincode formatted style sheet: {}",
                msg
            )
            .into(),
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
        }]
    }

    #[doc(hidden)]
    #[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
    pub fn add_json(&mut self, path: &str, json: &str) -> Vec<Warning> {
        let ss = CompiledStyleSheet::deserialize_json(json);
        let ret = match ss {
            Ok(ss) => {
                self.add(path, ss);
                Vec::with_capacity(0)
            }
            Err(err) => Self::deserialize_failed_warning(err),
        };
        if self.panic_on_warning {
            if let Some(w) = ret.last() {
                panic!("{:?}", w);
            }
        }
        ret
    }

    #[doc(hidden)]
    /// # Safety
    ///
    /// Deserialize CSS from json format, with zero copy support.
    /// This format requires stable type definition.
    /// Code changing invalidates the serialize result!
    ///
    /// **Safety**
    /// * The `ptr` pointed memory must be valid before `drop_callback` triggered.
    /// * The `ptr` pointed memory must be valid UTF-8 bytes.
    ///
    #[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
    pub unsafe fn add_json_zero_copy(
        &mut self,
        path: &str,
        ptr: *mut [u8],
        drop_callback: impl 'static + FnOnce(),
    ) -> Vec<Warning> {
        let ss = CompiledStyleSheet::deserialize_json_zero_copy(ptr, drop_callback);
        let ret = match ss {
            Ok(ss) => {
                self.add(path, ss);
                Vec::with_capacity(0)
            }
            Err(err) => Self::deserialize_failed_warning(err),
        };
        if self.panic_on_warning {
            if let Some(w) = ret.last() {
                panic!("{:?}", w);
            }
        }
        ret
    }

    /// Add a style sheet in the binary format.
    #[cfg(feature = "deserialize")]
    pub fn add_bincode(&mut self, path: &str, bincode: Vec<u8>) -> Vec<Warning> {
        let ss = CompiledStyleSheet::deserialize_bincode(bincode);
        let ret = match ss {
            Ok(ss) => {
                self.add(path, ss);
                Vec::with_capacity(0)
            }
            Err(err) => Self::deserialize_failed_warning(err),
        };
        if self.panic_on_warning {
            if let Some(w) = ret.last() {
                panic!("{:?}", w);
            }
        }
        ret
    }

    /// Add a style sheet in bincode format, with zero copy support.
    ///
    /// # Safety
    ///
    /// The `ptr` pointed memory must be valid before `drop_callback` triggered.
    #[cfg(feature = "deserialize")]
    pub unsafe fn add_bincode_zero_copy(
        &mut self,
        path: &str,
        ptr: *const [u8],
        drop_callback: impl 'static + FnOnce(),
    ) -> Vec<Warning> {
        let ss = CompiledStyleSheet::deserialize_bincode_zero_copy(ptr, drop_callback);
        let ret = match ss {
            Ok(ss) => {
                self.add(path, ss);
                Vec::with_capacity(0)
            }
            Err(err) => Self::deserialize_failed_warning(err),
        };
        if self.panic_on_warning {
            if let Some(w) = ret.last() {
                panic!("{:?}", w);
            }
        }
        ret
    }
}

/// Import information of style sheet resources.
#[cfg_attr(feature = "wasm-entrance", wasm_bindgen)]
#[derive(Debug, Default)]
pub struct StyleSheetImportIndex {
    pub(crate) deps: HashMap<String, (Vec<String>, bool, Cell<bool>)>,
}

impl StyleSheetImportIndex {
    /// Create an empty `StyleSheetImportIndex`.
    pub fn new() -> Self {
        Self::default()
    }

    /// List `@import` sources of the specified style sheet, and mark all style sheet as "visited".
    ///
    /// All returned style sheet paths will be marked "visited".
    /// Future calls to this function will never return "visited" ones again.
    pub fn query_and_mark_dependencies(&mut self, path: &str) -> Vec<String> {
        let mut ret = vec![];
        let path = drop_css_extension(path);
        fn rec(
            deps: &mut HashMap<String, (Vec<String>, bool, Cell<bool>)>,
            path: &str,
            ret: &mut Vec<String>,
        ) {
            let x = if let Some((x, marked, _)) = deps.get_mut(path) {
                if *marked {
                    return;
                }
                *marked = true;
                x.clone()
            } else {
                return;
            };
            for x in x.into_iter() {
                rec(deps, &x, ret);
            }
            ret.push(path.into());
        }
        rec(&mut self.deps, path, &mut ret);
        ret
    }

    /// List `@import` sources of the specified style sheet.
    ///
    /// If `recursive` is set, it returns direct and indirect dependencies.
    pub fn list_dependencies(&self, path: &str, recursive: bool) -> Vec<String> {
        let mut ret = vec![];
        let path = drop_css_extension(path);
        fn rec(
            deps: &HashMap<String, (Vec<String>, bool, Cell<bool>)>,
            path: &str,
            ret: &mut Vec<String>,
            recursive: bool,
        ) {
            if let Some((x, _, rec_marked)) = deps.get(path) {
                if rec_marked.get() {
                    return;
                }
                rec_marked.set(true);
                if recursive {
                    for x in x.iter().map(|x| x.as_str()) {
                        rec(deps, x, ret, recursive);
                    }
                }
                ret.push(path.into());
                rec_marked.set(false);
            }
        }
        rec(&self.deps, path, &mut ret, recursive);
        ret
    }
}

#[cfg_attr(feature = "wasm-entrance", wasm_bindgen)]
impl StyleSheetImportIndex {
    /// The JavaScript version of `query_and_mark_dependencies`.
    #[cfg(feature = "wasm-entrance")]
    #[wasm_bindgen(js_name = "queryAndMarkDependencies")]
    pub fn js_query_and_mark_dependencies(&mut self, path: &str) -> JsValue {
        let deps = self.query_and_mark_dependencies(path);
        let ret = js_sys::Array::new_with_length(deps.len() as u32);
        for dep in deps {
            ret.push(&JsValue::from(dep));
        }
        ret.dyn_into().unwrap()
    }

    #[doc(hidden)]
    #[cfg(all(feature = "serialize", feature = "serialize_json"))]
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(js_name = "serializeJson"))]
    pub fn serialize_json(&self) -> String {
        let s = borrow_resource::StyleSheetImportIndex::from_sheet(self);
        serde_json::to_string(&s).unwrap()
    }

    #[doc(hidden)]
    #[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(js_name = "deserializeJson"))]
    pub fn deserialize_json(s: &str) -> Self {
        let s: Result<borrow_resource::StyleSheetImportIndex, _> = serde_json::from_str(s);
        match s {
            Ok(ss) => ss.into_sheet(),
            Err(_) => {
                error!("Failed to deserialize json formatted style sheet import index. Use empty content instead.");
                Self {
                    deps: hashbrown::HashMap::default(),
                }
            }
        }
    }

    /// Serialize it to the binary format.
    #[cfg(feature = "serialize")]
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(js_name = "serializeBincode"))]
    pub fn serialize_bincode(&self) -> Vec<u8> {
        use float_pigment_consistent_bincode::Options;
        let s = borrow_resource::StyleSheetImportIndex::from_sheet(self);
        float_pigment_consistent_bincode::DefaultOptions::new()
            .allow_trailing_bytes()
            .serialize(&s)
            .unwrap()
    }

    /// Deserialize from the binary format.
    #[cfg(feature = "deserialize")]
    #[cfg_attr(
        feature = "wasm-entrance",
        wasm_bindgen(js_name = "deserializeBincode")
    )]
    pub fn deserialize_bincode(s: Vec<u8>) -> Self {
        use float_pigment_consistent_bincode::Options;
        let s: Result<borrow_resource::StyleSheetImportIndex, _> =
            float_pigment_consistent_bincode::DefaultOptions::new()
                .allow_trailing_bytes()
                .deserialize(&s);
        match s {
            Ok(ss) => ss.into_sheet(),
            Err(_) => {
                error!("Failed to deserialize bincode formatted style sheet import index. Use empty content instead.");
                Self {
                    deps: HashMap::default(),
                }
            }
        }
    }

    /// Deserialize from the binary format and merge into `self`.
    #[cfg(feature = "deserialize")]
    #[cfg_attr(feature = "wasm-entrance", wasm_bindgen(js_name = "mergeBincode"))]
    pub fn merge_bincode(&mut self, s: Vec<u8>) {
        use float_pigment_consistent_bincode::Options;
        let s: Result<borrow_resource::StyleSheetImportIndex, _> =
            float_pigment_consistent_bincode::DefaultOptions::new()
                .allow_trailing_bytes()
                .deserialize(&s);
        match s {
            Ok(ss) => ss.merge_to_sheet(self),
            Err(_) => {
                error!("Failed to merge bincode formatted style sheet import index. Use empty content instead.");
            }
        }
    }
}

impl StyleSheetImportIndex {
    /// Deserialize from the binary format with zero copy.
    ///
    /// # Safety
    ///
    /// The `ptr` pointed memory must be valid before `drop_callback` triggered.
    #[cfg(feature = "deserialize")]
    pub unsafe fn deserialize_bincode_zero_copy(
        ptr: *mut [u8],
        drop_callback: impl 'static + FnOnce(),
    ) -> Self {
        use float_pigment_consistent_bincode::Options;
        borrow::de_static_ref_zero_copy_env(
            ptr,
            |s| {
                let s: Result<borrow_resource::StyleSheetImportIndex, _> =
                    float_pigment_consistent_bincode::DefaultOptions::new()
                        .allow_trailing_bytes()
                        .deserialize(s);
                match s {
                    Ok(ss) => ss.into_sheet(),
                    Err(_) => {
                        error!("Failed to deserialize bincode formatted style sheet import index. Use empty content instead.");
                        Self {
                            deps: HashMap::default(),
                        }
                    }
                }
            },
            drop_callback,
        )
    }

    /// Deserialize from the binary format and merge into `self` with zero copy.
    ///
    /// # Safety
    ///
    /// The `ptr` pointed memory must be valid before `drop_callback` triggered.
    #[cfg(feature = "deserialize")]
    pub unsafe fn merge_bincode_zero_copy(
        &mut self,
        ptr: *mut [u8],
        drop_callback: impl 'static + FnOnce(),
    ) {
        use float_pigment_consistent_bincode::Options;
        borrow::de_static_ref_zero_copy_env(
            ptr,
            |s| {
                let s: Result<borrow_resource::StyleSheetImportIndex, _> =
                    float_pigment_consistent_bincode::DefaultOptions::new()
                        .allow_trailing_bytes()
                        .deserialize(s);
                match s {
                    Ok(ss) => ss.merge_to_sheet(self),
                    Err(_) => {
                        error!("Failed to merge bincode formatted style sheet import index. Use empty content instead.");
                    }
                }
            },
            drop_callback,
        )
    }

    #[doc(hidden)]
    /// # Safety
    ///
    /// The `ptr` pointed memory must be valid before `drop_callback` triggered.
    #[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
    pub unsafe fn deserialize_json_zero_copy(
        ptr: *mut [u8],
        drop_callback: impl 'static + FnOnce(),
    ) -> Self {
        borrow::de_static_ref_zero_copy_env(
            ptr,
            |s| {
                let s: Result<borrow_resource::StyleSheetImportIndex, _> =
                    serde_json::from_str(std::str::from_utf8_unchecked(s));
                match s {
                    Ok(ss) => ss.into_sheet(),
                    Err(_) => {
                        error!("Failed to deserialize json formatted style sheet import index. Use empty content instead.");
                        Self {
                            deps: hashbrown::HashMap::default(),
                        }
                    }
                }
            },
            drop_callback,
        )
    }
}

/// The style sheet index for debugging.
pub const TEMP_SHEET_INDEX: u16 = u16::MAX;

/// A group of ordered style sheets.
#[cfg_attr(feature = "wasm-entrance", wasm_bindgen)]
#[derive(Default, Clone)]
pub struct StyleSheetGroup {
    sheets: Vec<LinkedStyleSheet>,
    temp_sheet: Option<LinkedStyleSheet>,
}

impl StyleSheetGroup {
    /// Create an empty group.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the count of style sheets.
    pub fn len(&self) -> u16 {
        self.sheets.len() as u16
    }

    /// Return `true` if the group is empty.
    pub fn is_empty(&self) -> bool {
        self.sheets.is_empty()
    }

    /// Append a style sheet, returning its index.
    pub fn append(&mut self, sheet: LinkedStyleSheet) -> u16 {
        let ret = self.sheets.len();
        if Self::is_invalid_index(ret) {
            panic!("The number of stylesheets has reached the maximum limit.")
        }
        self.sheets.push(sheet);
        ret as u16
    }

    /// Replace a style sheet by its index.
    pub fn replace(&mut self, index: u16, sheet: LinkedStyleSheet) {
        self.sheets[index as usize] = sheet;
    }

    /// Append a style sheet from the resource, returning its index.
    pub fn append_from_resource(
        &mut self,
        res: &StyleSheetResource,
        path: &str,
        scope: Option<NonZeroUsize>,
    ) -> u16 {
        self.append_from_resource_with_warnings(res, path, scope).0
    }

    /// Append a style sheet from the resource, returning its index and warnings like @import not found.
    pub fn append_from_resource_with_warnings(
        &mut self,
        res: &StyleSheetResource,
        path: &str,
        scope: Option<NonZeroUsize>,
    ) -> (u16, Vec<Warning>) {
        let path = drop_css_extension(path);
        let (ss, warnings) = res.link(path, scope);
        let ret = self.sheets.len();
        if Self::is_invalid_index(ret) {
            panic!("The number of stylesheets has reached the maximum limit.")
        }
        self.sheets.push(ss);
        (ret as u16, warnings)
    }

    /// Replace a style sheet from the resource by its index.
    pub fn replace_from_resource(
        &mut self,
        index: u16,
        res: &StyleSheetResource,
        path: &str,
        scope: Option<NonZeroUsize>,
    ) {
        let path = drop_css_extension(path);
        let (ss, _warnings) = res.link(path, scope);
        self.sheets[index as usize] = ss;
    }

    /// Remove all style sheets.
    pub fn clear(&mut self) {
        self.sheets.truncate(0);
    }

    /// Get style sheet by index.
    pub fn style_sheet(&self, sheet_index: u16) -> Option<&LinkedStyleSheet> {
        if sheet_index != TEMP_SHEET_INDEX {
            self.sheets.get(sheet_index as usize)
        } else {
            self.temp_sheet.as_ref()
        }
    }

    /// Get font-face by sheet index.
    pub fn get_font_face(&self, sheet_index: u16) -> Option<Vec<Rc<FontFace>>> {
        self.style_sheet(sheet_index)
            .map(|sheet| sheet.get_font_face())
    }

    /// Get a rule by index.
    ///
    /// If sheet index is `TEMP_SHEET_INDEX` then the temporary style sheet will be used.
    pub fn get_rule(&self, sheet_index: u16, rule_index: u32) -> Option<Rc<Rule>> {
        if let Some(sheet) = self.style_sheet(sheet_index) {
            sheet.get_rule(rule_index)
        } else {
            None
        }
    }

    /// Add a rule to the temporary style sheet.
    ///
    /// The temporary style sheet is a style sheet which has the highest priority and no scope limits.
    /// The `rule_index` is returned.
    /// Re-query is needed when the style sheet is updated.
    /// Generally it is used for debugging.
    pub fn add_rule(&mut self, rule: Box<Rule>) -> u32 {
        if self.temp_sheet.is_none() {
            self.temp_sheet = Some(LinkedStyleSheet::new_empty());
        }
        let sheet = self.temp_sheet.as_mut().unwrap();
        sheet.add_rule(rule)
    }

    /// Replace an existing rule with the new rule.
    ///
    /// The existing rule is returned if success.
    /// If sheet index is `TEMP_SHEET_INDEX` then the temporary style sheet will be used.
    /// Re-query is needed when the style sheet is updated.
    /// Generally it is used for debugging.
    pub fn replace_rule(
        &mut self,
        sheet_index: u16,
        rule_index: u32,
        rule: Box<Rule>,
    ) -> Result<Rc<Rule>, Box<Rule>> {
        let sheet = if sheet_index != TEMP_SHEET_INDEX {
            self.sheets.get_mut(sheet_index as usize)
        } else {
            self.temp_sheet.as_mut()
        };
        if let Some(sheet) = sheet {
            sheet.replace_rule(rule_index, rule)
        } else {
            Err(rule)
        }
    }

    /// Query a single node selector (usually for testing only).
    ///
    /// Note that the font size and `em` values will be converted to `px` values.
    pub fn query_single<L: LengthNum>(
        &self,
        query: &StyleQuery,
        media_query_status: &MediaQueryStatus<L>,
        node_properties: &mut NodeProperties,
    ) {
        self.query_ancestor_path(&[query.clone()], media_query_status, node_properties, None)
    }

    /// Find rules that matches the query.
    ///
    /// The query is a `&[StyleQuery]` which means all selector information of the ancestors and the node itself.
    pub fn for_each_matched_rule<L: LengthNum>(
        &self,
        query: &[StyleQuery],
        media_query_status: &MediaQueryStatus<L>,
        mut f: impl FnMut(MatchedRuleRef, Option<&LinkedStyleSheet>),
    ) {
        for (index, sheet) in self.sheets.iter().enumerate() {
            sheet.for_each_matched_rule(
                query,
                media_query_status,
                index.min((TEMP_SHEET_INDEX - 1) as usize) as u16,
                |r| f(r, Some(sheet)),
            );
        }
        if let Some(sheet) = self.temp_sheet.as_ref() {
            sheet.for_each_matched_rule(query, media_query_status, u16::MAX, |r| f(r, None));
        }
    }

    /// Get a rule list that matches the query.
    ///
    /// The query is a `&[StyleQuery]` which means all selector information of the ancestors and the node itself.
    pub fn query_matched_rules<L: LengthNum>(
        &self,
        query: &[StyleQuery],
        media_query_status: &MediaQueryStatus<L>,
    ) -> MatchedRuleList {
        let mut rules = vec![];
        self.for_each_matched_rule(query, media_query_status, |matched_rule, style_sheet| {
            let r = MatchedRule {
                rule: matched_rule.rule.clone(),
                weight: matched_rule.weight,
                style_scope: style_sheet.and_then(|x| x.scope()),
            };
            rules.push(r);
        });
        MatchedRuleList { rules }
    }

    /// Query a node in tree ancestor path.
    ///
    /// The query is a `&[StyleQuery]` which means all selector information of the ancestors and the node itself.
    /// Note that the font size and `em` values will be converted to `px` values.
    pub fn query_ancestor_path<L: LengthNum>(
        &self,
        query: &[StyleQuery],
        media_query_status: &MediaQueryStatus<L>,
        node_properties: &mut NodeProperties,
        parent_node_properties: Option<&NodeProperties>,
    ) {
        let default_font_size = media_query_status.base_font_size.to_f32();
        let parent_font_size = match parent_node_properties {
            None => default_font_size,
            Some(x) => x
                .font_size_ref()
                .resolve_to_f32(media_query_status, default_font_size, true)
                .unwrap_or(default_font_size),
        };
        let rules = self.query_matched_rules(query, media_query_status);
        let current_font_size = rules.get_current_font_size(
            parent_font_size,
            parent_node_properties,
            &[],
            media_query_status,
        );
        rules.merge_node_properties(
            node_properties,
            parent_node_properties,
            current_font_size,
            &[],
        );
    }

    fn is_invalid_index(idx: usize) -> bool {
        idx > (u16::MAX as usize)
    }

    /// Search for an `@keyframe`.
    pub fn search_keyframes<L: LengthNum>(
        &self,
        style_scope: Option<NonZeroUsize>,
        name: &str,
        media_query_status: &MediaQueryStatus<L>,
    ) -> Option<Rc<KeyFrames>> {
        if let Some(sheet) = self.temp_sheet.as_ref() {
            if let Some(x) = sheet.search_keyframes(style_scope, name, media_query_status) {
                return Some(x);
            }
        }
        for sheet in self.sheets.iter().rev() {
            if let Some(x) = sheet.search_keyframes(style_scope, name, media_query_status) {
                return Some(x);
            }
        }
        None
    }
}
