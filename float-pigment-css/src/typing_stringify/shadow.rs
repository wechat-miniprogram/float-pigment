use super::*;

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
