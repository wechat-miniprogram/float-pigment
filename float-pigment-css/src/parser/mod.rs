//! The CSS parser module.

use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};

use cssparser::CowRcStr;
use cssparser::{
    match_ignore_ascii_case, parse_important, Delimiter, ParseError, ParseErrorKind, Parser,
    ParserInput, SourceLocation, SourcePosition, Token,
};

use self::property_value::font::{font_display, font_face_src, font_family_name};
use crate::property::*;
use crate::sheet::*;
use crate::typing::*;

pub mod hooks;
mod media;
use media::parse_media_expression_series;
pub(crate) mod property_value;
pub(crate) mod selector;
pub(crate) use selector::*;

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
        return format!("{s}{output_extension}");
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

/// Parse a string of the property value of the specified `property_name`.
pub fn parse_property_value_string(
    property_name: &str,
    source: &str,
) -> (Vec<PropertyMeta>, Vec<Warning>) {
    let mut parser_input = ParserInput::new(source);
    let mut parser = Parser::new(&mut parser_input);
    let mut properties = vec![];
    let mut state: ParseState = ParseState::new(None, StyleParsingDebugMode::None, None);
    let _ = parse_property_value(
        &mut parser,
        property_name,
        &mut properties,
        &mut state,
        None,
    );
    (properties, state.warnings)
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
    if !(key.eq_ignore_ascii_case("import") || key.eq_ignore_ascii_case("font-face")) {
        st.import_base_path = None;
    }
    match_ignore_ascii_case! { key,
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
        },
        "media" => {
            parse_media_block(parser, sheet, st);
        },
        // IDEA support @keyframes
        "keyframes" => {
            parse_keyframes_block(parser, sheet, st);
        },
        "font-face" => {
            parse_font_face_block(parser, sheet, st);
        },
        _ => {
            parser.skip_whitespace();
            let start = parser.current_source_location();
            parse_to_block_end(parser, false, st);
            st.add_warning_with_message(
                WarningKind::UnknownAtBlock,
                format!(r#"unsupported @{key} block"#),
                start,
                parser.current_source_location(),
            );
        },
    }
}
fn str_to_media_type(s: &str) -> Option<MediaType> {
    if s.eq_ignore_ascii_case("all") {
        Some(MediaType::All)
    } else if s.eq_ignore_ascii_case("screen") {
        Some(MediaType::Screen)
    } else {
        None
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
    use crate::{property::Property, typing::DisplayType};

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

    #[test]
    fn parse_property_value_string() {
        let (prop_none, warnings) = super::parse_property_value_string("display", "none");
        assert!(warnings.is_empty());
        assert_eq!(
            prop_none[0].property().unwrap(),
            Property::Display(DisplayType::None)
        );
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
