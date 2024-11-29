use super::*;

#[inline(never)]
pub(crate) fn filter_repr<'a, 't: 'a, 'i: 't>(
    parser: &'a mut Parser<'i, 't>,
    properties: &mut Vec<PropertyMeta>,
    st: &mut ParseState,
) -> Result<Vec<FilterFunc>, ParseError<'i, CustomError>> {
    parser.try_parse(|parser| {
        parser.parse_until_before(Delimiter::Semicolon, |parser| {
            let mut ret = vec![];
            while !parser.is_exhausted() {
                let _r = parser.try_parse(|parser| {
                    let r = url_str(parser, properties, st);
                    if let Ok(s) = r {
                        ret.push(FilterFunc::Url(s.into()));
                        return Ok(());
                    }
                    Err(CustomError::Unmatched)
                });
                if parser.is_exhausted() {
                    return Ok(ret);
                }
                let next: &str = &parser.expect_function()?.clone();
                let next: &str = &next.to_lowercase();
                match next {
                    "hue-rotate" => parser.parse_nested_block(|parser| {
                        if parser.is_exhausted() {
                            ret.push(FilterFunc::HueRotate(Angle::Deg(0.)));
                            return Ok(());
                        }
                        let ang = angle(parser, properties, st)?;
                        ret.push(FilterFunc::HueRotate(ang));
                        Ok(())
                    }),
                    "blur" => parser.parse_nested_block(|parser| {
                        if parser.is_exhausted() {
                            ret.push(FilterFunc::Blur(Length::Px(0.)));
                            return Ok(());
                        }
                        let len = length_without_percentage(parser, properties, st)?;
                        ret.push(FilterFunc::Blur(len));
                        Ok(())
                    }),
                    _ => parser.parse_nested_block(|parser| {
                        let mut use_default = false;
                        let mut val = 0.;
                        if parser.is_exhausted() {
                            use_default = true;
                        } else {
                            val = percentage_to_f32(parser, properties, st)?;
                        }
                        let r = match next {
                            "invert" => {
                                let len = Length::Ratio(val);
                                FilterFunc::Invert(len)
                            } // default 0
                            "opacity" => {
                                if use_default {
                                    val = 1.;
                                }
                                let len = Length::Ratio(val);
                                FilterFunc::Opacity(len)
                            } // default 1
                            "brightness" => {
                                if use_default {
                                    val = 1.;
                                }
                                let len = Length::Ratio(val);
                                FilterFunc::Brightness(len)
                            } // default 1
                            "contrast" => {
                                if use_default {
                                    val = 1.;
                                }
                                let len = Length::Ratio(val);
                                FilterFunc::Contrast(len)
                            } // default 1
                            "grayscale" => {
                                let len = Length::Ratio(val);
                                FilterFunc::Grayscale(len)
                            } // default 0
                            "sepia" => {
                                let len = Length::Ratio(val);
                                FilterFunc::Sepia(len)
                            } // default 0
                            "saturate" => {
                                if use_default {
                                    val = 1.;
                                }
                                let len = Length::Ratio(val);
                                FilterFunc::Saturate(len)
                            } // default 1
                            _ => return Err(parser.new_custom_error(CustomError::Unmatched)),
                        };
                        ret.push(r);
                        Ok(())
                    }),
                }?;
            }
            Ok(ret)
        })
    })
}
