use crate::NodeType;
use crate::{
    node::DumpNode, node::DumpOptions, node::DumpStyleMode, ChildOperation, Len, MeasureMode, Node,
    StyleSetter,
};
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

pub type RawMutPtr = *mut ();

pub type NullPtr = *const ();

pub type NodePtr = RawMutPtr;

/// # Safety
///
/// Convert a node instance to a string.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `recursive` - Recursive
/// * `layout` - Layout
/// * `style` - Style
///
/// # Returns
/// A string representation of the node
///
/// # Example
///
/// ```c
/// NodeToString(node, true, true, true);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeToString(
    node: NodePtr,
    recursive: bool,
    layout: bool,
    style: bool,
) -> *const c_char {
    let node = &*(node as *mut Node);
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
/// Free a string instance.
///
/// # Arguments
/// * `str` - Raw pointer to the string instance
///
/// # Example
///
/// ```c
/// FreeString(str);
/// ```
#[no_mangle]
pub unsafe extern "C" fn FreeString(str: *const c_char) {
    drop(CString::from_raw(str as *mut c_char));
}

/// # Safety
///
/// Create a node instance.
///
/// # Returns
/// Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodePtr node = NodeNew();
/// ```
///
#[no_mangle]
pub unsafe extern "C" fn NodeNew() -> NodePtr {
    Node::new_ptr() as NodePtr
}

/// # Safety
///
/// Free a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeFree(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeFree(node: NodePtr) {
    drop(Box::from_raw(node as *mut Node))
}

/// # Safety
///
/// Get the external host of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// Raw pointer to the external host
///
/// # Example
///
/// ```c
/// NodePtr external_host = NodeGetExternalHost(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeGetExternalHost(node: NodePtr) -> *mut () {
    let node = &*(node as *mut Node);
    let external_host = node.external_host();
    external_host.expect("[fp:: NodeGetExternalHost] external host is empty")
}

/// # Safety
///
/// Set the external host of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `external_host` - Raw pointer to the external host
///
/// # Example
///
/// ```c
/// NodeSetExternalHost(node, external_host);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeSetExternalHost(node: NodePtr, external_host: *mut ()) {
    let node = &*(node as *mut Node);
    node.set_external_host(Some(external_host));
}

/// # Safety
///
/// Set the node type of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeSetAsText(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeSetAsText(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_node_type(NodeType::Text);
}

/// # Safety
///
/// Insert a child node at a specific index.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `child` - Raw pointer to the child Node instance
/// * `index` - Index to insert the child node at
///
/// # Example
///
/// ```c
/// NodeInsertChild(node, child, index);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeInsertChild(node: NodePtr, child: NodePtr, index: usize) {
    let node = &*(node as *mut Node);
    let child = child as *mut Node;
    node.insert_child_at(child, index);
}

/// # Safety
///
/// Append a child node to a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `child` - Raw pointer to the child Node instance
///
/// # Example
///
/// ```c
/// NodeAppendChild(node, child);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeAppendChild(node: NodePtr, child: NodePtr) {
    let node = &*(node as *mut Node);
    let child = child as *mut Node;
    node.append_child(child);
}

/// # Safety
///
/// Insert a child node before a pivot node.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `child` - Raw pointer to the child Node instance
/// * `pivot` - Raw pointer to the pivot Node instance
///
/// # Example
///
/// ```c
/// NodeInsertBefore(node, child, pivot);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeInsertBefore(node: NodePtr, child: NodePtr, pivot: NodePtr) {
    let node = &*(node as *mut Node);
    let child = child as *mut Node;
    let pivot = pivot as *mut Node;
    node.insert_child_before(child, pivot);
}

/// # Safety
///
/// Remove a child node from a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `child` - Raw pointer to the child Node instance
///
/// # Example
///
/// ```c
/// NodeRemoveChild(node, child);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeRemoveChild(node: NodePtr, child: NodePtr) {
    let node = &*(node as *mut Node);
    let child = child as *mut Node;
    node.remove_child(child);
}

/// # Safety
///
/// Remove all children from a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeRemoveAllChildren(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeRemoveAllChildren(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.remove_all_children();
}

/// # Safety
///
/// Get a child node at a specific index.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `index` - Index to get the child node at
///
/// # Returns
/// * `NodePtr` - Raw pointer to the child Node instance
///
/// # Example
///
/// ```c
/// NodePtr child = NodeGetChild(node, index);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeGetChild(node: NodePtr, index: usize) -> NodePtr {
    let node = &*(node as *mut Node);
    let node_ptr = node
        .get_child_ptr_at(index)
        .expect("[fp:: NodeGetChild] Child is not found");
    node_ptr as NodePtr
}

/// # Safety
///
/// Get the parent node of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `NodePtr` - Raw pointer to the parent Node instance
///
/// # Example
///
/// ```c
/// NodePtr parent = NodeGetParent(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeGetParent(node: NodePtr) -> NodePtr {
    let node = &*(node as *mut Node);
    let node_ptr = node
        .parent_ptr()
        .expect("[fp:: NodeGetParent] Parent is not found");
    node_ptr as NodePtr
}

/// # Safety
///
/// Get the number of children of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `usize` - Number of children
///
/// # Example
///
/// ```c
/// usize child_count = NodeGetChildCount(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeGetChildCount(node: NodePtr) -> usize {
    let node = &*(node as *mut Node);
    node.children_len()
}

/// # Safety
///
/// Calculate the layout of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `available_width` - Available width
/// * `available_height` - Available height
/// * `viewport_width` - Viewport width
/// * `viewport_height` - Viewport height
///
/// # Example
///
/// ```c
/// NodeCalculateLayout(node, available_width, available_height, viewport_width, viewport_height);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeCalculateLayout(
    node: NodePtr,
    available_width: f32,
    available_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) {
    let node = &*(node as *mut Node);
    let available_width = if available_width.is_finite() {
        OptionNum::some(Len::from_f32(available_width))
    } else {
        OptionNum::none()
    };
    let available_height = if available_height.is_finite() {
        OptionNum::some(Len::from_f32(available_height))
    } else {
        OptionNum::none()
    };
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
/// Calculate the dry layout of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `available_width` - Available width
/// * `available_height` - Available height
/// * `viewport_width` - Viewport width
/// * `viewport_height` - Viewport height
///
/// # Example
///
/// ```c
/// NodeCalculateDryLayout(node, available_width, available_height, viewport_width, viewport_height);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeCalculateDryLayout(
    node: NodePtr,
    available_width: f32,
    available_height: f32,
    viewport_width: f32,
    viewport_height: f32,
) {
    let node = &*(node as *mut Node);
    let available_width = if available_width.is_finite() {
        OptionNum::some(Len::from_f32(available_width))
    } else {
        OptionNum::none()
    };
    let available_height = if available_height.is_finite() {
        OptionNum::some(Len::from_f32(available_height))
    } else {
        OptionNum::none()
    };
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
/// Calculate the layout of a node instance with a containing size.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `available_width` - Available width
/// * `available_height` - Available height
/// * `viewport_width` - Viewport width
/// * `viewport_height` - Viewport height
/// * `containing_width` - Containing width
/// * `containing_height` - Containing height
///
/// # Example
///
/// ```c
/// NodeCalculateLayoutWithContainingSize(node, available_width, available_height, viewport_width, viewport_height, containing_width, containing_height);
/// ```
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
    let node = &*(node as *mut Node);
    let available_width = if available_width.is_finite() {
        OptionNum::some(Len::from_f32(available_width))
    } else {
        crate::node::OptionNum::none()
    };
    let available_height = if available_height.is_finite() {
        OptionNum::some(Len::from_f32(available_height))
    } else {
        crate::node::OptionNum::none()
    };
    let containing_width = if containing_width.is_finite() {
        OptionNum::some(Len::from_f32(containing_width))
    } else {
        crate::node::OptionNum::none()
    };
    let containing_height = if containing_height.is_finite() {
        OptionNum::some(Len::from_f32(containing_height))
    } else {
        crate::node::OptionNum::none()
    };
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
/// Calculate the dry layout of a node instance with a containing size.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `available_width` - Available width
/// * `available_height` - Available height
/// * `viewport_width` - Viewport width
/// * `viewport_height` - Viewport height
/// * `containing_width` - Containing width
/// * `containing_height` - Containing height
///
/// # Example
///
/// ```c
/// NodeCalculateDryLayoutWithContainingSize(node, available_width, available_height, viewport_width, viewport_height, containing_width, containing_height);
/// ```
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
    let node = &*(node as *mut Node);
    let available_width = if available_width.is_finite() {
        OptionNum::some(Len::from_f32(available_width))
    } else {
        crate::node::OptionNum::none()
    };
    let available_height = if available_height.is_finite() {
        OptionNum::some(Len::from_f32(available_height))
    } else {
        crate::node::OptionNum::none()
    };
    let containing_width = if containing_width.is_finite() {
        OptionNum::some(Len::from_f32(containing_width))
    } else {
        crate::node::OptionNum::none()
    };
    let containing_height = if containing_height.is_finite() {
        OptionNum::some(Len::from_f32(containing_height))
    } else {
        crate::node::OptionNum::none()
    };
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
/// Mark a node instance as dirty.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeMarkDirty(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeMarkDirty(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.mark_dirty_propagate()
}

/// # Safety
///
/// Mark a node instance as dirty and propagate the dirty state to its descendants.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeMarkDirtyAndPropagateToDescendants(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeMarkDirtyAndPropagateToDescendants(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.mark_dirty_propagate_to_descendants()
}

/// # Safety
///
/// Set the resolve calc function for a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `resolve_calc` - Resolve calc function
///
/// # Example
///
/// ```c
/// NodeSetResolveCalc(node, resolve_calc);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeSetResolveCalc(node: NodePtr, resolve_calc: ResolveCalc) {
    let node = &*(node as *mut Node);
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
/// Set the measure function for a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `measure_func` - Measure function
///
/// # Example
///
/// ```c
/// NodeSetMeasureFunc(node, measure_func);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeSetMeasureFunc(node: NodePtr, measure_func: MeasureFunc) {
    let node = &*(node as *mut Node);
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
                node as NodePtr,
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
/// Clear the measure function for a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeClearMeasureFunc(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeClearMeasureFunc(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_measure_func(None);
}

/// # Safety
///
/// Check if a node instance has a measure function.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `bool` - True if the node has a measure function, false otherwise
///
/// # Example
///
/// ```c
/// NodeHasMeasureFunc(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeHasMeasureFunc(node: NodePtr) -> bool {
    let node = &*(node as *mut Node);
    node.has_measure_func()
}

/// # Safety
///
/// Set the baseline function for a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `baseline_func` - Baseline function
///
/// # Example
///
/// ```c
/// NodeSetBaselineFunc(node, baseline_func);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeSetBaselineFunc(node: NodePtr, baseline_func: BaselineFunc) {
    let node = &*(node as *mut Node);
    node.set_baseline_func(Some(Box::new(
        move |node: *mut Node, width: Len, height: Len| -> Len {
            Len::from_f32(baseline_func(
                node as NodePtr,
                width.to_f32(),
                height.to_f32(),
            ))
        },
    )));
}

/// # Safety
///
/// Clear the measure cache for a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeClearMeasureCache(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeClearMeasureCache(node: NodePtr) {
    let node: &Node = &*(node as *mut Node);
    node.clear_measure_cache();
    node.clear_baseline_cache();
}

/// # Safety
///
/// Set the dirty callback for a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `dirty_cb` - Dirty callback
///
/// # Example
///
/// ```c
/// NodeSetDirtyCallback(node, dirty_cb);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeSetDirtyCallback(node: NodePtr, dirty_cb: DirtyCallback) {
    let node = &*(node as *mut Node);
    node.set_dirty_callback(Some(Box::new(move |node: *mut Node| {
        dirty_cb(node as NodePtr)
    })))
}

/// # Safety
///
/// Clear the dirty callback for a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeClearDirtyCallback(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeClearDirtyCallback(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_dirty_callback(None);
}

/// # Safety
///
/// Check if a node instance is dirty.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `bool` - True if the node is dirty, false otherwise
///
/// # Example
///
/// ```c
/// NodeIsDirty(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeIsDirty(node: NodePtr) -> bool {
    let node = &*(node as *mut Node);
    node.is_dirty()
}

/// # Safety
///
/// Get the flex direction of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `FlexDirectionType` - Flex direction type
///
/// # Example
///
/// ```c
/// NodeStyleGetFlexDirection(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleGetFlexDirection(node: NodePtr) -> FlexDirectionType {
    let node = &*(node as *mut Node);
    node.style_manager().flex_direction().into()
}

/// # Safety
///
/// Set the display of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Display type
///
/// # Example
///
/// ```c
/// NodeStyleSetDisplay(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetDisplay(node: NodePtr, value: DisplayType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_display(value);
    }
}

/// # Safety
///
/// Set the box sizing of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Box sizing type
///
/// # Example
///
/// ```c
/// NodeStyleSetBoxSizing(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBoxSizing(node: NodePtr, value: BoxSizingType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_box_sizing(value);
    }
}

/// # Safety
///
/// Set the writing mode of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Writing mode type
///
/// # Example
///
/// ```c
/// NodeStyleSetWritingMode(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetWritingMode(node: NodePtr, value: WritingModeType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_writing_mode(value);
    }
}

/// # Safety
///
/// Set the position of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Position type
///
/// # Example
///
/// ```c
/// NodeStyleSetPosition(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPosition(node: NodePtr, value: PositionType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_position(value);
    }
}

/// # Safety
///
/// Set the left of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Left type
///
/// # Example
///
/// ```c
/// NodeStyleSetLeft(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetLeft(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_left(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the left of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetLeftNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetLeftNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_left(DefLength::Undefined);
}
/// # Safety
///
/// Set the left of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetLeftPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetLeftPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_left(DefLength::Percent(value));
}

/// # Safety
///
/// Set the left of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetLeftAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetLeftAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_left(DefLength::Auto);
}
/// # Safety
///
/// Set the left of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetLeftCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetLeftCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_left(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the right of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Right type
///
/// # Example
///
/// ```c
/// NodeStyleSetRight(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRight(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_right(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the right of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetRightNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRightNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_right(DefLength::Undefined);
}
/// # Safety
///
/// Set the right of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetRightPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRightPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_right(DefLength::Percent(value));
}
/// # Safety
///
/// Set the right of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetRightAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRightAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_right(DefLength::Auto);
}
/// # Safety
///
/// Set the right of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetRightCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRightCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_right(DefLength::Custom(calc_handle));
}
/// # Safety
///
/// Set the top of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Top type
///
/// # Example
///
/// ```c
/// NodeStyleSetTop(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetTop(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_top(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the top of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetTopNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetTopNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_top(DefLength::Undefined);
}
/// # Safety
///
/// Set the top of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetTopPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetTopPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_top(DefLength::Percent(value));
}
/// # Safety
///
/// Set the top of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetTopAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetTopAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_top(DefLength::Auto);
}
/// # Safety
///
/// Set the top of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetTopCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetTopCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_top(DefLength::Custom(calc_handle));
}
/// # Safety
///
/// Set the bottom of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Bottom type
///
/// # Example
///
/// ```c
/// NodeStyleSetBottom(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBottom(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_bottom(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the bottom of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBottomNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBottomNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_bottom(DefLength::Undefined);
}
/// # Safety
///
/// Set the bottom of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetBottomPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBottomPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_bottom(DefLength::Percent(value));
}
/// # Safety
///
/// Set the bottom of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBottomAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBottomAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_bottom(DefLength::Auto);
}
/// # Safety
///
/// Set the bottom of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetBottomCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBottomCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_bottom(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the overflow of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Overflow type
///
/// # Example
///
/// ```c
/// NodeStyleSetOverflowX(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetOverflowX(node: NodePtr, value: OverflowType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_overflow_x(value);
    }
}

/// # Safety
///
/// Set the overflow of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Overflow type
///
/// # Example
///
/// ```c
/// NodeStyleSetOverflowY(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetOverflowY(node: NodePtr, value: OverflowType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_overflow_y(value);
    }
}

/// # Safety
///
/// Set the width of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Width type
///
/// # Example
///
/// ```c
///
///
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetWidth(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_width(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the width of a node instance to undefined.  
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetWidthNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetWidthNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_width(DefLength::Undefined);
}

/// # Safety
///
/// Set the width of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetWidthPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetWidthPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_width(DefLength::Percent(value));
}
/// # Safety
///
/// Set the width of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetWidthAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetWidthAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_width(DefLength::Auto);
}
/// # Safety
///
/// Set the width of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetWidthCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetWidthCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_width(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the height of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Height type
///
/// # Example
///
/// ```c
/// NodeStyleSetHeight(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetHeight(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_height(DefLength::Points(Len::from_f32(value)));
}

/// # Safety
///
/// Set the height of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetHeightNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetHeightNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_height(DefLength::Undefined);
}
/// # Safety
///
/// Set the height of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetHeightPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetHeightPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_height(DefLength::Percent(value));
}
/// # Safety
///
/// Set the height of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetHeightAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetHeightAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_height(DefLength::Auto);
}
/// # Safety
///
/// Set the height of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetHeightCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetHeightCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_height(DefLength::Custom(calc_handle));
}
/// # Safety
///
/// Set the min width of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Min width type
///
/// # Example
///
/// ```c
/// NodeStyleSetMinWidth(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinWidth(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_min_width(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the min width of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMinWidthNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinWidthNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_min_width(DefLength::Undefined);
}
/// # Safety
///
/// Set the min width of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMinWidthPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinWidthPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_min_width(DefLength::Percent(value));
}
/// # Safety
///
/// Set the min width of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMinWidthAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinWidthAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_min_width(DefLength::Auto);
}
/// # Safety
///
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMinWidthCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinWidthCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_min_width(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the min height of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Min height type
///
/// # Example
///
/// ```c
/// NodeStyleSetMinHeight(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinHeight(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_min_height(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the min height of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMinHeightNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinHeightNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_min_height(DefLength::Undefined);
}
/// # Safety
///
/// Set the min height of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMinHeightPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinHeightPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_min_height(DefLength::Percent(value));
}
/// # Safety
///
/// Set the min height of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMinHeightAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinHeightAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_min_height(DefLength::Auto);
}
/// # Safety
///
/// Set the min height of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMinHeightCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMinHeightCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_min_height(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the max width of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Max width type
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxWidth(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxWidth(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_max_width(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the max width of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxWidthNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxWidthNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_max_width(DefLength::Undefined);
}
/// # Safety
///
/// Set the max width of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxWidthPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxWidthPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_max_width(DefLength::Percent(value));
}
/// # Safety
///
/// Set the max width of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxWidthAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxWidthAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_max_width(DefLength::Auto);
}
/// # Safety
///
/// Set the max width of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxWidthCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxWidthCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_max_width(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the max height of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Max height type
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxHeight(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxHeight(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_max_height(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the max height of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxHeightNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxHeightNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_max_height(DefLength::Undefined);
}
/// # Safety
///
/// Set the max height of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxHeightPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxHeightPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_max_height(DefLength::Percent(value));
}
/// # Safety
///
/// Set the max height of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxHeightAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxHeightAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_max_height(DefLength::Auto);
}
/// # Safety
///
/// Set the max height of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMaxHeightCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMaxHeightCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_max_height(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the margin left of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Margin left type
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginLeft(node, value);
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginLeft(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_left(DefLength::Points(Len::from_f32(value)));
}

/// # Safety
///
/// Set the margin left of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginLeftNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginLeftNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_left(DefLength::Undefined);
}
/// # Safety
///
/// Set the margin left of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginLeftPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginLeftPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_left(DefLength::Percent(value));
}
/// # Safety
///
/// Set the margin left of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginLeftAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginLeftAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_left(DefLength::Auto);
}
/// # Safety
///
/// Set the margin left of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginLeftCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginLeftCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_margin_left(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the margin right of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Margin right type
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginRight(node, value);
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginRight(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_right(DefLength::Points(Len::from_f32(value)));
}

/// # Safety
///
/// Set the margin right of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginRightNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginRightNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_right(DefLength::Undefined);
}

/// # Safety
///
/// Set the margin right of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginRightPercentage(node, value);
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginRightPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_right(DefLength::Percent(value));
}
/// # Safety
///
/// Set the margin right of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginRightAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginRightAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_right(DefLength::Auto);
}
/// # Safety
///
/// Set the margin right of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginRightCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginRightCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_margin_right(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the margin top of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Margin top type
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginTop(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginTop(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_top(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the margin top of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginTopNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginTopNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_top(DefLength::Undefined);
}
/// # Safety
///
/// Set the margin top of a node instance to a percentage.  
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginTopPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginTopPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_top(DefLength::Percent(value));
}
/// # Safety
///
/// Set the margin top of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginTopAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginTopAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_top(DefLength::Auto);
}
/// # Safety
///
/// Set the margin top of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginTopCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginTopCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_margin_top(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the margin bottom of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Margin bottom type
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginBottom(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginBottom(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_bottom(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the margin bottom of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginBottomNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginBottomNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_bottom(DefLength::Undefined);
}
/// # Safety
///
/// Set the margin bottom of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginBottomPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginBottomPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_margin_bottom(DefLength::Percent(value));
}
/// # Safety
///
/// Set the margin bottom of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginBottomAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginBottomAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_margin_bottom(DefLength::Auto);
}
/// # Safety
///
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetMarginBottomCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetMarginBottomCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_margin_bottom(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the padding left of a node instance.    
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Padding left type
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingLeft(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingLeft(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_left(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the padding left of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingLeftNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingLeftNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_left(DefLength::Undefined);
}
/// # Safety
///
/// Set the padding left of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingLeftPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingLeftPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_left(DefLength::Percent(value));
}
/// # Safety
///
/// Set the padding left of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingLeftAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingLeftAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_left(DefLength::Auto);
}
/// # Safety
///
/// Set the padding left of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingLeftCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingLeftCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_padding_left(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the padding right of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Padding right type
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingRight(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingRight(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_right(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the padding right of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingRightNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingRightNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_right(DefLength::Undefined);
}

/// # Safety
///
/// Set the padding right of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingRightPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingRightPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_right(DefLength::Percent(value));
}
/// # Safety
///
/// Set the padding right of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingRightAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingRightAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_right(DefLength::Auto);
}

/// # Safety
///
/// Set the padding right of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingRightCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingRightCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_padding_right(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the padding top of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Padding top type
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingTop(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingTop(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_top(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the padding top of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingTopNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingTopNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_top(DefLength::Undefined);
}
/// # Safety
///
/// Set the padding top of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingTopPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingTopPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_top(DefLength::Percent(value));
}
/// # Safety
///
/// Set the padding top of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingTopAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingTopAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_top(DefLength::Auto);
}
/// # Safety
///
/// Set the padding top of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingTopCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingTopCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_padding_top(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the padding bottom of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Padding bottom type
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingBottom(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingBottom(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_bottom(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the padding bottom of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingBottomNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingBottomNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_bottom(DefLength::Undefined);
}
/// # Safety
///
/// Set the padding bottom of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingBottomPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingBottomPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_padding_bottom(DefLength::Percent(value));
}
/// # Safety
///
/// Set the padding bottom of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingBottomAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingBottomAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_padding_bottom(DefLength::Auto);
}
/// # Safety
///
/// Set the padding bottom of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetPaddingBottomCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetPaddingBottomCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_padding_bottom(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the border left of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Border left type
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderLeft(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderLeft(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_left(DefLength::Points(Len::from_f32(value)));
}

/// # Safety
///
/// Set the border left of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderLeftNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderLeftNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_left(DefLength::Undefined);
}
/// # Safety
///
/// Set the border left of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderLeftPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderLeftPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_left(DefLength::Percent(value));
}
/// # Safety
///
/// Set the border left of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderLeftAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderLeftAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_left(DefLength::Auto);
}
/// # Safety
///
/// Set the border left of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderLeftCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderLeftCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_border_left(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the border right of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Border right type
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderRight(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderRight(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_right(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the border right of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderRightNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderRightNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_right(DefLength::Undefined);
}
/// # Safety
///
/// Set the border right of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderRightPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderRightPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_right(DefLength::Percent(value));
}
/// # Safety
///
/// Set the border right of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderRightAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderRightAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_right(DefLength::Auto);
}
/// # Safety
///
/// Set the border right of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderRightCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderRightCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_border_right(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the border top of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Border top type
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderTop(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderTop(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_top(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the border top of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderTopNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderTopNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_top(DefLength::Undefined);
}
/// # Safety
///
/// Set the border top of a node instance to a percentage.  
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderTopPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderTopPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_top(DefLength::Percent(value));
}
/// # Safety
///
/// Set the border top of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderTopAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderTopAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_top(DefLength::Auto);
}
/// # Safety
///
/// Set the border top of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderTopCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderTopCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_border_top(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the border bottom of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Border bottom type
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderBottom(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderBottom(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_bottom(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the border bottom of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderBottomNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderBottomNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_bottom(DefLength::Undefined);
}
/// # Safety
///
/// Set the border bottom of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderBottomPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderBottomPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_border_bottom(DefLength::Percent(value));
}
/// # Safety
///
/// Set the border bottom of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderBottomAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderBottomAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_border_bottom(DefLength::Auto);
}
/// # Safety
///
/// Set the border bottom of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetBorderBottomCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetBorderBottomCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_border_bottom(DefLength::Custom(calc_handle));
}
/// # Safety
///
/// Set the flex grow of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Flex grow
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexGrow(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexGrow(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_flex_grow(value);
}

/// # Safety
///
/// Set the flex shrink of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Flex shrink
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexShrink(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexShrink(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_flex_shrink(value);
}

/// # Safety
///
/// Set the flex basis of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Flex basis
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexBasis(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexBasis(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_flex_basis(DefLength::Points(Len::from_f32(value)));
}
/// # Safety
///
/// Set the flex basis of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexBasisAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexBasisAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_flex_basis(DefLength::Auto);
}
/// # Safety
///
/// Set the flex basis of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexBasisNone(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexBasisNone(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_flex_basis(DefLength::Undefined);
}
/// # Safety
///
/// Set the flex basis of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexBasisPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexBasisPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_flex_basis(DefLength::Percent(value));
}
/// # Safety
///
/// Set the flex basis of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexBasisCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexBasisCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_flex_basis(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the flex direction of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Flex direction
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexDirection(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexDirection(node: NodePtr, value: FlexDirectionType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_flex_direction(value);
    }
}

/// # Safety
///
/// Set the flex wrap of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Flex wrap
///
/// # Example
///
/// ```c
/// NodeStyleSetFlexWrap(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetFlexWrap(node: NodePtr, value: FlexWrapType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_flex_wrap(value);
    }
}

/// # Safety
///
/// Set the justify content of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Justify content
///
/// # Example
///
/// ```c
/// NodeStyleSetJustifyContent(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetJustifyContent(node: NodePtr, value: JustifyContentType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_justify_content(value);
    }
}

/// # Safety
///
/// Set the align content of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Align content
///
/// # Example
///
/// ```c
/// NodeStyleSetAlignContent(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAlignContent(node: NodePtr, value: AlignContentType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_align_content(value);
    }
}

/// # Safety
///
/// Set the align items of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Align items
///
/// # Example
///
/// ```c
/// NodeStyleSetAlignItems(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAlignItems(node: NodePtr, value: AlignItemsType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_align_items(value);
    }
}

/// # Safety
///
/// Set the align self of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Align self
///
/// # Example
///
/// ```c
/// NodeStyleSetAlignSelf(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAlignSelf(node: NodePtr, value: AlignSelfType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_align_self(value);
    }
}

/// # Safety
///
/// Set the order of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Order
///
/// # Example
///
/// ```c
/// NodeStyleSetOrder(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetOrder(node: NodePtr, value: i32) {
    let node = &*(node as *mut Node);
    node.set_order(value);
}

/// # Safety
///
/// Set the row gap of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Row gap
///
/// # Example
///
/// ```c
/// NodeStyleSetRowGap(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRowGap(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_row_gap(DefLength::Points(Len::from_f32(value)));
}

/// # Safety
///
/// Set the row gap of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetRowGapNormal(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRowGapNormal(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_row_gap(DefLength::Undefined);
}

/// # Safety
///
/// Set the row gap of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Row gap percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetRowGapPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRowGapPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_row_gap(DefLength::Percent(value));
}

/// # Safety
///
/// Set the row gap of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetRowGapCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetRowGapCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_row_gap(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the column gap of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Column gap
///
/// # Example
///
/// ```c
/// NodeStyleSetColumnGap(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetColumnGap(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_column_gap(DefLength::Points(Len::from_f32(value)));
}

/// # Safety
///
/// Set the column gap of a node instance to undefined.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetColumnGapNormal(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetColumnGapNormal(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_column_gap(DefLength::Undefined);
}

/// # Safety
///
/// Set the column gap of a node instance to a percentage.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Column gap percentage
///
/// # Example
///
/// ```c
/// NodeStyleSetColumnGapPercentage(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetColumnGapPercentage(node: NodePtr, value: f32) {
    let node = &*(node as *mut Node);
    node.set_column_gap(DefLength::Percent(value));
}

/// # Safety
///
/// Set the column gap of a node instance to a calc handle.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `calc_handle` - Calc handle
///
/// # Example
///
/// ```c
/// NodeStyleSetColumnGapCalcHandle(node, calc_handle);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetColumnGapCalcHandle(node: NodePtr, calc_handle: i32) {
    let node = &*(node as *mut Node);
    node.set_column_gap(DefLength::Custom(calc_handle));
}

/// # Safety
///
/// Set the text align of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `value` - Text align
///
/// # Example
///
/// ```c
/// NodeStyleSetTextAlign(node, value);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetTextAlign(node: NodePtr, value: TextAlignType) {
    let node = &*(node as *mut Node);
    if let Some(value) = value.to_inner_without_global() {
        node.set_text_align(value);
    }
}

/// # Safety
///
/// Set the aspect ratio of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
/// * `x` - Aspect ratio x
/// * `y` - Aspect ratio y
///
/// # Example
///
/// ```c
/// NodeStyleSetAspectRatio(node, x, y);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAspectRatio(node: NodePtr, x: f32, y: f32) {
    let node = &*(node as *mut Node);
    node.set_aspect_ratio(Some(x / y));
}

/// # Safety
///
/// Set the aspect ratio of a node instance to auto.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Example
///
/// ```c
/// NodeStyleSetAspectRatioAuto(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeStyleSetAspectRatioAuto(node: NodePtr) {
    let node = &*(node as *mut Node);
    node.set_aspect_ratio(None);
}

// layout getter

/// # Safety
///
/// Get the left position of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Left position
///
/// # Example
///
/// ```c
/// NodeLayoutGetLeft(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetLeft(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.layout_position().left.to_f32()
}

/// # Safety
///
/// Get the right position of a node instance.  
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Right position
///
/// # Example
///
/// ```c
/// NodeLayoutGetRight(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetRight(node: NodePtr) -> f32 {
    let _node = &*(node as *mut Node);
    // TODO: return real right
    0.
}

/// # Safety
///
/// Get the top position of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Top position
///
/// # Example
///
/// ```c
/// NodeLayoutGetTop(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetTop(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.layout_position().top.to_f32()
}

/// # Safety
///
/// Get the bottom position of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Bottom position
///
/// # Example
///
/// ```c
/// NodeLayoutGetBottom(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBottom(node: NodePtr) -> f32 {
    let _node = &*(node as *mut Node);
    // TODO: return real bottom
    0.
}

/// # Safety
///
/// Get the width of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Width
///
/// # Example
///
/// ```c
/// NodeLayoutGetWidth(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetWidth(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.layout_position().width.to_f32()
}

/// # Safety
///
/// Get the height of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Height
///
/// # Example
///
/// ```c
/// NodeLayoutGetHeight(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetHeight(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.layout_position().height.to_f32()
}

/// # Safety
///
/// Get the margin left of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Margin left
///
/// # Example
///
/// ```c
/// NodeLayoutGetMarginLeft(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginLeft(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().margin.left.to_f32()
}

/// # Safety
///
/// Get the margin right of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Margin right
///
/// # Example
///
/// ```c
/// NodeLayoutGetMarginRight(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginRight(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().margin.right.to_f32()
}

/// # Safety
///
/// Get the margin top of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Margin top
///
/// # Example
///
/// ```c
/// NodeLayoutGetMarginTop(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginTop(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().margin.top.to_f32()
}

/// # Safety
///
/// Get the margin bottom of a node instance.   
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Margin bottom
///
/// # Example
///
/// ```c
/// NodeLayoutGetMarginBottom(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetMarginBottom(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().margin.bottom.to_f32()
}

/// # Safety
///
/// Get the border left of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Border left
///
/// # Example
///
/// ```c
/// NodeLayoutGetBorderLeft(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderLeft(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().border.left.to_f32()
}

/// # Safety
///
/// Get the border right of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Border right
///
/// # Example
///
/// ```c
/// NodeLayoutGetBorderRight(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderRight(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().border.right.to_f32()
}

/// # Safety
///
/// Get the border top of a node instance.  
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Border top
///
/// # Example
///
/// ```c
/// NodeLayoutGetBorderTop(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderTop(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().border.top.to_f32()
}

/// # Safety
///
/// Get the border bottom of a node instance.   
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Border bottom
///
/// # Example
///
/// ```c
/// NodeLayoutGetBorderBottom(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetBorderBottom(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().border.bottom.to_f32()
}

/// # Safety
///
/// Get the padding left of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Padding left
///
/// # Example
///
/// ```c
/// NodeLayoutGetPaddingLeft(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingLeft(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().padding.left.to_f32()
}

/// # Safety
///
/// Get the padding right of a node instance.   
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Padding right
///
/// # Example
///
/// ```c
/// NodeLayoutGetPaddingRight(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingRight(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().padding.right.to_f32()
}

/// # Safety
///
/// Get the padding top of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Padding top
///
/// # Example
///
/// ```c
/// NodeLayoutGetPaddingTop(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingTop(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().padding.top.to_f32()
}

/// # Safety
///
/// Get the padding bottom of a node instance.
///
/// # Arguments
/// * `node` - Raw pointer to the Node instance
///
/// # Returns
/// * `f32` - Padding bottom
///
/// # Example
///
/// ```c
/// NodeLayoutGetPaddingBottom(node);
/// ```
#[no_mangle]
pub unsafe extern "C" fn NodeLayoutGetPaddingBottom(node: NodePtr) -> f32 {
    let node = &*(node as *mut Node);
    node.computed_style().padding.bottom.to_f32()
}
