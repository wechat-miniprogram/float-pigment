use float_pigment_css::{typing::*, StyleSheet, StyleSheetGroup};

mod utils;
use utils::*;

macro_rules! test_parse_property {
    ($prop: ident, $prop_name: expr, $str_value: expr, $value: expr) => {{
        let name = $prop_name;
        let value = $value;
        let str_value = $str_value;
        let style_str = format!(".a{{{}:{};}}", name, str_value);
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(&style_str);
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.$prop(), value);
    }};
}

#[test]
pub fn env() {
    test_parse_property!(
        margin_left,
        "margin-left",
        "env(safe-area-inset-bottom, 10px)",
        Length::Expr(LengthExpr::Env(
            "safe-area-inset-bottom".into(),
            Box::new(Length::Px(10.))
        ))
    );

    test_parse_property!(
        margin_left,
        "margin-left",
        "env(safe-area-inset-bottom, 10p)",
        Length::Expr(LengthExpr::Env(
            "safe-area-inset-bottom".into(),
            Box::new(Length::Px(0.))
        ))
    );
}
