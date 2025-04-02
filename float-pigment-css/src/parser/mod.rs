//! The CSS parser module.

use alloc::{
    borrow::ToOwned,
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};

use cssparser::{
    match_ignore_ascii_case, parse_important, parse_nth, BasicParseError, Delimiter, ParseError,
    ParseErrorKind, Parser, ParserInput, SourceLocation, SourcePosition, Token,
};
use cssparser::{BasicParseErrorKind, CowRcStr};

use self::property_value::font::{font_display, font_face_src, font_family_name};
use crate::property::*;
use crate::sheet::*;
use crate::typing::*;

pub mod hooks;
pub(crate) mod property_value;

pub(crate) const DEFAULT_INPUT_CSS_EXTENSION: &str = ".wxss";
pub(crate) const DEFAULT_OUTPUT_CSS_EXTENSION: &str = "";

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum CustomError {
    Unmatched,
    UnsupportedProperty,
    SkipErrorBlock,
    Unsupported,
    Eop,
    Reason(String),
    VariableCycle(String, bool),
    UnexpectedTokenInAttributeSelector,
    BadValueInAttr,
}

/// Warning kind.
#[allow(missing_docs)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningKind {
    Unknown = 0x10000,
    HooksGenerated,
    SerializationFailed,
    DeserializationFailed,
    UnsupportedSegment,
    UnknownAtBlock,
    InvalidMediaExpression,
    UnsupportedMediaSyntax,
    InvalidImportURL,
    MissingImportTarget,
    RecursiveImports,
    ImportNotOnTop,
    IllegalKeyframesBlock,
    IllegalKeyframesIdentifier,
    UnsupportedKeyframesSyntax,
    InvalidFontFaceProperty,
    InvalidSelector,
    UnsupportedSelector,
    InvalidPseudoElement,
    UnsupportedPseudoElement,
    InvalidPseudoClass,
    UnsupportedPseudoClass,
    InvalidProperty,
    UnsupportedProperty,
    MissingColonAfterProperty,
    InvalidEnvDefaultValue,
}

impl WarningKind {
    /// Get the error code.
    pub fn code(&self) -> u32 {
        *self as u32
    }

    /// Get a brief message of the error.
    pub fn static_message(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown error",
            Self::HooksGenerated => "warning from hooks",
            Self::SerializationFailed => "failed during serialization",
            Self::DeserializationFailed => "failed during deserialization",
            Self::UnsupportedSegment => "unsupported segment",
            Self::UnknownAtBlock => "unknown at-block",
            Self::InvalidMediaExpression => "invalid media expression",
            Self::UnsupportedMediaSyntax => "unsupported media syntax",
            Self::InvalidImportURL => "invalid @import URL",
            Self::ImportNotOnTop => "@import should appear before any other code blocks",
            Self::MissingImportTarget => "@import source not found",
            Self::RecursiveImports => "recursive @import",
            Self::IllegalKeyframesBlock => "illegal keyframes block",
            Self::IllegalKeyframesIdentifier => "illegal keyframes identifier",
            Self::UnsupportedKeyframesSyntax => "unsupported keyframes syntax",
            Self::InvalidFontFaceProperty => "invalid property inside @font-face",
            Self::InvalidSelector => "invalid selector",
            Self::UnsupportedSelector => "unsupported selector",
            Self::InvalidPseudoElement => "invalid pseudo element",
            Self::UnsupportedPseudoElement => "unsupported pseudo element",
            Self::InvalidPseudoClass => "invalid pseudo class",
            Self::UnsupportedPseudoClass => "unsupported pseudo class",
            Self::InvalidProperty => "invalid property",
            Self::UnsupportedProperty => "unsupported property",
            Self::MissingColonAfterProperty => "missing colon after property",
            Self::InvalidEnvDefaultValue => "the default value of `env()` is invalid",
        }
    }
}

impl core::fmt::Display for WarningKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.static_message())
    }
}

/// Warnings generated while parsing.
#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Warning {
    /// The category of the warning, which has a corresponding error code.
    pub kind: WarningKind,
    /// The detailed message.
    pub message: str_store::StrRef,
    /// The start line.
    pub start_line: u32,
    /// The start column in UTF-16 word.
    pub start_col: u32,
    /// The end line.
    pub end_line: u32,
    /// The end column in UTF-16 word.
    pub end_col: u32,
}

impl core::fmt::Debug for Warning {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            r#"Warning({} from line {} column {} to line {} column {}, #{})"#,
            self.message.as_str(),
            self.start_line,
            self.start_col,
            self.end_line,
            self.end_col,
            self.kind as u32
        )
    }
}

fn is_url(path: &str) -> bool {
    if path.starts_with("//") {
        return true;
    }
    // the URL protocol format is /[a-z][-+.a-z0-9]+/i
    let mut byte_iter = path.as_bytes().iter();
    let Some(c) = byte_iter.next() else {
        return false;
    };
    if c.to_ascii_lowercase().is_ascii_lowercase() {
        while let Some(c) = byte_iter.next() {
            if *c == b'-' {
                continue;
            }
            if *c == b'+' {
                continue;
            }
            if *c == b'.' {
                continue;
            }
            if c.is_ascii_lowercase() {
                continue;
            }
            if c.is_ascii_uppercase() {
                continue;
            }
            if c.is_ascii_digit() {
                continue;
            }
            if *c == b':' {
                return true;
            }
            break;
        }
    }
    false
}

fn resolve_relative_path(
    base: &str,
    rel: &str,
    input_extension: &str,
    output_extension: &str,
) -> String {
    let absolute_path = crate::path::resolve(base, rel);
    if input_extension.is_empty() && output_extension.is_empty() {
        return absolute_path;
    }
    if let Some(s) = absolute_path.strip_suffix(input_extension) {
        return format!("{}{}", s, output_extension);
    }
    if absolute_path.ends_with(output_extension) {
        return absolute_path;
    }
    absolute_path + output_extension
}

pub(crate) struct ParseState {
    import_base_path: Option<String>,
    media: Option<Rc<Media>>,
    warnings: Vec<Warning>,
    debug_mode: StyleParsingDebugMode,
    hooks: Option<Box<dyn hooks::Hooks>>,
}

impl ParseState {
    pub(crate) fn new(
        import_base_path: Option<String>,
        debug_mode: StyleParsingDebugMode,
        hooks: Option<Box<dyn hooks::Hooks>>,
    ) -> Self {
        Self {
            import_base_path,
            media: None,
            warnings: vec![],
            debug_mode,
            hooks,
        }
    }

    pub(crate) fn add_warning(
        &mut self,
        kind: WarningKind,
        start: SourceLocation,
        end: SourceLocation,
    ) {
        self.warnings.push(Warning {
            kind,
            message: kind.static_message().into(),
            start_line: start.line,
            start_col: start.column,
            end_line: end.line,
            end_col: end.column,
        })
    }

    pub(crate) fn add_warning_with_message(
        &mut self,
        kind: WarningKind,
        message: impl Into<String>,
        start: SourceLocation,
        end: SourceLocation,
    ) {
        self.warnings.push(Warning {
            kind,
            message: message.into().into(),
            start_line: start.line,
            start_col: start.column,
            end_line: end.line,
            end_col: end.column,
        })
    }
}

/// Parse string into a style sheet, returning it with warnings.
pub(crate) fn parse_style_sheet(path: &str, source: &str) -> (CompiledStyleSheet, Vec<Warning>) {
    parse_style_sheet_with_hooks(path, source, None)
}

/// Parse string into a style sheet, returning it with warnings.
///
/// Parser hooks can be attached in this function.
pub(crate) fn parse_style_sheet_with_hooks(
    path: &str,
    source: &str,
    hooks: Option<Box<dyn hooks::Hooks>>,
) -> (CompiledStyleSheet, Vec<Warning>) {
    let mut parser_input = ParserInput::new(source);
    let mut parser = Parser::new(&mut parser_input);
    let mut sheet = CompiledStyleSheet::new();
    let mut state = ParseState::new(Some(path.into()), StyleParsingDebugMode::None, hooks);
    parse_segment(&mut parser, &mut sheet, &mut state);
    (sheet, state.warnings)
}

/// The debug mode used in style parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StyleParsingDebugMode {
    /// Disable debug mode (best performance).
    None,
    /// Enable debug mode.
    Debug,
    /// Enable debug mode and mark all parsed properties disabled.
    DebugAndDisabled,
}

/// Parse inline style for an element, a.k.a. the style in `<div style="...">`.
pub fn parse_inline_style(
    source: &str,
    debug_mode: StyleParsingDebugMode,
) -> (Vec<PropertyMeta>, Vec<Warning>) {
    let mut trim_source = source.trim().to_string();
    if !trim_source.ends_with(';') {
        trim_source.push(';');
    }
    let mut parser_input = ParserInput::new(trim_source.as_str());
    let mut parser = Parser::new(&mut parser_input);
    let mut properties = vec![];
    let mut state: ParseState = ParseState::new(None, debug_mode, None);
    parse_property_list(&mut parser, &mut properties, &mut state, None);
    (properties, state.warnings)
}

pub(crate) fn parse_selector_only(source: &str) -> Result<Selector, Warning> {
    let mut parser_input = ParserInput::new(source);
    let mut parser = Parser::new(&mut parser_input);
    let mut state = ParseState::new(None, StyleParsingDebugMode::None, None);
    parse_selector(&mut parser, &mut state).map_err(|_| {
        let cur = parser.current_source_location();
        Warning {
            kind: WarningKind::InvalidSelector,
            message: WarningKind::InvalidSelector.to_string().into(),
            start_line: cur.line,
            start_col: cur.column,
            end_line: cur.line,
            end_col: cur.column,
        }
    })
}

pub(crate) fn parse_media_expression_only(source: &str) -> Result<Media, Warning> {
    let mut parser_input = ParserInput::new(source);
    let mut parser = Parser::new(&mut parser_input);
    let mut state = ParseState::new(None, StyleParsingDebugMode::None, None);
    parse_media_expression_series(&mut parser, &mut state).map_err(|_| {
        let cur = parser.current_source_location();
        Warning {
            kind: WarningKind::InvalidMediaExpression,
            message: WarningKind::InvalidMediaExpression.to_string().into(),
            start_line: cur.line,
            start_col: cur.column,
            end_line: cur.line,
            end_col: cur.column,
        }
    })
}

#[allow(dead_code)]
pub(crate) fn parse_color_to_rgba(source: &str) -> (u8, u8, u8, u8) {
    let mut parser_input = ParserInput::new(source);
    let mut parser = Parser::new(&mut parser_input);
    let ret = cssparser_color::Color::parse(&mut parser);
    ret.map(|color| match color {
        cssparser_color::Color::Rgba(rgba) => {
            (rgba.red, rgba.green, rgba.blue, (rgba.alpha * 256.) as u8)
        }
        _ => (0, 0, 0, 0),
    })
    .unwrap_or((0, 0, 0, 0))
}

fn parse_segment<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    sheet: &mut CompiledStyleSheet,
    st: &mut ParseState,
) {
    while !parser.is_exhausted() {
        parse_block(parser, sheet, st);
    }
}

fn parse_to_paren_end<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    need_warning: bool,
    st: &mut ParseState,
) {
    parser.skip_whitespace();
    let start = parser.current_source_location();
    let mut has_extra_chars = false;
    loop {
        let next = match parser.next() {
            Ok(x) => x,
            Err(_) => break,
        };
        match next {
            Token::CloseParenthesis => {
                break;
            }
            _ => {
                has_extra_chars = true;
            }
        }
    }
    if need_warning && has_extra_chars {
        let end = parser.current_source_location();
        st.add_warning(WarningKind::UnsupportedSegment, start, end);
    }
}

// may replace
fn parse_to_block_end<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    need_warning: bool,
    st: &mut ParseState,
) {
    parser.skip_whitespace();
    let start = parser.current_source_location();
    let mut has_extra_chars = false;
    loop {
        let next = match parser.next() {
            Ok(x) => x,
            Err(_) => break,
        };
        match next {
            Token::Semicolon => {
                break;
            }
            Token::CurlyBracketBlock => {
                break;
            }
            _ => {
                has_extra_chars = true;
            }
        }
    }
    if need_warning && has_extra_chars {
        let end = parser.current_source_location();
        st.add_warning(WarningKind::UnsupportedSegment, start, end);
    }
}

fn parse_block<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    sheet: &mut CompiledStyleSheet,
    st: &mut ParseState,
) {
    parser
        .try_parse(|parser| {
            // try parsing at keyword
            if let Token::AtKeyword(k) = parser.next()?.clone() {
                parse_at_keyword_block(parser, &k, sheet, st);
                Ok(())
            } else {
                Err(parser.new_custom_error(CustomError::Unmatched))
            }
        })
        .or_else(|err: ParseError<'_, CustomError>| {
            st.import_base_path = None;
            if let ParseErrorKind::Custom(err) = err.kind {
                if CustomError::Unmatched == err {
                    let rule = parse_rule(parser, st)?;
                    sheet.add_rule(rule);
                    return Ok(());
                }
                return Err(parser.new_custom_error(CustomError::Unmatched));
            }
            Err(parser.new_custom_error(CustomError::Unmatched))
        })
        .unwrap_or(())
}

fn parse_at_keyword_block<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    key: &str,
    sheet: &mut CompiledStyleSheet,
    st: &mut ParseState,
) {
    if !(key == "import" || key == "font-face") {
        st.import_base_path = None;
    }
    match key {
        "import" => {
            parser.skip_whitespace();
            let start = parser.current_source_location();
            match parser.expect_url_or_string() {
                Err(_) => {
                    parse_to_block_end(parser, false, st);
                    st.add_warning(
                        WarningKind::InvalidImportURL,
                        start,
                        parser.current_source_location(),
                    );
                }
                Ok(url) => {
                    let media = parser
                        .try_parse::<_, _, ParseError<CustomError>>(|parser| {
                            parser.expect_semicolon()?;
                            Ok(None)
                        })
                        .unwrap_or_else(|_| {
                            let media = parse_media_expression_series(parser, st);
                            match media {
                                Err(err) => {
                                    parse_to_block_end(parser, false, st);
                                    st.add_warning(
                                        WarningKind::UnsupportedMediaSyntax,
                                        err.location,
                                        err.location,
                                    );
                                    None
                                }
                                Ok(media) => {
                                    parse_to_block_end(parser, true, st);
                                    Some(Rc::new(media))
                                }
                            }
                        });
                    if let Some(base_path) = st.import_base_path.clone() {
                        let url: &str = &url;
                        if is_url(url) {
                            sheet.add_import(url.to_string(), media);
                        } else {
                            let path = resolve_relative_path(
                                base_path.as_str(),
                                url,
                                DEFAULT_INPUT_CSS_EXTENSION,
                                DEFAULT_OUTPUT_CSS_EXTENSION,
                            );
                            sheet.add_import(path, media);
                        }
                    } else {
                        st.add_warning(
                            WarningKind::ImportNotOnTop,
                            start,
                            parser.current_source_location(),
                        );
                    }
                }
            }
        }
        "media" => {
            parse_media_block(parser, sheet, st);
        }
        // IDEA support @keyframes
        "keyframes" => {
            parse_keyframes_block(parser, sheet, st);
        }
        "font-face" => {
            parse_font_face_block(parser, sheet, st);
        }
        _ => {
            parser.skip_whitespace();
            let start = parser.current_source_location();
            parse_to_block_end(parser, false, st);
            st.add_warning_with_message(
                WarningKind::UnknownAtBlock,
                format!(r#"unsupported @{} block"#, key),
                start,
                parser.current_source_location(),
            );
        }
    }
}

fn str_to_media_type(s: &str) -> Option<MediaType> {
    let s = s.to_lowercase();
    match s.as_str() {
        "all" => Some(MediaType::All),
        "screen" => Some(MediaType::Screen),
        _ => None,
    }
}

fn parse_media_block<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    sheet: &mut CompiledStyleSheet,
    st: &mut ParseState,
) {
    match parse_media_expression_series(parser, st) {
        Err(err) => {
            parse_to_block_end(parser, false, st);
            st.add_warning(
                WarningKind::UnsupportedMediaSyntax,
                err.location,
                err.location,
            );
        }
        Ok(media) => {
            if parser.expect_curly_bracket_block().is_ok() {
                let old_media = st.media.take();
                st.media = Some(Rc::new(media));
                parser
                    .parse_nested_block::<_, _, ParseError<'i, CustomError>>(|parser| {
                        parse_segment(parser, sheet, st);
                        Ok(())
                    })
                    .unwrap();
                st.media = old_media;
            }
        }
    }
}

fn parse_media_expression_series<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<Media, ParseError<'i, CustomError>> {
    let mut media = Media::new(st.media.clone());
    parser.parse_until_before(
        Delimiter::CurlyBracketBlock | Delimiter::Semicolon,
        |parser| {
            parser.parse_comma_separated(|parser| {
                let mut mq = MediaQuery::new();
                let next = parser.next()?.clone();
                match &next {
                    Token::Ident(s) => {
                        let s = s.to_owned().to_lowercase();
                        match s.as_str() {
                            "only" => {
                                mq.set_decorator(MediaTypeDecorator::Only);
                                let expr = parse_media_expression(parser, st)?;
                                mq.add_media_expression(expr);
                            }
                            "not" => {
                                mq.set_decorator(MediaTypeDecorator::Not);
                                let expr = parse_media_expression(parser, st)?;
                                mq.add_media_expression(expr);
                            }
                            _ => match str_to_media_type(&s) {
                                Some(mt) => mq.add_media_expression(MediaExpression::MediaType(mt)),
                                None => mq.add_media_expression(MediaExpression::Unknown),
                            },
                        }
                    }
                    Token::ParenthesisBlock => {
                        let expr = parse_media_expression_inner(parser, st)?;
                        mq.add_media_expression(expr);
                    }
                    _ => {
                        return Err(parser.new_unexpected_token_error(next));
                    }
                }
                loop {
                    match parser.try_parse(|parser| {
                        if parser.is_exhausted() {
                            return Err(parser.new_custom_error(CustomError::Unmatched));
                        }
                        let next = parser.next()?;
                        if let Token::Ident(s) = next {
                            let s = s.to_lowercase();
                            if s.as_str() == "and" {
                                let expr = parse_media_expression(parser, st)?;
                                mq.add_media_expression(expr);
                                return Ok(());
                            }
                        }
                        return Err(parser.new_custom_error(CustomError::Unmatched));
                    }) {
                        Ok(_) => {}
                        Err(err) => {
                            if let ParseErrorKind::Custom(err) = &err.kind {
                                if CustomError::Unmatched == *err {
                                    break;
                                }
                            }
                            return Err(err);
                        }
                    };
                }
                media.add_media_query(mq);
                Ok(())
            })
        },
    )?;
    Ok(media)
}

fn parse_media_expression<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    let token = parser.next()?.clone();
    match token {
        Token::Ident(s) => Ok(match str_to_media_type(&s) {
            Some(mt) => MediaExpression::MediaType(mt),
            None => MediaExpression::Unknown,
        }),
        Token::ParenthesisBlock => parse_media_expression_inner(parser, st),
        _ => Err(parser.new_unexpected_token_error(token)),
    }
}

fn parse_media_expression_inner<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<MediaExpression, ParseError<'i, CustomError>> {
    parser.parse_nested_block(|parser| {
        let token = parser.next()?.clone();
        if let Token::Ident(name) = &token {
            let expr = if parser.is_exhausted() {
                match str_to_media_type(name) {
                    Some(mt) => MediaExpression::MediaType(mt),
                    None => MediaExpression::Unknown,
                }
            } else {
                parser.expect_colon()?;
                let name: &str = name;
                match name {
                    "orientation" => {
                        let t = parser.expect_ident()?;
                        let t: &str = t;
                        match t {
                            "portrait" => MediaExpression::Orientation(Orientation::Portrait),
                            "landscape" => MediaExpression::Orientation(Orientation::Landscape),
                            _ => MediaExpression::Orientation(Orientation::None),
                        }
                    }
                    "width" => MediaExpression::Width(parse_px_length(parser, st)?),
                    "min-width" => MediaExpression::MinWidth(parse_px_length(parser, st)?),
                    "max-width" => MediaExpression::MaxWidth(parse_px_length(parser, st)?),
                    "height" => MediaExpression::Height(parse_px_length(parser, st)?),
                    "min-height" => MediaExpression::MinHeight(parse_px_length(parser, st)?),
                    "max-height" => MediaExpression::MaxHeight(parse_px_length(parser, st)?),
                    "prefers-color-scheme" => {
                        let t = parser.expect_ident()?;
                        let t: &str = t;
                        match t {
                            "light" => MediaExpression::Theme(Theme::Light),
                            "dark" => MediaExpression::Theme(Theme::Dark),
                            _ => MediaExpression::Unknown,
                        }
                    }
                    _ => MediaExpression::Unknown,
                }
            };
            parse_to_paren_end(parser, true, st);
            Ok(expr)
        } else {
            Err(parser.new_unexpected_token_error(token))
        }
    })
}

fn parse_keyframes_block<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    sheet: &mut CompiledStyleSheet,
    st: &mut ParseState,
) {
    parser.skip_whitespace();
    let start_location = parser.current_source_location();
    if let Ok(ident) = parse_keyframes_ident(parser) {
        if parser.expect_curly_bracket_block().is_err() {
            st.add_warning(
                WarningKind::IllegalKeyframesBlock,
                start_location,
                parser.current_source_location(),
            );
            return;
        }
        let keyframes = parser.parse_nested_block(|parser| {
            let mut keyframes = vec![];
            while !parser.is_exhausted() {
                keyframes.push(parse_keyframe_rule(parser, st)?);
            }
            Ok(keyframes)
        });
        match keyframes {
            Ok(keyframes) => sheet.add_keyframes(keyframes::KeyFrames::new(ident, keyframes)),
            Err(err) => {
                st.add_warning(
                    WarningKind::UnsupportedKeyframesSyntax,
                    err.location,
                    err.location,
                );
            }
        }
    } else {
        st.add_warning(
            WarningKind::IllegalKeyframesIdentifier,
            start_location,
            parser.current_source_location(),
        );
    }
}

fn parse_keyframe_rule<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<keyframes::KeyFrameRule, ParseError<'i, CustomError>> {
    let keyframe = parse_keyframe(parser)?;
    // get CloseCurlyBracket position
    let current_state = parser.state();
    let _ =
        parser.parse_until_after::<_, (), CustomError>(Delimiter::CurlyBracketBlock, |parser| {
            while !parser.is_exhausted() {
                parser.next()?;
            }
            Ok(())
        });
    let close_curly_block_position = parser.position();
    parser.reset(&current_state);
    parser.expect_curly_bracket_block()?;
    let mut properties: Vec<PropertyMeta> = vec![];
    parser.parse_nested_block::<_, _, CustomError>(|parser| {
        parse_property_list(
            parser,
            &mut properties,
            st,
            Some(close_curly_block_position),
        );
        Ok(())
    })?;
    Ok(keyframes::KeyFrameRule::new(keyframe, properties))
}

fn parse_keyframe<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<Vec<keyframes::KeyFrame>, ParseError<'i, CustomError>> {
    parser.parse_until_before(Delimiter::CurlyBracketBlock, |parser| {
        parser.parse_comma_separated(|parser| {
            let next = parser.next()?.clone();
            match next {
                Token::Percentage { unit_value, .. } => Ok(KeyFrame::Ratio(unit_value)),
                Token::Ident(ident) => {
                    let ident: &str = &ident.to_ascii_lowercase();
                    match ident {
                        "from" => Ok(KeyFrame::From),
                        "to" => Ok(KeyFrame::To),
                        _ => Err(parser.new_custom_error(CustomError::Unsupported)),
                    }
                }
                _ => Err(parser.new_custom_error(CustomError::Unsupported)),
            }
        })
    })
}

fn parse_keyframes_ident<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<String, ParseError<'i, CustomError>> {
    let ident = parser.parse_until_before(Delimiter::CurlyBracketBlock, |parser| {
        let ret = parser.expect_ident();
        Ok(ret?.to_string())
    })?;
    Ok(ident)
}

fn parse_font_face_block<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    sheet: &mut CompiledStyleSheet,
    st: &mut ParseState,
) {
    if parser.expect_curly_bracket_block().is_ok() {
        let mut font_face = FontFace::new();
        let mut properties = vec![];
        let _ = parser.parse_nested_block(|parser| -> Result<(), ParseError<'_, CustomError>> {
            loop {
                parser.skip_whitespace();
                if parser.is_exhausted() {
                    break;
                }
                let mut start_loc = parser.current_source_location();
                let start_pos = parser.position();
                parser
                    .parse_until_after(Delimiter::Semicolon, |parser| {
                        let (name, _) = &parse_property_name(parser, start_loc, start_pos, st)?;
                        let name: &str = name;
                        start_loc = parser.current_source_location();
                        match name {
                            "font-family" => {
                                let font_family: FontFamilyName = font_family_name(parser)?;
                                font_face.font_family = font_family;
                            }
                            "src" => {
                                let mut src: Vec<FontSrc> =
                                    font_face_src(parser, &mut properties, st)?;
                                src.iter_mut().for_each(|item| {
                                    if let FontSrc::Url(font_url) = item {
                                        let url = font_url.url.clone();
                                        if let Some(base_path) = &st.import_base_path {
                                            if !is_url(url.as_str()) {
                                                font_url.url = resolve_relative_path(
                                                    base_path,
                                                    url.as_str(),
                                                    "",
                                                    "",
                                                );
                                            }
                                        }
                                    }
                                });
                                font_face.src = src;
                            }
                            "font-style" => {
                                let font_style: FontStyleType =
                                    font_style_repr(parser, &mut properties, st)?;
                                font_face.font_style = Some(font_style);
                            }
                            "font-weight" => {
                                let font_weight: FontWeightType =
                                    font_weight_repr(parser, &mut properties, st)?;
                                font_face.font_weight = Some(font_weight);
                            }
                            "font-display" => {
                                let font_display: FontDisplay = font_display(parser)?;
                                font_face.font_display = Some(font_display);
                            }
                            _ => {
                                return Err(
                                    parser.new_custom_error(CustomError::UnsupportedProperty)
                                );
                            }
                        }
                        Ok(())
                    })
                    .unwrap_or_else(|_| {
                        st.add_warning(
                            WarningKind::InvalidFontFaceProperty,
                            start_loc,
                            parser.current_source_location(),
                        );
                    });
            }
            Ok(())
        });
        // if let Some(ff) = st.font_face.as_mut() {
        //     ff.push(font_face);
        // } else {
        //     st.font_face = Some(vec![font_face]);
        // }
        sheet.add_font_face(font_face);
    }
}
fn parse_px_length<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _st: &mut ParseState,
) -> Result<f32, ParseError<'i, CustomError>> {
    let next = parser.next()?;
    match next {
        Token::Number { value, .. } => {
            if *value == 0. {
                return Ok(0.);
            }
        }
        Token::Dimension { value, unit, .. } => {
            let unit: &str = unit;
            if unit == "px" {
                return Ok(*value);
            }
        }
        _ => {}
    }
    let next = next.clone();
    Err(parser.new_unexpected_token_error(next))
}

fn parse_rule<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<Box<Rule>, ParseError<'i, CustomError>> {
    match parse_selector(parser, st) {
        Ok(selector) => {
            // get CloseCurlyBracket position
            let current_state = parser.state();
            let _ = parser.parse_until_after::<_, (), CustomError>(
                Delimiter::CurlyBracketBlock,
                |parser| {
                    while !parser.is_exhausted() {
                        parser.next()?;
                    }
                    Ok(())
                },
            );
            let close_curly_block_position = parser.position();
            parser.reset(&current_state);
            parser.expect_curly_bracket_block()?;
            let mut properties: Vec<PropertyMeta> = vec![];
            parser.parse_nested_block::<_, _, CustomError>(|parser| {
                parse_property_list(
                    parser,
                    &mut properties,
                    st,
                    Some(close_curly_block_position),
                );
                Ok(())
            })?;
            if properties.is_empty() {
                return Err(parser.new_custom_error(CustomError::SkipErrorBlock));
            }
            Ok(Rule::new(selector, properties, st.media.clone()))
        }
        Err(_) => parser.parse_until_after(Delimiter::CurlyBracketBlock, |parser| {
            Err(parser.new_custom_error(CustomError::SkipErrorBlock))
        }),
    }
}

pub(crate) fn parse_not_function<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
    cur_frag: &mut SelectorFragment,
    prev_sep: &mut PrevSep,
    start_pos: SourcePosition,
    start_loc: SourceLocation,
) -> Result<(), ParseError<'i, CustomError>> {
    let selector = parser.parse_nested_block(|parser| parse_selector(parser, st))?;
    let mut frags = selector.fragments;
    if let Some(ref mut pseudo_classes) = cur_frag.pseudo_classes {
        match pseudo_classes.as_mut() {
            PseudoClasses::Not(v) => {
                v.append(&mut frags);
            }
            _ => {
                st.add_warning_with_message(
                    WarningKind::UnsupportedSelector,
                    format!(
                        r#"unsupported selector: {:?}"#,
                        parser.slice_from(start_pos).trim()
                    ),
                    start_loc,
                    parser.current_source_location(),
                );
                return Err(parser.new_custom_error(CustomError::Unsupported));
            }
        }
    } else {
        cur_frag.set_pseudo_classes(PseudoClasses::Not(frags));
    }
    *prev_sep = PrevSep::PseudoClassesNot;
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum NthType {
    Child,
    OfType,
}

pub(crate) fn parse_nth_function<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
    cur_frag: &mut SelectorFragment,
    prev_sep: &mut PrevSep,
    nth_type: NthType,
) -> Result<(), ParseError<'i, CustomError>> {
    parser.parse_nested_block(|parser| {
        let (a, b) = parse_nth(parser)?;
        if nth_type == NthType::OfType {
            cur_frag.set_pseudo_classes(PseudoClasses::NthOfType(a, b));
            *prev_sep = PrevSep::None;
            if parser.is_exhausted() {
                return Ok(());
            }
            return Err(parser.new_custom_error(CustomError::Unsupported));
        }
        if parser
            .try_parse(|parser| parser.expect_ident_matching("of"))
            .is_err()
        {
            cur_frag.set_pseudo_classes(PseudoClasses::NthChild(a, b, None));
            *prev_sep = PrevSep::None;
            if parser.is_exhausted() {
                return Ok(());
            }
            return Err(parser.new_custom_error(CustomError::Unsupported));
        }
        let selectors = parse_selector(parser, st)?;
        cur_frag.set_pseudo_classes(PseudoClasses::NthChild(
            a,
            b,
            Some(Box::new(selectors.fragments)),
        ));
        *prev_sep = PrevSep::None;
        Ok(())
    })
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum PrevSep {
    Init,
    None,
    Space,
    Child,
    Universal,
    NextSibling,
    SubsequentSibling,
    End,
    PseudoClassesNot,
}

pub(crate) fn parse_selector<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    st: &mut ParseState,
) -> Result<Selector, ParseError<'i, CustomError>> {
    let fragments = parser.parse_until_before(Delimiter::CurlyBracketBlock, |parser| {
        // let most_start_loc = parser.current_source_location();
        parser.parse_comma_separated(|parser| {
            parser.skip_whitespace();
            let item_start_loc = parser.current_source_location();
            let item_start_pos = parser.position();
            if parser.is_exhausted() {
                st.add_warning_with_message(
                    WarningKind::InvalidSelector,
                    format!(r#"selector not terminated: {}"#, parser.slice_from(item_start_pos).trim()),
                    item_start_loc,
                    parser.current_source_location(),
                );
                return Err(parser.new_custom_error(CustomError::Unsupported));
            }
            let mut cur_frag = SelectorFragment::new();
            let mut prev_sep = PrevSep::Init;
            macro_rules! clear_prev_sep {
                () => {
                    match prev_sep {
                        PrevSep::Space => {
                            cur_frag = SelectorFragment::with_relation(SelectorRelationType::Ancestor(
                                cur_frag,
                            ));
                        }
                        PrevSep::Child => {
                            cur_frag = SelectorFragment::with_relation(
                                SelectorRelationType::DirectParent(cur_frag),
                            );
                        }
                        PrevSep::NextSibling => {
                            cur_frag = SelectorFragment::with_relation(
                                SelectorRelationType::NextSibling(cur_frag)
                            )
                        }
                        PrevSep::SubsequentSibling => {
                            cur_frag = SelectorFragment::with_relation(
                                SelectorRelationType::SubsequentSibling(cur_frag)
                            )
                        }
                        _ => {}
                    }
                    prev_sep = PrevSep::None;
                };
            }
            while !parser.is_exhausted() {
                let start_loc = parser.current_source_location();
                let start_pos = parser.position();
                let next = match prev_sep {
                    PrevSep::None | PrevSep::PseudoClassesNot => parser.next_including_whitespace(),
                    PrevSep::End => {
                        st.add_warning_with_message(
                            WarningKind::UnsupportedSelector,
                            format!(r#"unsupported selector: {:?}"#, parser.slice_from(item_start_pos).trim()),
                            item_start_loc,
                            parser.current_source_location(),
                        );
                        Err(parser.new_basic_error(BasicParseErrorKind::EndOfInput))
                    },
                    _ => parser.next(),
                }?
                .clone();
                match next {
                    Token::Ident(ref s) => {
                        clear_prev_sep!();
                        cur_frag.set_tag_name(s);
                    }
                    Token::IDHash(ref s) => {
                        clear_prev_sep!();
                        cur_frag.set_id(s);
                    }
                    Token::Hash(_c) => {
                        st.add_warning_with_message(
                            WarningKind::InvalidSelector,
                            format!(r#"illegal ID selector: {}"#, parser.slice_from(start_pos).trim()),
                            start_loc,
                            parser.current_source_location(),
                        );
                        return Err(parser.new_custom_error(CustomError::Unsupported));
                    }
                    Token::Delim(c) => match c {
                        '.' => {
                            let class = parser.expect_ident().cloned().map_err(|_| {
                                st.add_warning_with_message(
                                    WarningKind::InvalidSelector,
                                    format!(r#"illegal classes name: {}"#, parser.slice_from(start_pos).trim()),
                                    start_loc,
                                    parser.current_source_location(),
                                );
                                parser.new_custom_error(CustomError::Unsupported)
                            })?;
                            clear_prev_sep!();
                            cur_frag.add_class(&class);
                        }
                        '>' => match prev_sep {
                            PrevSep::Init => {
                                st.add_warning_with_message(
                                    WarningKind::InvalidSelector,
                                    format!(r#"combinator (>) needs to appear after other selectors: {}"#, parser.slice_from(start_pos).trim()),
                                    start_loc,
                                    parser.current_source_location(),
                                );
                                return Err(parser.new_custom_error(CustomError::Unsupported));
                            }
                            _ => prev_sep = PrevSep::Child,
                        },
                        '+' => match prev_sep {
                            PrevSep::Init => {
                                st.add_warning_with_message(
                                    WarningKind::InvalidSelector,
                                    format!(r#"combinator (+) needs to appear after selector: {}"#, parser.slice_from(start_pos).trim()),
                                    start_loc,
                                    parser.current_source_location(),
                                );
                                return Err(parser.new_custom_error(CustomError::Unsupported));
                            }
                            _ => prev_sep = PrevSep::NextSibling,
                        }
                        '~' => match prev_sep {
                            PrevSep::Init => {
                                st.add_warning_with_message(
                                    WarningKind::InvalidSelector,
                                    format!(r#"combinator (~) needs to appear after selector: {}"#, parser.slice_from(start_pos).trim()),
                                    start_loc,
                                    parser.current_source_location(),
                                );
                                return Err(parser.new_custom_error(CustomError::Unsupported));
                            }
                            _ => prev_sep = PrevSep::SubsequentSibling
                        }
                        '*' => match prev_sep {
                            PrevSep::Space => {
                                cur_frag = SelectorFragment::with_relation(
                                    SelectorRelationType::Ancestor(cur_frag),
                                );
                                prev_sep = PrevSep::None;
                            }
                            PrevSep::Child => {
                                cur_frag = SelectorFragment::with_relation(
                                    SelectorRelationType::DirectParent(cur_frag),
                                );
                                prev_sep = PrevSep::None;
                            }
                            PrevSep::None => {
                                st.add_warning_with_message(
                                    WarningKind::InvalidSelector,
                                    format!(r#"universal selector (*) must be the first selector in the compound selector: {}"#, parser.slice_from(start_pos).trim()),
                                    start_loc,
                                    parser.current_source_location(),
                                );
                                return Err(parser.new_custom_error(CustomError::Unsupported));
                            }
                            _ => {
                                prev_sep = PrevSep::Universal;
                            }
                        },
                        _ => {
                            st.add_warning_with_message(
                                WarningKind::UnsupportedSelector,
                                format!(r#"unsupported selector: {}"#, parser.slice_from(start_pos).trim()),
                                start_loc,
                                parser.current_source_location(),
                            );
                            return Err(parser.new_custom_error(CustomError::Unsupported));
                        }
                    },
                    Token::Colon => match prev_sep {
                        PrevSep::Init => {
                            let next = parser.next_including_whitespace()?.clone();
                            match next {
                                Token::Colon => {
                                    let next = parser.next_including_whitespace()?.clone();
                                    match next {
                                        Token::Ident(pseudo_elements) => {
                                            let s = pseudo_elements.to_lowercase();
                                            match s.as_str() {
                                                "before" => {
                                                    cur_frag.set_pseudo_elements(PseudoElements::Before);
                                                    prev_sep = PrevSep::End
                                                }
                                                "after" => {
                                                    cur_frag.set_pseudo_elements(PseudoElements::After);
                                                    prev_sep = PrevSep::End
                                                }
                                                _ => {
                                                    st.add_warning_with_message(
                                                        WarningKind::UnsupportedPseudoElement,
                                                        format!("unsupported pseudo elements: {}", parser.slice_from(item_start_pos).trim()),
                                                        item_start_loc,
                                                        parser.current_source_location(),
                                                    );
                                                    return Err(
                                                        parser.new_custom_error(CustomError::Unsupported)
                                                    );
                                                }
                                            }
                                        }
                                        _ => {
                                            st.add_warning_with_message(
                                                WarningKind::UnsupportedSelector,
                                                format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                            return Err(parser.new_custom_error(CustomError::Unsupported));
                                        }
                                    }
                                }
                                Token::Ident(pseudo_classes) => {
                                    let s = pseudo_classes.to_lowercase();
                                    match s.as_str() {
                                        "first-child" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::FirstChild);
                                            prev_sep = PrevSep::None
                                        }
                                        "last-child" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::LastChild);
                                            prev_sep = PrevSep::None
                                        }
                                        "only-child" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::OnlyChild);
                                            prev_sep = PrevSep::None
                                        }
                                        "empty" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::Empty);
                                            prev_sep = PrevSep::None
                                        }
                                        "host" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::Host);
                                            prev_sep = PrevSep::End
                                        }
                                        // 
                                        "before" => {
                                            cur_frag.set_pseudo_elements(PseudoElements::Before);
                                            prev_sep = PrevSep::End;
                                            st.add_warning_with_message(
                                                WarningKind::InvalidPseudoElement,
                                                format!("pseudo-elements should begin with double colons (::): {}", parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                        }
                                        "after" => {
                                            cur_frag.set_pseudo_elements(PseudoElements::After);
                                            prev_sep = PrevSep::End;
                                            st.add_warning_with_message(
                                                WarningKind::InvalidPseudoElement,
                                                format!("pseudo-elements should begin with double colons (::): {}", parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                        }
                                        _ => {
                                            st.add_warning_with_message(
                                                WarningKind::UnsupportedPseudoClass,
                                                format!("unsupported pseudo class: {:?}", parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                            return Err(
                                                parser.new_custom_error(CustomError::Unsupported)
                                            );
                                        }
                                    }
                                }
                                Token::Function(ref name) => {
                                    let name: &str = name;
                                    match name {
                                        "not" => {
                                            parse_not_function(parser, st, &mut cur_frag, &mut prev_sep, item_start_pos, item_start_loc)?;
                                        },
                                        "nth-child" => {
                                            parse_nth_function(parser, st, &mut cur_frag, &mut prev_sep, NthType::Child)?;
                                        },
                                        "nth-of-type" => {
                                            parse_nth_function(parser, st, &mut cur_frag, &mut prev_sep, NthType::OfType)?;
                                        }
                                        _ => {
                                            st.add_warning_with_message(
                                                WarningKind::UnsupportedSelector,
                                                format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                            return Err(parser.new_custom_error(CustomError::Unsupported));
                                        }
                                    }
                                }
                                _ => {
                                    st.add_warning_with_message(
                                        WarningKind::UnsupportedSelector,
                                        format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                        item_start_loc,
                                        parser.current_source_location(),
                                    );
                                    return Err(parser.new_custom_error(CustomError::Unsupported));
                                }
                            }
                        }
                        PrevSep::None => {
                            let next = parser.next_including_whitespace()?.clone();
                            match next {
                                Token::Colon => {
                                    let next = parser.next_including_whitespace()?.clone();
                                    match next {
                                        Token::Ident(pseudo_elements) => {
                                            let s = pseudo_elements.to_lowercase();
                                            match s.as_str() {
                                                "before" => {
                                                    cur_frag.set_pseudo_elements(PseudoElements::Before);
                                                    prev_sep = PrevSep::End
                                                }
                                                "after" => {
                                                    cur_frag.set_pseudo_elements(PseudoElements::After);
                                                    prev_sep = PrevSep::End
                                                }
                                                _ => {
                                                    st.add_warning_with_message(
                                                        WarningKind::UnsupportedPseudoElement,
                                                        format!("unsupported pseudo element: {}", parser.slice_from(item_start_pos).trim()),
                                                        item_start_loc,
                                                        parser.current_source_location(),
                                                    );
                                                    return Err(
                                                        parser.new_custom_error(CustomError::Unsupported)
                                                    );
                                                }
                                            }
                                        }
                                        _ => {
                                            st.add_warning_with_message(
                                                WarningKind::UnsupportedSelector,
                                                format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                            return Err(parser.new_custom_error(CustomError::Unsupported));
                                        }
                                    }
                                }
                                Token::Ident(pseudo_classes) => {
                                    let s = pseudo_classes.to_lowercase();
                                    match s.as_str() {
                                        "first-child" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::FirstChild);
                                            prev_sep = PrevSep::None
                                        }
                                        "last-child" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::LastChild);
                                            prev_sep = PrevSep::None
                                        }
                                        "only-child" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::OnlyChild);
                                            prev_sep = PrevSep::None
                                        }
                                        "empty" => {
                                            cur_frag.set_pseudo_classes(PseudoClasses::Empty);
                                            prev_sep = PrevSep::None
                                        }
                                        // 
                                        "before" => {
                                            cur_frag.set_pseudo_elements(PseudoElements::Before);
                                            prev_sep = PrevSep::End;
                                            st.add_warning_with_message(
                                                WarningKind::InvalidPseudoElement,
                                                format!("pseudo-elements should begin with double colons (::): {}", parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                        }
                                        "after" => {
                                            cur_frag.set_pseudo_elements(PseudoElements::After);
                                            prev_sep = PrevSep::End;
                                            st.add_warning_with_message(
                                                WarningKind::InvalidPseudoElement,
                                                format!("pseudo-elements should begin with double colons (::): {}", parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                        }
                                        _ => {
                                            st.add_warning_with_message(
                                                WarningKind::UnsupportedPseudoClass,
                                                format!("unsupported pseudo class: {}", parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                            return Err(
                                                parser.new_custom_error(CustomError::Unsupported)
                                            );
                                        }
                                    }
                                }
                                Token::Function(ref name) => {
                                    let name: &str = name;
                                    match name {
                                        "not" => {
                                            parse_not_function(parser, st, &mut cur_frag, &mut prev_sep, item_start_pos, item_start_loc)?;
                                        },
                                        "nth-child" => {
                                            parse_nth_function(parser, st, &mut cur_frag, &mut prev_sep, NthType::Child)?;
                                        },
                                        "nth-of-type" => {
                                            parse_nth_function(parser, st, &mut cur_frag, &mut prev_sep, NthType::OfType)?;
                                        }
                                        _ => {
                                            st.add_warning_with_message(
                                                WarningKind::UnsupportedSelector,
                                                format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                            return Err(parser.new_custom_error(CustomError::Unsupported));
                                        }
                                    }
                                }
                                _ => {
                                    st.add_warning_with_message(
                                        WarningKind::UnsupportedSelector,
                                        format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                        item_start_loc,
                                        parser.current_source_location(),
                                    );
                                    return Err(parser.new_custom_error(CustomError::Unsupported));
                                }
                            }
                        }
                        PrevSep::PseudoClassesNot => {
                            let next = parser.next_including_whitespace()?.clone();
                            match next {
                                Token::Function(ref name) => {
                                    let name: &str = name;
                                    match name {
                                        "not" => {
                                           parse_not_function(parser, st, &mut cur_frag, &mut prev_sep, item_start_pos, item_start_loc)?;
                                        },
                                        _ => {
                                            st.add_warning_with_message(
                                                WarningKind::UnsupportedSelector,
                                                format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                                item_start_loc,
                                                parser.current_source_location(),
                                            );
                                            return Err(parser.new_custom_error(CustomError::Unsupported));
                                        }
                                    }
                                }
                                _ => {
                                    st.add_warning_with_message(
                                        WarningKind::UnsupportedSelector,
                                        format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                        item_start_loc,
                                        parser.current_source_location(),
                                    );
                                    return Err(parser.new_custom_error(CustomError::Unsupported));
                                }
                            }
                        }
                        _ => {
                            st.add_warning_with_message(
                                WarningKind::UnsupportedSelector,
                                format!(r#"unsupported selector: {}"#, parser.slice_from(item_start_pos).trim()),
                                item_start_loc,
                                parser.current_source_location(),
                            );
                            return Err(parser.new_custom_error(CustomError::Unsupported));
                        }
                    },
                    Token::WhiteSpace(_) => {
                        prev_sep = PrevSep::Space;
                    }
                    Token::CDC => {}
                    Token::CDO => {}
                    Token::Comment(_) => {
                        prev_sep = PrevSep::Space;
                    }
                    Token::SquareBracketBlock => {
                        clear_prev_sep!();
                        let attr = parser.parse_nested_block(|parser| {
                            parse_attribute_selector(parser)
                        })?;
                        cur_frag.add_attribute(attr);
                    }
                    _ => {
                        st.add_warning_with_message(
                            WarningKind::UnsupportedSelector,
                            format!(r#"unsupported selector: {}"#, parser.slice_from(start_pos).trim()),
                            start_loc,
                            parser.current_source_location(),
                        );
                        return Err(parser.new_custom_error(CustomError::Unsupported));
                    }
                };
            }
            // if let PrevSep::Init = prev_sep {
            //     st.add_warning(
            //         format!(r#"Selector should be set"#),
            //         item_start_loc,
            //         parser.current_source_location(),
            //     );
            //     return Err(parser.new_custom_error(CustomError::Unsupported));
            // };
            if let PrevSep::Child = prev_sep {
                st.add_warning_with_message(
                    WarningKind::InvalidSelector,
                    format!(r#"selector not terminated: {}"#, parser.slice_from(item_start_pos).trim()),
                    item_start_loc,
                    parser.current_source_location(),
                );
                return Err(parser.new_custom_error(CustomError::Unsupported));
            };
            Ok(cur_frag)
        })
    })?;
    Ok(Selector::from_fragments(fragments))
}

#[inline(always)]
fn parse_attribute_selector<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<Attribute, ParseError<'i, CustomError>> {
    parser.skip_whitespace();

    // parse attribute name
    let name = parser.expect_ident()?.to_string();

    // try parse operator
    let location: SourceLocation = parser.current_source_location();
    let operator = match parser.next() {
        // [name]
        Err(_) => return Ok(Attribute::new_set(name.to_string())),
        // [name=value]
        Ok(&Token::Delim('=')) => AttributeOperator::Exact,
        // [name~=value]
        Ok(&Token::IncludeMatch) => AttributeOperator::Contain,
        // [name|=value]
        Ok(&Token::DashMatch) => AttributeOperator::Hyphen,
        // [name^=value]
        Ok(&Token::PrefixMatch) => AttributeOperator::Begin,
        // [name$=value]
        Ok(&Token::SuffixMatch) => AttributeOperator::End,
        // [name*=value]
        Ok(&Token::SubstringMatch) => AttributeOperator::List,
        Ok(_) => {
            return Err(location.new_custom_error(CustomError::UnexpectedTokenInAttributeSelector))
        }
    };

    let value = match parser.expect_ident_or_string() {
        Ok(t) => t.clone(),
        Err(BasicParseError {
            kind: BasicParseErrorKind::UnexpectedToken(_),
            location,
        }) => return Err(location.new_custom_error(CustomError::BadValueInAttr)),
        Err(e) => return Err(e.into()),
    }
    .to_string();
    let never_matches = match operator {
        AttributeOperator::Exact | AttributeOperator::Hyphen => false,
        AttributeOperator::Begin | AttributeOperator::End | AttributeOperator::List => {
            value.is_empty()
        }
        AttributeOperator::Contain => value.is_empty() || value.contains(SELECTOR_WHITESPACE),
        AttributeOperator::Set => unreachable!(),
    };
    let attribute_flags = parse_attribute_flags(parser)?;
    Ok(Attribute {
        operator,
        case_insensitive: attribute_flags,
        never_matches,
        name,
        value: Some(value),
    })
}

#[inline(always)]
fn parse_attribute_flags<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
) -> Result<AttributeFlags, BasicParseError<'i>> {
    let location = parser.current_source_location();
    match parser.next() {
        Ok(t) => {
            if let Token::Ident(ref i) = t {
                Ok(match_ignore_ascii_case! {
                    i,
                    "i" => AttributeFlags::CaseInsensitive,
                    "s" => AttributeFlags::CaseSensitive,
                    _ => return Err(location.new_basic_unexpected_token_error(t.clone())),
                })
            } else {
                return Err(location.new_basic_unexpected_token_error(t.clone()));
            }
        }
        Err(_) => Ok(AttributeFlags::CaseSensitivityDependsOnName),
    }
}

#[inline(always)]
fn parse_property_list<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &'a mut Vec<PropertyMeta>,
    st: &mut ParseState,
    close_curly_block_position: Option<SourcePosition>,
) {
    loop {
        if st.debug_mode != StyleParsingDebugMode::None
            && parser
                .try_parse(|parser| loop {
                    let token = parser.next_including_whitespace_and_comments()?;
                    match token {
                        Token::Comment(s) => {
                            let mut commented_props =
                                parse_inline_style(s, StyleParsingDebugMode::DebugAndDisabled).0;
                            properties.append(&mut commented_props);
                            break Ok(());
                        }
                        Token::WhiteSpace(_) => {
                            continue;
                        }
                        _ => {
                            let token = token.clone();
                            break Err(parser.new_basic_unexpected_token_error(token));
                        }
                    }
                })
                .is_ok()
        {
            continue;
        }
        while parser.try_parse(|parser| parser.expect_semicolon()).is_ok() {}
        parser.skip_whitespace();
        if parser.is_exhausted() {
            break;
        }
        let prev_properties_len = properties.len();
        let start_loc = parser.current_source_location();
        let start_pos = parser.position();

        let mut rule_end_position = None;
        let current_state = parser.state();
        while !parser.is_exhausted() {
            if let Ok(&Token::Semicolon) = parser.next() {
                rule_end_position = Some(parser.position());
                break;
            }
        }
        if rule_end_position.is_none() {
            rule_end_position = close_curly_block_position;
        }
        parser.reset(&current_state);
        parser
            .parse_until_after(Delimiter::Semicolon, |parser| {
                let mut ret = if st.debug_mode != StyleParsingDebugMode::None {
                    parse_property_item_debug(
                        parser,
                        properties,
                        st,
                        st.debug_mode == StyleParsingDebugMode::DebugAndDisabled,
                        rule_end_position,
                    )
                } else {
                    parse_property_item(parser, properties, st, rule_end_position)
                };
                if ret.is_err() {
                    while !parser.is_exhausted() {
                        let _ = parser.next();
                    }
                    return ret;
                }
                if !parser.is_exhausted() {
                    ret = Err(parser.new_custom_error(CustomError::UnsupportedProperty));
                }
                ret
            })
            .unwrap_or_else(|err| {
                // restore properties state
                properties.drain(prev_properties_len..);
                let end_pos = parser.position();
                let end_loc = parser.current_source_location();
                let mut kind = WarningKind::UnsupportedProperty;
                let mut warning_tmpl = "unsupported property".to_string();
                if let ParseErrorKind::Custom(CustomError::Reason(s)) = err.kind {
                    kind = WarningKind::InvalidProperty;
                    warning_tmpl = s;
                }
                st.add_warning_with_message(
                    kind,
                    format!(
                        "{}: {}",
                        warning_tmpl,
                        parser.slice(start_pos..end_pos).trim()
                    ),
                    start_loc,
                    end_loc,
                );
            });
    }
}

#[inline(always)]
fn parse_property_item<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &'a mut Vec<PropertyMeta>,
    st: &mut ParseState,
    rule_end_position: Option<SourcePosition>,
) -> Result<(), ParseError<'i, CustomError>> {
    parser.skip_whitespace();
    let prop_name_start_loc = parser.current_source_location();
    let prop_name_start_pos = parser.position();
    let (name, is_custom_property) =
        parse_property_name(parser, prop_name_start_loc, prop_name_start_pos, st)?;
    if is_custom_property {
        parse_custom_property_value_with_important(parser, &name, properties, rule_end_position)?;
    } else {
        parse_property_value_with_important(
            parser,
            &name,
            properties,
            prop_name_start_loc,
            st,
            rule_end_position,
        )?;
    }
    Ok(())
}

#[inline(always)]
fn parse_property_item_debug<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &'a mut Vec<PropertyMeta>,
    st: &mut ParseState,
    disabled: bool,
    rule_end_position: Option<SourcePosition>,
) -> Result<(), ParseError<'i, CustomError>> {
    parser.skip_whitespace();
    let prev_properties_len = properties.len();
    let prop_name_start_index = parser.position();
    let prop_name_start_loc = parser.current_source_location();
    let (name, is_custom_property) =
        parse_property_name(parser, prop_name_start_loc, prop_name_start_index, st)?;
    let prop_value_start_index = parser.position();
    if is_custom_property {
        parse_custom_property_value_with_important(parser, &name, properties, rule_end_position)?;
    } else {
        parse_property_value_with_important(
            parser,
            &name,
            properties,
            prop_name_start_loc,
            st,
            rule_end_position,
        )?;
    }
    let mut is_important = false;
    let grouped_properties = properties
        .drain(prev_properties_len..)
        .map(|p| match p {
            PropertyMeta::Normal { property } => property,
            PropertyMeta::Important { property } => {
                is_important = true;
                property
            }
            PropertyMeta::DebugGroup { .. } => unreachable!(),
        })
        .collect::<Box<_>>();
    let name_with_colon = parser.slice(prop_name_start_index..prop_value_start_index);
    let name = &name_with_colon[0..(name_with_colon.len() - 1)];
    let value = parser.slice_from(prop_value_start_index);
    properties.push(PropertyMeta::DebugGroup {
        original_name_value: Box::new((name.into(), value.into())),
        properties: grouped_properties,
        important: is_important,
        disabled,
    });
    Ok(())
}

#[inline(always)]
fn parse_property_name<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    prop_name_start_loc: SourceLocation,
    prop_name_start_pos: SourcePosition,
    st: &mut ParseState,
) -> Result<(CowRcStr<'i>, bool), ParseError<'i, CustomError>> {
    let t = parser.expect_ident().cloned();
    let name = t.inspect_err(|_| {
        st.add_warning_with_message(
            WarningKind::InvalidProperty,
            format!(
                r#"invalid property: {}"#,
                parser.slice_from(prop_name_start_pos).trim()
            ),
            prop_name_start_loc,
            parser.current_source_location(),
        );
    })?;
    parser.expect_colon().inspect_err(|_| {
        st.add_warning_with_message(
            WarningKind::MissingColonAfterProperty,
            format!(
                r#"expect colon after property: {}"#,
                parser.slice_from(prop_name_start_pos).trim()
            ),
            prop_name_start_loc,
            parser.current_source_location(),
        );
    })?;
    let is_custom_property = name.starts_with("--");
    Ok((name, is_custom_property))
}

#[inline(always)]
fn parse_property_value_with_important<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    name: &str,
    properties: &'a mut Vec<PropertyMeta>,
    prop_name_start_loc: SourceLocation,
    st: &mut ParseState,
    rule_end_position: Option<SourcePosition>,
) -> Result<(), ParseError<'i, CustomError>> {
    let prev_properties_len = properties.len();
    let skip_parse_important =
        parse_property_value(parser, name, properties, st, rule_end_position)?;
    if !skip_parse_important {
        let is_important = parser.try_parse(parse_important).is_ok();
        if is_important {
            for pm in &mut properties[prev_properties_len..] {
                let mut pm2: PropertyMeta = PropertyMeta::Normal {
                    property: Property::Unknown,
                };
                core::mem::swap(&mut pm2, pm);
                *pm = match pm2 {
                    PropertyMeta::Normal { property } => PropertyMeta::Important { property },
                    PropertyMeta::Important { .. } => unreachable!(),
                    PropertyMeta::DebugGroup { .. } => unreachable!(),
                };
            }
        };
    }
    for mut pm in &mut properties[prev_properties_len..] {
        let ParseState {
            ref mut warnings,
            ref mut hooks,
            ..
        } = st;
        if let Some(hooks) = hooks.as_mut() {
            if let Some(p) = match &mut pm {
                PropertyMeta::Normal { property } => Some(property),
                PropertyMeta::Important { property } => Some(property),
                PropertyMeta::DebugGroup { .. } => None,
            } {
                let ctx = &mut hooks::ParserHooksContext {
                    warnings,
                    start_loc: prop_name_start_loc,
                    end_loc: parser.current_source_location(),
                };
                hooks.parsed_property(ctx, p);
            }
        }
    }
    Ok(())
}

#[inline(always)]
fn parse_custom_property_value_with_important<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    name: &str,
    properties: &'a mut Vec<PropertyMeta>,
    rule_end_position: Option<SourcePosition>,
) -> Result<(), ParseError<'i, CustomError>> {
    if name.len() <= 2 {
        return Err(parser.new_custom_error(CustomError::Unmatched));
    }
    let value_start_pos = parser.position();
    parser
        .parse_until_before::<_, _, CustomError>(Delimiter::Semicolon, |parser| {
            while !parser.is_exhausted() {
                parser.next()?;
            }
            let mut value: &str = parser
                .slice(value_start_pos..rule_end_position.unwrap_or_else(|| parser.position()));
            value = value.trim_end_matches(['\n', '}', ';']);
            if value.trim_end().ends_with("!important") {
                value = value.trim_end().trim_end_matches("!important");
                if value.trim_end().ends_with("!important") {
                    return Err(parser.new_custom_error(CustomError::Unmatched));
                }
                properties.push(PropertyMeta::Important {
                    property: Property::CustomProperty(CustomPropertyType::Expr(
                        name.trim().into(),
                        value.into(),
                    )),
                })
            } else {
                properties.push(PropertyMeta::Normal {
                    property: Property::CustomProperty(CustomPropertyType::Expr(
                        name.trim().into(),
                        value.into(),
                    )),
                });
            }
            // TODO impl debug
            Ok(())
        })
        .map_err(|_| parser.new_custom_error(CustomError::Unsupported))
}

#[cfg(test)]
mod test {

    use super::{is_url, parse_color_to_rgba, resolve_relative_path};

    #[test]
    fn parse_color_test() {
        let source = "#FFFFFF";
        let ret = parse_color_to_rgba(source);
        assert_eq!(ret.0, 255);
        assert_eq!(ret.1, 255);
        assert_eq!(ret.2, 255);
        assert_eq!(ret.3, 255);

        let source = "red";
        let ret = parse_color_to_rgba(source);
        assert_eq!(ret.0, 255);
        assert_eq!(ret.1, 0);
        assert_eq!(ret.2, 0);
        assert_eq!(ret.3, 255);
    }

    #[test]
    fn resolve_relative_path_test() {
        assert_eq!(
            resolve_relative_path("/src/components/a.wxss", "./hello.wxss", ".wxss", ".css"),
            "src/components/hello.css"
        );

        assert_eq!(
            resolve_relative_path("src/components/a.wxss", "./hello.wxss", ".wxss", ".css"),
            "src/components/hello.css"
        );

        assert_eq!(
            resolve_relative_path("src/components/a.wxss", "../hello.wxss", ".wxss", ".css"),
            "src/hello.css"
        );

        assert_eq!(
            resolve_relative_path("src/components/a.wxss", ".././hello.wxss", ".wxss", ".css"),
            "src/hello.css"
        );

        assert_eq!(
            resolve_relative_path(
                "src/components/test/a.wxss",
                "../../test/../hello.wxss",
                ".wxss",
                ".css"
            ),
            "src/hello.css"
        );

        assert_eq!(
            resolve_relative_path(
                "src/components/test/a.wxss",
                "../../test/../hello.wxss",
                "",
                ".css"
            ),
            "src/hello.wxss.css"
        );

        assert_eq!(
            resolve_relative_path(
                "src/components/a.wxss",
                "../../../../../hello.wxss",
                ".wxss",
                ".css"
            ),
            "../../../hello.css"
        );

        assert_eq!(
            resolve_relative_path("src/components/a.wxss", "/hello.wxss", ".wxss", ".css"),
            "hello.css"
        );

        assert_eq!(
            resolve_relative_path("src/components/././a.wxss", "/hello.wxss", ".wxss", ".css"),
            "hello.css"
        );

        assert_eq!(
            resolve_relative_path(
                "src/components/.\\.\\a.wxss",
                "/hello.wxss",
                ".wxss",
                ".css"
            ),
            "hello.css"
        );

        assert!(is_url("https://wxweb/float-pigment"));
        assert!(is_url("http://wxweb/float-pigment"));
        assert!(is_url("data:application/octet-stream;base64,AAEAAAALAIAAAwAwR1NVQrD+s+0AAAE4AAAAQk9TLzJAKEx+AAABfAAAAFZjbWFw65cFHQAAAhwAAAJQZ2x5ZvCRR/EAAASUAAAKtGhlYWQLKIN9AAAA4AAAADZoaGVhCCwD+gAAALwAAAAkaG10eEJo//8AA="));
        assert!(!is_url("www.wxweb/float-pigment"));
        assert!(!is_url("www.wxweb/float-pigment"));
    }

    #[cfg(test)]
    mod parse_inline_style {
        use crate::{
            parser::{parse_inline_style, StyleParsingDebugMode},
            typing::LengthType,
        };

        #[test]
        fn single_prop_ends_without_semicolon() {
            let (props, warnings) = parse_inline_style("width: 100px", StyleParsingDebugMode::None);
            assert!(warnings.is_empty());
            let width = props.get(0).unwrap().property().unwrap().width().unwrap();
            assert_eq!(width, LengthType::Px(100.));
        }

        #[test]
        fn single_prop_ends_with_semicolon() {
            let (props, warnings) =
                parse_inline_style("width: 100px;", StyleParsingDebugMode::None);
            assert!(warnings.is_empty());
            let width = props.get(0).unwrap().property().unwrap().width().unwrap();
            assert_eq!(width, LengthType::Px(100.));
        }

        #[test]
        fn multi_props_ends_with_semicolon() {
            let (props, warnings) =
                parse_inline_style("width: 100px;height: 200px;", StyleParsingDebugMode::None);
            assert!(warnings.is_empty());
            let width = props.get(0).unwrap().property().unwrap().width().unwrap();
            assert_eq!(width, LengthType::Px(100.));
            let height = props.get(1).unwrap().property().unwrap().height().unwrap();
            assert_eq!(height, LengthType::Px(200.));
        }

        #[test]
        fn multi_props_ends_without_semicolon() {
            let (props, warnings) =
                parse_inline_style("width: 100px;height: 200px ", StyleParsingDebugMode::None);
            assert!(warnings.is_empty());
            let width = props.get(0).unwrap().property().unwrap().width().unwrap();
            assert_eq!(width, LengthType::Px(100.));
            let height = props.get(1).unwrap().property().unwrap().height().unwrap();
            assert_eq!(height, LengthType::Px(200.));
        }
    }
}
