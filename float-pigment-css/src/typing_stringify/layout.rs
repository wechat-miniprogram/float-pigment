use super::*;

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::Block => "block",
                Self::Flex => "flex",
                Self::Inline => "inline",
                Self::InlineBlock => "inline-block",
                Self::Grid => "grid",
                Self::FlowRoot => "flow-root",
                Self::InlineFlex => "inline-flex",
                Self::InlineGrid => "inline-grid",
            }
        )
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::Static => "static",
                Position::Relative => "relative",
                Position::Absolute => "absolute",
                Position::Fixed => "fixed",
                Position::Sticky => "sticky",
            }
        )
    }
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Overflow::Visible => "visible",
                Overflow::Hidden => "hidden",
                Overflow::Auto => "auto",
                Overflow::Scroll => "scroll",
            }
        )
    }
}
impl fmt::Display for OverflowWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OverflowWrap::Normal => "normal",
                OverflowWrap::BreakWord => "break-word",
            }
        )
    }
}
impl fmt::Display for PointerEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PointerEvents::Auto => "auto",
                PointerEvents::None => "none",
                PointerEvents::WxRoot => "root",
            }
        )
    }
}
impl fmt::Display for WxEngineTouchEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WxEngineTouchEvent::Gesture => "gesture",
                WxEngineTouchEvent::Click => "click",
                WxEngineTouchEvent::None => "none",
            }
        )
    }
}
impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Visibility::Visible => "visible",
                Visibility::Hidden => "hidden",
                Visibility::Collapse => "collapse",
            }
        )
    }
}
impl fmt::Display for FlexWrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexWrap::NoWrap => "nowrap",
                FlexWrap::Wrap => "wrap",
                FlexWrap::WrapReverse => "wrap-reverse",
            }
        )
    }
}
impl fmt::Display for FlexDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FlexDirection::Row => "row",
                FlexDirection::RowReverse => "row-reverse",
                FlexDirection::Column => "column",
                FlexDirection::ColumnReverse => "column-reverse",
            }
        )
    }
}
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Auto => "auto",
                Direction::LTR => "ltr",
                Direction::RTL => "rtl",
            }
        )
    }
}
impl fmt::Display for WritingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WritingMode::HorizontalTb => "horizontal-tb",
                WritingMode::VerticalLr => "vertical-lr",
                WritingMode::VerticalRl => "vertical-rl",
            }
        )
    }
}
impl fmt::Display for AlignItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AlignItems::Stretch => "stretch",
                AlignItems::Normal => "normal",
                AlignItems::Center => "center",
                AlignItems::Start => "start",
                AlignItems::End => "end",
                AlignItems::FlexStart => "flex-start",
                AlignItems::FlexEnd => "flex-end",
                AlignItems::SelfStart => "self-start",
                AlignItems::SelfEnd => "self-end",
                AlignItems::Baseline => "baseline",
            }
        )
    }
}
impl fmt::Display for AlignSelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AlignSelf::Auto => "auto",
                AlignSelf::Normal => "normal",
                AlignSelf::Stretch => "stretch",
                AlignSelf::Center => "center",
                AlignSelf::Start => "start",
                AlignSelf::End => "end",
                AlignSelf::SelfStart => "self-start",
                AlignSelf::SelfEnd => "self-end",
                AlignSelf::FlexStart => "flex-start",
                AlignSelf::FlexEnd => "flex-end",
                AlignSelf::Baseline => "baseline",
            }
        )
    }
}
impl fmt::Display for AlignContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AlignContent::Normal => "normal",
                AlignContent::Start => "start",
                AlignContent::End => "end",
                AlignContent::Stretch => "stretch",
                AlignContent::Center => "center",
                AlignContent::FlexStart => "flex-start",
                AlignContent::FlexEnd => "flex-end",
                AlignContent::SpaceBetween => "space-between",
                AlignContent::SpaceAround => "space-around",
                AlignContent::SpaceEvenly => "space-evenly",
                AlignContent::Baseline => "baseline",
            }
        )
    }
}
impl fmt::Display for JustifyContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JustifyContent::Center => "center",
                JustifyContent::FlexStart => "flex-start",
                JustifyContent::FlexEnd => "flex-end",
                JustifyContent::SpaceBetween => "space-between",
                JustifyContent::SpaceAround => "space-around",
                JustifyContent::SpaceEvenly => "space-evenly",
                JustifyContent::Start => "start",
                JustifyContent::End => "end",
                JustifyContent::Left => "left",
                JustifyContent::Right => "right",
                JustifyContent::Stretch => "stretch",
                JustifyContent::Baseline => "baseline",
                JustifyContent::Normal => "normal",
            }
        )
    }
}
impl fmt::Display for JustifyItems {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JustifyItems::Stretch => "stretch",
                JustifyItems::Center => "center",
                JustifyItems::Start => "start",
                JustifyItems::End => "end",
                JustifyItems::FlexStart => "flex-start",
                JustifyItems::FlexEnd => "flex-end",
                JustifyItems::SelfStart => "self-start",
                JustifyItems::SelfEnd => "self-end",
                JustifyItems::Left => "left",
                JustifyItems::Right => "right",
                JustifyItems::Normal => "normal",
            }
        )
    }
}
impl fmt::Display for JustifySelf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JustifySelf::Auto => "auto",
                JustifySelf::Normal => "normal",
                JustifySelf::Stretch => "stretch",
                JustifySelf::Center => "center",
                JustifySelf::Start => "start",
                JustifySelf::End => "end",
                JustifySelf::FlexStart => "flex-start",
                JustifySelf::FlexEnd => "flex-end",
                JustifySelf::SelfStart => "self-start",
                JustifySelf::SelfEnd => "self-end",
                JustifySelf::Left => "left",
                JustifySelf::Right => "right",
            }
        )
    }
}
impl fmt::Display for TextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAlign::Left => "left",
                TextAlign::Center => "center",
                TextAlign::Right => "right",
                TextAlign::Justify => "justify",
                TextAlign::JustifyAll => "justify-all",
                TextAlign::Start => "start",
                TextAlign::End => "end",
                TextAlign::MatchParent => "match-parent",
            }
        )
    }
}

impl fmt::Display for WordBreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WordBreak::BreakWord => "break-word",
                WordBreak::BreakAll => "break-all",
                WordBreak::KeepAll => "keep-all",
            }
        )
    }
}

impl fmt::Display for WhiteSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WhiteSpace::Normal => "normal",
                WhiteSpace::NoWrap => "nowrap",
                WhiteSpace::Pre => "pre",
                WhiteSpace::PreWrap => "pre-wrap",
                WhiteSpace::PreLine => "pre-line",
                WhiteSpace::WxPreEdit => "-wx-pre-edit",
            }
        )
    }
}

impl fmt::Display for TextOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextOverflow::Clip => "clip",
                TextOverflow::Ellipsis => "ellipsis",
            }
        )
    }
}
impl fmt::Display for VerticalAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VerticalAlign::Baseline => "baseline",
                VerticalAlign::Top => "top",
                VerticalAlign::Middle => "middle",
                VerticalAlign::Bottom => "bottom",
                VerticalAlign::TextTop => "text-top",
                VerticalAlign::TextBottom => "text-bottom",
            }
        )
    }
}

impl fmt::Display for BoxSizing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoxSizing::ContentBox => "content-box",
                BoxSizing::PaddingBox => "padding-box",
                BoxSizing::BorderBox => "border-box",
            }
        )
    }
}
impl fmt::Display for BorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BorderStyle::None => "none",
                BorderStyle::Solid => "solid",
                BorderStyle::Dotted => "dotted",
                BorderStyle::Dashed => "dashed",
                BorderStyle::Hidden => "hidden",
                BorderStyle::Double => "double",
                BorderStyle::Groove => "groove",
                BorderStyle::Ridge => "ridge",
                BorderStyle::Inset => "inset",
                BorderStyle::Outset => "outset",
            }
        )
    }
}

impl fmt::Display for Scrollbar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Scrollbar::Auto => "auto",
                Scrollbar::Hidden => "hidden",
                Scrollbar::AutoHide => "auto-hide",
                Scrollbar::AlwaysShow => "always-show",
            }
        )
    }
}


impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Float::None => "none",
                Float::Left => "left",
                Float::Right => "right",
                Float::InlineStart => "inline-start",
                Float::InlineEnd => "inline-end",
            }
        )
    }
}

impl fmt::Display for Resize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Resize::None => "none",
                Resize::Both => "both",
                Resize::Horizontal => "horizontal",
                Resize::Vertical => "vertical",
                Resize::Block => "block",
                Resize::Inline => "inline",
            }
        )
    }
}
impl fmt::Display for ZIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                ZIndex::Auto => "auto",
                ZIndex::Num(a) => {
                    x = format!("{a}");
                    &x
                }
            }
        )
    }
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AspectRatio::Auto => write!(f, "auto"),
            AspectRatio::Ratio(x, y) => write!(f, "{x} / {y}"),
        }
    }
}

impl fmt::Display for Contain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Contain::None => write!(f, "none"),
            Contain::Content => write!(f, "content"),
            Contain::Strict => write!(f, "strict"),
            Contain::Multiple(v) => {
                let mut ret = vec![];
                v.iter().for_each(|key| match key {
                    ContainKeyword::Layout => ret.push("layout"),
                    ContainKeyword::Paint => ret.push("paint"),
                    ContainKeyword::Size => ret.push("size"),
                    ContainKeyword::Style => ret.push("style"),
                });
                write!(f, "{}", ret.join(" "))
            }
        }
    }
}


impl fmt::Display for Gap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gap::Normal => write!(f, "normal"),
            Gap::Length(length) => write!(f, "{length}"),
        }
    }
}


impl fmt::Display for GridTemplate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridTemplate::None => write!(f, "none"),
            GridTemplate::TrackList(list) => {
                let mut ret = vec![];
                list.iter().for_each(|x| match x {
                    TrackListItem::LineNames(line_names) => ret.push(
                        line_names
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<_>>()
                            .join(" "),
                    ),
                    TrackListItem::TrackSize(track_size) => ret.push(track_size.to_string()),
                });
                write!(f, "{}", ret.join(" "))
            }
        }
    }
}

impl fmt::Display for GridAutoFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridAutoFlow::Row => write!(f, "row"),
            GridAutoFlow::Column => write!(f, "column"),
            GridAutoFlow::RowDense => write!(f, "row dense"),
            GridAutoFlow::ColumnDense => write!(f, "column dense"),
        }
    }
}

impl fmt::Display for GridAuto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridAuto::List(list) => {
                let mut ret = vec![];
                list.iter().for_each(|x| ret.push(x.to_string()));
                write!(f, "{}", ret.join(" "))
            }
        }
    }
}

impl fmt::Display for TouchAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TouchAction::Auto => write!(f, "auto"),
            TouchAction::None => write!(f, "none"),
            TouchAction::Manipulation => write!(f, "manipulation"),
            TouchAction::Gestures(ges) => write!(f, "{}", ges),
        }
    }
}

impl fmt::Display for TouchActionGestures {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            pan_left,
            pan_right,
            pan_up,
            pan_down,
        } = self;
        let pan_x = if *pan_left && *pan_right {
            "pan-x"
        } else if *pan_left {
            "pan-left"
        } else if *pan_right {
            "pan-right"
        } else {
            ""
        };
        let pan_y = if *pan_up && *pan_down {
            "pan-y"
        } else if *pan_up {
            "pan-up"
        } else if *pan_down {
            "pan-down"
        } else {
            ""
        };
        let s: Vec<_> = [pan_x, pan_y]
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect();
        write!(f, "{}", s.join(" "))
    }
}
