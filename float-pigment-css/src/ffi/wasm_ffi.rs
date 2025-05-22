#![doc(hidden)]
#![cfg(feature = "wasm-entrance")]

#[cfg(feature = "wasm-entrance")]
use wasm_bindgen::prelude::*;

use crate::StyleSheetGroup;

#[cfg(all(target_arch = "wasm32", feature = "nodejs-package"))]
fn init_logger() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        console_log::init_with_level(log::Level::Debug).unwrap();
    });
}

#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", feature = "nodejs-package"))]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    init_logger();
    console_error_panic_hook::set_once();
}

/// Serialize CSS to the JSON format.
#[cfg(all(feature = "serialize", feature = "serialize_json"))]
#[cfg_attr(
    feature = "wasm-entrance",
    wasm_bindgen(js_name = "compileStyleSheetToJson")
)]
pub fn compile_style_sheet_to_json_impl(filename: &str, style_text: &str) -> String {
    use crate::compile_style_sheet_to_json;
    compile_style_sheet_to_json(filename, style_text)
}

/// Deserialize CSS from the JSON format.
#[cfg(all(feature = "deserialize", feature = "deserialize_json"))]
#[cfg_attr(
    feature = "wasm-entrance",
    wasm_bindgen(js_name = "styleSheetFromJson")
)]
pub fn style_sheet_from_json_impl(json: &str) -> StyleSheetGroup {
    use crate::style_sheet_from_json;
    style_sheet_from_json(json)
}

/// Serialize CSS to the binary format.
#[cfg(feature = "serialize")]
#[cfg_attr(
    feature = "wasm-entrance",
    wasm_bindgen(js_name = "compileStyleSheetToBincode")
)]
pub fn compile_style_sheet_to_bincode_impl(filename: &str, style_text: &str) -> Vec<u8> {
    use crate::compile_style_sheet_to_bincode;
    compile_style_sheet_to_bincode(filename, style_text)
}

/// Deserialize CSS from the bincode format.
#[cfg(feature = "deserialize")]
#[cfg_attr(
    feature = "wasm-entrance",
    wasm_bindgen(js_name = "styleSheetFromBincode")
)]
pub fn style_sheet_from_bincode_impl(bincode: Vec<u8>) -> StyleSheetGroup {
    use crate::style_sheet_from_bincode;
    style_sheet_from_bincode(bincode)
}
