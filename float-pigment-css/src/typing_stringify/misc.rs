use super::*;

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
impl fmt::Display for TextUnderlineOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x;
        write!(
            f,
            "{}",
            match self {
                TextUnderlineOffset::Auto => "auto",
                TextUnderlineOffset::Length(len) => {
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

