use super::*;

pub(super) fn parse_media_expression_series<'i, 't>(
    parser: &mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<Media, ParseError<'i, CustomError>> {
    let mut media = Media::new(st.media.clone());
    parser.parse_until_before(Delimiter::CurlyBracketBlock | Delimiter::Semicolon, |p| {
        if p.is_exhausted() {
            let mut query = MediaQuery::new();
            query.add_media_expression(MediaExpression::AlwaysTrue);
            media.add_media_query(query);
            return Ok(());
        }
        // MQ4 §3.2 replaces only the invalid entry (including an empty one) with not all.
        p.parse_comma_separated(|p| {
            let start = p.current_source_location();
            let query =
                match try_media_parse(p, st, |p, st| p.parse_entirely(|p| parse_query(p, st))) {
                    Ok(query) => query,
                    Err(_) => {
                        discard(p);
                        st.add_warning(
                            WarningKind::InvalidMediaExpression,
                            start,
                            p.current_source_location(),
                        );
                        let mut query = MediaQuery::new();
                        query.add_media_expression(MediaExpression::Unknown);
                        query
                    }
                };
            media.add_media_query(query);
            Ok(())
        })?;
        Ok(())
    })?;
    Ok(media)
}

// Parser backtracking must also roll back diagnostics from abandoned alternatives.
fn try_media_parse<'i, 't, T>(
    p: &mut Parser<'i, 't>,
    st: &mut ParseState,
    parse: impl FnOnce(&mut Parser<'i, 't>, &mut ParseState) -> Result<T, ParseError<'i, CustomError>>,
) -> Result<T, ParseError<'i, CustomError>> {
    let warnings = st.warnings.len();
    let result = p.try_parse(|p| parse(p, st));
    if result.is_err() {
        st.warnings.truncate(warnings);
    }
    result
}

fn discard(p: &mut Parser<'_, '_>) {
    while p.next().is_ok() {}
}

fn parse_query<'i, 't>(
    p: &mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<MediaQuery, ParseError<'i, CustomError>> {
    let mut query = MediaQuery::new();
    if let Ok(condition) = try_media_parse(p, st, |p, st| {
        p.parse_entirely(|p| parse_condition(p, st, true, 0))
    }) {
        query.add_media_expression(condition);
        return Ok(query);
    }
    if p.try_parse(|p| p.expect_ident_matching("not")).is_ok() {
        query.set_decorator(MediaTypeDecorator::Not);
    } else if p.try_parse(|p| p.expect_ident_matching("only")).is_ok() {
        query.set_decorator(MediaTypeDecorator::Only);
    }
    let name = p.expect_ident()?;
    if ["not", "only", "and", "or", "layer"]
        .iter()
        .any(|x| name.eq_ignore_ascii_case(x))
    {
        return Err(p.new_custom_error(CustomError::Unsupported));
    }
    query.add_media_expression(
        str_to_media_type(name)
            .map(MediaExpression::MediaType)
            .unwrap_or(MediaExpression::Unknown),
    );
    if !p.is_exhausted() {
        p.expect_ident_matching("and")?;
        query.add_media_expression(parse_condition(p, st, false, 0)?);
    }
    Ok(query)
}

fn parse_condition<'i, 't>(
    p: &mut Parser<'i, 't>,
    st: &mut ParseState,
    allow_or: bool,
    depth: usize,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    if depth >= 64 {
        return Err(p.new_custom_error(CustomError::Unsupported));
    }
    if p.try_parse(|p| p.expect_ident_matching("not")).is_ok() {
        return Ok(MediaExpression::Not(Box::new(parse_in_parens(
            p,
            st,
            depth + 1,
        )?)));
    }
    let first = parse_in_parens(p, st, depth + 1)?;
    if p.is_exhausted() {
        return Ok(first);
    }
    let op = p.expect_ident()?.clone();
    let is_and = op.eq_ignore_ascii_case("and");
    if !(is_and || allow_or && op.eq_ignore_ascii_case("or")) {
        return Err(p.new_custom_error(CustomError::Unsupported));
    }
    let mut expressions = vec![first, parse_in_parens(p, st, depth + 1)?];
    while !p.is_exhausted() {
        p.expect_ident_matching(if is_and { "and" } else { "or" })?;
        expressions.push(parse_in_parens(p, st, depth + 1)?);
    }
    Ok(if is_and {
        MediaExpression::And(expressions)
    } else {
        MediaExpression::Or(expressions)
    })
}

fn parse_in_parens<'i, 't>(
    p: &mut Parser<'i, 't>,
    st: &mut ParseState,
    depth: usize,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    match p.next()?.clone() {
        Token::ParenthesisBlock => p.parse_nested_block(|p| {
            let start = p.current_source_location();
            if let Ok(expr) = try_media_parse(p, st, |p, st| {
                p.parse_entirely(|p| parse_condition(p, st, true, depth))
            }) {
                return Ok(expr);
            }
            if let Ok(expr) = p.try_parse(|p| p.parse_entirely(parse_feature)) {
                if matches!(expr, MediaExpression::UnknownFeature) {
                    st.add_warning(
                        WarningKind::InvalidMediaExpression,
                        start,
                        p.current_source_location(),
                    );
                }
                return Ok(expr);
            }
            consume_general_enclosed(p, depth)?;
            st.add_warning(
                WarningKind::InvalidMediaExpression,
                start,
                p.current_source_location(),
            );
            Ok(MediaExpression::UnknownFeature)
        }),
        Token::Function(_) => p.parse_nested_block(|p| {
            consume_general_enclosed(p, depth)?;
            Ok(MediaExpression::UnknownFeature)
        }),
        token => Err(p.new_unexpected_token_error(token)),
    }
}

fn consume_general_enclosed<'i, 't>(
    p: &mut Parser<'i, 't>,
    depth: usize,
) -> Result<(), ParseError<'i, CustomError>> {
    if depth >= 128 {
        return Err(p.new_custom_error(CustomError::Unsupported));
    }
    while !p.is_exhausted() {
        match p.next()?.clone() {
            token @ (Token::BadString(_)
            | Token::BadUrl(_)
            | Token::CloseParenthesis
            | Token::CloseSquareBracket
            | Token::CloseCurlyBracket) => {
                return Err(p.new_unexpected_token_error(token));
            }
            Token::Function(_)
            | Token::ParenthesisBlock
            | Token::SquareBracketBlock
            | Token::CurlyBracketBlock => {
                p.parse_nested_block(|p| consume_general_enclosed(p, depth + 1))?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn boolean_feature(name: &str) -> MediaExpression {
    match_ignore_ascii_case! { name,
        "width" => MediaExpression::Boolean(SizedFeature::Width),
        "height" => MediaExpression::Boolean(SizedFeature::Height),
        "orientation" | "prefers-color-scheme" => MediaExpression::AlwaysTrue,
        "resolution" | "device-pixel-ratio" => MediaExpression::Not(Box::new(MediaExpression::Resolution(0.))),
        _ => MediaExpression::UnknownFeature,
    }
}

fn parse_feature<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    if let Ok(expr) = p.try_parse(parse_named_feature) {
        return Ok(expr);
    }
    // A value-first comparison: reverse its operator to keep the feature on the left.
    let start = p.state();
    parse_range_value(p)?;
    let op = comparison(p)?;
    let name = p.expect_ident()?.clone();
    let after_name = p.state();
    p.reset(&start);
    let left = range_value(p, &name, op.reverse())?;
    p.reset(&after_name);
    if p.is_exhausted() {
        return Ok(left);
    }
    let second = comparison(p)?;
    let same_direction = matches!(
        (op, second),
        (
            MediaComparison::Less | MediaComparison::LessEqual,
            MediaComparison::Less | MediaComparison::LessEqual
        ) | (
            MediaComparison::Greater | MediaComparison::GreaterEqual,
            MediaComparison::Greater | MediaComparison::GreaterEqual
        )
    );
    if !same_direction {
        return Err(p.new_custom_error(CustomError::Unsupported));
    }
    Ok(MediaExpression::And(vec![
        left,
        range_value(p, &name, second)?,
    ]))
}

fn parse_named_feature<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    let name = p.expect_ident()?.clone();
    if p.is_exhausted() {
        return Ok(boolean_feature(&name));
    }
    if p.try_parse(|p| p.expect_colon()).is_err() {
        let op = comparison(p)?;
        return range_value(p, &name, op);
    }
    if let Some(feature) = SizedFeature::from_name(&name) {
        return Ok(feature.into_expression(parse_length(p)?));
    }
    match_ignore_ascii_case! { &name,
        "orientation" => {
            let value = p.expect_ident()?;
            Ok(if value.eq_ignore_ascii_case("portrait") { MediaExpression::Orientation(Orientation::Portrait) }
               else if value.eq_ignore_ascii_case("landscape") { MediaExpression::Orientation(Orientation::Landscape) }
               else { MediaExpression::UnknownFeature })
        },
        "prefers-color-scheme" => {
            let value = p.expect_ident()?;
            Ok(if value.eq_ignore_ascii_case("light") { MediaExpression::Theme(Theme::Light) }
               else if value.eq_ignore_ascii_case("dark") { MediaExpression::Theme(Theme::Dark) }
               else { MediaExpression::UnknownFeature })
        },
        "resolution" | "min-resolution" | "max-resolution" |
        "device-pixel-ratio" | "min-device-pixel-ratio" | "max-device-pixel-ratio" => {
            let op = if name.eq_ignore_ascii_case("min-resolution") || name.eq_ignore_ascii_case("min-device-pixel-ratio") {
                MediaComparison::GreaterEqual
            } else if name.eq_ignore_ascii_case("max-resolution") || name.eq_ignore_ascii_case("max-device-pixel-ratio") {
                MediaComparison::LessEqual
            } else { MediaComparison::Equal };
            let bare = name.eq_ignore_ascii_case("device-pixel-ratio")
                || name.eq_ignore_ascii_case("min-device-pixel-ratio")
                || name.eq_ignore_ascii_case("max-device-pixel-ratio");
            resolution_expression(p, op, bare)
        },
        _ => Err(p.new_custom_error(CustomError::Unsupported)),
    }
}

fn comparison<'i, 't>(
    p: &mut Parser<'i, 't>,
) -> Result<MediaComparison, ParseError<'i, CustomError>> {
    let token = p.next()?.clone();
    let sign = match token {
        Token::Delim(c @ ('<' | '>' | '=')) => c,
        _ => return Err(p.new_unexpected_token_error(token)),
    };
    if sign == '=' {
        return Ok(MediaComparison::Equal);
    }
    let equal = p
        .try_parse(|p| match p.next_including_whitespace()? {
            Token::Delim('=') => Ok(()),
            _ => Err(p.new_custom_error::<_, CustomError>(CustomError::Unsupported)),
        })
        .is_ok();
    Ok(match (sign, equal) {
        ('<', false) => MediaComparison::Less,
        ('<', true) => MediaComparison::LessEqual,
        (_, false) => MediaComparison::Greater,
        (_, true) => MediaComparison::GreaterEqual,
    })
}

fn parse_range_value<'i, 't>(p: &mut Parser<'i, 't>) -> Result<(), ParseError<'i, CustomError>> {
    match p.next()?.clone() {
        Token::Number { .. } | Token::Dimension { .. } | Token::Ident(_) => Ok(()),
        token => Err(p.new_unexpected_token_error(token)),
    }
}

fn range_value<'i, 't>(
    p: &mut Parser<'i, 't>,
    name: &str,
    op: MediaComparison,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    match_ignore_ascii_case! { name,
        "width" => Ok(MediaExpression::Range(SizedFeature::Width, op, parse_length(p)?)),
        "height" => Ok(MediaExpression::Range(SizedFeature::Height, op, parse_length(p)?)),
        "resolution" => resolution_expression(p, op, false),
        "device-pixel-ratio" => resolution_expression(p, op, true),
        _ => Err(p.new_custom_error(CustomError::Unsupported)),
    }
}

fn resolution_expression<'i, 't>(
    p: &mut Parser<'i, 't>,
    op: MediaComparison,
    bare: bool,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    // Follow Chromium and CSS Values 4: reject negative resolution literals before conversion,
    // despite MQ4's negative-range rule. Width/height retain negative-range comparisons.
    let token = p.next()?.clone();
    let value = match &token {
        Token::Ident(s) if s.eq_ignore_ascii_case("infinite") => {
            return Ok(MediaExpression::InfiniteResolution(op))
        }
        Token::Number { value, .. } if bare && *value >= 0. => *value,
        Token::Dimension { value, unit, .. } if *value >= 0. => {
            if unit.eq_ignore_ascii_case("dppx") || unit.eq_ignore_ascii_case("x") {
                *value
            } else if unit.eq_ignore_ascii_case("dpi") {
                *value / 96.
            } else if unit.eq_ignore_ascii_case("dpcm") {
                *value * 2.54 / 96.
            } else {
                return Err(p.new_unexpected_token_error(token));
            }
        }
        _ => return Err(p.new_unexpected_token_error(token)),
    };
    Ok(match op {
        MediaComparison::Equal => MediaExpression::Resolution(value),
        MediaComparison::LessEqual => MediaExpression::MaxResolution(value),
        MediaComparison::GreaterEqual => MediaExpression::MinResolution(value),
        _ => MediaExpression::ResolutionRange(op, value),
    })
}

fn parse_length<'i, 't>(p: &mut Parser<'i, 't>) -> Result<Length, ParseError<'i, CustomError>> {
    let token = p.next()?.clone();
    match &token {
        Token::Number { value, .. } if *value == 0. => Ok(Length::Px(0.)),
        Token::Dimension { value, unit, .. } => {
            match_ignore_ascii_case! { unit,
                "px" => Ok(Length::Px(*value)), "em" => Ok(Length::Em(*value)), "rem" => Ok(Length::Rem(*value)),
                "vw" => Ok(Length::Vw(*value)), "vh" => Ok(Length::Vh(*value)),
                "vmin" => Ok(Length::Vmin(*value)), "vmax" => Ok(Length::Vmax(*value)),
                "in" => Ok(Length::Px(*value * 96.)), "cm" => Ok(Length::Px(*value * 96. / 2.54)),
                "mm" => Ok(Length::Px(*value * 96. / 25.4)), "q" => Ok(Length::Px(*value * 96. / 101.6)),
                "pt" => Ok(Length::Px(*value * 96. / 72.)), "pc" => Ok(Length::Px(*value * 16.)),
                _ => Err(p.new_unexpected_token_error(token)),
            }
        }
        _ => Err(p.new_unexpected_token_error(token)),
    }
}
