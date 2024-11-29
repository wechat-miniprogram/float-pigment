use float_pigment_css::{typing::*, StyleSheet, StyleSheetGroup};

mod utils;
use utils::*;

macro_rules! test_parse_property {
    ($prop: ident, $prop_name: expr, $str_value: expr, $value: expr) => {{
        let name = $prop_name;
        let value = $value;
        let str_value = $str_value;
        let style_str = format!(".a{{{}:{}}}", name, str_value);
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(&style_str);
        ssg.append(ss);
        let np = query(&ssg, "", "", ["a"], []);
        assert_eq!(np.$prop(), value);
    }};
}

#[test]
pub fn var_func() {
    test_parse_property!(
        width_type,
        "width",
        "var(--hello)",
        LengthType::Var(Box::new("var(--hello)".into()))
    );
    test_parse_property!(
        height_type,
        "height",
        "var(--hello-world)",
        LengthType::Var(Box::new("var(--hello-world)".into()))
    );
    test_parse_property!(
        height_type,
        "height",
        "var(--hello-world, 10px)",
        LengthType::Var(Box::new("var(--hello-world, 10px)".into()),)
    );
    test_parse_property!(
        margin_left_type,
        "margin-left",
        "var(--b)",
        LengthType::Var(Box::new("var(--b)".into()))
    );
    test_parse_property!(
        margin_left_type,
        "margin-left",
        "var(--b) !important",
        LengthType::Var(Box::new("var(--b) ".into()))
    );
    test_parse_property!(
        margin_right_type,
        "margin",
        "var(--b)",
        LengthType::VarInShorthand(Box::new("margin".into()), Box::new("var(--b)".into()))
    );
    test_parse_property!(
        margin_right_type,
        "margin",
        "var(--b) !important",
        LengthType::VarInShorthand(Box::new("margin".into()), Box::new("var(--b) ".into()))
    );
    test_parse_property!(
        padding_left_type,
        "padding",
        "var(--td-drawer-item-padding, 32rpx) var(--td-drawer-item-padding, 32rpx) var(--td-drawer-item-padding, 32rpx) var(--td-drawer-item-padding, 32rpx)",
        LengthType::VarInShorthand(Box::new("padding".into()), Box::new("var(--td-drawer-item-padding, 32rpx) var(--td-drawer-item-padding, 32rpx) var(--td-drawer-item-padding, 32rpx) var(--td-drawer-item-padding, 32rpx)".into()))
    )
}

#[test]
pub fn background_color_var() {
    test_parse_property!(
        background_color_type,
        "background-color",
        "var(--hello, var(--yellow))",
        ColorType::Var(Box::new("var(--hello, var(--yellow))".into()),)
    );
}

#[test]
pub fn custom_ident() {
    test_parse_property!(
        custom_property,
        "--hello",
        "10px",
        CustomProperty::Expr("--hello".into(), "10px".into())
    );
}

#[test]
pub fn custom_property_important() {
    test_parse_property!(
        custom_property,
        "--hello",
        "1px !important",
        CustomProperty::Expr("--hello".into(), "1px ".into())
    );
    test_parse_property!(
        custom_property,
        "--a",
        "var(--b) !important !important",
        CustomProperty::None
    );
    test_parse_property!(
        custom_property,
        "--a",
        " ",
        CustomProperty::Expr("--a".into(), " ".into())
    )
}

#[test]
pub fn custom_property_with_var() {
    test_parse_property!(
        custom_property,
        "--hello",
        "var(--b)",
        CustomProperty::Expr("--hello".into(), "var(--b)".into())
    )
}

#[test]
pub fn transform() {
    test_parse_property!(
        transform_type,
        "transform",
        "translate(var(--a), var(--b))",
        TransformType::Var(Box::new("translate(var(--a), var(--b))".into()))
    )
}

#[test]
pub fn calc() {
    test_parse_property!(
        width_type,
        "width",
        "calc(var(--b))",
        LengthType::Var(Box::new("calc(var(--b))".into()))
    );
    test_parse_property!(
        custom_property,
        "--hello",
        "40% + 10px * 2 + 20% / 2",
        CustomProperty::Expr("--hello".into(), "40% + 10px * 2 + 20% / 2".into())
    )
}

#[test]
pub fn end_with_semicolon() {
    test_parse_property!(
        width_type,
        "width",
        "var(10px);",
        LengthType::Var(Box::new("var(10px)".into()),)
    );
    test_parse_property!(
        custom_property,
        "--a",
        "var(10px);",
        CustomProperty::Expr("--a".into(), "var(10px)".into())
    );
}

#[test]
pub fn multi_rules() {
    let ss = StyleSheet::from_str(
        r#"
            .calc_1{--cale-1:calc(50% + 10px + 10%);height:var(--cale-1)}
            .calc_2{--cale-2:40% + 10px * 2 + 20% / 2;height:calc(var(--cale-2))}
            .calc_3{--cale-2:40% + 10px * 2 + 20% / 2;height:calc(var(--cale-2));}
        "#,
    );
    let mut ssg = StyleSheetGroup::new();
    ssg.append(ss);
    let np = query(&ssg, "", "", ["calc_1"], []);
    assert_eq!(
        np.custom_property(),
        CustomProperty::Expr("--cale-1".into(), "calc(50% + 10px + 10%)".into())
    );
    assert_eq!(
        np.height_type(),
        LengthType::Var(Box::new("var(--cale-1)".into()))
    );
    let np = query(&ssg, "", "", ["calc_2"], []);
    assert_eq!(
        np.custom_property(),
        CustomProperty::Expr("--cale-2".into(), "40% + 10px * 2 + 20% / 2".into())
    );
    assert_eq!(
        np.height_type(),
        LengthType::Var(Box::new("calc(var(--cale-2))".into()))
    );
    let np = query(&ssg, "", "", ["calc_3"], []);
    assert_eq!(
        np.custom_property(),
        CustomProperty::Expr("--cale-2".into(), "40% + 10px * 2 + 20% / 2".into())
    );
    assert_eq!(
        np.height_type(),
        LengthType::Var(Box::new("calc(var(--cale-2))".into()))
    );
}
