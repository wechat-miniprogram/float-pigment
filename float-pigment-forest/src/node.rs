use crate::{env::Env, layout::LayoutPosition, style::StyleManager};
use float_pigment_css::typing::{
    AlignContent, AlignItems, AlignSelf, BoxSizing, Direction, FlexDirection, FlexWrap,
    GridAutoFlow, JustifyContent, JustifyItems, JustifySelf, Overflow, Position, TextAlign,
    WritingMode,
};

use float_pigment_css::{length_num::*, typing::Display};
use float_pigment_layout::{ComputedStyle, DefLength, LayoutNode};
pub use float_pigment_layout::{OptionNum, OptionSize, Size};
use lru::LruCache;

use std::{
    cell::{Cell, Ref, RefCell, RefMut, UnsafeCell},
    ptr::{self},
};

pub type Len = float_pigment_css::fixed::FixedI32<float_pigment_css::fixed::types::extra::U10>;
pub type Length = DefLength<Len>;
pub type LayoutGridTemplate = float_pigment_layout::LayoutGridTemplate<Len>;
pub type LayoutGridAuto = float_pigment_layout::LayoutGridAuto<Len>;
pub type NodeId = usize;
pub type NodePtr = *mut Node;

#[inline(always)]
pub fn convert_node_ref_to_ptr(node: &Node) -> NodePtr {
    node as *const Node as *mut Node
}

#[inline(always)]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn get_ref_from_node_ptr(node_ptr: NodePtr) -> &'static Node {
    &*node_ptr
}

pub type ExternalHostPtr = *mut ();

pub(crate) type MeasureMinWidth = Len;
pub(crate) type MeasureMinHeight = Len;
pub(crate) type MeasureMaxWidth = Len;
pub(crate) type MeasureMaxHeight = Len;
pub(crate) type MeasureMaxContentWidth = Len;
pub(crate) type MeasureMaxContentHeight = Len;

pub(crate) type MeasureFn<L> = dyn Fn(
    NodePtr,
    MeasureMaxWidth,
    MeasureMode,
    MeasureMaxHeight,
    MeasureMode,
    MeasureMinWidth,
    MeasureMinHeight,
    MeasureMaxContentWidth,
    MeasureMaxContentHeight,
) -> Size<L>;

pub(crate) type BaselineFn<L> = dyn Fn(NodePtr, L, L) -> L;
pub(crate) type ResolveCalcFn<L> = dyn Fn(i32, L) -> L;
pub(crate) type DirtyCallbackFn = dyn Fn(NodePtr);

pub(crate) type MeasureCacheKeyMinSize = OptionSize<<Len as LengthNum>::Hashable>;
pub(crate) type MeasureCacheKeyMaxSize = OptionSize<<Len as LengthNum>::Hashable>;
pub(crate) type MeasureCacheKeyMaxContent = OptionSize<<Len as LengthNum>::Hashable>;
pub(crate) type MeasureCache = LruCache<
    (
        MeasureCacheKeyMinSize,
        MeasureCacheKeyMaxSize,
        MeasureCacheKeyMaxContent,
    ),
    Size<Len>,
>;
pub(crate) type BaselineCache = LruCache<Size<<Len as LengthNum>::Hashable>, Len>;

const CACHE_SIZE: usize = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum MeasureMode {
    Undefined,
    Exactly,
    AtMost,
}

#[derive(Copy, Clone, Debug)]
pub struct DumpOptions {
    pub recursive: bool,
    pub layout: bool,
    pub style: DumpStyleMode,
}

#[derive(Copy, Clone, Debug)]
pub enum DumpStyleMode {
    None,
    Full,
    Mutation,
}

pub trait DumpNode {
    unsafe fn dump_to_html(&self, options: DumpOptions, current_depth: u8) -> String;
}

impl DumpNode for Node {
    unsafe fn dump_to_html(&self, options: DumpOptions, current_depth: u8) -> String {
        let layout = options.layout.then_some(format!(
            "left: {}, top: {}, width: {}, height: {}",
            self.layout_position().left,
            self.layout_position().top,
            self.layout_position().width,
            self.layout_position().height,
        ));
        let style = match options.style {
            DumpStyleMode::None => None,
            DumpStyleMode::Mutation => Some(self.style_manager().mutation_to_string()),
            DumpStyleMode::Full => Some(self.style_manager().style_to_string()),
        };
        let children = (options.recursive && !self.children().is_empty()).then_some({
            let mut children_str = String::new();
            children_str.push('\n');
            self.children().iter().for_each(|child| {
                let child_str = child.dump_to_html(options, current_depth + 1);
                let tabs = (0..current_depth).map(|_| " ").collect::<String>();
                children_str.push_str(&tabs);
                children_str.push_str(&child_str);
            });
            children_str.push('\n');
            children_str.to_string()
        });
        let mut tag: String = match self.style_manager().display() {
            Display::None => "None".into(),
            Display::Block => "Block".into(),
            Display::Flex => "Flex".into(),
            Display::Inline => "Inline".into(),
            Display::InlineBlock => "InlineBlock".into(),
            Display::FlowRoot => "FlowRoot".into(),
            Display::Grid => "Grid".into(),
            Display::InlineFlex => "InlineFlex".into(),
            Display::InlineGrid => "InlineGrid".into(),
        };
        if self.has_measure_func() {
            tag = format!("Measurable{tag}");
        }
        if let Some(children) = children {
            if let Some(style) = style {
                format!(
                    "<{}#{:p} layout=\"{}\", style=\"{}\">{}",
                    tag,
                    self,
                    layout.unwrap_or_default(),
                    style,
                    children,
                )
            } else {
                format!(
                    "<{}#{:p} layout=\"{}\">{}",
                    tag,
                    self,
                    layout.unwrap_or_default(),
                    children,
                )
            }
        } else if let Some(style) = style {
            format!(
                "<{}#{:p} layout=\"{}\", style=\"{}\"/>\n",
                tag,
                self,
                layout.unwrap_or_default(),
                style,
            )
        } else {
            format!(
                "<{}#{:p} layout=\"{}\"/>\n",
                tag,
                self,
                layout.unwrap_or_default(),
            )
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
    Normal,
    Text,
    #[allow(unused)]
    Image,
}

#[repr(C)]
#[derive(Debug)]
pub struct Node {
    node_type: Cell<NodeType>,
    is_dirty: Cell<bool>,
    external_host: Cell<ExternalHostPtr>,
    parent: Cell<NodePtr>,
    children: RefCell<Vec<NodePtr>>,
    style_manager: RefCell<StyleManager>,
    pub(crate) layout_node: LayoutNode<Node>,
    measure_cache: UnsafeCell<Option<Box<MeasureCache>>>,
    baseline_cache: UnsafeCell<Option<Box<BaselineCache>>>,
    baseline_func: UnsafeCell<Option<Box<BaselineFn<Len>>>>,
    measure_func: UnsafeCell<Option<Box<MeasureFn<Len>>>>,
    resolve_calc: UnsafeCell<Option<Box<ResolveCalcFn<Len>>>>,
    dirty_callback: UnsafeCell<Option<Box<DirtyCallbackFn>>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            node_type: Cell::new(NodeType::Normal),
            external_host: Cell::new(std::ptr::null_mut()),
            children: RefCell::new(Vec::with_capacity(0)),
            parent: Cell::new(std::ptr::null_mut()),
            style_manager: RefCell::new(StyleManager::new()),
            layout_node: LayoutNode::new(),
            is_dirty: Cell::new(true),
            baseline_func: UnsafeCell::new(None),
            measure_func: UnsafeCell::new(None),
            resolve_calc: UnsafeCell::new(None),
            dirty_callback: UnsafeCell::new(None),
            measure_cache: UnsafeCell::new(None),
            baseline_cache: UnsafeCell::new(None),
        }
    }
    pub fn new_typed(node_type: NodeType) -> Self {
        let ret = Self::new();
        ret.node_type.set(node_type);
        ret
    }
    pub fn new_ptr() -> NodePtr {
        let self_node = Box::new(Self::new());
        Box::into_raw(self_node)
    }
    pub unsafe fn parent<'a>(&self) -> Option<&'a Node> {
        if self.parent.get().is_null() {
            None
        } else {
            Some(&*self.parent.get())
        }
    }
    pub fn set_parent(&self, parent: Option<NodePtr>) {
        if let Some(parent) = parent {
            self.parent.replace(parent);
        } else {
            self.parent.replace(std::ptr::null_mut());
        }
    }
    pub fn parent_ptr(&self) -> Option<NodePtr> {
        if self.parent.get().is_null() {
            None
        } else {
            Some(self.parent.get())
        }
    }
    pub unsafe fn children(&self) -> Vec<&Node> {
        self.children
            .borrow()
            .iter()
            .map(|node| &**node)
            .collect::<Vec<_>>()
    }
    pub fn children_len(&self) -> usize {
        self.children.borrow().len()
    }
    pub(crate) fn style_manager(&self) -> Ref<'_, StyleManager> {
        self.style_manager.borrow()
    }
    pub(crate) fn style_manager_mut(&self) -> RefMut<'_, StyleManager> {
        self.style_manager.borrow_mut()
    }
    pub(crate) fn computed_style(&self) -> ComputedStyle<Len> {
        self.layout_node.computed_style()
    }
    pub unsafe fn set_node_type(&self, node_type: NodeType) {
        let prev_type = self.node_type.get();
        if prev_type != node_type {
            if prev_type == NodeType::Text {
                *self.measure_cache.get() = None;
                *self.baseline_cache.get() = None;
            }
            self.node_type.replace(node_type);
        }
        if node_type == NodeType::Text && node_type != prev_type {
            *self.measure_cache.get() = Some(Box::new(LruCache::new(CACHE_SIZE)));
            *self.baseline_cache.get() = Some(Box::new(LruCache::new(CACHE_SIZE)));
        }
    }
    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub(crate) unsafe fn measure_cache(&self) -> Option<&mut MeasureCache> {
        if self.node_type() != NodeType::Text {
            return None;
        }
        (*self.measure_cache.get()).as_deref_mut()
    }

    pub(crate) unsafe fn clear_measure_cache(&self) {
        if let Some(cache) = self.measure_cache() {
            cache.clear();
        }
    }

    #[allow(clippy::mut_from_ref)]
    #[inline(always)]
    pub(crate) unsafe fn baseline_cache(&self) -> Option<&mut BaselineCache> {
        if self.node_type() != NodeType::Text {
            return None;
        }
        (*self.baseline_cache.get()).as_deref_mut()
    }

    pub(crate) unsafe fn clear_baseline_cache(&self) {
        if let Some(cache) = self.baseline_cache() {
            cache.clear();
        }
    }
    pub(crate) fn node_type(&self) -> NodeType {
        self.node_type.get()
    }
    pub(crate) unsafe fn baseline_func(&self) -> Option<&BaselineFn<Len>> {
        (*self.baseline_func.get()).as_deref()
    }
    pub fn set_baseline_func(&self, baseline_func: Option<Box<BaselineFn<Len>>>) {
        drop(std::mem::replace(
            unsafe { &mut *self.baseline_func.get() },
            baseline_func,
        ));
    }
    pub unsafe fn has_baseline_func(&self) -> bool {
        (*self.baseline_func.get()).is_some()
    }
    pub(crate) unsafe fn measure_func(&self) -> Option<&MeasureFn<Len>> {
        (*self.measure_func.get()).as_deref()
    }
    pub fn set_measure_func(&self, measure_func: Option<Box<MeasureFn<Len>>>) {
        drop(std::mem::replace(
            unsafe { &mut *self.measure_func.get() },
            measure_func,
        ));
    }
    pub fn has_measure_func(&self) -> bool {
        unsafe { (*self.measure_func.get()).is_some() }
    }
    pub(crate) fn resolve_calc(&self) -> Option<&ResolveCalcFn<Len>> {
        unsafe { (*self.resolve_calc.get()).as_deref() }
    }
    pub fn set_resolve_calc(&self, resolve_calc: Option<Box<ResolveCalcFn<Len>>>) {
        drop(std::mem::replace(
            unsafe { &mut *self.resolve_calc.get() },
            resolve_calc,
        ))
    }
    pub fn set_dirty_callback(&self, dirty_callback: Option<Box<DirtyCallbackFn>>) {
        drop(std::mem::replace(
            unsafe { &mut *self.dirty_callback.get() },
            dirty_callback,
        ));
    }
    pub fn has_dirty_callback(&self) -> bool {
        unsafe { (*self.dirty_callback.get()).is_some() }
    }
    pub(crate) fn dirty_callback(&self) -> Option<&DirtyCallbackFn> {
        unsafe { (*self.dirty_callback.get()).as_deref() }
    }
    pub fn external_host(&self) -> Option<ExternalHostPtr> {
        if self.external_host.get().is_null() {
            None
        } else {
            Some(self.external_host.get())
        }
    }
    pub fn set_external_host(&self, external_host: Option<ExternalHostPtr>) {
        if let Some(external_host) = external_host {
            self.external_host.replace(external_host);
        } else {
            self.external_host.replace(std::ptr::null_mut());
        }
    }
    pub(crate) fn is_dirty(&self) -> bool {
        self.is_dirty.get()
    }
    pub(crate) fn clear_dirty(&self) {
        self.is_dirty.set(false)
    }
    pub(crate) unsafe fn clear_dirty_recursive(&self) {
        if self.is_dirty() {
            self.clear_dirty();
            self.children()
                .iter()
                .for_each(|child| child.clear_dirty_recursive());
        }
    }
    pub unsafe fn mark_self_dirty(&self) {
        if self.is_dirty() {
            return;
        }
        self.is_dirty.set(true);
        if self.node_type() == NodeType::Text {
            self.clear_measure_cache();
            self.clear_baseline_cache();
        }
        if let Some(dirty_callback) = self.dirty_callback() {
            dirty_callback(convert_node_ref_to_ptr(self))
        }
        self.layout_node.mark_dirty(self);
    }
    pub unsafe fn mark_dirty_propagate_to_descendants(&self) {
        self.mark_self_dirty();
        unsafe {
            self.children
                .borrow()
                .iter()
                .for_each(|node| (**node).mark_dirty_propagate_to_descendants())
        }
    }
    pub unsafe fn mark_dirty_propagate(&self) {
        if !self.is_dirty() {
            self.mark_self_dirty();
            if let Some(parent) = self.parent() {
                parent.mark_dirty_propagate()
            }
        }
    }
    pub fn dry_layout(
        &self,
        available_size: OptionSize<Len>,
        viewport_size: float_pigment_layout::Size<Len>,
    ) {
        // FIXME
        self.layout_node.update_with_containing_size(
            &mut Env {
                screen_width: viewport_size.width,
                screen_height: viewport_size.height,
            },
            self,
            available_size,
            available_size,
        );
    }
    pub unsafe fn layout(
        &self,
        available_size: OptionSize<Len>,
        viewport_size: float_pigment_layout::Size<Len>,
    ) {
        // FIXME
        self.layout_node.update_with_containing_size(
            &mut Env {
                screen_width: viewport_size.width,
                screen_height: viewport_size.height,
            },
            self,
            available_size,
            available_size,
        );
        self.clear_dirty_recursive();
    }

    pub unsafe fn layout_with_containing_size(
        &self,
        available_size: OptionSize<Len>,
        viewport_size: float_pigment_layout::Size<Len>,
        containing_size: OptionSize<Len>,
    ) {
        self.layout_node.update_with_containing_size(
            &mut Env {
                screen_width: viewport_size.width,
                screen_height: viewport_size.height,
            },
            self,
            available_size,
            containing_size,
        );
        self.clear_dirty_recursive();
    }

    pub fn layout_position(&self) -> LayoutPosition {
        let layout = self.layout_node.result();
        LayoutPosition {
            left: layout.origin.x,
            top: layout.origin.y,
            width: layout.size.width,
            height: layout.size.height,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
pub trait ChildOperation {
    unsafe fn get_child_at(&self, idx: usize) -> Option<&Node>;
    unsafe fn get_child_ptr_at(&self, idx: usize) -> Option<NodePtr>;
    unsafe fn get_child_index(&self, child: NodePtr) -> Option<usize>;
    unsafe fn append_child(&self, child: NodePtr);
    unsafe fn insert_child_at(&self, child: NodePtr, idx: usize);
    unsafe fn insert_child_before(&self, child: NodePtr, pivot: NodePtr);
    unsafe fn remove_child(&self, child: NodePtr);
    unsafe fn remove_child_at(&self, idx: usize);
    unsafe fn remove_all_children(&self);
    unsafe fn for_each_child_node<'a, 'b: 'a, F>(&'b self, func: F)
    where
        F: FnMut(&'a Self, usize);
}

impl ChildOperation for Node {
    unsafe fn get_child_at(&self, idx: usize) -> Option<&Node> {
        self.children().get(idx).copied()
    }
    unsafe fn get_child_ptr_at(&self, idx: usize) -> Option<NodePtr> {
        self.children.borrow().get(idx).copied()
    }
    unsafe fn get_child_index(&self, child: NodePtr) -> Option<usize> {
        self.children()
            .iter()
            .position(|node| ptr::eq(*node, child))
    }
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    unsafe fn append_child(&self, child: NodePtr) {
        if let Some(prev_parent) = (*child).parent() {
            prev_parent.remove_child(child);
        }
        (*child).set_parent(Some(convert_node_ref_to_ptr(self)));
        self.children.borrow_mut().push(child);
        self.mark_dirty_propagate()
    }
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    unsafe fn insert_child_at(&self, child: NodePtr, idx: usize) {
        if let Some(prev_parent) = (*child).parent() {
            prev_parent.remove_child(child);
        }
        (*child).set_parent(Some(convert_node_ref_to_ptr(self)));
        self.children.borrow_mut().insert(idx, child);
        self.mark_dirty_propagate()
    }
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    unsafe fn insert_child_before(&self, child: NodePtr, pivot: NodePtr) {
        if let Some(prev_parent) = (*child).parent() {
            prev_parent.remove_child(child);
        }
        (*child).set_parent(Some(convert_node_ref_to_ptr(self)));
        let idx = self
            .children
            .borrow()
            .iter()
            .position(|node| std::ptr::eq(*node, pivot));
        if let Some(idx) = idx {
            self.children.borrow_mut().insert(idx, child)
        }
        self.mark_dirty_propagate();
    }
    unsafe fn remove_child(&self, child: NodePtr) {
        if self.children_len() == 0 {
            return;
        }
        let child_idx_opt = self
            .children
            .borrow()
            .iter()
            .position(|node| std::ptr::eq(*node, child));
        if let Some(child_idx) = child_idx_opt {
            let node = {
                let mut children = self.children.borrow_mut();
                let node = children[child_idx];
                children.remove(child_idx);
                node
            };
            (*node).set_parent(None);
        }

        self.mark_dirty_propagate();
    }
    unsafe fn remove_child_at(&self, idx: usize) {
        let len = self.children_len();
        if len == 0 || idx >= len {
            return;
        }
        if let Some(node) = self.children.borrow().get(idx) {
            (**node).set_parent(None);
        }
        self.children.borrow_mut().remove(idx);
        self.mark_dirty_propagate();
    }
    unsafe fn remove_all_children(&self) {
        self.for_each_child_node(|node, _| {
            (*node).set_parent(None);
        });
        self.children.borrow_mut().clear();
        self.mark_dirty_propagate()
    }
    unsafe fn for_each_child_node<'a, 'b: 'a, F>(&'b self, func: F)
    where
        F: FnMut(&'a Self, usize),
    {
        let mut func = func;
        self.children
            .borrow_mut()
            .iter_mut()
            .enumerate()
            .for_each(|(idx, node)| func(&**node, idx))
    }
}

pub trait StyleSetter {
    unsafe fn set_display(&self, value: Display);
    unsafe fn set_box_sizing(&self, value: BoxSizing);
    unsafe fn set_direction(&self, value: Direction);
    unsafe fn set_writing_mode(&self, value: WritingMode);
    unsafe fn set_position(&self, value: Position);
    unsafe fn set_left(&self, value: Length);
    unsafe fn set_top(&self, value: Length);
    unsafe fn set_right(&self, value: Length);
    unsafe fn set_bottom(&self, value: Length);
    unsafe fn set_overflow_x(&self, value: Overflow);
    unsafe fn set_overflow_y(&self, value: Overflow);
    unsafe fn set_width(&self, value: Length);
    unsafe fn set_height(&self, value: Length);
    unsafe fn set_min_width(&self, value: Length);
    unsafe fn set_min_height(&self, value: Length);
    unsafe fn set_max_width(&self, value: Length);
    unsafe fn set_max_height(&self, value: Length);
    unsafe fn set_margin(&self, value: Length);
    unsafe fn set_margin_left(&self, value: Length);
    unsafe fn set_margin_top(&self, value: Length);
    unsafe fn set_margin_right(&self, value: Length);
    unsafe fn set_margin_bottom(&self, value: Length);
    unsafe fn set_padding(&self, value: Length);
    unsafe fn set_padding_left(&self, value: Length);
    unsafe fn set_padding_top(&self, value: Length);
    unsafe fn set_padding_right(&self, value: Length);
    unsafe fn set_padding_bottom(&self, value: Length);
    unsafe fn set_border(&self, value: Length);
    unsafe fn set_border_left(&self, value: Length);
    unsafe fn set_border_top(&self, value: Length);
    unsafe fn set_border_right(&self, value: Length);
    unsafe fn set_border_bottom(&self, value: Length);
    unsafe fn set_flex_grow(&self, value: f32);
    unsafe fn set_flex_shrink(&self, value: f32);
    unsafe fn set_flex_basis(&self, value: Length);
    unsafe fn set_flex_direction(&self, value: FlexDirection);
    unsafe fn set_flex_wrap(&self, value: FlexWrap);
    unsafe fn set_justify_content(&self, value: JustifyContent);
    unsafe fn set_align_content(&self, value: AlignContent);
    unsafe fn set_align_items(&self, value: AlignItems);
    unsafe fn set_align_self(&self, value: AlignSelf);
    unsafe fn set_justify_items(&self, value: JustifyItems);
    unsafe fn set_justify_self(&self, value: JustifySelf);
    unsafe fn set_aspect_ratio(&self, value: Option<f32>);
    unsafe fn set_order(&self, value: i32);
    unsafe fn set_text_align(&self, value: TextAlign);
    unsafe fn set_row_gap(&self, value: Length);
    unsafe fn set_column_gap(&self, value: Length);
    unsafe fn set_grid_template_rows(&self, value: LayoutGridTemplate);
    unsafe fn set_grid_template_columns(&self, value: LayoutGridTemplate);
    unsafe fn set_grid_auto_flow(&self, value: GridAutoFlow);
    unsafe fn set_grid_auto_rows(&self, value: LayoutGridAuto);
    unsafe fn set_grid_auto_columns(&self, value: LayoutGridAuto);
}

impl StyleSetter for Node {
    unsafe fn set_flex_direction(&self, flex_direction: FlexDirection) {
        if self.style_manager_mut().set_flex_direction(flex_direction) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_direction(&self, direction: Direction) {
        if self.style_manager_mut().set_direction(direction) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_align_content(&self, align_content: AlignContent) {
        if self.style_manager_mut().set_align_content(align_content) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_align_items(&self, align_items: AlignItems) {
        if self.style_manager_mut().set_align_items(align_items) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_align_self(&self, align_self: AlignSelf) {
        if self.style_manager_mut().set_align_self(align_self) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_justify_items(&self, justify_items: JustifyItems) {
        if self.style_manager_mut().set_justify_items(justify_items) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_justify_self(&self, justify_self: JustifySelf) {
        if self.style_manager_mut().set_justify_self(justify_self) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_aspect_ratio(&self, aspect_ratio: Option<f32>) {
        if self.style_manager_mut().set_aspect_ratio(aspect_ratio) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_border(&self, border: Length) {
        let top_changed = self.style_manager_mut().set_border_top(border);
        let right_changed = self.style_manager_mut().set_border_right(border);
        let bottom_changed = self.style_manager_mut().set_border_bottom(border);
        let left_changed = self.style_manager_mut().set_border_left(border);
        if top_changed || right_changed || bottom_changed || left_changed {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_border_left(&self, border_left: Length) {
        if self.style_manager_mut().set_border_left(border_left) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_border_right(&self, border_right: Length) {
        if self.style_manager_mut().set_border_right(border_right) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_border_top(&self, border_top: Length) {
        if self.style_manager_mut().set_border_top(border_top) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_border_bottom(&self, border_bottom: Length) {
        if self.style_manager_mut().set_border_bottom(border_bottom) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_box_sizing(&self, box_sizing: BoxSizing) {
        if self.style_manager_mut().set_box_sizing(box_sizing) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_display(&self, display: Display) {
        if self.style_manager_mut().set_display(display) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_height(&self, height: Length) {
        if self.style_manager_mut().set_height(height) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_width(&self, width: Length) {
        if self.style_manager_mut().set_width(width) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_left(&self, left: Length) {
        if self.style_manager_mut().set_left(left) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_right(&self, right: Length) {
        if self.style_manager_mut().set_right(right) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_top(&self, top: Length) {
        if self.style_manager_mut().set_top(top) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_bottom(&self, bottom: Length) {
        if self.style_manager_mut().set_bottom(bottom) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_flex_shrink(&self, flex_shrink: f32) {
        if self.style_manager_mut().set_flex_shrink(flex_shrink) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_flex_grow(&self, flex_grow: f32) {
        if self.style_manager_mut().set_flex_grow(flex_grow) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_flex_wrap(&self, flex_wrap: FlexWrap) {
        if self.style_manager_mut().set_flex_wrap(flex_wrap) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_flex_basis(&self, flex_basis: Length) {
        if self.style_manager_mut().set_flex_basis(flex_basis) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_justify_content(&self, justify_content: JustifyContent) {
        if self
            .style_manager_mut()
            .set_justify_content(justify_content)
        {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_position(&self, position: Position) {
        if self.style_manager_mut().set_position(position) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_overflow_x(&self, overflow_x: Overflow) {
        if self.style_manager_mut().set_overflow_x(overflow_x) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_overflow_y(&self, overflow_y: Overflow) {
        if self.style_manager_mut().set_overflow_y(overflow_y) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_writing_mode(&self, writing_mode: WritingMode) {
        if self.style_manager_mut().set_writing_mode(writing_mode) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_margin(&self, margin: Length) {
        let top_changed = self.style_manager_mut().set_margin_top(margin);
        let right_changed = self.style_manager_mut().set_margin_right(margin);
        let bottom_changed = self.style_manager_mut().set_margin_bottom(margin);
        let left_changed = self.style_manager_mut().set_margin_left(margin);
        if top_changed || bottom_changed || right_changed || left_changed {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_margin_bottom(&self, margin_bottom: Length) {
        if self.style_manager_mut().set_margin_bottom(margin_bottom) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_margin_left(&self, margin_left: Length) {
        if self.style_manager_mut().set_margin_left(margin_left) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_margin_right(&self, margin_right: Length) {
        if self.style_manager_mut().set_margin_right(margin_right) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_margin_top(&self, margin_top: Length) {
        if self.style_manager_mut().set_margin_top(margin_top) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_max_height(&self, max_height: Length) {
        if self.style_manager_mut().set_max_height(max_height) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_max_width(&self, max_width: Length) {
        if self.style_manager_mut().set_max_width(max_width) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_min_height(&self, min_height: Length) {
        if self.style_manager_mut().set_min_height(min_height) {
            self.mark_dirty_propagate();
        }
    }

    unsafe fn set_min_width(&self, min_width: Length) {
        if self.style_manager_mut().set_min_width(min_width) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_padding(&self, padding: Length) {
        let top_changed = self.style_manager_mut().set_padding_top(padding);
        let right_changed = self.style_manager_mut().set_padding_right(padding);
        let bottom_changed = self.style_manager_mut().set_padding_bottom(padding);
        let left_changed = self.style_manager_mut().set_padding_left(padding);
        if top_changed || bottom_changed || left_changed || right_changed {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_padding_left(&self, padding_left: Length) {
        if self.style_manager_mut().set_padding_left(padding_left) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_padding_right(&self, padding_right: Length) {
        if self.style_manager_mut().set_padding_right(padding_right) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_padding_top(&self, padding_top: Length) {
        if self.style_manager_mut().set_padding_top(padding_top) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_padding_bottom(&self, padding_bottom: Length) {
        if self.style_manager_mut().set_padding_bottom(padding_bottom) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_order(&self, order: i32) {
        if self.style_manager_mut().set_order(order) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_text_align(&self, text_align: TextAlign) {
        if self.style_manager_mut().set_text_align(text_align) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_row_gap(&self, value: Length) {
        if self.style_manager_mut().set_row_gap(value) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_column_gap(&self, value: Length) {
        if self.style_manager_mut().set_column_gap(value) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_grid_template_rows(&self, value: LayoutGridTemplate) {
        if self.style_manager_mut().set_grid_template_rows(value) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_grid_template_columns(&self, value: LayoutGridTemplate) {
        if self.style_manager_mut().set_grid_template_columns(value) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_grid_auto_flow(&self, value: GridAutoFlow) {
        if self.style_manager_mut().set_grid_auto_flow(value) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_grid_auto_rows(&self, value: LayoutGridAuto) {
        if self.style_manager_mut().set_grid_auto_rows(value) {
            self.mark_dirty_propagate();
        }
    }
    unsafe fn set_grid_auto_columns(&self, value: LayoutGridAuto) {
        if self.style_manager_mut().set_grid_auto_columns(value) {
            self.mark_dirty_propagate();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::node::NodePtr;

    use super::*;
    fn new_node<'a>() -> (&'a Node, NodePtr) {
        let node_ptr = Node::new_ptr();
        (unsafe { get_ref_from_node_ptr(node_ptr) }, node_ptr)
    }
    #[test]
    fn append_child() {
        let (node_a, node_a_ptr) = new_node();
        let (node_b, node_b_ptr) = new_node();
        unsafe {
            node_a.append_child(node_b_ptr);
            assert!(std::ptr::eq(node_a, node_b.parent().unwrap()));
            assert!(std::ptr::eq(node_a.get_child_at(0).unwrap(), node_b));
            drop(Box::from_raw(node_a_ptr));
            drop(Box::from_raw(node_b_ptr));
        }
    }
    #[test]
    fn insert_child_at() {
        let (node_a, node_a_ptr) = new_node();
        let (node_b, node_b_ptr) = new_node();
        let (node_c, node_c_ptr) = new_node();
        unsafe {
            node_a.insert_child_at(node_b_ptr, 0);
            node_a.insert_child_at(node_c_ptr, 0);
            assert!(std::ptr::eq(node_a, node_b.parent().unwrap()));
            assert!(std::ptr::eq(node_a, node_c.parent().unwrap()));
            assert!(std::ptr::eq(node_a.get_child_at(0).unwrap(), node_c));
            assert!(std::ptr::eq(node_a.get_child_at(1).unwrap(), node_b));
            drop(Box::from_raw(node_a_ptr));
            drop(Box::from_raw(node_b_ptr));
            drop(Box::from_raw(node_c_ptr));
        }
    }

    #[test]
    fn remove_child() {
        let (node_a, node_a_ptr) = new_node();
        let (node_b, node_b_ptr) = new_node();
        unsafe {
            node_a.insert_child_at(node_b_ptr, 0);
            assert!(std::ptr::eq(node_a, node_b.parent().unwrap()));
            assert!(std::ptr::eq(node_a.get_child_at(0).unwrap(), node_b));
            node_a.remove_child(node_b_ptr);
            assert!(node_b.parent().is_none());
            assert_eq!(node_a.children_len(), 0usize);
            drop(Box::from_raw(node_a_ptr));
            drop(Box::from_raw(node_b_ptr));
        }
    }

    #[test]
    fn remove_child_at() {
        let (node_a, node_a_ptr) = new_node();
        let (node_b, node_b_ptr) = new_node();
        unsafe {
            node_a.insert_child_at(node_b_ptr, 0);
            assert!(std::ptr::eq(node_a, node_b.parent().unwrap()));
            assert!(std::ptr::eq(node_a.get_child_at(0).unwrap(), node_b));
            node_a.remove_child_at(0);
            assert_eq!(node_a.children_len(), 0usize);
            assert!(node_b.parent().is_none());
            drop(Box::from_raw(node_a_ptr));
            drop(Box::from_raw(node_b_ptr));
        }
    }
}
