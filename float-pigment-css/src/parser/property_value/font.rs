use alloc::string::ToString;

use super::*;

#[inline(never)]
pub(crate) fn font_face_src<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<Vec<FontSrc>, ParseError<'i, CustomError>> {
    parser.parse_comma_separated(|parser| {
        let next = parser.next()?.clone();
        let mut maybe_font_src = match &next {
            Token::Function(name) => {
                let name: &str = name;
                let fs = parser.parse_nested_block(|parser| match name {
                    "url" => {
                        let url = parser.expect_string()?.to_string();
                        Ok(FontSrc::Url(FontUrl { url, format: None }))
                    }
                    "local" => {
                        let name = font_family_name(parser)?;
                        Ok(FontSrc::Local(name))
                    }
                    _ => Err(parser.new_unexpected_token_error(next.clone())),
                })?;
                Ok(fs)
            }
            Token::UnquotedUrl(value) => {
                Ok(FontSrc::Url(FontUrl { url: value.to_string(), format: None}))
            }
            _ => Err(parser.new_unexpected_token_error(next.clone())),
        };
        if let Ok(FontSrc::Url(font_url)) = maybe_font_src.as_mut() {
            if !parser.is_exhausted() {
                let next = parser.next()?.clone();
                match &next {
                    Token::Function(name) => match name.to_string().as_str() {
                        "format" => {
                            let format = parser.parse_nested_block(|parser| {
                                parser.parse_comma_separated(
                                    |parser: &mut Parser| -> Result<String, ParseError<'_, CustomError>> {
                                        let s = parser.expect_string()?.to_string();
                                        Ok(s)
                                    }
                                )
                            })?;
                            font_url.format = Some(format);
                        },
                        _ => return Err(parser.new_unexpected_token_error(next.clone()))
                    },
                    _ => return Err(parser.new_unexpected_token_error(next.clone()))
                };
            }
        }
        maybe_font_src
    })
}

#[inline(never)]
pub(crate) fn font_display<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<FontDisplay, ParseError<'i, CustomError>> {
    let next: &str = &parser.expect_ident()?.clone();
    let next: &str = &next.to_lowercase();
    match next {
        "auto" => Ok(FontDisplay::Auto),
        "block" => Ok(FontDisplay::Block),
        "swap" => Ok(FontDisplay::Swap),
        "fallback" => Ok(FontDisplay::Fallback),
        "optional" => Ok(FontDisplay::Optional),
        _ => Err(parser.new_custom_error(CustomError::Unsupported)),
    }
}

#[inline(never)]
pub(crate) fn font_family_name<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<FontFamilyName, ParseError<'i, CustomError>> {
    let ret;
    let name = parser.next()?.clone();
    match name {
        Token::Ident(s) => {
            let s: &str = &s;
            match s.to_lowercase().as_str() {
                "serif" => ret = FontFamilyName::Serif,
                "sans-serif" => ret = FontFamilyName::SansSerif,
                "monospace" => ret = FontFamilyName::Monospace,
                "cursive" => ret = FontFamilyName::Cursive,
                "fantasy" => ret = FontFamilyName::Fantasy,
                "system-ui" => ret = FontFamilyName::SystemUi,
                _ => {
                    let mut v = vec![s.to_string()];
                    while !parser.is_exhausted() {
                        if next_is_comma(parser) || next_is_important(parser) {
                            break;
                        }
                        let name = parser.next()?.clone();
                        match name {
                            Token::Ident(s) => {
                                let s = s.to_string();
                                v.push(s);
                            }
                            _ => return Err(parser.new_unexpected_token_error::<CustomError>(name)),
                        }
                    }
                    let str = v.join(" ");
                    ret = FontFamilyName::Title(str.into());
                }
            }
        }
        Token::QuotedString(s) => {
            ret = FontFamilyName::Title(s.to_string().into());
        }
        _ => return Err(parser.new_unexpected_token_error(name)),
    }
    Ok(ret)
}

#[inline(never)]
pub(crate) fn font_family_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<FontFamilyType, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        parse_comma_separated_without_important(parser, |parser| font_family_name(parser))
            .map(|ret| FontFamilyType::Names(ret.into()))
    })
}
