use super::*;

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
