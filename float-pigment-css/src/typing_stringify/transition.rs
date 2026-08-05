use super::*;

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
                TransitionPropertyItem::TextUnderlineOffset => "text-underline-offset",
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

