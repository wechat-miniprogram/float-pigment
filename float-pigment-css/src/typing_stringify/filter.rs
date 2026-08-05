use super::*;

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
