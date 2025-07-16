use crate::{convert_node_ref_to_ptr, ffi::MeasureMode};
use float_pigment_css::length_num::LengthNum;
use float_pigment_layout::Size;

#[cfg(not(debug_assertions))]
use crate::ffi::{
    ExternalDirtyCallback, ExternalGetBaseline, ExternalMeasure, ExternalResolveCalc,
};

use crate::{Len, Node};

#[cfg(debug_assertions)]
use mock_external::{
    ExternalDirtyCallback, ExternalGetBaseline, ExternalMeasure, ExternalResolveCalc,
};

fn to_f32_with_max_to_infinity(v: Len) -> f32 {
    if v == Len::MAX {
        f32::INFINITY
    } else {
        v.to_f32()
    }
}

pub(crate) fn get_baseline_impl(node: &Node, width: Len, height: Len) -> Option<Len> {
    #[allow(unused_unsafe)]
    unsafe {
        let baseline = ExternalGetBaseline(
            convert_node_ref_to_ptr(node) as crate::ffi::NodePtr,
            width.to_f32(),
            height.to_f32(),
        );
        Some(Len::from_f32(baseline))
    }
}

pub(crate) fn dirty_callback_impl(node: &Node) {
    #[allow(unused_unsafe)]
    unsafe {
        ExternalDirtyCallback(convert_node_ref_to_ptr(node) as crate::ffi::NodePtr)
    };
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn measure_impl(
    node: &Node,
    max_width: Len,
    width_mode: MeasureMode,
    max_height: Len,
    height_mode: MeasureMode,
    min_width: Len,
    min_height: Len,
    max_content_width: Len,
    max_content_height: Len,
) -> Option<Size<Len>> {
    #[allow(unused_unsafe)]
    unsafe {
        let size = ExternalMeasure(
            convert_node_ref_to_ptr(node) as crate::ffi::NodePtr,
            to_f32_with_max_to_infinity(max_width),
            width_mode,
            to_f32_with_max_to_infinity(max_height),
            height_mode,
            min_width.to_f32(),
            min_height.to_f32(),
            to_f32_with_max_to_infinity(max_content_width),
            to_f32_with_max_to_infinity(max_content_height),
        );
        Some(Size::new(
            Len::from_f32(size.width),
            Len::from_f32(size.height),
        ))
    }
}

pub(crate) fn resolve_calc_impl(node: &Node, calc_handle: i32, owner: Len) -> Len {
    #[allow(unused_unsafe)]
    unsafe {
        Len::from_f32(ExternalResolveCalc(
            convert_node_ref_to_ptr(node) as crate::ffi::NodePtr,
            calc_handle,
            owner.to_f32(),
        ))
    }
}

#[cfg(debug_assertions)]
pub mod mock_external {

    #[derive(Debug, Default, Clone)]
    pub struct TextInfo {
        pub(crate) font_size: f32,
        // raw_text: String
        pub(crate) text_len: usize,
    }

    impl TextInfo {
        pub(crate) fn measure(
            &self,
            min_width: f32,
            min_height: f32,
            max_width: f32,
            max_height: f32,
            max_content_width: f32,
            max_content_height: f32,
        ) -> (f32, f32) {
            let text_len = self.text_len;
            let text_width = self.font_size * text_len as f32;
            let max_w = max_width.min(max_content_width);
            let max_h = max_height.min(max_content_height);
            let measured_width;
            let measured_height;
            if text_width <= max_w {
                // single line
                measured_width = text_width;
                measured_height = self.font_size;
            } else {
                // multi line
                let mut row_count = (max_w.to_f32() / self.font_size).floor();
                if row_count < 1. {
                    row_count = 1.;
                }
                let col_count = (text_len as f32 / row_count).ceil();
                measured_width = (row_count * self.font_size) as i32 as f32;
                measured_height = (col_count * self.font_size) as i32 as f32;
            }
            println!(
                "text_info: {self:?}, width: {min_width:?} ~ {max_width:?}, height: {min_height:?} ~ {max_height:?}, max_content_width: {max_content_width:?}, max_content_height: {max_content_height:?}, measured_width: {measured_width:?}, measured_height: {measured_height:?}"
            );
            (measured_width, measured_height.min(max_h))
        }
    }

    pub struct TextInfoBuilder(TextInfo);

    impl Default for TextInfoBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    impl TextInfoBuilder {
        pub fn new() -> Self {
            Self(TextInfo {
                font_size: 16.,
                // raw_text: String::new(),
                text_len: 0,
            })
        }
        pub fn with_text_len(mut self, text_len: usize) -> Self {
            self.0.text_len = text_len;
            self
        }
        pub fn with_font_size(mut self, font_size: f32) -> Self {
            self.0.font_size = font_size;
            self
        }
        pub fn set_font_size(&mut self, font_size: f32) {
            self.0.font_size = font_size;
        }
        pub fn build(self) -> TextInfo {
            self.0
        }
    }
    use std::{cell::RefCell, collections::HashMap};

    use float_pigment_css::length_num::LengthNum;

    use crate::{ffi::MeasureMode, get_ref_from_node_ptr, Node, NodePtr};

    pub enum MeasureType {
        Text(TextInfo),
        SpecifiedSize((f32, f32)),
        #[allow(clippy::type_complexity)]
        Custom(
            Box<
                dyn Fn(&Node, f32, MeasureMode, f32, MeasureMode, f32, f32, f32, f32) -> (f32, f32),
            >,
        ),
    }

    thread_local! {
        static COLLECTION: RefCell<HashMap<*mut Node,  MeasureType>> = RefCell::new(HashMap::new());
    }

    pub fn set_node_measure_type(node: *mut Node, measure_type: MeasureType) {
        COLLECTION.with(|collection| {
            let mut collection = collection.borrow_mut();
            collection.insert(node, measure_type);
        });
    }

    #[allow(non_snake_case)]
    pub(crate) fn ExternalGetBaseline(_node: *mut (), _width: f32, height: f32) -> f32 {
        let mut res = 0.;
        COLLECTION.with(|collection| {
            let collection = collection.borrow();
            if let Some(measure_type) = collection.get(&(_node as *mut Node)) {
                match measure_type {
                    MeasureType::Text(text_info) => {
                        res = text_info.font_size;
                    }
                    _ => res = height,
                }
            }
        });
        res
    }

    #[allow(clippy::too_many_arguments)]
    #[allow(non_snake_case)]
    pub(crate) fn ExternalMeasure(
        node: *mut (),
        max_width: f32,
        _width_mode: MeasureMode,
        max_height: f32,
        _height_mode: MeasureMode,
        min_width: f32,
        min_height: f32,
        max_content_width: f32,
        max_content_height: f32,
    ) -> crate::ffi::Size {
        let mut res = crate::ffi::Size {
            width: 0.,
            height: 0.,
        };
        COLLECTION.with(|collection| {
            let collection = collection.borrow();
            if let Some(measure_type) = collection.get(&(node as *mut Node)) {
                match measure_type {
                    MeasureType::Text(text_info) => {
                        let (width, height) = text_info.measure(
                            min_width,
                            min_height,
                            max_width,
                            max_height,
                            max_content_width,
                            max_content_height,
                        );
                        res = crate::ffi::Size { width, height };
                    }
                    MeasureType::SpecifiedSize((width, height)) => {
                        res = crate::ffi::Size {
                            width: *width,
                            height: *height,
                        };
                    }
                    MeasureType::Custom(func) => {
                        let (width, height) = func(
                            unsafe { get_ref_from_node_ptr(node as NodePtr) },
                            max_width,
                            _width_mode,
                            max_height,
                            _height_mode,
                            min_width,
                            min_height,
                            max_content_width,
                            max_content_height,
                        );
                        res = crate::ffi::Size { width, height }
                    }
                }
            }
        });
        res
    }

    #[allow(non_snake_case)]
    pub(crate) fn ExternalResolveCalc(_node: *mut (), _calc_handle: i32, _owner: f32) -> f32 {
        todo!()
    }

    #[allow(non_snake_case)]
    pub(crate) fn ExternalDirtyCallback(node: *mut ()) {
        use crate::DumpNode;
        println!("trigger dirty callback of Node: {}", unsafe {
            get_ref_from_node_ptr(node as *mut Node).dump_to_html(
                crate::DumpOptions {
                    recursive: false,
                    layout: true,
                    style: crate::DumpStyleMode::None,
                },
                0,
            )
        });
    }
}
