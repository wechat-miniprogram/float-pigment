use alloc::string::ToString;
use core::convert::TryInto;

use cssparser::{ParseError, Parser, Token};

use super::*;
use borrow::Array;

pub(crate) mod calc;
use calc::*;
pub(crate) mod background;
pub(crate) mod filter;
pub(crate) mod font;
pub(crate) mod gradient;
pub(crate) mod var;

#[allow(unused_macros)]
macro_rules! print_next_token {
    ($parser: ident) => {
        let _ = $parser.try_parse(|parser| -> Result<(), ()> {
            let next = parser.next();
            println!("{:?}", next);
            Err(())
        });
    };
}

#[inline(never)]
fn next_is_important<'a, 't: 'a, 'i: 't>(parser: &'a mut Parser<'i, 't>) -> bool {
    let mut ret = false;
    let _ = parser.try_parse(|parser| -> Result<(), ()> {
        if parse_important(parser).is_ok() {
            ret = true;
        }
        Err(())
    });
    ret
}

#[inline(never)]
fn next_is_comma<'a, 't: 'a, 'i: 't>(parser: &'a mut Parser<'i, 't>) -> bool {
    let mut ret = false;
    let _ = parser.try_parse(|parser| -> Result<(), ()> {
        if parser.expect_comma().is_ok() {
            ret = true;
        }
        Err(())
    });
    ret
}

#[inline]
pub fn parse_comma_separated_without_important<'a, 't: 'a, 'i: 't, T, F>(
    parser: &'a mut Parser<'i, 't>,
    mut parse_one: F,
) -> Result<Vec<T>, ParseError<'i, CustomError>>
where
    T: core::fmt::Debug,
    F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, CustomError>>,
{
    let mut values = Vec::with_capacity(1);
    loop {
        parser.skip_whitespace();
        values.push(parse_one(parser)?);
        let is_important = next_is_important(parser);
        if is_important {
            return Ok(values);
        }
        match parser.next() {
            Err(_) => return Ok(values),
            Ok(&Token::Comma) => continue,
            Ok(_) => return Err(parser.new_custom_error(CustomError::Unsupported)),
        }
    }
}

#[inline(never)]
fn parse_env_inner<'a, 't: 'a, 'i: 't, T: 'static>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
    f: impl FnOnce(&mut Parser<'i, 't>, &mut ParseState) -> Result<T, ParseError<'i, CustomError>>,
) -> Result<(String, Option<T>), ParseError<'i, CustomError>> {
    let name = parser.expect_ident()?.clone();
    if parser.is_exhausted() {
        return Ok((name.to_string(), None));
    }
    parser.expect_comma()?;
    let v = f(parser, st)?;
    Ok((name.to_string(), Some(v)))
}

#[inline(never)]
pub(crate) fn color_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<crate::typing::Color, ParseError<'i, CustomError>> {
    let color = cssparser_color::Color::parse(parser)
        .map_err(|_| parser.new_custom_error(CustomError::Unsupported))?;
    Ok(match color {
        cssparser_color::Color::CurrentColor => Color::CurrentColor,
        cssparser_color::Color::Rgba(rgba) => {
            Color::Specified(rgba.red, rgba.green, rgba.blue, (rgba.alpha * 256.) as u8)
        }
        cssparser_color::Color::Hsl(hsl) => {
            let (red, green, blue) = cssparser_color::hsl_to_rgb(
                hsl.hue.unwrap_or(0.),
                hsl.saturation.unwrap_or(0.),
                hsl.lightness.unwrap_or(0.),
            );
            Color::Specified(
                red as u8,
                green as u8,
                blue as u8,
                hsl.alpha.unwrap_or(0.) as u8,
            )
        }
        cssparser_color::Color::Hwb(hwb) => {
            let (red, green, blue) = cssparser_color::hwb_to_rgb(
                hwb.hue.unwrap_or(0.),
                hwb.whiteness.unwrap_or(0.),
                hwb.blackness.unwrap_or(0.),
            );
            Color::Specified(
                red as u8,
                green as u8,
                blue as u8,
                hwb.alpha.unwrap_or(0.) as u8,
            )
        }
        _ => {
            return Err(parser.new_custom_error(CustomError::Unsupported));
        }
    })
}

#[inline(never)]
pub(crate) fn length<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Length, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Number { value, .. } => {
            if *value == 0. {
                return Ok(Length::Px(0.));
            }
        }
        Token::Percentage { unit_value, .. } => {
            return Ok(Length::Ratio(*unit_value));
        }
        Token::Dimension { value, unit, .. } => {
            let unit: &str = unit;
            match unit {
                "px" => return Ok(Length::Px(*value)),
                "vw" => return Ok(Length::Vw(*value)),
                "vh" => return Ok(Length::Vh(*value)),
                "rem" => return Ok(Length::Rem(*value)),
                "rpx" => return Ok(Length::Rpx(*value)),
                "em" => return Ok(Length::Em(*value)),
                "vmin" => return Ok(Length::Vmin(*value)),
                "vmax" => return Ok(Length::Vmax(*value)),
                _ => {}
            }
        }
        Token::Ident(ident) => {
            let ident: &str = ident;
            if ident == "auto" {
                return Ok(Length::Auto);
            }
        }
        Token::Function(name) => match &**name {
            "env" => {
                let (name, default_value) = parser.parse_nested_block(|parser| {
                    parse_env_inner(parser, st, |parser, st| {
                        env_default_value(parser, properties, st)
                    })
                })?;
                return Ok(Length::Expr(LengthExpr::Env(
                    name.into(),
                    Box::new(default_value.unwrap_or(Length::Undefined)),
                )));
            }
            "calc" => {
                return parse_calc_inner(parser, properties, st, ExpectValueType::NumberAndLength)
                    .map(|ret| {
                        if let Some(r) = ComputeCalcExpr::<Length>::try_compute(&ret) {
                            return r;
                        }
                        Length::Expr(LengthExpr::Calc(Box::new(ret)))
                    });
            }
            _ => {}
        },
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn env_default_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Length, ParseError<'i, CustomError>> {
    parser.skip_whitespace();
    let start = parser.current_source_location();
    let len = length(parser, properties, st);
    let end = parser.current_source_location();
    if len.is_err() {
        st.add_warning_with_message(
            WarningKind::InvalidEnvDefaultValue,
            "the second parameter of `env()` must be a length value",
            start,
            end,
        );
        return Ok(Length::Px(0.));
    }
    len
}

#[inline(never)]
pub(crate) fn line_names<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Vec<String>, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    if let Token::SquareBracketBlock = next {
        let line_names = parser.parse_nested_block(|parser| {
            let mut line_names = Vec::with_capacity(1);
            while !parser.is_exhausted() {
                let line_name = custom_ident_repr(parser, properties, st)?;
                line_names.push(line_name);
            }
            Ok(line_names)
        })?;
        return Ok(line_names);
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn fr_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut Vec<PropertyMeta>,
    _st: &mut ParseState,
) -> Result<f32, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Dimension { value, unit, .. } => match unit as &str {
            "fr" => return Ok(*value),
            _ => {}
        },
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn length_without_percentage<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Length, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Number { value, .. } => {
            if *value == 0. {
                return Ok(Length::Px(0.));
            }
        }
        Token::Dimension { value, unit, .. } => {
            let unit: &str = unit;
            match unit {
                "px" => return Ok(Length::Px(*value)),
                "vw" => return Ok(Length::Vw(*value)),
                "vh" => return Ok(Length::Vh(*value)),
                "rem" => return Ok(Length::Rem(*value)),
                "rpx" => return Ok(Length::Rpx(*value)),
                "em" => return Ok(Length::Em(*value)),
                "vmin" => return Ok(Length::Vmin(*value)),
                "vmax" => return Ok(Length::Vmax(*value)),
                _ => {}
            }
        }
        Token::Ident(ident) => {
            let ident: &str = ident;
            if ident == "auto" {
                return Ok(Length::Auto);
            }
        }
        Token::Function(name) => match &**name {
            "env" => {
                let (name, default_value) = parser.parse_nested_block(|parser| {
                    parse_env_inner(parser, st, |parser, st| {
                        env_default_value(parser, properties, st)
                    })
                })?;
                return Ok(Length::Expr(LengthExpr::Env(
                    name.into(),
                    Box::new(default_value.unwrap_or(Length::Undefined)),
                )));
            }
            "calc" => {
                return parse_calc_inner(parser, properties, st, ExpectValueType::NumberAndLength)
                    .map(|ret| {
                        if let Some(r) = ComputeCalcExpr::<Length>::try_compute(&ret) {
                            return r;
                        }
                        Length::Expr(LengthExpr::Calc(Box::new(ret)))
                    });
            }
            _ => {}
        },
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn is_non_negative_length(length: &Length) -> bool {
    match length {
        Length::Auto | Length::Undefined | Length::Expr(_) => true,
        Length::Px(v) => *v >= 0.,
        Length::Em(v) => *v >= 0.,
        Length::Rem(v) => *v >= 0.,
        Length::Ratio(v) => *v >= 0.,
        Length::Rpx(v) => *v >= 0.,
        Length::Vh(v) => *v >= 0.,
        Length::Vw(v) => *v >= 0.,
        Length::Vmax(v) => *v >= 0.,
        Length::Vmin(v) => *v >= 0.,
    }
}

#[allow(dead_code)]
#[inline(never)]
pub(crate) fn non_negative_length<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Length, ParseError<'i, CustomError>> {
    let next = parser.next()?.clone();
    match &next {
        Token::Number { value, .. } => {
            if *value == 0. {
                return Ok(Length::Px(0.));
            }
        }
        Token::Percentage { unit_value, .. } => {
            if *unit_value < 0. {
                return Err(parser.new_unexpected_token_error(next));
            }
            return Ok(Length::Ratio(*unit_value));
        }
        Token::Dimension { value, unit, .. } => {
            if *value < 0. {
                return Err(parser.new_unexpected_token_error(next));
            }
            let unit: &str = unit;
            match unit {
                "px" => return Ok(Length::Px(*value)),
                "vw" => return Ok(Length::Vw(*value)),
                "vh" => return Ok(Length::Vh(*value)),
                "rem" => return Ok(Length::Rem(*value)),
                "rpx" => return Ok(Length::Rpx(*value)),
                "em" => return Ok(Length::Em(*value)),
                "vmin" => return Ok(Length::Vmin(*value)),
                "vmax" => return Ok(Length::Vmax(*value)),
                _ => {}
            }
        }
        Token::Ident(ident) => {
            let ident: &str = ident;
            if ident == "auto" {
                return Ok(Length::Auto);
            }
        }
        Token::Function(name) => match &**name {
            "env" => {
                let (name, default_value) = parser.parse_nested_block(|parser| {
                    parse_env_inner(parser, st, |parser, st| {
                        env_default_value(parser, properties, st)
                    })
                })?;
                return Ok(Length::Expr(LengthExpr::Env(
                    name.into(),
                    Box::new(default_value.unwrap_or(Length::Undefined)),
                )));
            }
            "calc" => {
                return parse_calc_inner(parser, properties, st, ExpectValueType::NumberAndLength)
                    .map(|ret| {
                        if let Some(r) = ComputeCalcExpr::<Length>::try_compute(&ret) {
                            return r;
                        }
                        Length::Expr(LengthExpr::Calc(Box::new(ret)))
                    });
            }
            _ => {}
        },
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn angle<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Angle, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Number { value, .. } => {
            if *value == 0. {
                return Ok(Angle::Deg(0.));
            }
        }
        Token::Dimension { value, unit, .. } => {
            let unit: &str = unit;
            match unit {
                "deg" => return Ok(Angle::Deg(*value)),
                "grad" => return Ok(Angle::Grad(*value)),
                "rad" => return Ok(Angle::Rad(*value)),
                "turn" => return Ok(Angle::Turn(*value)),
                _ => {}
            }
        }
        Token::Function(name) => {
            if &**name == "calc" {
                return parse_calc_inner(parser, properties, st, ExpectValueType::AngleAndLength)
                    .map(|ret| {
                        if let Some(r) = ComputeCalcExpr::<Angle>::try_compute(&ret) {
                            return r;
                        }
                        Angle::Calc(Box::new(ret))
                    });
            }
        }
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[allow(dead_code)]
#[inline(never)]
pub(crate) fn string<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<String, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Ident(s) => {
            return Ok(s.to_string());
        }
        Token::QuotedString(s) => {
            return Ok(s.to_string());
        }
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn percentage<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    accept_angle: bool,
) -> Result<Length, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Number { value, .. } => {
            return Ok(Length::Ratio(*value));
        }
        Token::Percentage { unit_value, .. } => return Ok(Length::Ratio(*unit_value)),
        Token::Function(name) => {
            if &**name == "calc" {
                return parse_calc_inner(
                    parser,
                    properties,
                    st,
                    if accept_angle {
                        ExpectValueType::AngleAndLength
                    } else {
                        ExpectValueType::NumberAndLength
                    },
                )
                .map(|ret| {
                    if let Some(r) = ComputeCalcExpr::<Length>::try_compute(&ret) {
                        return r;
                    }
                    Length::Expr(LengthExpr::Calc(Box::new(ret)))
                });
            }
        }
        _ => {}
    }
    Err(parser.new_custom_error(CustomError::Unmatched))
}

#[inline(never)]
pub(crate) fn percentage_to_f32<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<f32, ParseError<'i, CustomError>> {
    let x = percentage(parser, properties, st, false)?;
    if let Some(x) = x.ratio_to_f32() {
        return Ok(x);
    }
    Err(parser.new_custom_error(CustomError::Unmatched))
}

#[inline(never)]
pub(crate) fn float_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<f32, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    if let Token::Number { value, .. } = next {
        return Ok(*value);
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn number<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Number, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Number { value, .. } => {
            return Ok(Number::F32(*value));
        }
        Token::Function(name) => {
            if &**name == "calc" {
                return parse_calc_inner(parser, properties, st, ExpectValueType::Number).map(
                    |ret| {
                        if let Some(r) = ComputeCalcExpr::<Number>::try_compute(&ret) {
                            return r;
                        }
                        Number::Calc(Box::new(ret))
                    },
                );
            }
        }
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn non_negative_number<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Number, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Number { value, .. } => {
            if *value >= 0. {
                return Ok(Number::F32(*value));
            }
        }
        Token::Function(name) => {
            if &**name == "calc" {
                return parse_calc_inner(parser, properties, st, ExpectValueType::Number).map(
                    |ret| {
                        if let Some(r) = ComputeCalcExpr::<Number>::try_compute(&ret) {
                            return r;
                        }
                        Number::Calc(Box::new(ret))
                    },
                );
            }
        }
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn line_width<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Length, ParseError<'i, CustomError>> {
    // match thin | medium | thick
    // The thin, medium, and thick keywords are equivalent to 1px, 3px, and 5px.
    parser
        .try_parse(|parser| {
            let token = parser.next()?;
            match token {
                Token::Ident(keyword) => {
                    let keyword = &**keyword;
                    match keyword {
                        "thin" => Ok(Length::Px(1.)),
                        "medium" => Ok(Length::Px(3.)),
                        "thick" => Ok(Length::Px(5.)),
                        _ => Err(parser.new_custom_error(CustomError::Unmatched)),
                    }
                }
                _ => Err(parser.new_custom_error(CustomError::Unmatched)),
            }
        })
        .or_else(|_: ParseError<'i, CustomError>| {
            // match length
            // Negative values are invalid.
            non_negative_length(parser, properties, st)
        })
}

#[inline(never)]
pub(crate) fn time_u32_ms<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<u32, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    if let Token::Dimension { value, unit, .. } = next {
        let unit: &str = unit;
        match unit {
            "s" => return Ok((*value * 1000.) as u32),
            "ms" => return Ok((*value) as u32),
            _ => {}
        }
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn time_i32_ms<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<i32, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    if let Token::Dimension { value, unit, .. } = next {
        let unit: &str = unit;
        let unit: &str = &unit.to_lowercase();
        match unit {
            "s" => return Ok((*value * 1000.) as i32),
            "ms" => return Ok((*value) as i32),
            _ => {}
        }
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

#[inline(never)]
pub(crate) fn transform_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Transform, ParseError<'i, CustomError>> {
    let mut trans = vec![];
    while !parser.is_exhausted() {
        if parser
            .try_parse::<_, (), _>(|parser| Err(parser.expect_semicolon().is_ok()))
            .unwrap_err()
            || parser
                .try_parse::<_, (), _>(|parser| Err(parse_important(parser).is_ok()))
                .unwrap_err()
        {
            break;
        }
        let next = parser.next()?.clone();
        match &next {
            Token::Function(name) => {
                let name: &str = &name.to_string().to_lowercase();
                parser.parse_nested_block(|parser| {
                    let trans_item: TransformItem;
                    match name {
                        "matrix" => {
                            let mut v = Vec::with_capacity(6);
                            for i in 0..6 {
                                let x = float_repr(parser, properties, st)?;
                                v.push(x);
                                if i < 5 {
                                    parser.expect_comma()?;
                                }
                            }
                            trans_item = TransformItem::Matrix(v.try_into().unwrap_or([0f32; 6]));
                        }
                        "matrix3d" => {
                            let mut v = Vec::with_capacity(6);
                            for i in 0..16 {
                                let x = float_repr(parser, properties, st)?;
                                v.push(x);
                                if i < 15 {
                                    parser.expect_comma()?;
                                }
                            }
                            trans_item =
                                TransformItem::Matrix3D(v.try_into().unwrap_or([0f32; 16]));
                        }
                        "translate" => {
                            let x = length(parser, properties, st)?;
                            if parser.is_exhausted() {
                                trans_item = TransformItem::Translate2D(x, Length::Px(0.));
                            } else {
                                let comma = parser.expect_comma();
                                if comma.is_err() {
                                    return Err(parser.new_unexpected_token_error(next.clone()));
                                }
                                let y = length(parser, properties, st);
                                match y {
                                    Ok(y) => trans_item = TransformItem::Translate2D(x, y),
                                    Err(_) => {
                                        return Err(parser.new_unexpected_token_error(next.clone()))
                                    }
                                }
                            }
                        }
                        "translatex" => {
                            let x = length(parser, properties, st)?;
                            trans_item = TransformItem::Translate2D(x, Length::Px(0.));
                        }
                        "translatey" => {
                            let y = length(parser, properties, st)?;
                            trans_item = TransformItem::Translate2D(Length::Px(0.), y);
                        }
                        "translatez" => {
                            let z = length(parser, properties, st)?;
                            trans_item =
                                TransformItem::Translate3D(Length::Px(0.), Length::Px(0.), z);
                        }
                        "translate3d" => {
                            let x = length(parser, properties, st)?;
                            parser.expect_comma()?;
                            let y = length(parser, properties, st)?;
                            parser.expect_comma()?;
                            let z = length(parser, properties, st)?;
                            trans_item = TransformItem::Translate3D(x, y, z);
                        }
                        "scale" => {
                            // match number
                            let x = percentage_to_f32(parser, properties, st)?;

                            if parser.is_exhausted() {
                                trans_item = TransformItem::Scale2D(x, x);
                            } else {
                                let comma = parser.expect_comma();
                                if comma.is_err() {
                                    return Err(parser.new_unexpected_token_error(next.clone()));
                                }
                                let y = percentage_to_f32(parser, properties, st)?;
                                trans_item = TransformItem::Scale2D(x, y);
                            }
                        }
                        "scalex" => {
                            let x = percentage_to_f32(parser, properties, st)?;
                            trans_item = TransformItem::Scale2D(x, 1.);
                        }
                        "scaley" => {
                            let y = percentage_to_f32(parser, properties, st)?;
                            trans_item = TransformItem::Scale2D(1., y);
                        }
                        "scalez" => {
                            let z = percentage_to_f32(parser, properties, st)?;
                            trans_item = TransformItem::Scale3D(1., 1., z);
                        }
                        "scale3d" => {
                            let x = percentage_to_f32(parser, properties, st)?;
                            parser.expect_comma()?;
                            let y = percentage_to_f32(parser, properties, st)?;
                            parser.expect_comma()?;
                            let z = percentage_to_f32(parser, properties, st)?;
                            trans_item = TransformItem::Scale3D(x, y, z);
                        }
                        "rotate" => {
                            let a = angle(parser, properties, st)?;
                            trans_item = TransformItem::Rotate2D(a);
                        }
                        "rotatex" => {
                            let a = angle(parser, properties, st)?;
                            trans_item = TransformItem::Rotate3D(1., 0., 0., a);
                        }
                        "rotatey" => {
                            let a = angle(parser, properties, st)?;
                            trans_item = TransformItem::Rotate3D(0., 1., 0., a);
                        }
                        "rotatez" => {
                            let a = angle(parser, properties, st)?;
                            trans_item = TransformItem::Rotate3D(0., 0., 1., a);
                        }
                        "rotate3d" => {
                            let x = float_repr(parser, properties, st)?;
                            parser.expect_comma()?;
                            let y = float_repr(parser, properties, st)?;
                            parser.expect_comma()?;
                            let z = float_repr(parser, properties, st)?;
                            parser.expect_comma()?;
                            let a = angle(parser, properties, st)?;
                            trans_item = TransformItem::Rotate3D(x, y, z, a);
                        }
                        "skew" => {
                            let x = angle(parser, properties, st)?;
                            if parser.is_exhausted() {
                                trans_item = TransformItem::Skew(x.clone(), x);
                            } else {
                                let comma = parser.expect_comma();
                                if comma.is_err() {
                                    return Err(parser.new_unexpected_token_error(next.clone()));
                                }
                                let y = angle(parser, properties, st);
                                match y {
                                    Ok(y) => trans_item = TransformItem::Skew(x, y),
                                    Err(_) => {
                                        return Err(parser.new_unexpected_token_error(next.clone()))
                                    }
                                }
                            }
                        }
                        "skewx" => {
                            let x = angle(parser, properties, st)?;
                            trans_item = TransformItem::Skew(x, Angle::Deg(0.))
                        }
                        "skewy" => {
                            let y = angle(parser, properties, st)?;
                            trans_item = TransformItem::Skew(Angle::Deg(0.), y)
                        }
                        "perspective" => {
                            let v = length(parser, properties, st)?;
                            trans_item = TransformItem::Perspective(v);
                        }
                        _ => return Err(parser.new_unexpected_token_error(next.clone())),
                    }
                    trans.push(trans_item);
                    Ok(())
                })?
            }
            Token::Ident(name) => {
                let name: &str = name;
                if name == "none" {
                    return Ok(Transform::Series(Array::empty()));
                }
            }
            _ => return Err(parser.new_unexpected_token_error(next.clone())),
        }
    }
    Ok(Transform::Series(trans.into()))
}

#[inline(never)]
pub(crate) fn url_str<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<String, ParseError<'i, CustomError>> {
    parser
        .expect_url()
        .map(|x| x.to_string())
        .map_err(|_| parser.new_custom_error(CustomError::Unsupported))
}

#[inline(never)]
pub(crate) fn hash_token_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<String, ParseError<'i, CustomError>> {
    let next = parser.next()?.clone();
    match &next {
        Token::IDHash(token) => {
            let t: &str = token;
            Ok(t.to_string())
        }
        _ => Err(parser.new_unexpected_token_error::<CustomError>(next.clone())),
    }
}

#[inline(never)]
pub(crate) fn element_func_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<BackgroundImageItem, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let fn_name = parser.expect_function()?.clone();
        match fn_name.to_string().as_str() {
            "element" => parser.parse_nested_block(|parser| {
                let hash = hash_token_repr(parser)?;
                Ok(BackgroundImageItem::Element(hash.into()))
            }),
            _ => Err(parser.new_custom_error(CustomError::Unsupported)),
        }
    })
}

pub(crate) fn custom_ident_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut Vec<PropertyMeta>,
    _st: &mut ParseState,
) -> Result<String, ParseError<'i, CustomError>> {
    let next = parser.next()?.clone();
    match &next {
        Token::Ident(name) => {
            return Ok(name.to_string());
        }
        Token::QuotedString(name) => {
            return Ok(name.to_string());
        }
        _ => return Err(parser.new_unexpected_token_error::<CustomError>(next.clone())),
    }
}

#[inline(never)]
pub(crate) fn image_func_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut [PropertyMeta],
    st: &mut ParseState,
) -> Result<BackgroundImageItem, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let fn_name = parser.expect_function()?.clone();
        match fn_name.to_string().as_str() {
            "image" => parser.parse_nested_block(|parser| {
                // image_tags
                let mut image_tags = ImageTags::LTR;
                let _ = parser.try_parse(|parser| {
                    let ret = image_tags_repr(parser);
                    if let Ok(r) = ret.clone() {
                        image_tags = r;
                    }
                    ret
                });
                // image_source & color
                let mut image_src = ImageSource::None;
                let mut color = Color::Undefined;
                let mut need_parse_image_src = false;
                // try parse color
                let _ = parser.try_parse(|parser| {
                    let ret = color_repr(parser, properties, st);
                    match ret.clone() {
                        Ok(r) => color = r,
                        Err(_) => need_parse_image_src = true,
                    }
                    ret
                });
                // try parse image_source
                if need_parse_image_src {
                    let _ = parser.try_parse(|parser| {
                        let ret = url_str(parser, properties, st);
                        if let Ok(r) = ret.clone() {
                            image_src = ImageSource::Url(r.into());
                            // comma
                            let comma = parser.expect_comma();
                            if comma.is_ok() {
                                let ret = color_repr(parser, properties, st);
                                if let Ok(r) = ret {
                                    color = r;
                                }
                            }
                        }
                        ret
                    });
                }
                Ok(BackgroundImageItem::Image(image_tags, image_src, color))
            }),
            _ => Err(parser.new_custom_error(CustomError::Unsupported)),
        }
    })
}

#[inline(never)]
pub(crate) fn image_tags_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<ImageTags, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let next = parser.next()?.clone();
        match &next {
            Token::Ident(s) => {
                let s: &str = s;
                match s {
                    "ltr" => Ok(ImageTags::LTR),
                    "rtl" => Ok(ImageTags::RTL),
                    _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                }
            }
            _ => Err(parser.new_custom_error(CustomError::Unsupported)),
        }
    })
}
