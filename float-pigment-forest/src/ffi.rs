use crate::NodeType;
use crate::{
    node::DumpNode, node::DumpOptions, node::DumpStyleMode, ChildOperation, Len, MeasureMode, Node,
    StyleSetter,
};
use concat_idents::concat_idents;
use float_pigment_css::length_num::*;
use float_pigment_css::property::PropertyValueWithGlobal;
use float_pigment_css::typing::{
    AlignContentType, AlignItemsType, AlignSelfType, BoxSizingType, DisplayType, FlexDirectionType,
    FlexWrapType, JustifyContentType, OverflowType, PositionType, TextAlignType, WritingModeType,
};
use float_pigment_layout::{DefLength, OptionNum};
use std::{ffi::CString, os::raw::c_char};

pub type Width = f32;
pub type Height = f32;
pub type Baseline = f32;
pub type MeasureMinWidth = f32;
pub type MeasureMinHeight = f32;
pub type MeasureMaxWidth = f32;
pub type MeasureMaxHeight = f32;
pub type MeasureMaxContentWidth = f32;
pub type MeasureMaxContentHeight = f32;

pub type BaselineFunc = unsafe extern "C" fn(NodePtr, Width, Height) -> Baseline;

pub type MeasureFunc = unsafe extern "C" fn(
    NodePtr,
    MeasureMaxWidth,
    MeasureMode,
    MeasureMaxHeight,
    MeasureMode,
    MeasureMinWidth,
    MeasureMinHeight,
    MeasureMaxContentWidth,
    MeasureMaxContentHeight,
) -> Size;

pub type CalcHandle = i32;

pub type ResolveCalc = unsafe extern "C" fn(CalcHandle, f32) -> f32;

pub type DirtyCallback = unsafe extern "C" fn(NodePtr);

#[repr(C)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
impl From<Size> for float_pigment_layout::Size<Len> {
    fn from(val: Size) -> Self {
        float_pigment_layout::Size::new(Len::from_f32(val.width), Len::from_f32(val.height))
    }
}

#[repr(C)]
pub struct NodePtr {
    pub ptr: *mut (),
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeToString(
    node: NodePtr,
    recursive: bool,
    layout: bool,
    style: bool,
) -> *const c_char {
    let node = &*(node.ptr as *mut Node);
    let node_str = node.dump_to_html(
        DumpOptions {
            recursive,
            layout,
            style: if style {
                DumpStyleMode::Mutation
            } else {
                DumpStyleMode::None
            },
        },
        1,
    );
    let node_str = CString::new(node_str).expect("CString new error");
    node_str.into_raw()
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn FreeString(str: *const c_char) {
    drop(CString::from_raw(str as *mut c_char));
}
/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeNew() -> NodePtr {
    let node_ptr = Node::new_ptr();
    NodePtr {
        ptr: node_ptr as *mut (),
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeFree(node: NodePtr) {
    drop(Box::from_raw(node.ptr as *mut Node))
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeGetExternalHost(node: NodePtr) -> *mut () {
    let node = &*(node.ptr as *mut Node);
    let external_host = node.external_host();
    external_host.expect("[fp:: NodeGetExternalHost] external host is empty")
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeSetExternalHost(node: NodePtr, external_host: *mut ()) {
    let node = &*(node.ptr as *mut Node);
    node.set_external_host(Some(external_host));
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeSetAsText(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.set_node_type(NodeType::Text);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeInsertChild(node: NodePtr, child: NodePtr, index: usize) {
    let node = &*(node.ptr as *mut Node);
    let child = child.ptr as *mut Node;
    node.insert_child_at(child, index);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeAppendChild(node: NodePtr, child: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    let child = child.ptr as *mut Node;
    node.append_child(child);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeInsertBefore(node: NodePtr, child: NodePtr, pivot: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    let child = child.ptr as *mut Node;
    let pivot = pivot.ptr as *mut Node;
    node.insert_child_before(child, pivot);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeRemoveChild(node: NodePtr, child: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    let child = child.ptr as *mut Node;
    node.remove_child(child);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeRemoveAllChildren(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.remove_all_children();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeGetChild(node: NodePtr, index: usize) -> NodePtr {
    let node = &*(node.ptr as *mut Node);
    let node_ptr = node
        .get_child_ptr_at(index)
        .expect("[fp:: NodeGetChild] Child is not found");
    NodePtr {
        ptr: node_ptr as *mut (),
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeGetParent(node: NodePtr) -> NodePtr {
    let node = &*(node.ptr as *mut Node);
    let node_ptr = node
        .parent_ptr()
        .expect("[fp:: NodeGetParent] Parent is not found");
    NodePtr {
        ptr: node_ptr as *mut (),
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeGetChildCount(node: NodePtr) -> usize {
    let node = &*(node.ptr as *mut Node);
    node.children_len()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeCalculateLayout(
    node: NodePtr,
    available_width: f32,
    available_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) {
    let node = &*(node.ptr as *mut Node);
    let available_width = available_width
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_width)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let available_height = available_height
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_height)))
        .unwrap_or_else(crate::node::OptionNum::none);
    node.layout(
        crate::node::OptionSize::new(available_width, available_height),
        crate::node::Size::new(
            Len::from_f32(viewport_width),
            Len::from_f32(viewport_height),
        ),
    );
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeCalculateDryLayout(
    node: NodePtr,
    available_width: f32,
    available_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) {
    let node = &*(node.ptr as *mut Node);
    let available_width = available_width
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_width)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let available_height = available_height
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_height)))
        .unwrap_or_else(crate::node::OptionNum::none);
    node.dry_layout(
        crate::node::OptionSize::new(available_width, available_height),
        crate::node::Size::new(
            Len::from_f32(viewport_width),
            Len::from_f32(viewport_height),
        ),
    );
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeCalculateLayoutWithContainingSize(
    node: NodePtr,
    available_width: f32,
    available_height: f32,
    viewport_width: f32,
    viewport_height: f32,
    containing_width: f32,
    containing_height: f32,
) {
    let node = &*(node.ptr as *mut Node);
    let available_width = available_width
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_width)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let available_height = available_height
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_height)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let containing_width = containing_width
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(containing_width)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let containing_height = containing_height
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(containing_height)))
        .unwrap_or_else(crate::node::OptionNum::none);
    node.layout_with_containing_size(
        crate::node::OptionSize::new(available_width, available_height),
        crate::node::Size::new(
            Len::from_f32(viewport_width),
            Len::from_f32(viewport_height),
        ),
        crate::node::OptionSize::new(containing_width, containing_height),
    );
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeCalculateDryLayoutWithContainingSize(
    node: NodePtr,
    available_width: f32,
    available_height: f32,
    viewport_width: f32,
    viewport_height: f32,
    containing_width: f32,
    containing_height: f32,
) {
    let node = &*(node.ptr as *mut Node);
    let available_width = available_width
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_width)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let available_height = available_height
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(available_height)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let containing_width = containing_width
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(containing_width)))
        .unwrap_or_else(crate::node::OptionNum::none);
    let containing_height = containing_height
        .is_finite()
        .then(|| OptionNum::some(Len::from_f32(containing_height)))
        .unwrap_or_else(crate::node::OptionNum::none);
    node.layout_with_containing_size(
        crate::node::OptionSize::new(available_width, available_height),
        crate::node::Size::new(
            Len::from_f32(viewport_width),
            Len::from_f32(viewport_height),
        ),
        crate::node::OptionSize::new(containing_width, containing_height),
    );
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeMarkDirty(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.mark_dirty_propagate()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeMarkDirtyAndPropagateToDescendants(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.mark_dirty_propagate_to_descendants()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeSetResolveCalc(node: NodePtr, resolve_calc: ResolveCalc) {
    let node = &*(node.ptr as *mut Node);
    node.set_resolve_calc(Some(Box::new(move |handle: i32, parent: Len| -> Len {
        let parent_f32 = parent.to_f32();
        let ret = resolve_calc(handle, parent_f32);
        Len::from_f32(ret)
    })));
}

pub(crate) fn convert_len_max_to_infinity(v: Len) -> f32 {
    if v == Len::MAX {
        f32::INFINITY
    } else {
        v.to_f32()
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeSetMeasureFunc(node: NodePtr, measure_func: MeasureFunc) {
    let node = &*(node.ptr as *mut Node);
    node.set_measure_func(Some(Box::new(
        move |node: *mut Node,
              max_width: crate::node::MeasureMaxWidth,
              width_mode: MeasureMode,
              max_height: crate::node::MeasureMaxHeight,
              height_mode: MeasureMode,
              min_width: crate::node::MeasureMinWidth,
              min_height: crate::node::MeasureMinHeight,
              max_content_width: crate::node::MeasureMaxContentWidth,
              max_content_height: crate::node::MeasureMaxContentHeight|
              -> crate::node::Size<Len> {
            measure_func(
                NodePtr {
                    ptr: node as *mut (),
                },
                convert_len_max_to_infinity(max_width),
                width_mode,
                convert_len_max_to_infinity(max_height),
                height_mode,
                min_width.to_f32(),
                min_height.to_f32(),
                convert_len_max_to_infinity(max_content_width),
                convert_len_max_to_infinity(max_content_height),
            )
            .into()
        },
    )));
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeClearMeasureFunc(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.set_measure_func(None);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeHasMeasureFunc(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.has_measure_func();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeSetBaselineFunc(node: NodePtr, baseline_func: BaselineFunc) {
    let node = &*(node.ptr as *mut Node);
    node.set_baseline_func(Some(Box::new(
        move |node: *mut Node, width: Len, height: Len| -> Len {
            Len::from_f32(baseline_func(
                NodePtr {
                    ptr: node as *mut (),
                },
                width.to_f32(),
                height.to_f32(),
            ))
        },
    )));
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeClearMeasureCache(node: NodePtr) {
    let node: &Node = &*(node.ptr as *mut Node);
    node.clear_measure_cache();
    node.clear_baseline_cache();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeSetDirtyCallback(node: NodePtr, dirty_cb: DirtyCallback) {
    let node = &*(node.ptr as *mut Node);
    node.set_dirty_callback(Some(Box::new(move |node: *mut Node| {
        dirty_cb(NodePtr {
            ptr: node as *mut (),
        })
    })))
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeClearDirtyCallback(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.set_dirty_callback(None);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeIsDirty(node: NodePtr) -> bool {
    let node = &*(node.ptr as *mut Node);
    node.is_dirty()
}

// style getter
macro_rules! gen_style_getter {
    ($ffi_name: ident, $prop_name: ident, $prop_type: ty) => {
        /// # Safety
        ///
        #[no_mangle]
        pub unsafe extern "C" fn $ffi_name(node: NodePtr) -> $prop_type {
            let node = &*(node.ptr as *mut Node);
            node.style_manager().$prop_name().into()
        }
    };
}

gen_style_getter!(NodeStyleGetFlexDirection, flex_direction, FlexDirectionType);

// style setter
macro_rules! gen_style_setter_with_length {
    ($ffi_name: ident, $prop_name: ident) => {
        /// # Safety
        ///
        #[no_mangle]
        pub unsafe extern "C" fn $ffi_name(node: NodePtr, value: f32) {
            let node = &*(node.ptr as *mut Node);
            node.$prop_name(DefLength::Points(Len::from_f32(value)));
        }
        concat_idents!(ffi_name = $ffi_name, None, {
            /// # Safety
            ///
            #[no_mangle]
            pub unsafe extern "C" fn ffi_name(node: NodePtr) {
                let node = &*(node.ptr as *mut Node);
                node.$prop_name(DefLength::Undefined);
            }
        });
        concat_idents!(ffi_name = $ffi_name, Percentage, {
            /// # Safety
            ///
            #[no_mangle]
            pub unsafe extern "C" fn ffi_name(node: NodePtr, value: f32) {
                let node = &*(node.ptr as *mut Node);
                node.$prop_name(DefLength::Percent(value));
            }
        });
        concat_idents!(ffi_name = $ffi_name, Auto, {
            /// # Safety
            ///
            #[no_mangle]
            pub unsafe extern "C" fn ffi_name(node: NodePtr) {
                let node = &*(node.ptr as *mut Node);
                node.$prop_name(DefLength::Auto);
            }
        });
        concat_idents!(ffi_name = $ffi_name, CalcHandle, {
            /// # Safety
            ///
            #[no_mangle]
            pub unsafe extern "C" fn ffi_name(node: NodePtr, calc_handle: i32) {
                let node = &*(node.ptr as *mut Node);
                node.$prop_name(DefLength::Custom(calc_handle));
            }
        });
    };
}
// macro_rules! gen_style_setter {
//     ($ffi_name: ident, $prop_name: ident, $prop_type: ty) => {
//         #[no_mangle]
//         /// # Safety
//         ///
//         pub unsafe extern "C" fn $ffi_name(node: NodePtr, value: $prop_type) {
//             let node = &*(node.ptr as *mut Node);
//             if let Some(value) = value.to_inner_without_global() {
//                 node.$prop_name(value.into());
//             }
//         }
//     };
// }

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetDisplay(node: NodePtr, value: DisplayType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_display(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBoxSizing(node: NodePtr, value: BoxSizingType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_box_sizing(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetWritingMode(node: NodePtr, value: WritingModeType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_writing_mode(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPosition(node: NodePtr, value: PositionType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_position(value);
    }
}

gen_style_setter_with_length!(NodeStyleSetLeft, set_left);
gen_style_setter_with_length!(NodeStyleSetRight, set_right);
gen_style_setter_with_length!(NodeStyleSetTop, set_top);
gen_style_setter_with_length!(NodeStyleSetBottom, set_bottom);

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetOverflowX(node: NodePtr, value: OverflowType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_overflow_x(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetOverflowY(node: NodePtr, value: OverflowType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_overflow_y(value);
    }
}

gen_style_setter_with_length!(NodeStyleSetWidth, set_width);
gen_style_setter_with_length!(NodeStyleSetHeight, set_height);
gen_style_setter_with_length!(NodeStyleSetMinWidth, set_min_width);
gen_style_setter_with_length!(NodeStyleSetMinHeight, set_min_height);
gen_style_setter_with_length!(NodeStyleSetMaxWidth, set_max_width);
gen_style_setter_with_length!(NodeStyleSetMaxHeight, set_max_height);

gen_style_setter_with_length!(NodeStyleSetMarginLeft, set_margin_left);
gen_style_setter_with_length!(NodeStyleSetMarginRight, set_margin_right);
gen_style_setter_with_length!(NodeStyleSetMarginTop, set_margin_top);
gen_style_setter_with_length!(NodeStyleSetMarginBottom, set_margin_bottom);

gen_style_setter_with_length!(NodeStyleSetPaddingLeft, set_padding_left);
gen_style_setter_with_length!(NodeStyleSetPaddingRight, set_padding_right);
gen_style_setter_with_length!(NodeStyleSetPaddingTop, set_padding_top);
gen_style_setter_with_length!(NodeStyleSetPaddingBottom, set_padding_bottom);

gen_style_setter_with_length!(NodeStyleSetBorderLeft, set_border_left);
gen_style_setter_with_length!(NodeStyleSetBorderRight, set_border_right);
gen_style_setter_with_length!(NodeStyleSetBorderTop, set_border_top);
gen_style_setter_with_length!(NodeStyleSetBorderBottom, set_border_bottom);

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexGrow(node: NodePtr, value: f32) {
    let node = &*(node.ptr as *mut Node);
    node.set_flex_grow(value);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexShrink(node: NodePtr, value: f32) {
    let node = &*(node.ptr as *mut Node);
    node.set_flex_shrink(value);
}

gen_style_setter_with_length!(NodeStyleSetFlexBasis, set_flex_basis);

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexDirection(node: NodePtr, value: FlexDirectionType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_flex_direction(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexWrap(node: NodePtr, value: FlexWrapType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_flex_wrap(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetJustifyContent(node: NodePtr, value: JustifyContentType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_justify_content(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAlignContent(node: NodePtr, value: AlignContentType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_align_content(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAlignItems(node: NodePtr, value: AlignItemsType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_align_items(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAlignSelf(node: NodePtr, value: AlignSelfType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_align_self(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetOrder(node: NodePtr, value: i32) {
    let node = &*(node.ptr as *mut Node);
    node.set_order(value);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetTextAlign(node: NodePtr, value: TextAlignType) {
    let node = &*(node.ptr as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_text_align(value);
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAspectRatio(node: NodePtr, x: f32, y: f32) {
    let node = &*(node.ptr as *mut Node);
    node.set_aspect_ratio(Some(x / y));
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAspectRatioAuto(node: NodePtr) {
    let node = &*(node.ptr as *mut Node);
    node.set_aspect_ratio(None);
}

// layout getter

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetLeft(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.layout_position().left.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetRight(node: NodePtr) -> f32 {
    let _node = &*(node.ptr as *mut Node);
    // TODO: return real right
    0.
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetTop(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.layout_position().top.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBottom(node: NodePtr) -> f32 {
    let _node = &*(node.ptr as *mut Node);
    // TODO: return real bottom
    0.
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetWidth(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.layout_position().width.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetHeight(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.layout_position().height.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginLeft(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().margin.left.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginRight(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().margin.right.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginTop(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().margin.top.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginBottom(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().margin.bottom.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderLeft(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().border.left.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderRight(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().border.right.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderTop(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().border.top.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderBottom(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().border.bottom.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingLeft(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().padding.left.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingRight(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().padding.right.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingTop(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().padding.top.to_f32()
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingBottom(node: NodePtr) -> f32 {
    let node = &*(node.ptr as *mut Node);
    node.computed_style().padding.bottom.to_f32()
}
