use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::sheet::borrow::Array;
use crate::typing::*;
use core::fmt;
use cssparser::ToCss;

fn generate_array_str<T: fmt::Display>(array: &Array<T>) -> String {
    let mut str = String::new();
    for index in 0..array.len() {
        str.push_str(&array[index].to_string());
        if index + 1 < array.len() {
            str.push_str(", ");
        }
    }
    str
}

impl fmt::Display for CalcExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{num}"),
            Self::Angle(angle) => write!(f, "{angle}"),
            Self::Length(length) => write!(f, "{length}"),
            Self::Div(lhs, rhs) => write!(f, "{lhs}/{rhs}"),
            Self::Mul(lhs, rhs) => write!(f, "{lhs}*{rhs}"),
            Self::Plus(lhs, rhs) => write!(f, "{lhs} + {rhs}"),
            Self::Sub(lhs, rhs) => write!(f, "{lhs} - {rhs}"),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Number::F32(a) => a.to_string(),
                Number::I32(a) => a.to_string(),
                Number::Calc(expr) => expr.to_string(),
            }
        )
    }
}
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
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut color = String::new();
        write!(
            f,
            "{}",
            match self {
                Self::Undefined => "undefined",
                Self::CurrentColor => "currentcolor",
                Self::Specified(r, g, b, a) => {
                    let rgba = cssparser_color::RgbaLegacy::new(*r, *g, *b, (*a) as f32 / 255.);
                    cssparser_color::Color::Rgba(rgba).to_css(&mut color)?;
                    let str = match color.as_str() {
                        "rgb(0, 0, 0)" => "black",
                        "rgb(192, 192, 192)" => "silver",
                        "rgb(128, 128, 128)" => "gray",
                        "rgb(255, 255, 255)" => "white",
                        "rgb(128, 0, 0)" => "maroon",
                        "rgb(255, 0, 0)" => "red",
                        "rgb(128, 0, 128)" => "purple",
                        "rgb(255, 0, 255)" => "fuchsia",
                        "rgb(0, 128, 0)" => "green",
                        "rgb(0, 255, 0)" => "lime",
                        "rgb(128, 128, 0)" => "olive",
                        "rgb(255, 255, 0)" => "yellow",
                        "rgb(0, 0, 128)" => "navy",
                        "rgb(0, 0, 255)" => "blue",
                        "rgb(0, 128, 128)" => "teal",
                        "rgb(0, 255, 255)" => "aqua",
                        "rgb(240, 248, 255)" => "aliceblue",
                        "rgb(250, 235, 215)" => "antiquewhite",
                        "rgb(127, 255, 212)" => "aquamarine",
                        "rgb(240, 255, 255)" => "azure",
                        "rgb(245, 245, 220)" => "beige",
                        "rgb(255, 228, 196)" => "bisque",
                        "rgb(255, 235, 205)" => "blanchedalmond",
                        "rgb(138, 43, 226)" => "blueviolet",
                        "rgb(165, 42, 42)" => "brown",
                        "rgb(222, 184, 135)" => "burlywood",
                        "rgb(95, 158, 160)" => "cadetblue",
                        "rgb(127, 255, 0)" => "chartreuse",
                        "rgb(210, 105, 30)" => "chocolate",
                        "rgb(255, 127, 80)" => "coral",
                        "rgb(100, 149, 237)" => "cornflowerblue",
                        "rgb(255, 248, 220)" => "cornsilk",
                        "rgb(220, 20, 60)" => "crimson",
                        // "rgb(0, 255, 255)" => "cyan",
                        "rgb(0, 0, 139)" => "darkblue",
                        "rgb(0, 139, 139)" => "darkcyan",
                        "rgb(184, 134, 11)" => "darkgoldenrod",
                        "rgb(169, 169, 169)" => "darkgray",
                        "rgb(0, 100, 0)" => "darkgreen",
                        // "rgb(169, 169, 169)" => "darkgrey",
                        "rgb(189, 183, 107)" => "darkkhaki",
                        "rgb(139, 0, 139)" => "darkmagenta",
                        "rgb(85, 107, 47)" => "darkolivegreen",
                        "rgb(255, 140, 0)" => "darkorange",
                        "rgb(153, 50, 204)" => "darkorchid",
                        "rgb(139, 0, 0)" => "darkred",
                        "rgb(233, 150, 122)" => "darksalmon",
                        "rgb(143, 188, 143)" => "darkseagreen",
                        "rgb(72, 61, 139)" => "darkslateblue",
                        "rgb(47, 79, 79)" => "darkslategray",
                        // "rgb(47, 79, 79)" => "darkslategrey",
                        "rgb(0, 206, 209)" => "darkturquoise",
                        "rgb(148, 0, 211)" => "darkviolet",
                        "rgb(255, 20, 147)" => "deeppink",
                        "rgb(0, 191, 255)" => "deepskyblue",
                        "rgb(105, 105, 105)" => "dimgray",
                        // "rgb(105, 105, 105)" => "dimgrey",
                        "rgb(30, 144, 255)" => "dodgerblue",
                        "rgb(178, 34, 34)" => "firebrick",
                        "rgb(255, 250, 240)" => "floralwhite",
                        "rgb(34, 139, 34)" => "forestgreen",
                        "rgb(220, 220, 220)" => "gainsboro",
                        "rgb(248, 248, 255)" => "ghostwhite",
                        "rgb(255, 215, 0)" => "gold",
                        "rgb(218, 165, 32)" => "goldenrod",
                        "rgb(173, 255, 47)" => "greenyellow",
                        // "rgb(128, 128, 128)" => "grey",
                        "rgb(240, 255, 240)" => "honeydew",
                        "rgb(255, 105, 180)" => "hotpink",
                        "rgb(205, 92, 92)" => "indianred",
                        "rgb(75, 0, 130)" => "indigo",
                        "rgb(255, 255, 240)" => "ivory",
                        "rgb(240, 230, 140)" => "khaki",
                        "rgb(230, 230, 250)" => "lavender",
                        "rgb(255, 240, 245)" => "lavenderblush",
                        "rgb(124, 252, 0)" => "lawngreen",
                        "rgb(255, 250, 205)" => "lemonchiffon",
                        "rgb(173, 216, 230)" => "lightblue",
                        "rgb(240, 128, 128)" => "lightcoral",
                        "rgb(224, 255, 255)" => "lightcyan",
                        "rgb(250, 250, 210)" => "lightgoldenrodyellow",
                        "rgb(211, 211, 211)" => "lightgray",
                        "rgb(144, 238, 144)" => "lightgreen",
                        // "rgb(211, 211, 211)" => "lightgrey",
                        "rgb(255, 182, 193)" => "lightpink",
                        "rgb(255, 160, 122)" => "lightsalmon",
                        "rgb(32, 178, 170)" => "lightseagreen",
                        "rgb(135, 206, 250)" => "lightskyblue",
                        // "rgb(119, 136, 153)" => "lightslategray",
                        "rgb(119, 136, 153)" => "lightslategrey",
                        "rgb(176, 196, 222)" => "lightsteelblue",
                        "rgb(255, 255, 224)" => "lightyellow",
                        "rgb(50, 205, 50)" => "limegreen",
                        // "rgb(250, 240, 230)" => "linen",
                        // "rgb(255, 0, 255)" => "magenta",
                        "rgb(102, 205, 170)" => "mediumaquamarine",
                        "rgb(0, 0, 205)" => "mediumblue",
                        "rgb(186, 85, 211)" => "mediumorchid",
                        "rgb(147, 112, 219)" => "mediumpurple",
                        "rgb(60, 179, 113)" => "mediumseagreen",
                        "rgb(123, 104, 238)" => "mediumslateblue",
                        "rgb(0, 250, 154)" => "mediumspringgreen",
                        "rgb(72, 209, 204)" => "mediumturquoise",
                        "rgb(199, 21, 133)" => "mediumvioletred",
                        "rgb(25, 25, 112)" => "midnightblue",
                        "rgb(245, 255, 250)" => "mintcream",
                        "rgb(255, 228, 225)" => "mistyrose",
                        "rgb(255, 228, 181)" => "moccasin",
                        "rgb(255, 222, 173)" => "navajowhite",
                        "rgb(253, 245, 230)" => "oldlace",
                        "rgb(107, 142, 35)" => "olivedrab",
                        "rgb(255, 165, 0)" => "orange",
                        "rgb(255, 69, 0)" => "orangered",
                        "rgb(218, 112, 214)" => "orchid",
                        "rgb(238, 232, 170)" => "palegoldenrod",
                        "rgb(152, 251, 152)" => "palegreen",
                        "rgb(175, 238, 238)" => "paleturquoise",
                        "rgb(219, 112, 147)" => "palevioletred",
                        "rgb(255, 239, 213)" => "papayawhip",
                        "rgb(255, 218, 185)" => "peachpuff",
                        "rgb(205, 133, 63)" => "peru",
                        "rgb(255, 192, 203)" => "pink",
                        "rgb(221, 160, 221)" => "plum",
                        "rgb(176, 224, 230)" => "powderblue",
                        "rgb(102, 51, 153)" => "rebeccapurple",
                        "rgb(188, 143, 143)" => "rosybrown",
                        "rgb(65, 105, 225)" => "royalblue",
                        "rgb(139, 69, 19)" => "saddlebrown",
                        "rgb(250, 128, 114)" => "salmon",
                        "rgb(244, 164, 96)" => "sandybrown",
                        "rgb(46, 139, 87)" => "seagreen",
                        "rgb(255, 245, 238)" => "seashell",
                        "rgb(160, 82, 45)" => "sienna",
                        "rgb(135, 206, 235)" => "skyblue",
                        "rgb(106, 90, 205)" => "slateblue",
                        // "rgb(112, 128, 144)" => "slategray",
                        "rgb(112, 128, 144)" => "slategrey",
                        "rgb(255, 250, 250)" => "snow",
                        "rgb(0, 255, 127)" => "springgreen",
                        "rgb(70, 130, 180)" => "steelblue",
                        "rgb(210, 180, 140)" => "tan",
                        "rgb(216, 191, 216)" => "thistle",
                        "rgb(255, 99, 71)" => "tomato",
                        "rgb(64, 224, 208)" => "turquoise",
                        "rgb(238, 130, 238)" => "violet",
                        "rgb(245, 222, 179)" => "wheat",
                        "rgb(245, 245, 245)" => "whitesmoke",
                        "rgb(154, 205, 50)" => "yellowgreen",
                        x => x,
                    };
                    color = str.to_string();
                    &color
                }
            }
        )
    }
}
impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp;
        write!(
            f,
            "{}",
            match self {
                Length::Undefined => "null",
                Length::Auto => "auto",
                Length::Px(x) => {
                    tmp = format!("{x}px");
                    &tmp
                }
                Length::Vw(x) => {
                    tmp = format!("{x}vw");
                    &tmp
                }
                Length::Vh(x) => {
                    tmp = format!("{x}vh");
                    &tmp
                }
                Length::Rem(x) => {
                    tmp = format!("{x}rem");
                    &tmp
                }
                Length::Rpx(x) => {
                    tmp = format!("{x}rpx");
                    &tmp
                }
                Length::Em(x) => {
                    tmp = format!("{x}em");
                    &tmp
                }
                Length::Ratio(x) => {
                    tmp = format!("{:.0}%", x * 100.0);
                    &tmp
                }
                Length::Expr(expr) => {
                    match expr {
                        LengthExpr::Calc(calc_expr) => {
                            tmp = calc_expr.to_string();
                            &tmp
                        }
                        _ => "not support",
                    }
                }
                Length::Vmin(x) => {
                    tmp = format!("{x}vmin");
                    &tmp
                }
                Length::Vmax(x) => {
                    tmp = format!("{x}vmax");
                    &tmp
                }
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
impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Angle::Deg(x) => {
                    format!("{x}deg")
                }
                Angle::Grad(x) => {
                    format!("{x}grad")
                }
                Angle::Rad(x) => {
                    format!("{x}rad")
                }
                Angle::Turn(x) => {
                    format!("{x}turn")
                }
                Angle::Calc(expr) => expr.to_string(),
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
impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                FontWeight::Normal => "normal",
                FontWeight::Bold => "bold",
                FontWeight::Bolder => "bolder",
                FontWeight::Lighter => "lighter",
                FontWeight::Num(a) => {
                    x = a.to_string();
                    &x
                }
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
impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                LineHeight::Normal => "normal",
                LineHeight::Length(a) => {
                    x = a.to_string();
                    &x
                }
                LineHeight::Num(a) => {
                    x = a.to_string();
                    &x
                }
            }
        )
    }
}
impl fmt::Display for FontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str: String = String::new();
        write!(
            f,
            "{}",
            match self {
                FontFamily::Names(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for FontFamilyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: String;
        write!(
            f,
            "{}",
            match self {
                FontFamilyName::Serif => "serif",
                FontFamilyName::SansSerif => "sans-serif",
                FontFamilyName::Monospace => "monospace",
                FontFamilyName::Cursive => "cursive",
                FontFamilyName::Fantasy => "fantasy",
                FontFamilyName::Title(a) => {
                    str = format!("\"{}\"", a.to_string());
                    &str
                }
                FontFamilyName::SystemUi => "system-ui",
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
impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                Transform::Series(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push(' ');
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for TransformItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                TransformItem::None => "none",
                TransformItem::Matrix(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    str = format!("matrix({})", &str);
                    &str
                }
                TransformItem::Matrix3D(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    str = format!("matrix3d({})", &str);
                    &str
                }
                TransformItem::Translate2D(x, y) => {
                    str = format!("translate({x}, {y})");
                    &str
                }
                TransformItem::Translate3D(x, y, z) => {
                    str = format!("translate3d({x}, {y}, {z})");
                    &str
                }
                TransformItem::Scale2D(x, y) => {
                    str = format!("scale({x}, {y})");
                    &str
                }
                TransformItem::Scale3D(x, y, z) => {
                    str = format!("scale3d({x}, {y}, {z})");
                    &str
                }
                TransformItem::Rotate2D(x) => {
                    str = format!("rotate({x})");
                    &str
                }
                TransformItem::Rotate3D(x, y, z, deg) => {
                    str = format!("rotate3d({x}, {y}, {z}, {deg})");
                    &str
                }
                TransformItem::Skew(x, y) => {
                    str = format!("skew({x}, {y})");
                    &str
                }
                TransformItem::Perspective(x) => {
                    str = format!("perspective({x})");
                    &str
                }
            }
        )
    }
}

impl fmt::Display for TransitionProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                TransitionProperty::List(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for TransitionPropertyItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TransitionPropertyItem::None => "none",
                TransitionPropertyItem::Transform => "transform",
                TransitionPropertyItem::TransformOrigin => "transform-origin",
                TransitionPropertyItem::LineHeight => "line-height",
                TransitionPropertyItem::Opacity => "opacity",
                TransitionPropertyItem::All => "all",
                TransitionPropertyItem::Height => "height",
                TransitionPropertyItem::Width => "width",
                TransitionPropertyItem::MinHeight => "min-height",
                TransitionPropertyItem::MaxHeight => "max-height",
                TransitionPropertyItem::MinWidth => "min-width",
                TransitionPropertyItem::MaxWidth => "max-width",
                TransitionPropertyItem::MarginTop => "margin-top",
                TransitionPropertyItem::MarginRight => "margin-right",
                TransitionPropertyItem::MarginLeft => "margin-left",
                TransitionPropertyItem::MarginBottom => "margin-bottom",
                TransitionPropertyItem::Margin => "margin",
                TransitionPropertyItem::PaddingTop => "padding-top",
                TransitionPropertyItem::PaddingRight => "padding-right",
                TransitionPropertyItem::PaddingBottom => "padding-bottom",
                TransitionPropertyItem::PaddingLeft => "padding-left",
                TransitionPropertyItem::Padding => "padding",
                TransitionPropertyItem::Top => "top",
                TransitionPropertyItem::Right => "right",
                TransitionPropertyItem::Bottom => "bottom",
                TransitionPropertyItem::Left => "left",
                TransitionPropertyItem::FlexGrow => "flex-grow",
                TransitionPropertyItem::FlexShrink => "flex-shrink",
                TransitionPropertyItem::FlexBasis => "flex-basis",
                TransitionPropertyItem::Flex => "flex",
                TransitionPropertyItem::BorderTopWidth => "border-top-width",
                TransitionPropertyItem::BorderRightWidth => "border-right-width",
                TransitionPropertyItem::BorderBottomWidth => "border-bottom-width",
                TransitionPropertyItem::BorderLeftWidth => "border-left-width",
                TransitionPropertyItem::BorderTopColor => "border-top-color",
                TransitionPropertyItem::BorderRightColor => "border-right-color",
                TransitionPropertyItem::BorderBottomColor => "border-bottom-color",
                TransitionPropertyItem::BorderLeftColor => "border-left-color",
                TransitionPropertyItem::BorderTopLeftRadius => "border-top-left-radius",
                TransitionPropertyItem::BorderTopRightRadius => "border-top-right-radius",
                TransitionPropertyItem::BorderBottomLeftRadius => "border-bottom-left-radius",
                TransitionPropertyItem::BorderBottomRightRadius => "border-bottom-right-radius",
                TransitionPropertyItem::Border => "border",
                TransitionPropertyItem::BorderWidth => "border-width",
                TransitionPropertyItem::BorderColor => "border-color",
                TransitionPropertyItem::BorderRadius => "border-radius",
                TransitionPropertyItem::BorderLeft => "border-left",
                TransitionPropertyItem::BorderTop => "border-top",
                TransitionPropertyItem::BorderRight => "border-right",
                TransitionPropertyItem::BorderBottom => "border-bottom",
                TransitionPropertyItem::Font => "font",
                TransitionPropertyItem::ZIndex => "z-index",
                TransitionPropertyItem::BoxShadow => "box-shadow",
                TransitionPropertyItem::BackdropFilter => "backdrop-filter",
                TransitionPropertyItem::Filter => "filter",
                TransitionPropertyItem::Color => "color",
                TransitionPropertyItem::TextDecorationColor => "text-decoration-color",
                TransitionPropertyItem::TextDecorationThickness => "text-decoration-thickness",
                TransitionPropertyItem::FontSize => "font-size",
                TransitionPropertyItem::FontWeight => "font-weight",
                TransitionPropertyItem::LetterSpacing => "letter-spacing",
                TransitionPropertyItem::WordSpacing => "word-spacing",
                TransitionPropertyItem::BackgroundColor => "background-color",
                TransitionPropertyItem::BackgroundPosition => "background-position",
                TransitionPropertyItem::BackgroundSize => "background-size",
                TransitionPropertyItem::Background => "background",
                TransitionPropertyItem::BackgroundPositionX => "background-position-x",
                TransitionPropertyItem::BackgroundPositionY => "background-position-y",
                TransitionPropertyItem::MaskPosition => "mask-position",
                TransitionPropertyItem::MaskPositionX => "mask-position-x",
                TransitionPropertyItem::MaskPositionY => "mask-position-y",
                TransitionPropertyItem::MaskSize => "mask-size",
                TransitionPropertyItem::Mask => "mask",
            }
        )
    }
}

impl fmt::Display for StepPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StepPosition::End => "end",
                StepPosition::JumpStart => "jump-start",
                StepPosition::JumpEnd => "jump-end",
                StepPosition::JumpNone => "jump-none",
                StepPosition::JumpBoth => "jump-both",
                StepPosition::Start => "start",
            }
        )
    }
}

impl fmt::Display for TransitionTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                TransitionTime::List(array) => {
                    for index in 0..array.len() {
                        if array[index] >= 1000 {
                            let s: u32 = array[index] / 1000;
                            str.push_str(&s.to_string());
                            str.push('s');
                        } else {
                            str.push_str(&array[index].to_string());
                            str.push_str("ms");
                        }

                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
                TransitionTime::ListI32(array) => {
                    for index in 0..array.len() {
                        if array[index] >= 1000 {
                            let s: i32 = array[index] / 1000;
                            str.push_str(&s.to_string());
                            str.push('s');
                        } else {
                            str.push_str(&array[index].to_string());
                            str.push_str("ms");
                        }

                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
            }
        )
    }
}

impl fmt::Display for TransitionTimingFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                TransitionTimingFn::List(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for TransitionTimingFnItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str;
        write!(
            f,
            "{}",
            match self {
                TransitionTimingFnItem::Linear => "linear",
                TransitionTimingFnItem::Ease => "ease",
                TransitionTimingFnItem::EaseIn => "ease-in",
                TransitionTimingFnItem::EaseOut => "ease-out",
                TransitionTimingFnItem::EaseInOut => "ease-in-out",
                TransitionTimingFnItem::StepStart => "step-start",
                TransitionTimingFnItem::StepEnd => "step-end",
                TransitionTimingFnItem::Steps(x, y) => {
                    str = format!("steps({x}, {y})");
                    &str
                }
                TransitionTimingFnItem::CubicBezier(x, y, z, a) => {
                    str = format!("cubic-bezier({x}, {y}, {z}, {a})");
                    &str
                }
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

impl fmt::Display for BackgroundRepeat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundRepeat::List(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for BackgroundRepeatItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str;
        write!(
            f,
            "{}",
            match self {
                BackgroundRepeatItem::Pos(x, y) => {
                    str = format!("{x} {y}");
                    &str
                }
            }
        )
    }
}
impl fmt::Display for BackgroundRepeatValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BackgroundRepeatValue::Repeat => "repeat",
                BackgroundRepeatValue::NoRepeat => "no-repeat",
                BackgroundRepeatValue::Space => "space",
                BackgroundRepeatValue::Round => "round",
            }
        )
    }
}

impl fmt::Display for BackgroundSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundSize::List(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for BackgroundSizeItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str;
        write!(
            f,
            "{}",
            match self {
                BackgroundSizeItem::Auto => "auto",
                BackgroundSizeItem::Length(x, y) => {
                    str = format!("{x} {y}");
                    &str
                }
                BackgroundSizeItem::Cover => "cover",
                BackgroundSizeItem::Contain => "contain",
            }
        )
    }
}
impl fmt::Display for BackgroundImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundImage::List(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push_str(", ");
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for ImageTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageTags::LTR => "ltr",
                ImageTags::RTL => "rtl",
            }
        )
    }
}
impl fmt::Display for ImageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str;
        write!(
            f,
            "{}",
            match self {
                ImageSource::None => "none",
                ImageSource::Url(x) => {
                    str = format!("url({})", x.to_string());
                    &str
                }
            }
        )
    }
}
impl fmt::Display for BackgroundImageItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tmp;
        write!(
            f,
            "{}",
            match self {
                BackgroundImageItem::None => "none",
                BackgroundImageItem::Url(x) => {
                    tmp = format!("url(\"{}\")", x.to_string());
                    &tmp
                }
                BackgroundImageItem::Gradient(x) => {
                    tmp = x.to_string();
                    &tmp
                }
                BackgroundImageItem::Image(x, y, z) => {
                    tmp = String::from("image(");
                    // ignore default LTR
                    if *x != ImageTags::LTR {
                        tmp.push_str(&x.to_string());
                        tmp.push(' ');
                    }

                    if *y != ImageSource::None {
                        tmp.push_str(&y.to_string());
                    }
                    if *z != Color::Undefined {
                        tmp.push_str(", ");
                        tmp.push_str(&z.to_string());
                    }
                    tmp.push(')');
                    &tmp
                }
                BackgroundImageItem::Element(x) => {
                    tmp = format!("element(#{})", x.to_string());
                    &tmp
                }
            }
        )
    }
}
impl fmt::Display for BackgroundImageGradientItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundImageGradientItem::LinearGradient(x, array) => {
                    str.push_str("linear-gradient(");
                    // ignore default 180
                    if *x != Angle::Deg(180.0) {
                        str.push_str(&x.to_string());
                        str.push_str(", ");
                    }
                    str.push_str(&generate_array_str(array));
                    str.push(')');
                    &str
                }
                BackgroundImageGradientItem::RadialGradient(x, y, z, array) => {
                    str = generate_array_str(array);
                    str = format!("radial-gradient({x} {y} at {z}, {str})");
                    &str
                }
                BackgroundImageGradientItem::ConicGradient(gradient) => {
                    str = format!(
                        "conic-gradient(from {} at {}, {})",
                        gradient.angle,
                        gradient.position,
                        generate_array_str(&gradient.items)
                    );
                    &str
                }
            }
        )
    }
}

impl fmt::Display for GradientSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp;
        write!(
            f,
            "{}",
            match self {
                GradientSize::ClosestSide => "closest-side",
                GradientSize::ClosestCorner => "closest-corner",
                GradientSize::FarthestSide => "farthest-side",
                GradientSize::FarthestCorner => "farthest-corner",
                GradientSize::Len(x, y) => {
                    tmp = format!("{x} {y}");
                    &tmp
                }
            }
        )
    }
}
impl fmt::Display for GradientPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp;
        // left bottom  => 0% 100%

        write!(
            f,
            "{}",
            match self {
                GradientPosition::Pos(x, y) => {
                    let horizontal_str = match x {
                        y if *y == Length::Ratio(0.5) => "center".to_string(),
                        y if *y == Length::Ratio(0.0) => "left".to_string(),
                        y if *y == Length::Ratio(1.0) => "right".to_string(),
                        x => x.to_string(),
                    };
                    let vertical_str = match y {
                        n if *n == Length::Ratio(0.5) => "center".to_string(),
                        n if *n == Length::Ratio(0.0) => "top".to_string(),
                        n if *n == Length::Ratio(1.0) => "bottom".to_string(),
                        y => y.to_string(),
                    };

                    if horizontal_str == vertical_str {
                        "center"
                    } else {
                        tmp = format!("{horizontal_str} {vertical_str}");
                        &tmp
                    }
                }
                GradientPosition::SpecifiedPos(x, y) => {
                    let horizontal_str = match x {
                        GradientSpecifiedPos::Left(v) => format!("left {}", v),
                        GradientSpecifiedPos::Right(v) => format!("right {}", v),
                        GradientSpecifiedPos::Top(v) => format!("top {}", v),
                        GradientSpecifiedPos::Bottom(v) => format!("bottom {}", v),
                    };

                    let vertical_str = match y {
                        GradientSpecifiedPos::Left(v) => format!("left {}", v),
                        GradientSpecifiedPos::Right(v) => format!("right {}", v),
                        GradientSpecifiedPos::Top(v) => format!("top {}", v),
                        GradientSpecifiedPos::Bottom(v) => format!("bottom {}", v),
                    };
                    tmp = format!("{horizontal_str} {vertical_str}");
                    &tmp
                }
            }
        )
    }
}
impl fmt::Display for GradientShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GradientShape::Ellipse => "ellipse",
                GradientShape::Circle => "circle",
            }
        )
    }
}
impl fmt::Display for GradientColorItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tmp = String::new();
        write!(
            f,
            "{}",
            match self {
                GradientColorItem::ColorHint(color, len) => {
                    tmp.push_str(&color.to_string());
                    // ignore auto
                    if *len != Length::Auto {
                        tmp.push(' ');
                        tmp.push_str(&len.to_string());
                    }
                    &tmp
                }
                GradientColorItem::SimpleColorHint(color) => {
                    tmp.push_str(&color.to_string());
                    &tmp
                }
                GradientColorItem::AngleOrPercentageColorHint(color, angle_or_percentage) => {
                    tmp.push_str(&color.to_string());
                    tmp.push(' ');
                    match angle_or_percentage {
                        AngleOrPercentage::Angle(angle) => {
                            tmp.push_str(&angle.to_string());
                        }
                        AngleOrPercentage::Percentage(percentage) => {
                            tmp.push_str(&percentage.to_string());
                        }
                    }
                    &tmp
                }
            }
        )
    }
}
impl fmt::Display for BackgroundPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundPosition::List(items) => {
                    for index in 0..items.len() {
                        x.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            x.push_str(", ");
                        }
                    }
                    &x
                }
            }
        )
    }
}
impl fmt::Display for BackgroundPositionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundPositionItem::Pos(x, y) => {
                    let horizontal_str = &x.to_string();
                    let vertical_str = &y.to_string();
                    if *horizontal_str == "center" && *vertical_str == "center" {
                        str.push_str("center");
                    } else if vertical_str == "center" {
                        str.push_str(horizontal_str);
                    } else {
                        str = format!("{horizontal_str} {vertical_str}");
                    }
                    &str
                }
                BackgroundPositionItem::Value(v) => {
                    str = v.to_string();
                    &str
                }
            }
        )
    }
}
impl fmt::Display for BackgroundPositionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundPositionValue::Top(top) => {
                    match top {
                        n if *n == Length::Ratio(0.) => {
                            x.push_str("top");
                        }
                        n if *n == Length::Ratio(1.) => {
                            x.push_str("bottom");
                        }
                        n if *n == Length::Ratio(0.5) => {
                            x.push_str("center");
                        }
                        // top ratio not need keyword
                        Length::Ratio(ratio) => x.push_str(&Length::Ratio(*ratio).to_string()),
                        other => {
                            x = format!("{} {}", "top", &other.to_string());
                        }
                    }
                    &x
                }
                BackgroundPositionValue::Bottom(bottom) => {
                    match bottom {
                        n if *n == Length::Ratio(0.) => {
                            x.push_str("bottom");
                        }
                        n if *n == Length::Ratio(1.) => {
                            x.push_str("top");
                        }
                        n if *n == Length::Ratio(0.5) => {
                            x.push_str("center");
                        }
                        other => {
                            x = format!("{} {}", "bottom", &other.to_string());
                        }
                    }
                    &x
                }
                BackgroundPositionValue::Left(left) => {
                    match left {
                        n if *n == Length::Ratio(0.) => {
                            x.push_str("left");
                        }
                        n if *n == Length::Ratio(1.) => {
                            x.push_str("right");
                        }
                        n if *n == Length::Ratio(0.5) => {
                            x.push_str("center");
                        }
                        // left ratio not need keyword
                        Length::Ratio(ratio) => x.push_str(&Length::Ratio(*ratio).to_string()),
                        other => {
                            x = format!("{} {}", "left", &other.to_string());
                        }
                    }
                    &x
                }
                BackgroundPositionValue::Right(right) => {
                    match right {
                        n if *n == Length::Ratio(0.) => {
                            x.push_str("right");
                        }
                        n if *n == Length::Ratio(1.) => {
                            x.push_str("left");
                        }
                        n if *n == Length::Ratio(0.5) => {
                            x.push_str("center");
                        }
                        other => {
                            x = format!("{} {}", "right", &other.to_string());
                        }
                    }
                    &x
                }
            }
        )
    }
}
impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = String::new();
        write!(
            f,
            "{}",
            match self {
                FontStyle::Normal => "normal",
                FontStyle::Italic => "italic",
                FontStyle::Oblique(a) => {
                    x.push_str("oblique");
                    if *a != Angle::Deg(14.) {
                        x.push(' ');
                        x.push_str(&a.to_string());
                    }
                    &x
                }
            }
        )
    }
}
impl fmt::Display for BackgroundClip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundClip::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push_str(", ")
                        }
                    }
                    &str
                }
            }
        )
    }
}

impl fmt::Display for BackgroundClipItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BackgroundClipItem::BorderBox => "border-box",
                BackgroundClipItem::PaddingBox => "padding-box",
                BackgroundClipItem::ContentBox => "content-box",
                BackgroundClipItem::Text => "text",
            }
        )
    }
}

impl fmt::Display for BackgroundOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundOrigin::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push_str(", ")
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for BackgroundOriginItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BackgroundOriginItem::BorderBox => "border-box",
                BackgroundOriginItem::PaddingBox => "padding-box",
                BackgroundOriginItem::ContentBox => "content-box",
            }
        )
    }
}
impl fmt::Display for BackgroundAttachmentItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BackgroundAttachmentItem::Scroll => "scroll",
                BackgroundAttachmentItem::Fixed => "fixed",
                BackgroundAttachmentItem::Local => "local",
            }
        )
    }
}
impl fmt::Display for BackgroundAttachment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackgroundAttachment::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push_str(", ")
                        }
                    }
                    &str
                }
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
impl fmt::Display for ListStyleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                ListStyleType::Disc => "disc",
                ListStyleType::None => "none",
                ListStyleType::Circle => "circle",
                ListStyleType::Square => "square",
                ListStyleType::Decimal => "decimal",
                ListStyleType::CjkDecimal => "cjk-decimal",
                ListStyleType::DecimalLeadingZero => "decimal-leading-zero",
                ListStyleType::LowerRoman => "lower-roman",
                ListStyleType::UpperRoman => "upper-roman",
                ListStyleType::LowerGreek => "lower-greek",
                ListStyleType::LowerAlpha => "lower-alpha",
                ListStyleType::LowerLatin => "lower-latin",
                ListStyleType::UpperAlpha => "upper-alpha",
                ListStyleType::UpperLatin => "upper-latin",
                ListStyleType::Armenian => "armenian",
                ListStyleType::Georgian => "georgian",
                ListStyleType::CustomIdent(str) => {
                    x = str.to_string();
                    &x
                }
            }
        )
    }
}
impl fmt::Display for ListStyleImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str;
        write!(
            f,
            "{}",
            match self {
                ListStyleImage::None => "none",
                ListStyleImage::Url(x) => {
                    str = format!("url({})", x.to_string());
                    &str
                }
            }
        )
    }
}
impl fmt::Display for ListStylePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ListStylePosition::Outside => "outside",
                ListStylePosition::Inside => "inside",
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
impl fmt::Display for TextShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                TextShadow::None => "none",
                TextShadow::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push_str(", ")
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for TextShadowItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tmp = String::new();
        write!(
            f,
            "{}",
            match self {
                TextShadowItem::TextShadowValue(offsety, offsetx, blurradius, color) => {
                    if *offsety != Length::Px(0.) {
                        tmp.push_str(&offsety.to_string());
                    }
                    if *offsetx != Length::Px(0.) {
                        tmp.push(' ');
                        tmp.push_str(&offsetx.to_string());
                    }
                    if *blurradius != Length::Px(0.) && *blurradius != Length::Undefined {
                        tmp.push(' ');
                        tmp.push_str(&blurradius.to_string());
                    }
                    if *color != Color::Undefined {
                        tmp.push(' ');
                        tmp.push_str(&color.to_string());
                    }
                    &tmp
                }
            }
        )
    }
}
impl fmt::Display for TextDecorationLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                TextDecorationLine::None => "none",
                TextDecorationLine::SpellingError => "spelling-error",
                TextDecorationLine::GrammarError => "grammar-error",
                TextDecorationLine::List(array) => {
                    for index in 0..array.len() {
                        str.push_str(&array[index].to_string());
                        if index + 1 < array.len() {
                            str.push(' ');
                        }
                    }
                    &str
                }
            }
        )
    }
}

impl fmt::Display for TextDecorationLineItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextDecorationLineItem::Overline => "overline",
                TextDecorationLineItem::LineThrough => "line-through",
                TextDecorationLineItem::Underline => "underline",
                TextDecorationLineItem::Blink => "blink",
            }
        )
    }
}
impl fmt::Display for TextDecorationStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextDecorationStyle::Solid => "solid",
                TextDecorationStyle::Double => "double",
                TextDecorationStyle::Dotted => "dotted",
                TextDecorationStyle::Dashed => "dashed",
                TextDecorationStyle::Wavy => "wavy",
            }
        )
    }
}
impl fmt::Display for TextDecorationThickness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                TextDecorationThickness::Auto => "auto",
                TextDecorationThickness::FromFont => "from-font",
                TextDecorationThickness::Length(len) => {
                    x = len.to_string();
                    &x
                }
            }
        )
    }
}
impl fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                LetterSpacing::Normal => "normal",
                LetterSpacing::Length(len) => {
                    x = len.to_string();
                    &x
                }
            }
        )
    }
}
impl fmt::Display for WordSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                WordSpacing::Normal => "normal",
                WordSpacing::Length(len) => {
                    x = len.to_string();
                    &x
                }
            }
        )
    }
}
impl fmt::Display for BorderRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tmp;
        write!(
            f,
            "{}",
            match self {
                BorderRadius::Pos(x, y) => {
                    tmp = format!("{x} {y}");
                    &tmp
                }
            }
        )
    }
}
impl fmt::Display for BoxShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BoxShadow::None => "none",
                BoxShadow::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push_str(", ")
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for BoxShadowItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BoxShadowItem::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push(' ')
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for ShadowItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                ShadowItemType::Inset => "inset",
                ShadowItemType::OffsetX(len) => {
                    x = len.to_string();
                    &x
                }
                ShadowItemType::OffsetY(len) => {
                    x = len.to_string();
                    &x
                }
                ShadowItemType::BlurRadius(len) => {
                    x = len.to_string();
                    &x
                }
                ShadowItemType::SpreadRadius(len) => {
                    x = len.to_string();
                    &x
                }
                ShadowItemType::Color(len) => {
                    x = len.to_string();
                    &x
                }
            }
        )
    }
}
impl fmt::Display for BackdropFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                BackdropFilter::None => "none",
                BackdropFilter::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push(' ')
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                Filter::None => "none",
                Filter::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push(' ')
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for FilterFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                FilterFunc::Url(len) => {
                    x = format!("url({})", len.to_string());
                    &x
                }
                FilterFunc::Blur(len) => {
                    x = format!("blur({len})");
                    &x
                }
                FilterFunc::Brightness(len) => {
                    x = format!("brightness({len})");
                    &x
                }
                FilterFunc::Contrast(len) => {
                    x = format!("contranst({len})");
                    &x
                }
                FilterFunc::DropShadow(len) => {
                    x = format!("drop-shadow({len})");
                    &x
                }
                FilterFunc::Grayscale(len) => {
                    x = format!("grayscale({len})");
                    &x
                }
                FilterFunc::HueRotate(len) => {
                    x = format!("hue-rotate({len})");
                    &x
                }
                FilterFunc::Invert(len) => {
                    x = format!("invert({len})");
                    &x
                }
                FilterFunc::Opacity(len) => {
                    x = format!("opacity({len})");
                    &x
                }
                FilterFunc::Saturate(len) => {
                    x = format!("saturate({len})");
                    &x
                }
                FilterFunc::Sepia(len) => {
                    x = format!("sepia({len})");
                    &x
                }
            }
        )
    }
}
impl fmt::Display for DropShadow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                DropShadow::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push(' ')
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for TransformOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tmp = String::new();
        write!(
            f,
            "{}",
            match self {
                TransformOrigin::LengthTuple(x, y, z) => {
                    let horizontal_str = match x {
                        n if *n == Length::Ratio(0.5) => "center".to_string(),
                        n if *n == Length::Ratio(0.0) => "left".to_string(),
                        n if *n == Length::Ratio(1.0) => "right".to_string(),
                        x => x.to_string(),
                    };
                    let vertical_str = match y {
                        n if *n == Length::Ratio(0.5) => "center".to_string(),
                        n if *n == Length::Ratio(0.0) => "top".to_string(),
                        n if *n == Length::Ratio(1.0) => "bottom".to_string(),
                        y => y.to_string(),
                    };

                    if horizontal_str == vertical_str {
                        tmp.push_str("center");
                    } else {
                        tmp = format!("{horizontal_str} {vertical_str}");
                    }

                    match z {
                        n if *n == Length::Px(0.) => {}
                        y => {
                            tmp.push(' ');
                            tmp.push_str(&y.to_string())
                        }
                    }

                    &tmp
                }
                TransformOrigin::Left => "left",
                TransformOrigin::Right => "right",
                TransformOrigin::Center => "center",
                TransformOrigin::Bottom => "bottom",
                TransformOrigin::Top => "top",
                TransformOrigin::Length(len) => {
                    tmp = len.to_string();
                    &tmp
                }
            }
        )
    }
}

impl fmt::Display for MaskMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        write!(
            f,
            "{}",
            match self {
                MaskMode::List(items) => {
                    for index in 0..items.len() {
                        str.push_str(&items[index].to_string());
                        if index < items.len() - 1 {
                            str.push_str(", ")
                        }
                    }
                    &str
                }
            }
        )
    }
}
impl fmt::Display for MaskModeItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MaskModeItem::MatchSource => "match-source",
                MaskModeItem::Alpha => "alpha",
                MaskModeItem::Luminance => "luminance",
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

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::None => write!(f, "none"),
            Content::Normal => write!(f, "normal"),
            Content::Str(x) => write!(f, "'{}'", x.to_string()),
            Content::Url(x) => write!(f, "'{}'", x.to_string()),
        }
    }
}

impl fmt::Display for CustomProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomProperty::None => write!(f, "none"),
            CustomProperty::Expr(key, value) => {
                write!(f, "{}:{}", key.to_string(), value.to_string())
            }
        }
    }
}

impl fmt::Display for AnimationName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = vec![];
        match self {
            AnimationName::List(list) => list.iter().for_each(|x| match x {
                AnimationNameItem::None => ret.push("none".to_string()),
                AnimationNameItem::CustomIdent(ident) => ret.push(ident.to_string()),
            }),
        }
        write!(f, "{}", ret.join(","))
    }
}

impl fmt::Display for AnimationDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret: Vec<&str> = vec![];
        match self {
            AnimationDirection::List(list) => list.iter().for_each(|x| match x {
                AnimationDirectionItem::Normal => ret.push("normal"),
                AnimationDirectionItem::Alternate => ret.push("alternate"),
                AnimationDirectionItem::AlternateReverse => ret.push("alternate-reverse"),
                AnimationDirectionItem::Reverse => ret.push("reverse"),
            }),
        }
        write!(f, "{}", ret.join(","))
    }
}

impl fmt::Display for AnimationFillMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = vec![];
        match self {
            AnimationFillMode::List(list) => list.iter().for_each(|x| match x {
                AnimationFillModeItem::None => ret.push("none"),
                AnimationFillModeItem::Forwards => ret.push("forwards"),
                AnimationFillModeItem::Backwards => ret.push("backwords"),
                AnimationFillModeItem::Both => ret.push("both"),
            }),
        }
        write!(f, "{}", ret.join(","))
    }
}

impl fmt::Display for AnimationIterationCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = vec![];
        match self {
            AnimationIterationCount::List(list) => list.iter().for_each(|x| match x {
                AnimationIterationCountItem::Infinite => ret.push("infinite".to_string()),
                AnimationIterationCountItem::Number(num) => ret.push(num.to_string()),
            }),
        }
        write!(f, "{}", ret.join(","))
    }
}

impl fmt::Display for AnimationPlayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = vec![];
        match self {
            AnimationPlayState::List(list) => list.iter().for_each(|x| match x {
                AnimationPlayStateItem::Running => ret.push("running"),
                AnimationPlayStateItem::Paused => ret.push("paused"),
            }),
        }
        write!(f, "{}", ret.join(","))
    }
}

impl fmt::Display for WillChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = vec![];
        match self {
            WillChange::Auto => ret.push("auto".to_string()),
            WillChange::List(list) => list.iter().for_each(|feature| match feature {
                AnimateableFeature::Contents => ret.push("contents".to_string()),
                AnimateableFeature::ScrollPosition => ret.push("scroll-position".to_string()),
                AnimateableFeature::CustomIdent(x) => ret.push(x.to_string()),
            }),
        };
        write!(f, "{}", ret.join(","))
    }
}

impl fmt::Display for FontFeatureSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ret = vec![];
        match self {
            FontFeatureSettings::Normal => ret.push("normal".to_string()),
            FontFeatureSettings::FeatureTags(tags) => tags.iter().for_each(|feature_tag_value| {
                ret.push(format!(
                    "{} {}",
                    feature_tag_value.opentype_tag.to_string(),
                    feature_tag_value.value
                ));
            }),
        };
        write!(f, "{}", ret.join(","))
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

impl fmt::Display for TrackSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrackSize::Length(length) => write!(f, "{length}"),
            TrackSize::MinContent => write!(f, "min-content"),
            TrackSize::MaxContent => write!(f, "max-content"),
            TrackSize::Fr(x) => write!(f, "{x}fr"),
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
