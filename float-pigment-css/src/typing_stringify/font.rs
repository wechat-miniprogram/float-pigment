use super::*;

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

