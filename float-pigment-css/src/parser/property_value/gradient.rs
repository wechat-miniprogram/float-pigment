use alloc::string::ToString;
use core::f32::consts::PI;

use super::*;

#[inline(never)]
fn resolved_gradient_color_stops<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Vec<GradientColorItem>, ParseError<'i, CustomError>> {
    let mut has_percent = false;
    let mut has_auto = false;
    let mut has_dimension = false;
    // parse color stops
    let mut colors: Vec<GradientColorItem> = vec![];
    parser.parse_comma_separated(|parser| {
        let color = match parser.try_parse(|parser| color_repr(parser, properties, st)) {
            Ok(r) => r,
            Err(_) => Color::Undefined,
        };
        if parser.is_exhausted() {
            has_auto = true;
            colors.push(GradientColorItem::ColorHint(color, Length::Auto));
            return Ok(());
        }
        let hint = length(parser, properties, st)?;
        match &hint {
            Length::Ratio(_) => has_percent = true,
            _ => has_dimension = true,
        }
        colors.push(GradientColorItem::ColorHint(color.clone(), hint));
        if !parser.is_exhausted() {
            let hint = length(parser, properties, st)?;
            colors.push(GradientColorItem::ColorHint(color, hint));
        }
        Ok(())
    })?;

    // calc stops
    let color_len = colors.len();
    if color_len < 2 {
        return Err(parser.new_custom_error(CustomError::Unsupported));
    }
    // normalize specified value of color stops
    let mut prev_specified_color_hint = None;
    let mut idx = 0;
    let mut prev_color = None;
    loop {
        if idx >= color_len {
            break;
        }
        let GradientColorItem::ColorHint(color, hint) = colors.get_mut(idx).unwrap() else {
            return Err(parser.new_custom_error(CustomError::Unsupported));
        };
        match hint {
            Length::Auto => {
                if idx == 0 {
                    *hint = Length::Ratio(0.);
                } else if idx == color_len - 1 {
                    *hint = Length::Ratio(1.);
                }
            }
            Length::Ratio(ratio) => {
                if prev_specified_color_hint.is_none() {
                    prev_specified_color_hint = Some(*ratio);
                } else {
                    let prev = prev_specified_color_hint.unwrap();
                    if *ratio <= prev {
                        *ratio = prev;
                    } else {
                        prev_specified_color_hint = Some(*ratio);
                    }
                }
            }
            _ => {}
        }
        if color == &Color::Undefined {
            if idx == 0 {
                return Err(parser.new_custom_error(CustomError::Unsupported));
            }
            if let Some(prev) = prev_color.clone() {
                *color = prev;
            }
        } else {
            prev_color = Some(color.clone());
        }
        idx += 1;
    }
    if has_dimension && has_auto && has_percent {
        return Ok(colors);
    }
    // calc auto value of color stops
    let mut start = None;
    let mut idx = 0;
    let mut auto_cnt = 0;
    loop {
        if idx >= color_len {
            break;
        };
        let GradientColorItem::ColorHint(_, hint) = &colors[idx] else {
            unreachable!()
        };
        match hint {
            Length::Auto => {
                if start.is_none() {
                    let mut cur_idx = idx as isize;
                    loop {
                        cur_idx -= 1;
                        if cur_idx < 0 {
                            break;
                        }
                        if let GradientColorItem::ColorHint(_, Length::Ratio(ratio)) =
                            colors.get(cur_idx as usize).unwrap()
                        {
                            start = Some((cur_idx as usize, *ratio));
                            break;
                        }
                    }
                }
                auto_cnt += 1;
            }
            Length::Ratio(end_ratio) => {
                if let Some((start_idx, start_ratio)) = start {
                    let item = (*end_ratio - start_ratio) / ((auto_cnt + 1) as f32);
                    let mut cnt = 0;
                    for i in (start_idx + 1)..idx {
                        let GradientColorItem::ColorHint(_, hint) = colors.get_mut(i).unwrap()
                        else {
                            unreachable!()
                        };
                        if let Length::Auto = hint {
                            cnt += 1;
                            *hint = Length::Ratio(start_ratio + (cnt as f32 * item));
                        }
                    }
                    start = None;
                    auto_cnt = 0;
                }
            }
            _ => {}
        }
        idx += 1;
    }
    Ok(colors)
}

#[derive(Copy, Clone)]
enum ColorHintUnit {
    AngleOrPercentage,
    #[allow(unused)]
    Length,
}

#[derive(Clone, Debug)]
enum ColorHint {
    AngleOrPercentage(AngleOrPercentage),
    Length(Length),
}

fn parse_gradient_color_hint<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    accept_unit: ColorHintUnit,
) -> Result<ColorHint, ParseError<'i, CustomError>> {
    match accept_unit {
        ColorHintUnit::Length => {
            let length = length(parser, properties, st)?;
            Ok(ColorHint::Length(length))
        }
        ColorHintUnit::AngleOrPercentage => {
            let maybe_angle = parser.try_parse(|parser| angle(parser, properties, st));
            if let Ok(angle) = maybe_angle {
                if let Angle::Calc(_) = angle {
                    return Err(parser.new_custom_error(CustomError::Unsupported));
                }
                return Ok(ColorHint::AngleOrPercentage(AngleOrPercentage::Percentage(
                    angle.to_rad().to_f32() / (2. * PI),
                )));
            }
            let maybe_percentage =
                parser.try_parse(|parser| percentage(parser, properties, st, true));
            if let Ok(Length::Ratio(percentage)) = maybe_percentage {
                return Ok(ColorHint::AngleOrPercentage(AngleOrPercentage::Percentage(
                    percentage,
                )));
            }
            Err(parser.new_custom_error(CustomError::Unmatched))
        }
    }
}

#[inline(never)]
fn gradient_color_stops<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    accept_unit: ColorHintUnit,
) -> Result<Vec<GradientColorItem>, ParseError<'i, CustomError>> {
    let mut need_recalc = false;
    // parse color stops
    let mut colors: Vec<GradientColorItem> = vec![];
    let mut prev_specified_color_hint_percentage = None;
    parser.parse_comma_separated(|parser| {
        let color = color_repr(parser, properties, st)?;
        if parser.is_exhausted() {
            colors.push(GradientColorItem::SimpleColorHint(color));
            need_recalc = true;
            return Ok(());
        }
        let hint = parse_gradient_color_hint(parser, properties, st, accept_unit)?;
        match accept_unit {
            ColorHintUnit::AngleOrPercentage => {
                let mut is_pushed = false;
                if let ColorHint::AngleOrPercentage(v) = hint {
                    if let Some(AngleOrPercentage::Percentage(prev_percentage)) =
                        prev_specified_color_hint_percentage
                    {
                        if let AngleOrPercentage::Percentage(cur_percentage) = v {
                            if prev_percentage > cur_percentage {
                                colors.push(GradientColorItem::AngleOrPercentageColorHint(
                                    color.clone(),
                                    AngleOrPercentage::Percentage(prev_percentage),
                                ));
                                is_pushed = true;
                                prev_specified_color_hint_percentage =
                                    Some(AngleOrPercentage::Percentage(prev_percentage));
                            }
                        }
                    }
                    if !is_pushed {
                        colors.push(GradientColorItem::AngleOrPercentageColorHint(
                            color.clone(),
                            v.clone(),
                        ));
                        prev_specified_color_hint_percentage = Some(v);
                    }
                }
            }
            ColorHintUnit::Length => {
                if let ColorHint::Length(v) = hint {
                    colors.push(GradientColorItem::ColorHint(color.clone(), v))
                }
            }
        }
        if !parser.is_exhausted() {
            let hint = parse_gradient_color_hint(parser, properties, st, accept_unit)?;
            match accept_unit {
                ColorHintUnit::AngleOrPercentage => {
                    let mut is_pushed = false;
                    if let ColorHint::AngleOrPercentage(v) = hint {
                        if let Some(AngleOrPercentage::Percentage(prev_percentage)) =
                            prev_specified_color_hint_percentage
                        {
                            if let AngleOrPercentage::Percentage(cur_percentage) = v {
                                if prev_percentage > cur_percentage {
                                    colors.push(GradientColorItem::AngleOrPercentageColorHint(
                                        color.clone(),
                                        AngleOrPercentage::Percentage(prev_percentage),
                                    ));
                                    is_pushed = true;
                                    prev_specified_color_hint_percentage =
                                        Some(AngleOrPercentage::Percentage(prev_percentage));
                                }
                            }
                        }
                        if !is_pushed {
                            colors.push(GradientColorItem::AngleOrPercentageColorHint(
                                color.clone(),
                                v.clone(),
                            ));
                            prev_specified_color_hint_percentage = Some(v);
                        }
                    }
                }
                ColorHintUnit::Length => {
                    if let ColorHint::Length(v) = hint {
                        colors.push(GradientColorItem::ColorHint(color, v))
                    }
                }
            }
        }
        Ok(())
    })?;
    if need_recalc {
        let colors_len = colors.len();
        if let Some(color) = colors.get_mut(0) {
            if let GradientColorItem::SimpleColorHint(color_hint) = color {
                *color = GradientColorItem::AngleOrPercentageColorHint(
                    color_hint.clone(),
                    AngleOrPercentage::Percentage(0.),
                )
            }
        }
        if let Some(color) = colors.get_mut(colors_len - 1) {
            if let GradientColorItem::SimpleColorHint(color_hint) = color {
                *color = GradientColorItem::AngleOrPercentageColorHint(
                    color_hint.clone(),
                    AngleOrPercentage::Percentage(1.),
                )
            }
        }

        // calc auto value of color stops
        let mut start = None;
        let mut idx = 0;
        let mut simple_hint_cnt = 0;
        loop {
            if idx >= colors.len() {
                break;
            };

            if let Some(GradientColorItem::SimpleColorHint(_)) = colors.get(idx) {
                if start.is_none() {
                    let mut cur_idx = idx as isize;
                    loop {
                        cur_idx -= 1;
                        if cur_idx < 0 {
                            break;
                        }
                        if let GradientColorItem::AngleOrPercentageColorHint(_, hint) =
                            colors.get(cur_idx as usize).unwrap()
                        {
                            let ratio = match hint {
                                AngleOrPercentage::Angle(angle) => {
                                    angle.to_rad().to_f32() / (2. * PI)
                                }
                                AngleOrPercentage::Percentage(percentage) => *percentage,
                            };
                            start = Some((cur_idx as usize, ratio));
                            break;
                        }
                    }
                }
                simple_hint_cnt += 1;
            }

            if let Some(GradientColorItem::AngleOrPercentageColorHint(_, hint)) = colors.get(idx) {
                if let Some((start_idx, start_ratio)) = start {
                    let end_ratio = match hint {
                        AngleOrPercentage::Angle(angle) => angle.to_rad().to_f32() / (2. * PI),
                        AngleOrPercentage::Percentage(percentage) => *percentage,
                    };
                    let item = (end_ratio - start_ratio) / ((simple_hint_cnt + 1) as f32);
                    let mut cnt = 0;
                    for i in (start_idx + 1)..idx {
                        let color_item = colors.get_mut(i).unwrap();
                        cnt += 1;
                        if let GradientColorItem::SimpleColorHint(color) = color_item {
                            *color_item = GradientColorItem::AngleOrPercentageColorHint(
                                color.clone(),
                                AngleOrPercentage::Percentage(start_ratio + (cnt as f32 * item)),
                            )
                        }
                    }
                    start = None;
                    simple_hint_cnt = 0;
                }
            }
            idx += 1;
        }
    }
    Ok(colors)
}

#[inline(never)]
pub(crate) fn gradient_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundImageItem, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let fn_name = parser.expect_function()?.clone();
        match fn_name.to_string().as_str() {
            "linear-gradient" => {
                parser.parse_nested_block(|parser| {
                    // parse deg
                    let mut deg = Angle::Deg(180.);
                    if let Ok(parsed_deg) = linear_gradient_to_angle(parser, properties, st) {
                        deg = parsed_deg
                    }
                    let colors = resolved_gradient_color_stops(parser, properties, st)?;
                    Ok(BackgroundImageItem::Gradient(
                        BackgroundImageGradientItem::LinearGradient(deg, colors.into()),
                    ))
                })
            }
            "radial-gradient" => parser.parse_nested_block(|parser| {
                let mut shape = GradientShape::Ellipse;
                let mut size = GradientSize::FarthestCorner;
                let mut position = GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5));
                if let Ok((s, si, p)) = radial_gradient_repr(parser, properties, st) {
                    shape = s;
                    size = si;
                    position = p;
                }
                let colors = resolved_gradient_color_stops(parser, properties, st)?;
                Ok(BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::RadialGradient(
                        shape,
                        size,
                        position,
                        colors.into(),
                    ),
                ))
            }),
            "conic-gradient" => parser.parse_nested_block(|parser| {
                let mut angle = Angle::Deg(0.);
                let mut position = GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5));
                match parse_conic_gradient_angle_position(parser, properties, st) {
                    (Some(parsed_angle), Some(parsed_position)) => {
                        angle = parsed_angle;
                        position = parsed_position;
                        parser.expect_comma()?;
                    }
                    (Some(parsed_angle), None) => {
                        angle = parsed_angle;
                        parser.expect_comma()?;
                    }
                    (None, Some(parsed_position)) => {
                        position = parsed_position;
                        parser.expect_comma()?;
                    }
                    (None, None) => {}
                }
                let items =
                    gradient_color_stops(parser, properties, st, ColorHintUnit::AngleOrPercentage)?;

                Ok(BackgroundImageItem::Gradient(
                    BackgroundImageGradientItem::ConicGradient(ConicGradientItem {
                        angle,
                        position,
                        items: items.into(),
                    }),
                ))
            }),
            _ => {
                Err(parser.new_custom_error(CustomError::Unsupported))
            }
        }
    })
}

#[inline(never)]
pub(crate) fn linear_gradient_to_angle<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Angle, ParseError<'i, CustomError>> {
    let mut deg = Angle::Deg(180.);
    parser.try_parse(|parser| {
        parser.parse_until_after(Delimiter::Comma, |parser| {
            // parse [to syntax]
            parser
                .try_parse(|parser| {
                    let token = parser.expect_ident()?.clone();
                    let token = token.to_string();
                    if token == "to" {
                        let dir_1 = parser.expect_ident()?.clone();
                        let dir_1 = dir_1.to_string();
                        if parser.is_exhausted() {
                            match dir_1.as_str() {
                                "top" => return Ok(Angle::Deg(0.)),
                                "right" => return Ok(Angle::Deg(90.)),
                                "bottom" => return Ok(Angle::Deg(180.)),
                                "left" => return Ok(Angle::Deg(270.)),
                                _ => return Err(parser.new_custom_error(CustomError::Unmatched)),
                            }
                        }
                        let dir_2 = parser.expect_ident()?.clone();
                        let dir_2 = dir_2.to_string();
                        match (dir_1.as_str(), dir_2.as_str()) {
                            ("top", "right") | ("right", "top") => deg = Angle::Deg(45.),
                            ("bottom", "right") | ("right", "bottom") => deg = Angle::Deg(135.),
                            ("bottom", "left") | ("left", "bottom") => deg = Angle::Deg(225.),
                            ("left", "top") | ("top", "left") => deg = Angle::Deg(315.),
                            _ => return Err(parser.new_custom_error(CustomError::Unmatched)),
                        }
                        return Ok(deg.clone());
                    }
                    Err(parser.new_custom_error(CustomError::Unmatched))
                })
                .or_else(|_e: ParseError<'i, CustomError>| {
                    // parse deg
                    deg = angle(parser, properties, st)?;
                    Ok(deg.clone())
                })
        })
    })
}

#[inline(never)]
fn radial_gradient_shape<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<GradientShape, ParseError<'i, CustomError>> {
    parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
        let next = parser.next();
        if next.is_err() {
            return Err(parser.new_custom_error(CustomError::Unsupported));
        }
        let next = next.unwrap();
        if let Token::Ident(c) = next {
            return match c.to_string().as_str() {
                "circle" => Ok(GradientShape::Circle),
                "ellipse" => Ok(GradientShape::Ellipse),
                _ => Err(parser.new_custom_error(CustomError::Unsupported)),
            };
        }
        Err(parser.new_custom_error(CustomError::Unsupported))
    })
}

fn gradient_position_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    strict: bool,
) -> Result<GradientPosition, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let next = parser.expect_ident_matching("at");
        if next.is_err() {
            return Err(parser.new_custom_error(CustomError::Eop));
        }
        let ret: Result<Length, ParseError<'i, CustomError>> = parser.try_parse(|parser| {
            let x = length(parser, properties, st)?;
            Ok(x)
        });
        let mut pos_y = Length::Ratio(0.5);
        let pos_x = if let Ok(ret) = ret {
            ret
        } else {
            parser.try_parse(|parser| {
                let next = parser.expect_ident();
                if next.is_err() {
                    return Err(parser.new_custom_error(CustomError::Unsupported));
                }
                match next.unwrap().to_string().as_str() {
                    "center" => Ok(Length::Ratio(0.5)),
                    "left" => Ok(Length::Ratio(0.)),
                    "right" => Ok(Length::Ratio(1.)),
                    "bottom" => {
                        pos_y = Length::Ratio(1.);
                        Ok(Length::Ratio(0.5))
                    }
                    "top" => {
                        pos_y = Length::Ratio(0.);
                        Ok(Length::Ratio(0.5))
                    }
                    _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                }
            })?
        };

        if parser.is_exhausted() {
            return Ok(GradientPosition::Pos(pos_x, pos_y));
        }
        let ret: Result<Length, ParseError<'i, CustomError>> = parser.try_parse(|parser| {
            let y = length(parser, properties, st)?;
            Ok(y)
        });
        let pos_y = if let Ok(r) = ret {
            r
        } else {
            parser.try_parse::<_, _, ParseError<'i, CustomError>>(|parser| {
                let next = parser.expect_ident();
                if next.is_err() {
                    return Err(parser.new_custom_error(CustomError::Unsupported));
                }
                match next.unwrap().to_string().as_str() {
                    "center" => Ok(Length::Ratio(0.5)),
                    "top" => Ok(Length::Ratio(0.)),
                    "bottom" => Ok(Length::Ratio(1.)),
                    _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                }
            })?
        };
        if !strict {
            return Ok(GradientPosition::Pos(pos_x, pos_y));
        }
        if parser.is_exhausted() {
            return Ok(GradientPosition::Pos(pos_x, pos_y));
        }
        Err(parser.new_custom_error(CustomError::Unsupported))
    })
}

#[inline(never)]
pub(crate) fn radial_gradient_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<(GradientShape, GradientSize, GradientPosition), ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        parser.parse_until_after(Delimiter::Comma, |parser| {
            let mut illegal = false;
            let mut double_size = false;
            // try match shape
            let shape = match radial_gradient_shape(parser) {
                Ok(r) => r,
                Err(_) => GradientShape::Ellipse,
            };
            // match size
            let size = match parser.try_parse(|parser| {
                let next = parser.next()?.clone();
                let ret: Result<_, ParseError<'i, CustomError>> = match next {
                    Token::Ident(c) => match c.to_string().as_str() {
                        "closest-side" => Ok(GradientSize::ClosestSide),
                        "closest-corner" => Ok(GradientSize::ClosestCorner),
                        "farthest-side" => Ok(GradientSize::FarthestSide),
                        "farthest-corner" => Ok(GradientSize::FarthestCorner),
                        _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                    },
                    Token::Percentage { unit_value, .. } => Ok(GradientSize::Len(
                        Length::Ratio(unit_value),
                        Length::Ratio(unit_value),
                    )),
                    Token::Dimension { value, unit, .. } => match unit.to_string().as_str() {
                        "px" => Ok(GradientSize::Len(Length::Px(value), Length::Px(value))),
                        "vw" => Ok(GradientSize::Len(Length::Vw(value), Length::Vw(value))),
                        "vh" => Ok(GradientSize::Len(Length::Vh(value), Length::Vh(value))),
                        "rem" => Ok(GradientSize::Len(Length::Rem(value), Length::Rem(value))),
                        "rpx" => Ok(GradientSize::Len(Length::Rpx(value), Length::Rpx(value))),
                        "em" => Ok(GradientSize::Len(Length::Em(value), Length::Em(value))),
                        "vmin" => Ok(GradientSize::Len(Length::Vmin(value), Length::Vmin(value))),
                        "vmax" => Ok(GradientSize::Len(Length::Vmax(value), Length::Vmax(value))),
                        _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                    },
                    _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                };
                #[allow(clippy::question_mark)]
                if ret.is_err() {
                    return ret;
                }
                if let GradientSize::Len(x, _) = ret.clone().unwrap() {
                    let r = parser.try_parse(|parser| {
                        let next = parser.next();
                        if next.is_err() {
                            return Err(parser.new_custom_error(CustomError::Eop));
                        }
                        match next.unwrap() {
                            Token::Ident(s) => match s.to_string().as_str() {
                                "at" | "ellipse" | "circle" => {
                                    Err(parser.new_custom_error(CustomError::Eop))
                                }
                                _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                            },
                            Token::Percentage { unit_value, .. } => {
                                Ok(GradientSize::Len(x, Length::Ratio(*unit_value)))
                            }
                            Token::Dimension { value, unit, .. } => {
                                match unit.to_string().as_str() {
                                    "px" => Ok(GradientSize::Len(x, Length::Px(*value))),
                                    "vw" => Ok(GradientSize::Len(x, Length::Vw(*value))),
                                    "vh" => Ok(GradientSize::Len(x, Length::Vh(*value))),
                                    "rem" => Ok(GradientSize::Len(x, Length::Rem(*value))),
                                    "rpx" => Ok(GradientSize::Len(x, Length::Rpx(*value))),
                                    "em" => Ok(GradientSize::Len(x, Length::Em(*value))),
                                    "vmin" => Ok(GradientSize::Len(x, Length::Vmin(*value))),
                                    "vmax" => Ok(GradientSize::Len(x, Length::Vmax(*value))),
                                    _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                                }
                            }
                            _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                        }
                        .inspect(|_| {
                            if shape == GradientShape::Circle {
                                illegal = true;
                            }
                            double_size = true;
                        })
                    });
                    if r.is_ok() {
                        return r;
                    }
                    if let Err(ParseError {
                        kind: ParseErrorKind::Custom(e),
                        ..
                    }) = &r
                    {
                        if *e != CustomError::Eop {
                            return r;
                        }
                    }
                }
                ret
            }) {
                Ok(r) => r,
                Err(_) => GradientSize::FarthestCorner,
            };
            // try match shape again
            let shape = match radial_gradient_shape(parser) {
                Ok(r) => {
                    if double_size && r == GradientShape::Circle {
                        illegal = true;
                    }
                    if !double_size && r == GradientShape::Ellipse {
                        illegal = true;
                    }
                    r
                }
                Err(_) => shape,
            };
            if illegal {
                return Err(parser.new_custom_error(CustomError::Unsupported));
            }
            // match position
            let position = match gradient_position_repr(parser, properties, st, true) {
                Ok(r) => r,
                Err(_) => GradientPosition::Pos(Length::Ratio(0.5), Length::Ratio(0.5)),
            };

            if !parser.is_exhausted() {
                return Err(parser.new_custom_error(CustomError::Unsupported));
            }
            Ok((shape, size, position))
        })
    })
}

#[inline(never)]
pub(crate) fn parse_conic_gradient_angle_position<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> (Option<Angle>, Option<GradientPosition>) {
    let angle = parser
        .try_parse(|parser| {
            parser.expect_ident_matching("from")?;
            angle(parser, properties, st)
        })
        .ok();
    let position = parser
        .try_parse(|parser| gradient_position_repr(parser, properties, st, false))
        .ok();
    (angle, position)
}
