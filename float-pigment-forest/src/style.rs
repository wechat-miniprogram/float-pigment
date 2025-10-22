use std::cell::Cell;

use float_pigment_css::{num_traits::Zero, typing::GridAutoFlow};
// use float_pigment_forest_macro::{FieldCount, StyleManagerMutation};

use crate::{LayoutGridTemplate, Len, Length};
use float_pigment_css::typing::{
    AlignContent, AlignItems, AlignSelf, BoxSizing, Direction, Display, FlexDirection, FlexWrap,
    JustifyContent, Overflow, Position, TextAlign, WritingMode,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref GLOBAL_BOX_STYLE: Box<BoxStyle> = Box::<BoxStyle>::default();
    static ref GLOBAL_SIZE_STYLE: Box<SizeStyle> = Box::<SizeStyle>::default();
    static ref GLOBAL_SIZE_LIMIT_STYLE: Box<SizeLimitStyle> = Box::<SizeLimitStyle>::default();
    static ref GLOBAL_OTHER_STYLE: Box<OtherStyle> = Box::<OtherStyle>::default();
    static ref GLOBAL_POSITION_STYLE: Box<PositionStyle> = Box::<PositionStyle>::default();
    static ref GLOBAL_MARGIN_STYLE: Box<MarginStyle> = Box::<MarginStyle>::default();
    static ref GLOBAL_PADDING_STYLE: Box<PaddingStyle> = Box::<PaddingStyle>::default();
    static ref GLOBAL_BORDER_STYLE: Box<BorderStyle> = Box::<BorderStyle>::default();
    static ref GLOBAL_FLEX_STYLE: Box<FlexStyle> = Box::<FlexStyle>::default();
    static ref GLOBAL_GRID_STYLE: Box<GridStyle> = Box::<GridStyle>::default();
}

#[derive(Debug)]
struct BoxStyle {
    pub display: Display,
    pub box_sizing: BoxSizing,
}
impl Default for BoxStyle {
    fn default() -> Self {
        Self {
            display: Display::Block,
            box_sizing: BoxSizing::ContentBox,
        }
    }
}

#[derive(Debug)]
struct SizeStyle {
    pub width: Length,
    pub height: Length,
}
impl Default for SizeStyle {
    fn default() -> Self {
        Self {
            width: Length::Auto,
            height: Length::Auto,
        }
    }
}

#[derive(Debug)]
struct SizeLimitStyle {
    pub min_width: Length,
    pub min_height: Length,
    pub max_width: Length,
    pub max_height: Length,
}
impl Default for SizeLimitStyle {
    fn default() -> Self {
        Self {
            min_width: Length::Auto,
            min_height: Length::Auto,
            max_width: Length::Auto,
            max_height: Length::Auto,
        }
    }
}

#[derive(Debug)]
struct OtherStyle {
    pub direction: Direction,
    pub writing_mode: WritingMode,
    pub overflow_x: Overflow,
    pub overflow_y: Overflow,
    pub text_align: TextAlign,
    pub aspect_ratio: Option<f32>,
    pub row_gap: Length,
    pub column_gap: Length,
}

impl Default for OtherStyle {
    fn default() -> Self {
        Self {
            direction: Direction::LTR,
            writing_mode: WritingMode::HorizontalTb,
            overflow_x: Overflow::Visible,
            overflow_y: Overflow::Visible,
            text_align: TextAlign::Start,
            aspect_ratio: None,
            row_gap: Length::Undefined,
            column_gap: Length::Undefined,
        }
    }
}

#[derive(Debug)]
struct PositionStyle {
    pub position: Position,
    pub left: Length,
    pub right: Length,
    pub top: Length,
    pub bottom: Length,
}
impl Default for PositionStyle {
    fn default() -> Self {
        Self {
            position: Position::Relative,
            left: Length::Auto,
            right: Length::Auto,
            top: Length::Auto,
            bottom: Length::Auto,
        }
    }
}

#[derive(Debug)]
struct MarginStyle {
    pub margin_left: Length,
    pub margin_right: Length,
    pub margin_top: Length,
    pub margin_bottom: Length,
}
impl Default for MarginStyle {
    fn default() -> Self {
        Self {
            margin_left: Length::Points(Len::zero()),
            margin_right: Length::Points(Len::zero()),
            margin_top: Length::Points(Len::zero()),
            margin_bottom: Length::Points(Len::zero()),
        }
    }
}

#[derive(Debug)]
struct BorderStyle {
    pub border_left: Length,
    pub border_right: Length,
    pub border_top: Length,
    pub border_bottom: Length,
}
impl Default for BorderStyle {
    fn default() -> Self {
        Self {
            border_left: Length::Undefined,
            border_right: Length::Undefined,
            border_top: Length::Undefined,
            border_bottom: Length::Undefined,
        }
    }
}

#[derive(Debug)]
struct PaddingStyle {
    pub padding_left: Length,
    pub padding_right: Length,
    pub padding_top: Length,
    pub padding_bottom: Length,
}
impl Default for PaddingStyle {
    fn default() -> Self {
        Self {
            padding_left: Length::Undefined,
            padding_right: Length::Undefined,
            padding_top: Length::Undefined,
            padding_bottom: Length::Undefined,
        }
    }
}

#[derive(Debug)]
struct FlexStyle {
    pub order: i32,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_content: AlignContent,
    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub flex_basis: Length,
}
impl Default for FlexStyle {
    fn default() -> Self {
        Self {
            order: 0,
            flex_grow: 0.,
            flex_shrink: 1.,
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::Stretch,
            align_items: AlignItems::Stretch,
            align_self: AlignSelf::Auto,
            flex_basis: Length::Auto,
        }
    }
}

#[derive(Debug)]
struct GridStyle {
    pub grid_template_rows: LayoutGridTemplate,
    pub grid_template_columns: LayoutGridTemplate,
    pub grid_auto_flow: GridAutoFlow,
}

impl Default for GridStyle {
    fn default() -> Self {
        Self {
            grid_template_rows: LayoutGridTemplate::None,
            grid_template_columns: LayoutGridTemplate::None,
            grid_auto_flow: GridAutoFlow::Row,
        }
    }
}

type StylePtr<T> = *mut T;

fn global_style_ptr<T>(style: &T) -> StylePtr<T> {
    style as *const T as StylePtr<T>
}

#[derive(Debug)]
pub(crate) struct StyleManager {
    box_style: Cell<StylePtr<BoxStyle>>,              // 0
    size_style: Cell<StylePtr<SizeStyle>>,            // 1
    size_limit_style: Cell<StylePtr<SizeLimitStyle>>, // 2
    flex_style: Cell<StylePtr<FlexStyle>>,            // 3
    grid_style: Cell<StylePtr<GridStyle>>,            // 4
    position_style: Cell<StylePtr<PositionStyle>>,    // 5
    margin_style: Cell<StylePtr<MarginStyle>>,        // 6
    border_style: Cell<StylePtr<BorderStyle>>,        // 7
    padding_style: Cell<StylePtr<PaddingStyle>>,      // 8
    other_style: Cell<StylePtr<OtherStyle>>,          // 9
    is_cloned: Cell<u16>,
}
#[derive(Clone, Copy)]
enum StyleBit {
    Box = 1 << 0,
    Size = 1 << 1,
    SizeLimit = 1 << 2,
    Flex = 1 << 3,
    Grid = 1 << 4,
    Position = 1 << 5,
    Margin = 1 << 6,
    Border = 1 << 7,
    Padding = 1 << 8,
    Other = 1 << 9,
}

impl Drop for StyleManager {
    fn drop(&mut self) {
        // check & clear cloned style
        unsafe {
            if self.style_is_cloned(StyleBit::Box) {
                drop(Box::from_raw(self.box_style.get()))
            }
            if self.style_is_cloned(StyleBit::Size) {
                drop(Box::from_raw(self.size_style.get()))
            }
            if self.style_is_cloned(StyleBit::SizeLimit) {
                drop(Box::from_raw(self.size_limit_style.get()))
            }
            if self.style_is_cloned(StyleBit::Flex) {
                drop(Box::from_raw(self.flex_style.get()))
            }
            if self.style_is_cloned(StyleBit::Grid) {
                drop(Box::from_raw(self.grid_style.get()))
            }
            if self.style_is_cloned(StyleBit::Position) {
                drop(Box::from_raw(self.position_style.get()))
            }
            if self.style_is_cloned(StyleBit::Margin) {
                drop(Box::from_raw(self.margin_style.get()))
            }
            if self.style_is_cloned(StyleBit::Border) {
                drop(Box::from_raw(self.border_style.get()))
            }
            if self.style_is_cloned(StyleBit::Padding) {
                drop(Box::from_raw(self.padding_style.get()))
            }
            if self.style_is_cloned(StyleBit::Other) {
                drop(Box::from_raw(self.other_style.get()))
            }
        }
    }
}

impl StyleManager {
    pub(crate) fn new() -> Self {
        Self {
            box_style: Cell::new(global_style_ptr(GLOBAL_BOX_STYLE.as_ref())),
            size_style: Cell::new(global_style_ptr(GLOBAL_SIZE_STYLE.as_ref())),
            size_limit_style: Cell::new(global_style_ptr(GLOBAL_SIZE_LIMIT_STYLE.as_ref())),
            flex_style: Cell::new(global_style_ptr(GLOBAL_FLEX_STYLE.as_ref())),
            grid_style: Cell::new(global_style_ptr(GLOBAL_GRID_STYLE.as_ref())),
            position_style: Cell::new(global_style_ptr(GLOBAL_POSITION_STYLE.as_ref())),
            margin_style: Cell::new(global_style_ptr(GLOBAL_MARGIN_STYLE.as_ref())),
            border_style: Cell::new(global_style_ptr(GLOBAL_BORDER_STYLE.as_ref())),
            padding_style: Cell::new(global_style_ptr(GLOBAL_PADDING_STYLE.as_ref())),
            other_style: Cell::new(global_style_ptr(GLOBAL_OTHER_STYLE.as_ref())),
            is_cloned: Cell::new(0b0),
        }
    }

    pub(crate) fn style_to_string(&self) -> String {
        format!(
            "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
            self.box_style(),
            self.size_style(),
            self.size_limit_style(),
            self.flex_style(),
            self.grid_style(),
            self.position_style(),
            self.margin_style(),
            self.padding_style(),
            self.border_style(),
            self.other_style(),
        )
    }

    pub(crate) fn mutation_to_string(&self) -> String {
        let mut s = String::new();
        if self.style_is_cloned(StyleBit::Box) {
            s.push_str(&format!("{:?}", self.box_style()));
        }
        if self.style_is_cloned(StyleBit::Size) {
            s.push_str(&format!("{:?}", self.size_style()));
        }
        if self.style_is_cloned(StyleBit::SizeLimit) {
            s.push_str(&format!("{:?}", self.size_limit_style()));
        }
        if self.style_is_cloned(StyleBit::Flex) {
            s.push_str(&format!("{:?}", self.flex_style()));
        }
        if self.style_is_cloned(StyleBit::Grid) {
            s.push_str(&format!("{:?}", self.grid_style()));
        }
        if self.style_is_cloned(StyleBit::Position) {
            s.push_str(&format!("{:?}", self.position_style()));
        }
        if self.style_is_cloned(StyleBit::Margin) {
            s.push_str(&format!("{:?}", self.margin_style()));
        }
        if self.style_is_cloned(StyleBit::Border) {
            s.push_str(&format!("{:?}", self.border_style()));
        }
        if self.style_is_cloned(StyleBit::Padding) {
            s.push_str(&format!("{:?}", self.padding_style()));
        }
        if self.style_is_cloned(StyleBit::Other) {
            s.push_str(&format!("{:?}", self.other_style()));
        }
        s
    }

    fn style_is_cloned(&self, style_bit: StyleBit) -> bool {
        self.is_cloned.get() & (style_bit as u16) != 0
    }

    fn clone_style(&self, bit: StyleBit) {
        if !self.style_is_cloned(bit) {
            match &bit {
                StyleBit::Box => {
                    self.box_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Size => {
                    self.size_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::SizeLimit => {
                    self.size_limit_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Flex => {
                    self.flex_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Grid => {
                    self.grid_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Margin => {
                    self.margin_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Padding => {
                    self.padding_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Border => {
                    self.border_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Other => {
                    self.other_style.replace(Box::into_raw(Box::default()));
                }
                StyleBit::Position => {
                    self.position_style.replace(Box::into_raw(Box::default()));
                }
            }
            self.is_cloned.replace(self.is_cloned.get() | (bit as u16));
        }
    }

    #[allow(clippy::mut_from_ref)]
    fn box_style(&self) -> &mut BoxStyle {
        unsafe { &mut *self.box_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn size_style(&self) -> &mut SizeStyle {
        unsafe { &mut *self.size_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn flex_style(&self) -> &mut FlexStyle {
        unsafe { &mut *self.flex_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn grid_style(&self) -> &mut GridStyle {
        unsafe { &mut *self.grid_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn other_style(&self) -> &mut OtherStyle {
        unsafe { &mut *self.other_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn size_limit_style(&self) -> &mut SizeLimitStyle {
        unsafe { &mut *self.size_limit_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn margin_style(&self) -> &mut MarginStyle {
        unsafe { &mut *self.margin_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn padding_style(&self) -> &mut PaddingStyle {
        unsafe { &mut *self.padding_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn border_style(&self) -> &mut BorderStyle {
        unsafe { &mut *self.border_style.get() }
    }

    #[allow(clippy::mut_from_ref)]
    fn position_style(&self) -> &mut PositionStyle {
        unsafe { &mut *self.position_style.get() }
    }

    // getter
    pub(crate) fn display(&self) -> Display {
        self.box_style().display.clone()
    }

    pub(crate) fn set_display(&self, value: Display) -> bool {
        if self.box_style().display == value {
            return false;
        }
        self.clone_style(StyleBit::Box);
        self.box_style().display = value;
        true
    }

    pub(crate) fn box_sizing(&self) -> BoxSizing {
        self.box_style().box_sizing.clone()
    }

    pub(crate) fn set_box_sizing(&self, value: BoxSizing) -> bool {
        if self.box_style().box_sizing == value {
            return false;
        }
        self.clone_style(StyleBit::Box);
        self.box_style().box_sizing = value;
        true
    }

    pub(crate) fn width(&self) -> Length {
        self.size_style().width
    }

    pub(crate) fn set_width(&self, value: Length) -> bool {
        if self.size_style().width == value {
            return false;
        }
        self.clone_style(StyleBit::Size);
        self.size_style().width = value;
        true
    }

    pub(crate) fn height(&self) -> Length {
        self.size_style().height
    }

    pub(crate) fn set_height(&self, value: Length) -> bool {
        if self.size_style().height == value {
            return false;
        }
        self.clone_style(StyleBit::Size);
        self.size_style().height = value;
        true
    }

    pub(crate) fn min_width(&self) -> Length {
        self.size_limit_style().min_width
    }

    pub(crate) fn set_min_width(&self, value: Length) -> bool {
        if self.size_limit_style().min_width == value {
            return false;
        }
        self.clone_style(StyleBit::SizeLimit);
        self.size_limit_style().min_width = value;
        true
    }

    pub(crate) fn max_width(&self) -> Length {
        self.size_limit_style().max_width
    }

    pub(crate) fn set_max_width(&self, value: Length) -> bool {
        if self.size_limit_style().max_width == value {
            return false;
        }
        self.clone_style(StyleBit::SizeLimit);
        self.size_limit_style().max_width = value;
        true
    }

    pub(crate) fn min_height(&self) -> Length {
        self.size_limit_style().min_height
    }

    pub(crate) fn set_min_height(&self, value: Length) -> bool {
        if self.size_limit_style().min_height == value {
            return false;
        }
        self.clone_style(StyleBit::SizeLimit);
        self.size_limit_style().min_height = value;
        true
    }

    pub(crate) fn max_height(&self) -> Length {
        self.size_limit_style().max_height
    }

    pub(crate) fn set_max_height(&self, value: Length) -> bool {
        if self.size_limit_style().max_height == value {
            return false;
        }
        self.clone_style(StyleBit::SizeLimit);
        self.size_limit_style().max_height = value;
        true
    }

    pub(crate) fn direction(&self) -> Direction {
        self.other_style().direction.clone()
    }

    pub(crate) fn set_direction(&self, value: Direction) -> bool {
        if self.other_style().direction == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().direction = value;
        true
    }

    pub(crate) fn writing_mode(&self) -> WritingMode {
        self.other_style().writing_mode.clone()
    }

    pub(crate) fn set_writing_mode(&self, value: WritingMode) -> bool {
        if self.other_style().writing_mode == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().writing_mode = value;
        true
    }
    #[allow(unused)]
    pub(crate) fn overflow_x(&self) -> Overflow {
        self.other_style().overflow_x.clone()
    }

    #[allow(unused)]
    pub(crate) fn set_overflow_x(&self, value: Overflow) -> bool {
        if self.other_style().overflow_x == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().overflow_x = value;
        true
    }

    #[allow(unused)]
    pub(crate) fn overflow_y(&self) -> Overflow {
        self.other_style().overflow_y.clone()
    }

    #[allow(unused)]
    pub(crate) fn set_overflow_y(&self, value: Overflow) -> bool {
        if self.other_style().overflow_y == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().overflow_y = value;
        true
    }

    pub(crate) fn text_align(&self) -> TextAlign {
        self.other_style().text_align.clone()
    }

    pub(crate) fn set_text_align(&self, value: TextAlign) -> bool {
        if self.other_style().text_align == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().text_align = value;
        true
    }

    pub(crate) fn aspect_ratio(&self) -> Option<f32> {
        self.other_style().aspect_ratio
    }

    pub(crate) fn set_aspect_ratio(&self, value: Option<f32>) -> bool {
        if self.other_style().aspect_ratio == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().aspect_ratio = value;
        true
    }

    pub(crate) fn position(&self) -> Position {
        self.position_style().position.clone()
    }

    pub(crate) fn set_position(&self, value: Position) -> bool {
        if self.position_style().position == value {
            return false;
        }
        self.clone_style(StyleBit::Position);
        self.position_style().position = value;
        true
    }

    pub(crate) fn left(&self) -> Length {
        self.position_style().left
    }

    pub(crate) fn set_left(&self, value: Length) -> bool {
        if self.position_style().left == value {
            return false;
        }
        self.clone_style(StyleBit::Position);
        self.position_style().left = value;
        true
    }

    pub(crate) fn right(&self) -> Length {
        self.position_style().right
    }

    pub(crate) fn set_right(&self, value: Length) -> bool {
        if self.position_style().right == value {
            return false;
        }
        self.clone_style(StyleBit::Position);
        self.position_style().right = value;
        true
    }

    pub(crate) fn top(&self) -> Length {
        self.position_style().top
    }

    pub(crate) fn set_top(&self, value: Length) -> bool {
        if self.position_style().top == value {
            return false;
        }
        self.clone_style(StyleBit::Position);
        self.position_style().top = value;
        true
    }

    pub(crate) fn bottom(&self) -> Length {
        self.position_style().bottom
    }

    pub(crate) fn set_bottom(&self, value: Length) -> bool {
        if self.position_style().bottom == value {
            return false;
        }
        self.clone_style(StyleBit::Position);
        self.position_style().bottom = value;
        true
    }

    pub(crate) fn margin_left(&self) -> Length {
        self.margin_style().margin_left
    }

    pub(crate) fn set_margin_left(&self, value: Length) -> bool {
        if self.margin_style().margin_left == value {
            return false;
        }
        self.clone_style(StyleBit::Margin);
        self.margin_style().margin_left = value;
        true
    }

    pub(crate) fn margin_right(&self) -> Length {
        self.margin_style().margin_right
    }

    pub(crate) fn set_margin_right(&self, value: Length) -> bool {
        if self.margin_style().margin_right == value {
            return false;
        }
        self.clone_style(StyleBit::Margin);
        self.margin_style().margin_right = value;
        true
    }

    pub(crate) fn margin_top(&self) -> Length {
        self.margin_style().margin_top
    }

    pub(crate) fn set_margin_top(&self, value: Length) -> bool {
        if self.margin_style().margin_top == value {
            return false;
        }
        self.clone_style(StyleBit::Margin);
        self.margin_style().margin_top = value;
        true
    }

    pub(crate) fn margin_bottom(&self) -> Length {
        self.margin_style().margin_bottom
    }

    pub(crate) fn set_margin_bottom(&self, value: Length) -> bool {
        if self.margin_style().margin_bottom == value {
            return false;
        }
        self.clone_style(StyleBit::Margin);
        self.margin_style().margin_bottom = value;
        true
    }

    pub(crate) fn border_left(&self) -> Length {
        self.border_style().border_left
    }

    pub(crate) fn set_border_left(&self, value: Length) -> bool {
        if self.border_style().border_left == value {
            return false;
        }
        self.clone_style(StyleBit::Border);
        self.border_style().border_left = value;
        true
    }

    pub(crate) fn border_right(&self) -> Length {
        self.border_style().border_right
    }

    pub(crate) fn set_border_right(&self, value: Length) -> bool {
        if self.border_style().border_right == value {
            return false;
        }
        self.clone_style(StyleBit::Border);
        self.border_style().border_right = value;
        true
    }

    pub(crate) fn border_top(&self) -> Length {
        self.border_style().border_top
    }

    pub(crate) fn set_border_top(&self, value: Length) -> bool {
        if self.border_style().border_top == value {
            return false;
        }
        self.clone_style(StyleBit::Border);
        self.border_style().border_top = value;
        true
    }

    pub(crate) fn border_bottom(&self) -> Length {
        self.border_style().border_bottom
    }

    pub(crate) fn set_border_bottom(&self, value: Length) -> bool {
        if self.border_style().border_bottom == value {
            return false;
        }
        self.clone_style(StyleBit::Border);
        self.border_style().border_bottom = value;
        true
    }

    pub(crate) fn padding_left(&self) -> Length {
        self.padding_style().padding_left
    }

    pub(crate) fn set_padding_left(&self, value: Length) -> bool {
        if self.padding_style().padding_left == value {
            return false;
        }
        self.clone_style(StyleBit::Padding);
        self.padding_style().padding_left = value;
        true
    }

    pub(crate) fn padding_right(&self) -> Length {
        self.padding_style().padding_right
    }

    pub(crate) fn set_padding_right(&self, value: Length) -> bool {
        if self.padding_style().padding_right == value {
            return false;
        }
        self.clone_style(StyleBit::Padding);
        self.padding_style().padding_right = value;
        true
    }

    pub(crate) fn padding_top(&self) -> Length {
        self.padding_style().padding_top
    }

    pub(crate) fn set_padding_top(&self, value: Length) -> bool {
        if self.padding_style().padding_top == value {
            return false;
        }
        self.clone_style(StyleBit::Padding);
        self.padding_style().padding_top = value;
        true
    }

    pub(crate) fn padding_bottom(&self) -> Length {
        self.padding_style().padding_bottom
    }

    pub(crate) fn set_padding_bottom(&self, value: Length) -> bool {
        if self.padding_style().padding_bottom == value {
            return false;
        }
        self.clone_style(StyleBit::Padding);
        self.padding_style().padding_bottom = value;
        true
    }

    pub(crate) fn order(&self) -> i32 {
        self.flex_style().order
    }

    pub(crate) fn set_order(&self, value: i32) -> bool {
        if self.flex_style().order == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().order = value;
        true
    }

    pub(crate) fn flex_grow(&self) -> f32 {
        self.flex_style().flex_grow
    }

    pub(crate) fn set_flex_grow(&self, value: f32) -> bool {
        if self.flex_style().flex_grow == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().flex_grow = value;
        true
    }

    pub(crate) fn flex_shrink(&self) -> f32 {
        self.flex_style().flex_shrink
    }

    pub(crate) fn set_flex_shrink(&self, value: f32) -> bool {
        if self.flex_style().flex_shrink == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().flex_shrink = value;
        true
    }

    pub(crate) fn flex_direction(&self) -> FlexDirection {
        self.flex_style().flex_direction.clone()
    }

    pub(crate) fn set_flex_direction(&self, value: FlexDirection) -> bool {
        if self.flex_style().flex_direction == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().flex_direction = value;
        true
    }

    pub(crate) fn justify_content(&self) -> JustifyContent {
        self.flex_style().justify_content.clone()
    }

    pub(crate) fn set_justify_content(&self, value: JustifyContent) -> bool {
        if self.flex_style().justify_content == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().justify_content = value;
        true
    }

    pub(crate) fn align_content(&self) -> AlignContent {
        self.flex_style().align_content.clone()
    }

    pub(crate) fn set_align_content(&self, value: AlignContent) -> bool {
        if self.flex_style().align_content == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().align_content = value;
        true
    }

    pub(crate) fn flex_wrap(&self) -> FlexWrap {
        self.flex_style().flex_wrap.clone()
    }

    pub(crate) fn set_flex_wrap(&self, value: FlexWrap) -> bool {
        if self.flex_style().flex_wrap == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().flex_wrap = value;
        true
    }

    pub(crate) fn align_items(&self) -> AlignItems {
        self.flex_style().align_items.clone()
    }

    pub(crate) fn set_align_items(&self, value: AlignItems) -> bool {
        if self.flex_style().align_items == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().align_items = value;
        true
    }

    pub(crate) fn align_self(&self) -> AlignSelf {
        self.flex_style().align_self.clone()
    }

    pub(crate) fn set_align_self(&self, value: AlignSelf) -> bool {
        if self.flex_style().align_self == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().align_self = value;
        true
    }

    pub(crate) fn flex_basis(&self) -> Length {
        self.flex_style().flex_basis
    }

    pub(crate) fn set_flex_basis(&self, value: Length) -> bool {
        if self.flex_style().flex_basis == value {
            return false;
        }
        self.clone_style(StyleBit::Flex);
        self.flex_style().flex_basis = value;
        true
    }

    pub(crate) fn row_gap(&self) -> Length {
        self.other_style().row_gap.clone()
    }

    pub(crate) fn set_row_gap(&self, value: Length) -> bool {
        if self.other_style().row_gap == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().row_gap = value;
        true
    }

    pub(crate) fn column_gap(&self) -> Length {
        self.other_style().column_gap.clone()
    }

    pub(crate) fn set_column_gap(&self, value: Length) -> bool {
        if self.other_style().column_gap == value {
            return false;
        }
        self.clone_style(StyleBit::Other);
        self.other_style().column_gap = value;
        true
    }

    pub(crate) fn grid_template_rows(&self) -> LayoutGridTemplate {
        self.grid_style().grid_template_rows.clone()
    }

    pub(crate) fn set_grid_template_rows(&self, value: LayoutGridTemplate) -> bool {
        if self.grid_style().grid_template_rows == value {
            return false;
        }
        self.clone_style(StyleBit::Grid);
        self.grid_style().grid_template_rows = value;
        true
    }

    pub(crate) fn grid_template_columns(&self) -> LayoutGridTemplate {
        self.grid_style().grid_template_columns.clone()
    }

    pub(crate) fn set_grid_template_columns(&self, value: LayoutGridTemplate) -> bool {
        if self.grid_style().grid_template_columns == value {
            return false;
        }
        self.clone_style(StyleBit::Grid);
        self.grid_style().grid_template_columns = value;
        true
    }

    pub(crate) fn grid_auto_flow(&self) -> GridAutoFlow {
        self.grid_style().grid_auto_flow.clone()
    }

    pub(crate) fn set_grid_auto_flow(&self, value: GridAutoFlow) -> bool {
        if self.grid_style().grid_auto_flow == value {
            return false;
        }
        self.clone_style(StyleBit::Grid);
        self.grid_style().grid_auto_flow = value;
        true
    }
}
