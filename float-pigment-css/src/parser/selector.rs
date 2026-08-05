use super::*;
use alloc::string::ToString;
use cssparser::{
    match_ignore_ascii_case, parse_nth, BasicParseError, BasicParseErrorKind, Delimiter,
    ParseError, Parser, ParserInput, SourceLocation, SourcePosition, Token,
};

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
                                                "selection" => {
                                                    cur_frag.set_pseudo_elements(PseudoElements::Selection);
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
                                        "selection" => {
                                            cur_frag.set_pseudo_elements(PseudoElements::Selection);
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
                                                "selection" => {
                                                    cur_frag.set_pseudo_elements(PseudoElements::Selection);
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
        Ok(&Token::IncludeMatch) => AttributeOperator::List,
        // [name|=value]
        Ok(&Token::DashMatch) => AttributeOperator::Hyphen,
        // [name^=value]
        Ok(&Token::PrefixMatch) => AttributeOperator::Begin,
        // [name$=value]
        Ok(&Token::SuffixMatch) => AttributeOperator::End,
        // [name*=value]
        Ok(&Token::SubstringMatch) => AttributeOperator::Contain,
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
                Err(location.new_basic_unexpected_token_error(t.clone()))
            }
        }
        Err(_) => Ok(AttributeFlags::CaseSensitivityDependsOnName),
    }
}
