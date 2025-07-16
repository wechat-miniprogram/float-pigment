use proc_macro2::{Ident, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{parse::ParseStream, parse_macro_input, ItemStruct, Result, Type};

pub(crate) struct ParseInput {
    token: TokenStream,
}

impl ParseInput {
    fn gen_getter(input: ParseStream) -> Result<Self> {
        let origin: ItemStruct = input.parse()?;
        let mut props_getter = proc_macro2::TokenStream::new();
        origin.fields.into_iter().for_each(|field| {
            if let Some(prop_name) = field.ident {
                if let Type::Path(tp) = field.ty {
                    if let Some(segment) = tp.path.segments.last() {
                        props_getter.append_all(quote! {
                            #[allow(unused)]
                            pub(crate) fn #prop_name(&self) -> #segment {
                                self.style.#prop_name.clone()
                            }
                        });
                    }
                }
            }
        });
        let getter = quote! {
            impl StyleManager {
                #props_getter
            }
        };
        Ok(ParseInput { token: getter })
    }

    fn gen_setter(input: ParseStream) -> Result<Self> {
        let origin: ItemStruct = input.parse()?;
        let mut props_setter = proc_macro2::TokenStream::new();
        origin
            .fields
            .into_iter()
            .enumerate()
            .for_each(|(idx, field)| {
                if let Some(prop_name) = field.ident {
                    if let Type::Path(tp) = field.ty {
                        if let Some(segment) = tp.path.segments.last() {
                            let func = Ident::new(&format!("set_{prop_name}"), prop_name.span());
                            props_setter.append_all(quote! {
                                #[allow(unused)]
                                pub(crate) fn #func(&mut self, value: #segment) -> bool {
                                    if value != self.style.#prop_name {
                                        self.style.#prop_name = value;
                                        self.mutation.set(#idx, true);
                                        return true;
                                    }
                                    false
                                }
                            });
                        }
                    }
                }
            });
        let setter = quote! {
            impl StyleManager {
                #props_setter
            }
        };
        Ok(ParseInput { token: setter })
    }

    fn gen_mutation(input: ParseStream) -> Result<Self> {
        let origin: ItemStruct = input.parse()?;
        let mut idx_matcher = proc_macro2::TokenStream::new();
        origin
            .fields
            .into_iter()
            .enumerate()
            .for_each(|(idx, field)| {
                if let Some(prop_name) = field.ident {
                    let prop_name_str = prop_name.to_string().replace('_', "-");
                    if let Type::Path(tp) = field.ty {
                        if let Some(segment) = tp.path.segments.last() {
                            if segment.ident.to_string().contains("Length") {
                                idx_matcher.append_all(quote! {
                                    #idx => format!("{}:{}", #prop_name_str, crate::def_length_to_string(&self.style.#prop_name)),
                                });
                            } else  {
                                idx_matcher.append_all(quote! {
                                    #idx => format!("{}:{:?}", #prop_name_str, self.style.#prop_name),
                                });
                            }
                        }
                    }
                }
            });
        let mutation = quote! {
            impl StyleManager {
                pub(crate) fn mutation_to_string(&self) -> String {
                    let mut mutations = vec![];
                    self.mutation.iter().enumerate().for_each(|(idx, changed)| {
                        if changed {
                            let s = match idx {
                                #idx_matcher
                                _ => format!("Unsupported Property"),
                            };
                            mutations.push(format!("{}", s));
                        }
                    });
                    mutations.join(";")
                }
            }
        };
        Ok(ParseInput { token: mutation })
    }
}

pub(crate) fn gen_getter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p = parse_macro_input!(input with ParseInput::gen_getter);
    proc_macro::TokenStream::from(p.token)
}

pub(crate) fn gen_setter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p = parse_macro_input!(input with ParseInput::gen_setter);
    proc_macro::TokenStream::from(p.token)
}

pub(crate) fn gen_mutation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let p = parse_macro_input!(input with ParseInput::gen_mutation);
    proc_macro::TokenStream::from(p.token)
}
