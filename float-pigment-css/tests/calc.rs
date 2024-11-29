#![allow(clippy::approx_constant)]

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
pub fn calc() {
    test_parse_property!(
        margin_left,
        "margin-left",
        "calc(10px + 10px)",
        Length::Px(20.)
    );
    test_parse_property!(
        margin_left,
        "margin-left",
        "calc(10px + 10deg)",
        Length::Undefined
    );
    test_parse_property!(margin_left, "margin-left", "calc(10px / 2)", Length::Px(5.));
    test_parse_property!(
        margin_left,
        "margin-left",
        "calc(10px / 0)",
        Length::Undefined
    );
    test_parse_property!(
        margin_left,
        "margin-left",
        "calc(10px / 3)",
        Length::Px(3.3333333)
    );

    test_parse_property!(
        margin_left,
        "margin-left",
        "calc(10px * 3)",
        Length::Px(30.)
    );
    // FIXME
    // test_parse_property!(
    //     margin_left,
    //     "margin-left",
    //     "calc(10px + 20vh + 30px)",
    //     Length::Expr(LengthExpr::Calc(CalcExpr::Plus(
    //         Box::new(CalcExpr::Length(Box::new(Length::Px(40.)))),
    //         Box::new(CalcExpr::Length(Box::new(Length::Vh(20.))))
    //     )))
    // );
    test_parse_property!(
        margin_left,
        "margin-left",
        "calc(10rpx + (20vh + 30px))",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Rpx(10.)))),
            Box::new(CalcExpr::Plus(
                Box::new(CalcExpr::Length(Box::new(Length::Vh(20.)))),
                Box::new(CalcExpr::Length(Box::new(Length::Px(30.))))
            ))
        ))))
    );
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(10deg + (20deg + 30deg)))",
        Transform::Series(vec![TransformItem::Rotate2D(Angle::Rad(1.0471976))].into())
    );

    // FIXME
    // test_parse_property!(
    //     margin_left,
    //     "margin-left",
    //     "calc((10px + 20px) * 3)",
    //     Length::Px(90.)
    // );

    // FIXME
    // test_parse_property!(
    //     margin_left,
    //     "margin-left",
    //     "calc((10px + 20px) * 30px)",
    //     Length::Undefined
    // );

    // FIXME
    // test_parse_property!(
    //     flex_direction,
    //     "flex-direction",
    //     "calc((10px + 20px) * 30px)",
    //     FlexDirection::Row
    // );
}

#[test]
pub fn compute_calc() {
    // compute Length
    test_parse_property!(width, "width", "calc(100px)", Length::Px(100.));
    test_parse_property!(width, "width", "calc(100px + 200px)", Length::Px(300.));

    // compute angle
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(0.5turn))",
        Transform::Series(vec![TransformItem::Rotate2D(Angle::Rad(3.1415927)),].into())
    );
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(45deg + 45deg))",
        Transform::Series(vec![TransformItem::Rotate2D(Angle::Rad(1.570_796_4)),].into())
    );
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(45deg + 0.125turn))",
        Transform::Series(vec![TransformItem::Rotate2D(Angle::Rad(1.570_796_4)),].into())
    );
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(45deg * 2))",
        Transform::Series(vec![TransformItem::Rotate2D(Angle::Rad(1.570_796_4)),].into())
    );
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(180deg / 2))",
        Transform::Series(vec![TransformItem::Rotate2D(Angle::Rad(1.570_796_4)),].into())
    );
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(2 / 180deg))",
        Transform::Series(vec![].into())
    );
    test_parse_property!(
        transform,
        "transform",
        "rotate(calc(2 * 45deg))",
        Transform::Series(vec![TransformItem::Rotate2D(Angle::Rad(1.570_796_4)),].into())
    );
    // compute number
    test_parse_property!(flex_grow, "flex-grow", "calc(1 + 2)", Number::F32(3.));
    test_parse_property!(flex_grow, "flex-grow", "calc(1 + 2 + 3)", Number::F32(6.));
    test_parse_property!(flex_grow, "flex-grow", "calc(4 + (2 - 3))", Number::F32(3.));
    test_parse_property!(
        flex_grow,
        "flex-grow",
        "calc(10 - (8 - 3))",
        Number::F32(5.)
    );
    test_parse_property!(
        flex_grow,
        "flex-grow",
        "calc(10 + (2 - 3) - (3 + (1 + 2)))",
        Number::F32(3.)
    );
}

#[test]
pub fn calc_ch_ex_lang() {
    test_parse_property!(
        left,
        "left",
        "calc(1vh + 5vw + 10px + 1rem)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Plus(
                Box::new(CalcExpr::Plus(
                    Box::new(CalcExpr::Length(Box::new(Length::Vh(1.)))),
                    Box::new(CalcExpr::Length(Box::new(Length::Vw(5.))))
                )),
                Box::new(CalcExpr::Length(Box::new(Length::Px(10.))))
            )),
            Box::new(CalcExpr::Length(Box::new(Length::Rem(1.))))
        ))))
    );

    // FIXME
    // test_parse_property!(
    //     font_size,
    //     "font-size",
    //     "calc(17px + 0.5 * (1rem - 16px))",
    //     Length::Expr(LengthExpr::Calc(CalcExpr::Plus(
    //         Box::new(CalcExpr::Length(Box::new(Length::Px(9.)))),
    //         Box::new(CalcExpr::Length(Box::new(Length::Rem(0.5))))
    //     )))
    // );
}

#[test]
pub fn calc_height_block_1() {
    test_parse_property!(height, "height", "calc(50px)", Length::Px(50.));
    test_parse_property!(height, "height", "calc(50%)", Length::Ratio(0.5));
    test_parse_property!(
        height,
        "height",
        "calc(25px + 50%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(25.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
        ))))
    );

    test_parse_property!(
        height,
        "height",
        "calc(150% / 2 - 30px)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.75)))),
            Box::new(CalcExpr::Length(Box::new(Length::Px(30.)))),
        ))))
    );

    // FIXME
    // test_parse_property!(
    //     height,
    //     "height",
    //     "calc(40px + 10% - 20% / 2)",
    //     Length::Px(40.)
    // );

    test_parse_property!(
        height,
        "height",
        "calc(40px - 10%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Px(40.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.1)))),
        ))))
    );
}

#[test]
pub fn calc_in_calc() {
    test_parse_property!(height, "height", "calc(calc(100%))", Length::Ratio(1.));
    test_parse_property!(
        height,
        "height",
        "calc(calc(calc(100%)))",
        Length::Ratio(1.)
    );
    test_parse_property!(
        height,
        "height",
        "calc(calc(10px + 10%) + 10px)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Plus(
                Box::new(CalcExpr::Length(Box::new(Length::Px(10.)))),
                Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.1))))
            )),
            Box::new(CalcExpr::Length(Box::new(Length::Px(10.))))
        ))))
    );
}

#[test]
pub fn calc_margin_block_1() {
    test_parse_property!(
        margin_top,
        "margin",
        "calc(10px + 1%) 0 0 0",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(10.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.01)))),
        ))))
    );
    test_parse_property!(
        margin_right,
        "margin",
        "0 calc(10px + 1%) 0 0",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(10.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.01)))),
        ))))
    );
    test_parse_property!(
        margin_bottom,
        "margin",
        "0 0 calc(10px + 1%) 0",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(10.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.01)))),
        ))))
    );
    test_parse_property!(
        margin_left,
        "margin",
        "0 0 0 calc(10px + 1%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(10.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.01)))),
        ))))
    );
    test_parse_property!(
        margin_left,
        "margin",
        "calc(10px + 1%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(10.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.01)))),
        ))))
    );
    test_parse_property!(
        margin_bottom,
        "margin",
        "calc(10px + 1%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(10.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.01)))),
        ))))
    );
}

#[test]
pub fn calc_max_height_block_1() {
    test_parse_property!(max_height, "max-height", "calc(50px)", Length::Px(50.));
    test_parse_property!(max_height, "max-height", "calc(50%)", Length::Ratio(0.5));
    test_parse_property!(
        max_height,
        "max-height",
        "calc(25px + 50%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Px(25.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
        ))))
    );
    test_parse_property!(
        max_height,
        "max-height",
        "calc(150% / 2 - 30px)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.75)))),
            Box::new(CalcExpr::Length(Box::new(Length::Px(30.)))),
        ))))
    );
    // FIXME
    // test_parse_property!(
    //    max_height,
    //    "max-height",
    //     "calc(40px + 10% - 20% / 2)",
    //     Length::Px(40.)
    // );
    test_parse_property!(
        max_height,
        "max-height",
        "calc(40px - 10%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Px(40.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.1)))),
        ))))
    );
}

#[test]
pub fn calc_max_width_block_1() {
    test_parse_property!(
        max_width,
        "max-width",
        "calc(50% - 3px)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
            Box::new(CalcExpr::Length(Box::new(Length::Px(3.)))),
        ))))
    );

    // FIXME
    // test_parse_property!(
    //     max_width,
    //     "max-width",
    //     "calc(25% - 3px + 25%)",
    //     Length::Expr(LengthExpr::Calc(CalcExpr::Sub(
    //         Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
    //         Box::new(CalcExpr::Length(Box::new(Length::Px(3.)))),
    //     )))
    // );

    // FIXME
    // test_parse_property!(
    //     max_width,
    //     "max-width",
    //     "calc(25% - 3px + 12.5% * 2)",
    //     Length::Expr(LengthExpr::Calc(CalcExpr::Sub(
    //         Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
    //         Box::new(CalcExpr::Length(Box::new(Length::Px(3.)))),
    //     )))
    // );

    // FIXME
    // test_parse_property!(
    //     max_width,
    //     "max-width",
    //     "calc(25% - 3px + 12.5%*2)",
    //     Length::Expr(LengthExpr::Calc(CalcExpr::Sub(
    //         Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
    //         Box::new(CalcExpr::Length(Box::new(Length::Px(3.)))),
    //     )))
    // );

    // FIXME
    // test_parse_property!(
    //     max_width,
    //     "max-width",
    //     "calc(25% - 3px + 2*12.5%)",
    //     Length::Expr(LengthExpr::Calc(CalcExpr::Sub(
    //         Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
    //         Box::new(CalcExpr::Length(Box::new(Length::Px(3.)))),
    //     )))
    // );

    // FIXME
    // test_parse_property!(
    //     max_width,
    //     "max-width",
    //     "calc(25% - 3px + 2 * 12.5%)",
    //     Length::Expr(LengthExpr::Calc(CalcExpr::Sub(
    //         Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
    //         Box::new(CalcExpr::Length(Box::new(Length::Px(3.)))),
    //     )))
    // );

    test_parse_property!(
        max_width,
        "max-width",
        "calc(30% + 20%)",
        Length::Ratio(0.5)
    );
}

#[test]
pub fn calc_offset_absolute_left_1() {
    test_parse_property!(left, "left", "calc(-50px)", Length::Px(-50.));
    test_parse_property!(left, "left", "calc(-50%)", Length::Ratio(-0.5));
    test_parse_property!(
        left,
        "left",
        "calc(-25px - 50%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Px(-25.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.5)))),
        ))))
    );
    test_parse_property!(
        left,
        "left",
        "calc(-150% / 2 - 30px)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(-0.75)))),
            Box::new(CalcExpr::Length(Box::new(Length::Px(30.)))),
        ))))
    );
    // FIXME
    // test_parse_property!(
    //    left,
    //    "left",
    //     "calc(-40px + 10% - 20% / 2)",
    //     Length::Px(-40.)
    // );
    test_parse_property!(
        left,
        "left",
        "calc(-40px - 10%)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Sub(
            Box::new(CalcExpr::Length(Box::new(Length::Px(-40.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Ratio(0.1)))),
        ))))
    );
}

#[test]
pub fn calc_rem_lang() {
    test_parse_property!(font_size, "font-size", "calc(1rem + 1em)", Length::Px(32.));

    test_parse_property!(
        left,
        "left",
        "calc(1rem + 1em)",
        Length::Expr(LengthExpr::Calc(Box::new(CalcExpr::Plus(
            Box::new(CalcExpr::Length(Box::new(Length::Rem(1.)))),
            Box::new(CalcExpr::Length(Box::new(Length::Em(1.)))),
        ))))
    );
}

#[test]
pub fn calc_fraction() {
    test_parse_property!(width, "width", "calc(1/2 * 100%)", Length::Ratio(0.5))
}

#[test]
pub fn calc_operator_whitespace() {
    test_parse_property!(width, "width", "calc(1px + 2px)", Length::Px(3.));
    test_parse_property!(width, "width", "calc(1px - 2px)", Length::Px(-1.));
    test_parse_property!(width, "width", "calc(1px -2px)", Length::Undefined);
    test_parse_property!(width, "width", "calc(1px +2px)", Length::Undefined);
    test_parse_property!(width, "width", "calc(1px- 2px)", Length::Undefined);
    test_parse_property!(width, "width", "calc(1px+ 2px)", Length::Undefined);

    test_parse_property!(width, "width", "calc(1px  *  3)", Length::Px(3.));
    test_parse_property!(width, "width", "calc(1px *3)", Length::Px(3.));
    test_parse_property!(width, "width", "calc(1px*3)", Length::Px(3.));
    test_parse_property!(width, "width", "calc(3px /   1)", Length::Px(3.));
    test_parse_property!(width, "width", "calc(3px /1)", Length::Px(3.));
    test_parse_property!(width, "width", "calc(3px/1)", Length::Px(3.));
}
