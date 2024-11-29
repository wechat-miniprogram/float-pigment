use alloc::string::ToString;
use core::{f32::consts::PI, marker::PhantomData};

use super::*;

impl CalcExpr {
    /// Check if the expression is a number.
    pub fn is_number(&self) -> bool {
        if let CalcExpr::Number(_) = self {
            return true;
        }
        false
    }

    /// Get the number value if the expression is a number.
    pub fn get_number(&self) -> Option<&Number> {
        match self {
            CalcExpr::Number(num) => Some(num.as_ref()),
            _ => None,
        }
    }

    /// Check if the expression is a static number `0`.
    pub fn is_zero(&self) -> bool {
        match self {
            CalcExpr::Number(v) => match *v.as_ref() {
                Number::F32(f) => f == 0.,
                Number::I32(i) => i == 0,
                _ => false,
            },
            _ => unreachable!(),
        }
    }

    /// Check if the expression is a literal value, a.k.a. does not contain any operator.
    pub fn is_specified_value(&self) -> bool {
        match self {
            CalcExpr::Angle(angle) => !matches!(angle.as_ref(), Angle::Calc(_)),
            CalcExpr::Number(num) => !matches!(num.as_ref(), Number::Calc(_)),
            CalcExpr::Length(length) => !matches!(
                length.as_ref(),
                Length::Expr(_) | Length::Auto | Length::Undefined
            ),
            _ => false,
        }
    }

    /// Multiply `rhs` if `mul` is true; divide `rhs` otherwise.
    pub fn mul_div(&self, rhs: f32, mul: bool) -> Self {
        match self {
            CalcExpr::Angle(angle) => {
                let v = if mul {
                    angle.to_f32() * rhs
                } else {
                    angle.to_f32() / rhs
                };
                let ret = match angle.as_ref() {
                    Angle::Deg(_) => Angle::Deg(v),
                    Angle::Grad(_) => Angle::Grad(v),
                    Angle::Rad(_) => Angle::Rad(v),
                    Angle::Turn(_) => Angle::Turn(v),
                    Angle::Calc(_) => unreachable!(),
                };
                CalcExpr::Angle(Box::new(ret))
            }
            CalcExpr::Length(length) => {
                let v = if mul {
                    length.to_f32() * rhs
                } else {
                    length.to_f32() / rhs
                };
                let ret = match length.as_ref() {
                    Length::Px(_) => Length::Px(v),
                    Length::Em(_) => Length::Em(v),
                    Length::Rpx(_) => Length::Rpx(v),
                    Length::Ratio(_) => Length::Ratio(v),
                    Length::Rem(_) => Length::Rem(v),
                    Length::Vh(_) => Length::Vh(v),
                    Length::Vw(_) => Length::Vw(v),
                    Length::Vmax(_) => Length::Vmax(v),
                    Length::Vmin(_) => Length::Vmin(v),
                    _ => unreachable!(),
                };
                CalcExpr::Length(Box::new(ret))
            }
            CalcExpr::Number(num) => {
                let ret = if mul {
                    num.to_f32() * rhs
                } else {
                    num.to_f32() / rhs
                };
                CalcExpr::Number(Box::new(Number::F32(ret)))
            }
            _ => unreachable!(),
        }
    }
}

impl Angle {
    /// Convert to `rad` form.
    ///
    /// Panics if it is an expression.
    pub fn to_rad(&self) -> Angle {
        match self {
            Angle::Rad(rad) => Angle::Rad(*rad),
            Angle::Deg(deg) => Angle::Rad(deg * PI / 180.),
            Angle::Grad(grad) => Angle::Rad(grad * PI / 200.),
            Angle::Turn(turn) => Angle::Rad(turn * 2. * PI),
            _ => panic!("not a literal value"),
        }
    }

    /// Create a new value from a turn-based value.
    pub fn from_ratio(turn: f32) -> Angle {
        Angle::Rad(turn * 2. * PI)
    }

    /// Erase the unit and leave the `f32` value.
    ///
    /// Panics if it is an expression.
    pub fn to_f32(&self) -> f32 {
        match self {
            Angle::Calc(_) => panic!("not a literal value"),
            Angle::Rad(v) => *v,
            Angle::Deg(v) => *v,
            Angle::Grad(v) => *v,
            Angle::Turn(v) => *v,
        }
    }
}

impl Length {
    /// Erase the unit and leave the `f32` value.
    ///
    /// Panics if it is an expression or it does not contain a unit.
    pub fn to_f32(&self) -> f32 {
        match self {
            Length::Px(v) => *v,
            Length::Em(v) => *v,
            Length::Rpx(v) => *v,
            Length::Ratio(v) => *v,
            Length::Rem(v) => *v,
            Length::Vh(v) => *v,
            Length::Vw(v) => *v,
            Length::Vmax(v) => *v,
            Length::Vmin(v) => *v,
            Length::Expr(_) | Length::Auto | Length::Undefined => panic!("not a literal value"),
        }
    }
}

pub(crate) struct ComputeCalcExpr<T> {
    _mark: PhantomData<*const T>,
}

impl ComputeCalcExpr<Angle> {
    pub fn try_compute(expr: &CalcExpr) -> Option<Angle> {
        match expr {
            CalcExpr::Angle(angle) => return Some(angle.as_ref().clone().to_rad()),
            CalcExpr::Plus(l, r) | CalcExpr::Sub(l, r) => {
                let l = Self::try_compute(l)?;

                let r = Self::try_compute(r)?;
                match expr {
                    CalcExpr::Plus(_, _) => Some(Angle::Rad(l.to_f32() + r.to_f32())),
                    CalcExpr::Sub(_, _) => Some(Angle::Rad(l.to_f32() - r.to_f32())),
                    _ => None,
                }
            }
            CalcExpr::Mul(l, r) | CalcExpr::Div(l, r) => {
                let l = Self::try_compute(l)?;
                let r = ComputeCalcExpr::<Number>::try_compute(r)?;
                match expr {
                    CalcExpr::Mul(_, _) => Some(Angle::Rad(l.to_f32() * r.to_f32())),
                    CalcExpr::Div(_, _) => Some(Angle::Rad(l.to_f32() / r.to_f32())),
                    _ => None,
                }
            }
            CalcExpr::Length(l) => match l.as_ref() {
                Length::Ratio(ratio) => Some(Angle::from_ratio(*ratio)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl ComputeCalcExpr<Number> {
    pub fn try_compute(expr: &CalcExpr) -> Option<Number> {
        match expr {
            CalcExpr::Number(num) => Some(*num.clone()),
            CalcExpr::Plus(l, r)
            | CalcExpr::Sub(l, r)
            | CalcExpr::Mul(l, r)
            | CalcExpr::Div(l, r) => {
                let l = Self::try_compute(l)?;
                let r = Self::try_compute(r)?;
                match expr {
                    CalcExpr::Plus(_, _) => Some(Number::F32(l.to_f32() + r.to_f32())),
                    CalcExpr::Sub(_, _) => Some(Number::F32(l.to_f32() - r.to_f32())),
                    CalcExpr::Mul(_, _) => Some(Number::F32(l.to_f32() * r.to_f32())),
                    CalcExpr::Div(_, _) => Some(Number::F32(l.to_f32() / r.to_f32())),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum LengthUnit {
    Px,
    Vw,
    Vh,
    Rem,
    Rpx,
    Em,
    Ratio,
    Vmin,
    Vmax,

    Undefined,
    Expr,
    Auto,
}

impl LengthUnit {
    pub(crate) fn is_specified_unit(&self) -> bool {
        !matches!(self, Self::Undefined | Self::Expr | Self::Auto)
    }
    pub(crate) fn to_length(unit: LengthUnit, value: f32) -> Length {
        match unit {
            LengthUnit::Auto => Length::Auto,
            LengthUnit::Undefined => Length::Undefined,
            LengthUnit::Expr => todo!(),
            LengthUnit::Px => Length::Px(value),
            LengthUnit::Vw => Length::Vw(value),
            LengthUnit::Vh => Length::Vh(value),
            LengthUnit::Rem => Length::Rem(value),
            LengthUnit::Em => Length::Em(value),
            LengthUnit::Rpx => Length::Rpx(value),
            LengthUnit::Ratio => Length::Ratio(value),
            LengthUnit::Vmin => Length::Vmin(value),
            LengthUnit::Vmax => Length::Vmax(value),
        }
    }
}

impl ComputeCalcExpr<Length> {
    pub(crate) fn try_compute(expr: &CalcExpr) -> Option<Length> {
        match expr {
            CalcExpr::Length(l) => Some(*l.clone()),
            CalcExpr::Angle(_) | CalcExpr::Number(_) => None,
            CalcExpr::Plus(l, r) | CalcExpr::Sub(l, r) => {
                let l = Self::try_compute(l)?;
                let r = Self::try_compute(r)?;
                let ((l_unit, l_v), (r_unit, r_v)) =
                    (Self::length_unit_value(&l), Self::length_unit_value(&r));
                // TODO
                // merge same unit
                if (l_unit == r_unit) && l_unit.is_specified_unit() {
                    match expr {
                        CalcExpr::Plus(_, _) => {
                            return Some(LengthUnit::to_length(l_unit, l_v + r_v));
                        }
                        CalcExpr::Sub(_, _) => {
                            return Some(LengthUnit::to_length(l_unit, l_v - r_v));
                        }
                        _ => unreachable!(),
                    }
                }
                //
                None
            }
            _ => None,
        }
    }
    pub(crate) fn length_unit_value(length: &Length) -> (LengthUnit, f32) {
        match length {
            Length::Px(v) => (LengthUnit::Px, *v),
            Length::Em(v) => (LengthUnit::Em, *v),
            Length::Ratio(v) => (LengthUnit::Ratio, *v),
            Length::Rem(v) => (LengthUnit::Rem, *v),
            Length::Rpx(v) => (LengthUnit::Rpx, *v),
            Length::Vh(v) => (LengthUnit::Vh, *v),
            Length::Vw(v) => (LengthUnit::Vw, *v),
            Length::Vmax(v) => (LengthUnit::Vmax, *v),
            Length::Vmin(v) => (LengthUnit::Vmin, *v),
            Length::Undefined => (LengthUnit::Undefined, f32::NAN),
            Length::Auto => (LengthUnit::Auto, f32::NAN),
            Length::Expr(_) => (LengthUnit::Expr, f32::NAN),
        }
    }
}

#[inline(never)]
fn next_operator<'a, 't: 'a, 'i: 't>(parser: &'a mut Parser<'i, 't>) -> Option<Operator> {
    let mut ret = None;
    let _ = parser.try_parse::<_, (), ParseError<'_, CustomError>>(|parser| {
        let token = parser.next()?.clone();
        ret = match token {
            Token::Delim(c) => match c {
                '+' => Some(Operator::Plus),
                '-' => Some(Operator::Sub),
                '*' => Some(Operator::Mul),
                '/' => Some(Operator::Div),
                _ => None,
            },
            _ => None,
        };
        Err(parser.new_custom_error(CustomError::Unsupported))
    });
    ret
}

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Plus,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ExpectValueType {
    Number,
    NumberAndLength,
    NumberAndAngle,
    AngleAndLength,
}

#[inline(never)]
fn combine_calc_expr(operator: Operator, lhs: CalcExpr, rhs: CalcExpr) -> CalcExpr {
    let mut final_lhs = Box::new(lhs.clone());
    if let CalcExpr::Length(length_expr) = lhs {
        if let Length::Expr(LengthExpr::Calc(length_expr)) = *length_expr {
            final_lhs = length_expr
        }
    }
    let mut final_rhs = Box::new(rhs.clone());
    if let CalcExpr::Length(length_expr) = rhs {
        if let Length::Expr(LengthExpr::Calc(length_expr)) = *length_expr {
            final_rhs = length_expr
        }
    }

    match operator {
        Operator::Plus => CalcExpr::Plus(final_lhs, final_rhs),
        Operator::Sub => CalcExpr::Sub(final_lhs, final_rhs),
        Operator::Mul => CalcExpr::Mul(final_lhs, final_rhs),
        Operator::Div => CalcExpr::Div(final_lhs, final_rhs),
    }
}

#[inline(never)]
pub(crate) fn parse_calc_inner<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    expect_type: ExpectValueType,
) -> Result<CalcExpr, ParseError<'i, CustomError>> {
    parser.parse_nested_block(|parser| {
        let ret = parse_calc_sum_expr(parser, properties, st, expect_type)?;
        Ok(ret)
    })
}

#[inline(never)]
fn parse_calc_sum_expr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    expect_type: ExpectValueType,
) -> Result<CalcExpr, ParseError<'i, CustomError>> {
    let mut expr = parse_calc_product_expr(parser, properties, st, expect_type)?;
    loop {
        let op = next_operator(parser);
        if !(op.is_some() && (op == Some(Operator::Plus) || op == Some(Operator::Sub))) {
            return Ok(expr);
        }
        /*
         * The + and - operators must be surrounded by whitespace.
         */
        parser.next_including_whitespace()?;
        parser.next()?;
        parser.next_including_whitespace()?;
        let rhs = parse_calc_product_expr(parser, properties, st, expect_type)?;
        if let Some(op) = op {
            match op {
                Operator::Plus | Operator::Sub => {
                    expr = combine_calc_expr(op, expr, rhs);
                }
                _ => unreachable!(),
            }
        } else {
            return Err(parser.new_custom_error(CustomError::Unsupported));
        }
    }
}

#[inline(never)]
fn parse_calc_product_expr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    expect_type: ExpectValueType,
) -> Result<CalcExpr, ParseError<'i, CustomError>> {
    let mut expr = parse_calc_parenthesis_expr(parser, properties, st, expect_type)?;
    loop {
        let op = next_operator(parser);
        if !(op.is_some() && (op == Some(Operator::Mul) || op == Some(Operator::Div))) {
            return Ok(expr);
        }
        /*
         * The * and / operators do not require whitespace, but adding it for consistency is recommended.
         */
        parser.next()?;
        let rhs = parse_calc_parenthesis_expr(parser, properties, st, expect_type)?;
        match op.unwrap() {
            Operator::Mul => {
                if expr.is_number() && rhs.is_number() {
                    let l_v = expr.get_number().unwrap();
                    let r_v = rhs.get_number().unwrap();
                    expr = CalcExpr::Number(Box::new(Number::F32(l_v.to_f32() * r_v.to_f32())));
                } else if expr.is_number() || rhs.is_number() {
                    if expr.is_number() {
                        if rhs.is_specified_value() {
                            expr = rhs.mul_div(expr.get_number().unwrap().to_f32(), true);
                        } else {
                            expr = combine_calc_expr(Operator::Mul, rhs, expr);
                        }
                    } else if expr.is_specified_value() {
                        expr = expr.mul_div(rhs.get_number().unwrap().to_f32(), true);
                    } else {
                        expr = combine_calc_expr(Operator::Mul, expr, rhs);
                    }
                } else {
                    return Err(parser.new_custom_error(CustomError::Unsupported));
                }
            }
            Operator::Div => {
                // NAN & zero
                if !rhs.is_number() || rhs.is_zero() {
                    return Err(
                        parser.new_custom_error(CustomError::Reason("divided by zero".to_string()))
                    );
                }
                if expr.is_number() && rhs.is_number() {
                    let l_v = expr.get_number().unwrap();
                    let r_v = rhs.get_number().unwrap();
                    expr = CalcExpr::Number(Box::new(Number::F32(l_v.to_f32() / r_v.to_f32())));
                } else if expr.is_specified_value() && rhs.is_number() {
                    expr = expr.mul_div(rhs.get_number().unwrap().to_f32(), false)
                } else {
                    expr = combine_calc_expr(Operator::Div, expr, rhs);
                }
            }
            _ => unreachable!(),
        }
    }
}

#[inline(never)]
fn parse_calc_parenthesis_expr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    expect_type: ExpectValueType,
) -> Result<CalcExpr, ParseError<'i, CustomError>> {
    let value = parse_calc_value(parser, properties, st, expect_type);
    if value.is_ok() {
        return value;
    }
    parser.try_parse::<_, CalcExpr, ParseError<'_, CustomError>>(|parser| {
        parser.expect_parenthesis_block()?;
        parser.parse_nested_block(|parser| parse_calc_sum_expr(parser, properties, st, expect_type))
    })
}

#[inline(never)]
fn parse_calc_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    expect_type: ExpectValueType,
) -> Result<CalcExpr, ParseError<'i, CustomError>> {
    // match number
    let num =
        parser.try_parse::<_, Number, ParseError<'_, _>>(|parser| number(parser, properties, st));
    if let Ok(num) = num {
        return Ok(CalcExpr::Number(Box::new(num)));
    }
    // match length
    let length =
        parser.try_parse::<_, Length, ParseError<'_, _>>(|parser| length(parser, properties, st));
    if let Ok(length) = length {
        if expect_type == ExpectValueType::NumberAndLength
            || expect_type == ExpectValueType::AngleAndLength
        {
            return Ok(CalcExpr::Length(Box::new(length)));
        }
    }
    // match angle
    let angle =
        parser.try_parse::<_, Angle, ParseError<'_, _>>(|parser| angle(parser, properties, st));
    if let Ok(angle) = angle {
        if expect_type == ExpectValueType::NumberAndAngle
            || expect_type == ExpectValueType::AngleAndLength
        {
            return Ok(CalcExpr::Angle(Box::new(angle)));
        }
    }
    Err(parser.new_custom_error(CustomError::Unmatched))
}
