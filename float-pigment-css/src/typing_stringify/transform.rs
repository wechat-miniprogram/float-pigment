use super::*;

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

