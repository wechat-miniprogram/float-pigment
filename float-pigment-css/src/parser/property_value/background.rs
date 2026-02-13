use alloc::string::ToString;

use super::*;

#[derive(PartialEq, Clone, Debug, Copy)]
enum Pos {
    Center,
    Left,
    Right,
    Bottom,
    Top,
}

impl Pos {
    fn is_same_direction(a: Self, b: Self) -> bool {
        match a {
            Pos::Left => b == Pos::Right || b == Pos::Left,
            Pos::Right => b == Pos::Right || b == Pos::Left,
            Pos::Top => b == Pos::Top || b == Pos::Bottom,
            Pos::Bottom => b == Pos::Top || b == Pos::Bottom,
            Pos::Center => false,
        }
    }
    fn is_valid_direction(a: Self, is_horizontal: bool) -> bool {
        match a {
            Pos::Left | Pos::Right => is_horizontal,
            Pos::Top | Pos::Bottom => !is_horizontal,
            Pos::Center => true,
        }
    }
}

#[derive(Debug)]
enum ValueWrapper {
    Keyword(Pos),
    Length(Length),
}

fn try_parse_pos_keyword<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<Pos, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let next = parser.next()?.clone();
        let name;
        if let Token::Ident(s) = next.clone() {
            name = s.to_string().to_lowercase();
        } else {
            return Err(parser.new_unexpected_token_error(next.clone()));
        }
        let ret = match name.as_str() {
            "center" => Ok(Pos::Center),
            "left" => Ok(Pos::Left),
            "right" => Ok(Pos::Right),
            "bottom" => Ok(Pos::Bottom),
            "top" => Ok(Pos::Top),
            _ => Err(parser.new_unexpected_token_error(next)),
        };
        ret
    })
}
fn try_parse_length<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Length, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| length_percentage(parser, properties, st))
}

fn bg_pos_is_end<'a, 't: 'a, 'i: 't>(parser: &'a mut Parser<'i, 't>) -> bool {
    parser
        .try_parse(|parser| -> Result<(), bool> {
            loop {
                let next = parser.next();
                if next.is_err() {
                    Err(true)?;
                }
                let next = next.unwrap().clone();
                match next {
                    Token::Comma | Token::Semicolon | Token::CurlyBracketBlock => {
                        break;
                    }
                    Token::Delim(d) => {
                        if d == '!' && parser.expect_ident_matching("important").is_ok() {
                            break;
                        }
                        Err(false)?;
                    }
                    _ => {
                        Err(false)?;
                    }
                }
            }
            Err(true)
        })
        .unwrap_err()
}
#[inline(never)]
pub(crate) fn background_position_x_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionType, ParseError<'i, CustomError>> {
    background_position_value(parser, properties, st, true)
}

#[inline(never)]
pub(crate) fn background_position_y_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionType, ParseError<'i, CustomError>> {
    background_position_value(parser, properties, st, false)
}

#[inline(never)]
pub(crate) fn background_position_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    is_horizontal: bool,
) -> Result<BackgroundPositionType, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let pos_arr: Vec<BackgroundPositionItem> = parser.parse_comma_separated(|parser| {
            let dir = try_parse_pos_keyword(parser);
            let len = try_parse_length(parser, properties, st);
            match (dir, len) {
                (Ok(dir), Ok(len)) => {
                    if !Pos::is_valid_direction(dir, is_horizontal) {
                        return Err(parser.new_custom_error(CustomError::Unsupported));
                    }
                    match dir {
                        Pos::Left => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Left(len),
                        )),
                        Pos::Right => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Right(len),
                        )),
                        Pos::Top => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Top(len),
                        )),
                        Pos::Bottom => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Bottom(len),
                        )),
                        Pos::Center => Err(parser.new_custom_error(CustomError::Unsupported)),
                    }
                }
                (Err(_), Ok(len)) => {
                    if is_horizontal {
                        return Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Left(len),
                        ));
                    }
                    Ok(BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                        len,
                    )))
                }
                (Ok(dir), Err(_)) => {
                    if !Pos::is_valid_direction(dir, is_horizontal) {
                        return Err(parser.new_custom_error(CustomError::Unsupported));
                    }
                    match dir {
                        Pos::Left => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Left(Length::Ratio(0.)),
                        )),
                        Pos::Right => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Left(Length::Ratio(1.0)),
                        )),
                        Pos::Top => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Top(Length::Ratio(0.)),
                        )),
                        Pos::Bottom => Ok(BackgroundPositionItem::Value(
                            BackgroundPositionValue::Top(Length::Ratio(1.)),
                        )),
                        Pos::Center => {
                            if is_horizontal {
                                return Ok(BackgroundPositionItem::Value(
                                    BackgroundPositionValue::Left(Length::Ratio(0.5)),
                                ));
                            }
                            Ok(BackgroundPositionItem::Value(BackgroundPositionValue::Top(
                                Length::Ratio(0.5),
                            )))
                        }
                    }
                }
                _ => Err(parser.new_custom_error(CustomError::Unsupported)),
            }
        })?;
        Ok(BackgroundPositionType::List(pos_arr.into()))
    })
}

#[inline(never)]
pub(crate) fn _bg_pos_single_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    with_check: bool,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        if let Ok(key) = try_parse_pos_keyword(parser) {
            match key {
                Pos::Left => {
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    ))
                }
                Pos::Right => {
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(1.)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    ))
                }
                Pos::Center => {
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    ))
                }
                Pos::Top => {
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        BackgroundPositionValue::Top(Length::Ratio(0.)),
                    ))
                }
                Pos::Bottom => {
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        BackgroundPositionValue::Top(Length::Ratio(1.)),
                    ))
                }
            }
        }
        let len = try_parse_length(parser, properties, st)?;
        if with_check && !bg_pos_is_end(parser) {
            Err(parser.new_custom_error(CustomError::Unsupported))?;
        }
        Ok(BackgroundPositionItem::Pos(
            BackgroundPositionValue::Left(len),
            BackgroundPositionValue::Top(Length::Ratio(0.5)),
        ))
    })
}

#[inline(never)]
pub(crate) fn bg_pos_single_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    _bg_pos_single_value(parser, properties, st, true)
}

#[inline(never)]
pub(crate) fn bg_pos_single_value_without_extra_check<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    _bg_pos_single_value(parser, properties, st, false)
}

#[inline(never)]
pub(crate) fn _bg_pos_two_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    with_check: bool,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let first: ValueWrapper;
        let second: ValueWrapper;
        if let Ok(key) = try_parse_pos_keyword(parser) {
            first = ValueWrapper::Keyword(key);
        } else {
            let len = try_parse_length(parser, properties, st)?;
            first = ValueWrapper::Length(len);
        };
        if let Ok(key) = try_parse_pos_keyword(parser) {
            second = ValueWrapper::Keyword(key);
        } else {
            let len = try_parse_length(parser, properties, st)?;
            second = ValueWrapper::Length(len);
        };
        if with_check && !bg_pos_is_end(parser) {
            Err(parser.new_custom_error(CustomError::Unsupported))?;
        }
        match (first, second) {
            (ValueWrapper::Keyword(f), ValueWrapper::Keyword(s)) => {
                let mut _f = f;
                let mut _s = s;
                if (f == s && (f != Pos::Center)) || Pos::is_same_direction(f, s) {
                    Err(parser.new_custom_error(CustomError::Unmatched))?
                }
                if s == Pos::Left || s == Pos::Right || f == Pos::Bottom || f == Pos::Top {
                    _f = s;
                    _s = f;
                }
                let x = match _f {
                    Pos::Left => BackgroundPositionValue::Left(Length::Ratio(0.)),
                    Pos::Center => BackgroundPositionValue::Left(Length::Ratio(0.5)),
                    Pos::Right => BackgroundPositionValue::Left(Length::Ratio(1.)),
                    _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
                };
                let y = match _s {
                    Pos::Top => BackgroundPositionValue::Top(Length::Ratio(0.)),
                    Pos::Center => BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    Pos::Bottom => BackgroundPositionValue::Top(Length::Ratio(1.)),
                    _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
                };
                Ok(BackgroundPositionItem::Pos(x, y))
            }
            (ValueWrapper::Length(f), ValueWrapper::Length(s)) => Ok(BackgroundPositionItem::Pos(
                BackgroundPositionValue::Left(f),
                BackgroundPositionValue::Top(s),
            )),
            (ValueWrapper::Keyword(f), ValueWrapper::Length(s)) => {
                if f == Pos::Center {
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        BackgroundPositionValue::Top(s),
                    ));
                }
                if f == Pos::Bottom || f == Pos::Top {
                    Err(parser.new_custom_error(CustomError::Unmatched))?
                }
                let x = match f {
                    Pos::Left => BackgroundPositionValue::Left(Length::Ratio(0.)),
                    Pos::Right => BackgroundPositionValue::Left(Length::Ratio(1.)),
                    _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
                };
                Ok(BackgroundPositionItem::Pos(
                    x,
                    BackgroundPositionValue::Top(s),
                ))
            }
            (ValueWrapper::Length(f), ValueWrapper::Keyword(s)) => {
                if s == Pos::Center {
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(f),
                        BackgroundPositionValue::Top(Length::Ratio(0.5)),
                    ));
                }
                if s == Pos::Left || s == Pos::Right {
                    Err(parser.new_custom_error(CustomError::Unmatched))?
                }
                let y = match s {
                    Pos::Top => BackgroundPositionValue::Top(Length::Ratio(0.)),
                    Pos::Bottom => BackgroundPositionValue::Top(Length::Ratio(1.)),
                    _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
                };
                Ok(BackgroundPositionItem::Pos(
                    BackgroundPositionValue::Left(f),
                    y,
                ))
            }
        }
    })
}

#[inline(never)]
pub(crate) fn bg_pos_two_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    _bg_pos_two_value(parser, properties, st, true)
}

#[inline(never)]
pub(crate) fn bg_pos_two_value_without_extra_check<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    _bg_pos_two_value(parser, properties, st, false)
}

#[inline(never)]
pub(crate) fn _bg_pos_three_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
    with_check: bool,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let f_k = try_parse_pos_keyword(parser)?;
        let mut f_v = None;
        let mut s_v = None;
        if let Ok(len) = try_parse_length(parser, properties, st) {
            f_v = Some(len);
        }
        let s_k = try_parse_pos_keyword(parser)?;
        if f_v.is_none() {
            let len = try_parse_length(parser, properties, st)?;
            s_v = Some(len);
        }
        if (f_k == Pos::Center && f_v.is_some())
            || (s_k == Pos::Center && s_v.is_some())
            || (f_v.is_some() && s_v.is_some())
            || (f_k != Pos::Center && f_k == s_k)
            || Pos::is_same_direction(f_k, s_k)
            || (with_check && !bg_pos_is_end(parser))
        {
            Err(parser.new_custom_error(CustomError::Unsupported))?
        };
        let mut x = BackgroundPositionValue::Left(Length::Ratio(0.5));
        let mut set_x = false;
        let mut y = BackgroundPositionValue::Left(Length::Ratio(0.5));
        if let Some(v) = f_v {
            match f_k {
                Pos::Left => {
                    x = BackgroundPositionValue::Left(v);
                    set_x = true;
                }
                Pos::Right => {
                    x = BackgroundPositionValue::Right(v);
                    set_x = true;
                }
                Pos::Top => {
                    y = BackgroundPositionValue::Top(v);
                }
                Pos::Bottom => {
                    y = BackgroundPositionValue::Bottom(v);
                }
                _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
            }
            match s_k {
                Pos::Left => {
                    x = BackgroundPositionValue::Left(Length::Ratio(0.));
                }
                Pos::Right => {
                    x = BackgroundPositionValue::Left(Length::Ratio(1.));
                }
                Pos::Top => {
                    y = BackgroundPositionValue::Top(Length::Ratio(0.));
                }
                Pos::Bottom => {
                    y = BackgroundPositionValue::Top(Length::Ratio(1.));
                }
                Pos::Center => {
                    if set_x {
                        return Ok(BackgroundPositionItem::Pos(
                            x,
                            BackgroundPositionValue::Top(Length::Ratio(0.5)),
                        ));
                    }
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        y,
                    ));
                }
            }
        } else {
            let v = s_v.unwrap();
            match s_k {
                Pos::Left => {
                    x = BackgroundPositionValue::Left(v);
                    set_x = true;
                }
                Pos::Right => {
                    x = BackgroundPositionValue::Right(v);
                    set_x = true;
                }
                Pos::Top => y = BackgroundPositionValue::Top(v),
                Pos::Bottom => y = BackgroundPositionValue::Bottom(v),
                _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
            }
            match f_k {
                Pos::Left => {
                    x = BackgroundPositionValue::Left(Length::Ratio(0.));
                }
                Pos::Right => {
                    x = BackgroundPositionValue::Left(Length::Ratio(1.));
                }
                Pos::Top => {
                    y = BackgroundPositionValue::Top(Length::Ratio(0.));
                }
                Pos::Bottom => {
                    y = BackgroundPositionValue::Top(Length::Ratio(1.));
                }
                Pos::Center => {
                    if set_x {
                        return Ok(BackgroundPositionItem::Pos(
                            x,
                            BackgroundPositionValue::Top(Length::Ratio(0.5)),
                        ));
                    }
                    return Ok(BackgroundPositionItem::Pos(
                        BackgroundPositionValue::Left(Length::Ratio(0.5)),
                        y,
                    ));
                }
            }
        }
        Ok(BackgroundPositionItem::Pos(x, y))
    })
}

#[inline(never)]
pub(crate) fn bg_pos_three_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    _bg_pos_three_value(parser, properties, st, true)
}

#[inline(never)]
pub(crate) fn bg_pos_three_value_without_extra_check<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    _bg_pos_three_value(parser, properties, st, false)
}

#[inline(never)]
pub(crate) fn bg_pos_four_value<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<BackgroundPositionItem, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        let mut f_k = try_parse_pos_keyword(parser)?;
        let mut f_v = length_percentage(parser, properties, st)?;
        let mut s_k = try_parse_pos_keyword(parser)?;
        let mut s_v = length_percentage(parser, properties, st)?;
        if f_k == Pos::Center
            || s_k == Pos::Center
            || f_k == s_k
            || Pos::is_same_direction(f_k, s_k)
        {
            return Err(parser.new_custom_error(CustomError::Unsupported));
        }
        if f_k == Pos::Bottom || f_k == Pos::Top || s_k == Pos::Left || s_k == Pos::Right {
            (f_k, s_k) = (s_k, f_k);
            (f_v, s_v) = (s_v, f_v);
        }
        let x = match f_k {
            Pos::Left => BackgroundPositionValue::Left(f_v),
            Pos::Right => BackgroundPositionValue::Right(f_v),
            _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
        };
        let y = match s_k {
            Pos::Top => BackgroundPositionValue::Top(s_v),
            Pos::Bottom => BackgroundPositionValue::Bottom(s_v),
            _ => return Err(parser.new_custom_error(CustomError::Unsupported)),
        };
        Ok(BackgroundPositionItem::Pos(x, y))
    })
}
