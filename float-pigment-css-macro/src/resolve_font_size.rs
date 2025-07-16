use proc_macro2::TokenStream;
use quote::*;
use syn::parse::*;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

fn find_generic_def_ref(generics: &Generics) -> (TokenStream, TokenStream, &Option<WhereClause>) {
    let Generics {
        params,
        where_clause,
        ..
    } = generics;
    let params_def = params.iter();
    let params_ref = params.iter().map(|x| match x {
        GenericParam::Lifetime(x) => quote!(#x),
        GenericParam::Type(x) => {
            let x = &x.ident;
            quote!(#x)
        }
        GenericParam::Const(x) => {
            let x = &x.ident;
            quote!(#x)
        }
    });
    let gen_def = quote!(<#(#params_def),*>);
    let gen_ref = quote!(<#(#params_ref),*>);
    (gen_def, gen_ref, where_clause)
}

fn find_resolve_fn(attrs: &[Attribute]) -> Option<TokenStream> {
    for attr in attrs.iter() {
        if attr.path.is_ident("resolve_font_size") {
            struct ParenPath(Path);
            impl Parse for ParenPath {
                fn parse(input: ParseStream) -> Result<Self> {
                    let content;
                    parenthesized!(content in input);
                    Ok(Self(content.parse()?))
                }
            }
            return Some(match parse2::<ParenPath>(attr.tokens.clone()) {
                Ok(x) => {
                    let x = x.0;
                    quote!(#x)
                }
                Err(err) => err.to_compile_error(),
            });
        }
    }
    None
}

fn find_resolve_fn_list(fields: &Punctuated<Field, syn::token::Comma>) -> Vec<Option<TokenStream>> {
    fields
        .iter()
        .map(|field| find_resolve_fn(&field.attrs))
        .collect()
}

enum DeriveResolveFontSize {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

impl Parse for DeriveResolveFontSize {
    fn parse(input: ParseStream) -> Result<Self> {
        let x: Item = input.parse()?;
        let ret = match x {
            Item::Struct(x) => Self::Struct(x),
            Item::Enum(x) => Self::Enum(x),
            _ => Err(Error::new_spanned(x, "expected struct or enum"))?,
        };
        Ok(ret)
    }
}

impl ToTokens for DeriveResolveFontSize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(x) => {
                let ItemStruct {
                    ident,
                    generics,
                    fields,
                    ..
                } = x;
                let (gen_def, gen_ref, where_clause) = find_generic_def_ref(generics);
                let body = match fields {
                    Fields::Named(x) => {
                        let names = x.named.iter().map(|x| &x.ident);
                        let resolve_fn = find_resolve_fn_list(&x.named)
                            .into_iter()
                            .map(|x| x.unwrap_or(quote!(ResolveFontSize::resolve_font_size)));
                        quote! {
                            #(#resolve_fn(&mut self.#names, font_size);)*
                        }
                    }
                    Fields::Unnamed(x) => {
                        let names = x.unnamed.iter().enumerate().map(|(i, _)| Index::from(i));
                        let resolve_fn = find_resolve_fn_list(&x.unnamed)
                            .into_iter()
                            .map(|x| x.unwrap_or(quote!(ResolveFontSize::resolve_font_size)));
                        quote! {
                            #(#resolve_fn(&mut self.#names, font_size);)*
                        }
                    }
                    Fields::Unit => quote!(),
                };
                tokens.append_all(quote! {
                    impl #gen_def ResolveFontSize for #ident #gen_ref #where_clause {
                        fn resolve_font_size(&mut self, font_size: f32) {
                            #body
                        }
                    }
                });
            }
            Self::Enum(x) => {
                let ItemEnum {
                    ident,
                    generics,
                    variants,
                    ..
                } = x;
                let (gen_def, gen_ref, where_clause) = find_generic_def_ref(generics);
                let body = variants.iter().map(|x| {
                    let Variant {
                        attrs,
                        ident,
                        fields,
                        ..
                    } = x;
                    let resolve_fn = find_resolve_fn(attrs)
                        .unwrap_or(quote!(ResolveFontSize::resolve_font_size));
                    match fields {
                        Fields::Named(x) => {
                            let names: Vec<_> = x.named.iter().map(|x| &x.ident).collect();
                            let resolve_fn = find_resolve_fn_list(&x.named)
                                .into_iter()
                                .map(|x| x.unwrap_or_else(|| resolve_fn.clone()));
                            quote! {
                                Self::#ident { #(ref mut #names),* } => {
                                    #(#resolve_fn(#names, font_size);)*
                                }
                            }
                        }
                        Fields::Unnamed(x) => {
                            let names: Vec<_> = x
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(i, x)| Ident::new(&format!("a{i}"), x.ty.span()))
                                .collect();
                            let resolve_fn = find_resolve_fn_list(&x.unnamed)
                                .into_iter()
                                .map(|x| x.unwrap_or_else(|| resolve_fn.clone()));
                            quote! {
                                Self::#ident(#(ref mut #names),*) => {
                                    #(#resolve_fn(#names, font_size);)*
                                }
                            }
                        }
                        Fields::Unit => quote! {
                            Self::#ident => {}
                        },
                    }
                });
                tokens.append_all(quote! {
                    impl #gen_def ResolveFontSize for #ident #gen_ref #where_clause {
                        fn resolve_font_size(&mut self, font_size: f32) {
                            match self {
                                #(#body)*
                            }
                        }
                    }
                });
            }
        }
    }
}

// style sheet parser struct composer
pub(crate) fn derive_resolve_font_size(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let style_syntax = parse_macro_input!(tokens as DeriveResolveFontSize);

    let ret = quote! {
        #style_syntax
    };
    // panic!(proc_macro::TokenStream::from(ret).to_string());
    proc_macro::TokenStream::from(ret)
}
