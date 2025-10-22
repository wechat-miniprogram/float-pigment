use cssparser::{ParseError, Parser, Token};

use crate::{
    parser::{property_value::custom_ident_repr, CustomError, ParseState},
    sheet::PropertyMeta,
    typing::GridAutoFlow,
};

#[inline(never)]
pub(crate) fn line_names<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut [PropertyMeta],
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
    _properties: &mut [PropertyMeta],
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
pub(crate) fn grid_auto_flow_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    _properties: &mut [PropertyMeta],
    _st: &mut ParseState,
) -> Result<GridAutoFlow, ParseError<'i, CustomError>> {
    #[derive(PartialEq, Clone, Copy)]
    enum FlowDirection {
        Row,
        Column,
    }
    let mut flow_direction = None;
    let mut dense = false;
    let check_flow_direction = move |target: FlowDirection| {
        if flow_direction.is_none() {
            return true;
        }
        flow_direction.unwrap() != target
    };
    while !parser.is_exhausted() {
        let next = parser.next()?;
        match next {
            Token::Ident(ident) => {
                let ident: &str = ident;
                match ident {
                    "row" if check_flow_direction(FlowDirection::Row) => {
                        flow_direction.replace(FlowDirection::Row);
                    }
                    "column" if check_flow_direction(FlowDirection::Column) => {
                        flow_direction.replace(FlowDirection::Column);
                    }
                    "dense" if !dense => {
                        dense = true;
                    }
                    _ => {
                        let next = next.clone();
                        return Err(parser.new_unexpected_token_error(next));
                    }
                }
            }
            _ => {
                let next = next.clone();
                return Err(parser.new_unexpected_token_error(next));
            }
        }
    }
    match (flow_direction, dense) {
        (Some(FlowDirection::Row), true) => Ok(GridAutoFlow::RowDense),
        (Some(FlowDirection::Column), true) => Ok(GridAutoFlow::ColumnDense),
        (Some(FlowDirection::Row), false) => Ok(GridAutoFlow::Row),
        (Some(FlowDirection::Column), false) => Ok(GridAutoFlow::Column),
        (None, true) => Ok(GridAutoFlow::RowDense),
        (None, _) => Err(parser.new_custom_error(CustomError::Unmatched)),
    }
}
