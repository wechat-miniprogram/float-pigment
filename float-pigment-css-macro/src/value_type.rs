use proc_macro2::TokenStream;
use punctuated::Punctuated;
use quote::*;
use syn::parse::*;
use syn::*;

const PRESERVE_GLOBAL_VALUE_RANGE: usize = 64;

struct DeriveList {
    content: Punctuated<Path, Token![,]>,
}

impl Parse for DeriveList {
    fn parse(input: ParseStream) -> Result<Self> {
        let content = Punctuated::parse_terminated(input)?;
        Ok(DeriveList { content })
    }
}

struct PropertyValueType {
    original_type: ItemEnum,
    extra_type: ItemEnum,
    trait_name: Option<Path>,
}

impl Parse for PropertyValueType {
    fn parse(input: ParseStream) -> Result<Self> {
        let original_type: ItemEnum = input.parse()?;
        let mut extra_type = original_type.clone();

        // prepend global values
        let new_variants = {
            let s = r#"
                #[allow(missing_docs)]
                pub enum T {
                    Invalid,
                    Initial,
                    Inherit,
                    Unset,
                    Var(Box<StrRef>),
                    VarInShorthand(Box<StrRef>, Box<StrRef>),
                    Invalid0,
                }
            "#;
            let mut new_variants = parse_str::<ItemEnum>(s)?.variants;
            for i in new_variants.len()..PRESERVE_GLOBAL_VALUE_RANGE {
                let mut empty_slot = new_variants.last().unwrap().clone();
                empty_slot.ident = Ident::new(&format!("Invalid{:X}", i), empty_slot.ident.span());
                new_variants.push(empty_slot);
            }
            new_variants
        };
        for (i, v) in new_variants.into_iter().enumerate() {
            extra_type.variants.insert(i, v);
        }

        // remove serde derives
        for attr in extra_type.attrs.iter_mut() {
            if attr.path.is_ident("derive") {
                let args: DeriveList = attr.parse_args()?;
                let items = args
                    .content
                    .into_iter()
                    .filter(|arg| !arg.is_ident("Deserialize"));
                attr.tokens = quote! {
                    (#(#items),*)
                };
            }
        }

        Ok(Self {
            original_type,
            extra_type,
            trait_name: None,
        })
    }
}

impl PropertyValueType {
    fn set_trait_name(&mut self, attr: AttrFormat) {
        self.trait_name = Some(attr.trait_path);
        self.extra_type.ident = attr.ident;
    }
}

impl ToTokens for PropertyValueType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            original_type,
            extra_type,
            trait_name,
        } = self;
        let trait_name = trait_name.as_ref().unwrap();
        let original_type_name = &original_type.ident;
        let extra_type_name = &extra_type.ident;

        let impl_from_variants = original_type.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Named(x) => {
                    let names = x.named.iter().map(|x| x.ident.as_ref().unwrap()).collect::<Vec<_>>();
                    quote! {
                        #original_type_name::#variant_name { #(#names),* } => #extra_type_name::#variant_name { #(#names),* },
                    }
                }
                Fields::Unnamed(x) => {
                    let names = x.unnamed.iter().enumerate().map(|(index, _)| Ident::new(&format!("_{}", index), variant_name.span())).collect::<Vec<_>>();
                    quote! {
                        #original_type_name::#variant_name(#(#names),*) => #extra_type_name::#variant_name(#(#names),*),
                    }
                }
                Fields::Unit => {
                    quote! {
                        #original_type_name::#variant_name => #extra_type_name::#variant_name,
                    }
                }
            }
        }).collect::<Vec<_>>();

        let impl_to_inner_variants = original_type.variants.iter().map(|variant| {
            let variant_name = &variant.ident;
            match &variant.fields {
                Fields::Named(x) => {
                    let names = x.named.iter().map(|x| x.ident.as_ref().unwrap()).collect::<Vec<_>>();
                    quote! {
                        #extra_type_name::#variant_name { #(#names),* } => #original_type_name::#variant_name { #(#names),* },
                    }
                }
                Fields::Unnamed(x) => {
                    let names = x.unnamed.iter().enumerate().map(|(index, _)| Ident::new(&format!("_{}", index), variant_name.span())).collect::<Vec<_>>();
                    quote! {
                        #extra_type_name::#variant_name(#(#names),*) => #original_type_name::#variant_name(#(#names),*),
                    }
                }
                Fields::Unit => {
                    quote! {
                        #extra_type_name::#variant_name => #original_type_name::#variant_name,
                    }
                }
            }
        }).collect::<Vec<_>>();

        tokens.append_all(quote! {
            #original_type
            #extra_type

            impl Default for #extra_type_name {
                fn default() -> Self {
                    Self::Invalid
                }
            }

            impl From<#original_type_name> for #extra_type_name {
                fn from(x: #original_type_name) -> #extra_type_name {
                    match x {
                        #(#impl_from_variants)*
                    }
                }
            }

            impl #trait_name for #extra_type_name {
                type Inner = #original_type_name;
                #[inline]
                fn initial() -> Self {
                    #extra_type_name::Initial
                }
                #[inline]
                fn inherit() -> Self {
                    #extra_type_name::Inherit
                }
                #[inline]
                fn unset() -> Self {
                    #extra_type_name::Unset
                }
                #[inline]
                fn var(expr: String) -> Self {
                    #extra_type_name::Var(Box::new(expr.into()))
                }
                #[inline]
                fn var_in_shorthand(short_hand: String, expr: String) -> Self {
                    #extra_type_name::VarInShorthand(Box::new(short_hand.into()), Box::new(expr.into()))
                }
                #[inline]
                fn to_inner_without_global(&self) -> Option<Self::Inner> {
                    let ret = match self.clone() {
                        #(#impl_to_inner_variants)*
                        _ => None?,
                    };
                    Some(ret)
                }
                #[inline]
                fn to_inner(&self, parent: Option<&Self::Inner>, default_value: Self::Inner, default_inherit: bool) -> Option<Self::Inner> {
                    match self {
                        #extra_type_name::Invalid => None?,
                        #extra_type_name::Initial => Some(default_value),
                        #extra_type_name::Inherit => Some(parent.cloned().unwrap_or_else(|| default_value)),
                        #extra_type_name::Unset => if !default_inherit {
                            Some(default_value)
                        } else {
                            Some(parent.cloned().unwrap_or_else(|| default_value))
                        },
                        // #extra_type_name::Var(_) => None,
                        // #extra_type_name::VarWithDeclarationValue(_, _) => None,
                        _ => #trait_name::to_inner_without_global(self),
                    }
                }
            }

            impl core::fmt::Display for #extra_type_name {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "{}", match self {
                        #extra_type_name::Invalid => "invalid".to_string(),
                        #extra_type_name::Initial => "initial".to_string(),
                        #extra_type_name::Inherit => "inherit".to_string(),
                        #extra_type_name::Unset => "unset".to_string(),
                        #extra_type_name::Var(expr) => format!("{}", expr.to_string()),
                        #extra_type_name::VarInShorthand(_short_hand, expr) => format!("{}", expr.to_string()),
                        _ => self.to_inner_without_global().map(|x| alloc::string::ToString::to_string(&x)).unwrap_or_else(|| "invalid".to_string()),
                    })
                }
            }
        });

        let deser_name = extra_type_name.to_string();
        const DESER_VARIANTS_GLOBAL_STR: [&str; 6] = [
            "Invalid",
            "Initial",
            "Inherit",
            "Unset",
            "Var",
            "VarInShorthand",
        ];
        let deser_variants_global_str = DESER_VARIANTS_GLOBAL_STR.iter();
        let deser_variants_placeholder =
            ["_"; PRESERVE_GLOBAL_VALUE_RANGE - DESER_VARIANTS_GLOBAL_STR.len()].iter();
        let deser_variants_str = original_type.variants.iter().map(|v| v.ident.to_string());
        let deser_variants = original_type.variants.iter().enumerate().map(|(index, v)| {
            let i = index + PRESERVE_GLOBAL_VALUE_RANGE;
            let ident = &v.ident;
            match &v.fields {
                syn::Fields::Unit => quote! {
                    (#i, variant) => {
                        variant.unit_variant()?;
                        Ok(#extra_type_name::#ident)
                    }
                },
                syn::Fields::Unnamed(list) => {
                    let list_quote = list.unnamed.iter().map(|_| quote! {
                        seq.next_element()?.unwrap_or_default()
                    });
                    let count = list.unnamed.len();
                    quote! {
                        (#i, variant) => {
                            struct VariantVisitor;
                            impl<'de> serde::de::Visitor<'de> for VariantVisitor {
                                type Value = #extra_type_name;
                                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                                    formatter.write_str("enum [PropertyValueType] [variant]")
                                }
                                #[inline]
                                fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error> {
                                    Ok(#extra_type_name::#ident(#(#list_quote),*))
                                }
                            }
                            variant.tuple_variant(#count, VariantVisitor)
                        }
                    }
                }
                syn::Fields::Named(list) => {
                    let list_quote = list.named.iter().map(|field| {
                        let name = &field.ident;
                        quote! {
                            #name: seq.next_element()?.unwrap_or_default()
                        }
                    });
                    let list_ident_index_list = list.named.iter().enumerate().map(|(i, _)| i);
                    let list_ident_list = list.named.iter().map(|field| field.ident.as_ref().unwrap());
                    let list_ident_list2 = list.named.iter().map(|field| field.ident.as_ref().unwrap());
                    let list_ty_list = list.named.iter().map(|field| &field.ty);
                    let list_name_bytes_list = list.named.iter().map(|field| {
                        let name = field.ident.as_ref().unwrap();
                        syn::LitByteStr::new(name.to_string().as_bytes(), name.span())
                    });
                    quote! {
                        (#i, variant) => {
                            struct VariantVisitor;
                            impl<'de> serde::de::Visitor<'de> for VariantVisitor {
                                type Value = #extra_type_name;
                                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                                    formatter.write_str("enum [PropertyValueType] [variant]")
                                }
                                #[inline]
                                fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error> {
                                    Ok(#extra_type_name::#ident {
                                        #(#list_quote),*
                                    })
                                }
                                #[inline]
                                fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> core::result::Result<Self::Value, A::Error> {
                                    struct NamedVisitor;
                                    impl<'de> serde::de::Visitor<'de> for VariantVisitor {
                                        type Value = u64;
                                        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                                            formatter.write_str("enum [PropertyValueType] [variant] [field]")
                                        }
                                        #[inline]
                                        fn visit_u64<E: serde::de::Error>(self, v: u64) -> core::result::Result<Self::Value, E> {
                                            Ok(v)
                                        }
                                        #[inline]
                                        fn visit_str<E: serde::de::Error>(self, v: &str) -> core::result::Result<Self::Value, E> {
                                            self.visit_bytes(v.as_bytes())
                                        }
                                        #[inline]
                                        fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> core::result::Result<Self::Value, E> {
                                            let i = [#(#list_name_bytes_list),*].into_iter().position(|a| *a == v).unwrap_or(0);
                                            Ok(i as u64)
                                        }
                                    }
                                    let mut ret = #extra_type_name::#ident {
                                        #(#list_ident_list: Default::default()),*
                                    };
                                    while let Some(key) = map.next_key::<NamedVisitor>()? {
                                        match key {
                                            #(#list_ident_index_list => { ret.#list_ident_list2 = map.next_value::<#list_ty_list>(); })*
                                            _ => {}
                                        }
                                    }
                                    Ok(ret)
                                }
                            }
                            variant.tuple_variant(1, VariantVisitor)
                        }
                    }
                }
            }
        });
        tokens.append_all(quote! {
            impl<'de> serde::de::Deserialize<'de> for #extra_type_name {
                fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    struct ExtraVisitor;

                    impl<'de> serde::de::Visitor<'de> for ExtraVisitor {
                        type Value = #extra_type_name;

                        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                            formatter.write_str("enum [PropertyValueType]")
                        }

                        #[inline]
                        fn visit_enum<A: serde::de::EnumAccess<'de>>(self, data: A) -> core::result::Result<Self::Value, A::Error> {
                            use serde::de::VariantAccess;

                            struct ExtraFieldVisitor;

                            impl<'de> serde::de::Visitor<'de> for ExtraFieldVisitor {
                                type Value = u64;

                                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                                    formatter.write_str("variant identifier")
                                }

                                #[inline]
                                fn visit_u64<E: serde::de::Error>(self, v: u64) -> core::result::Result<Self::Value, E> {
                                    Ok(v)
                                }

                                #[inline]
                                fn visit_str<E: serde::de::Error>(self, v: &str) -> core::result::Result<Self::Value, E> {
                                    let i = VARIANTS.into_iter().position(|a| *a == v).unwrap_or(0);
                                    Ok(i as u64)
                                }

                                #[inline]
                                fn visit_bytes<E: serde::de::Error>(self, v: &[u8]) -> core::result::Result<Self::Value, E> {
                                    let i = VARIANTS.into_iter().position(|a| a.as_bytes() == v).unwrap_or(0);
                                    Ok(i as u64)
                                }
                            }

                            match data.variant()? {
                                (1, variant) => {
                                    variant.unit_variant()?;
                                    Ok(#extra_type_name::Initial)
                                }
                                (2, variant) => {
                                    variant.unit_variant()?;
                                    Ok(#extra_type_name::Inherit)
                                }
                                (3, variant) => {
                                    variant.unit_variant()?;
                                    Ok(#extra_type_name::Unset)
                                }
                                (4, variant) => {
                                    struct VariantVisitor;
                                    impl<'de> serde::de::Visitor<'de> for VariantVisitor {
                                        type Value = #extra_type_name;
                                        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                                            formatter.write_str("enum [PropertyValueType] [variant]")
                                        }
                                        #[inline]
                                        fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error> {
                                            Ok(#extra_type_name::Var({
                                                seq.next_element()?.unwrap_or_default()
                                            }))
                                        }
                                    }
                                    variant.tuple_variant(1, VariantVisitor)
                                }
                                (5, variant) => {
                                    struct VariantVisitor;
                                    impl<'de> serde::de::Visitor<'de> for VariantVisitor {
                                        type Value = #extra_type_name;
                                        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                                            formatter.write_str("enum [PropertyValueType] [variant]")
                                        }
                                        #[inline]
                                        fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error> {
                                            Ok(#extra_type_name::VarInShorthand({
                                                seq.next_element()?.unwrap_or_default()
                                            }, {
                                                seq.next_element()?.unwrap_or_default()
                                            }))
                                        }
                                    }
                                    variant.tuple_variant(1, VariantVisitor)
                                }
                                #(#deser_variants)*
                                (_, variant) => {
                                    variant.unit_variant()?;
                                    Ok(#extra_type_name::Invalid)
                                }
                            }
                        }
                    }

                    const VARIANTS: &'static [&'static str] = &[
                        #(#deser_variants_global_str,)*
                        #(#deser_variants_placeholder,)*
                        #(#deser_variants_str,)*
                    ];
                    deserializer.deserialize_enum(#deser_name, VARIANTS, ExtraVisitor)
                }
            }
        });
    }
}

struct AttrFormat {
    trait_path: Path,
    ident: Ident,
}

impl Parse for AttrFormat {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_path = input.parse()?;
        input.parse::<Token![for]>()?;
        let ident = input.parse()?;
        Ok(Self { trait_path, ident })
    }
}

pub fn property_value_type(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut struct_syntax = parse_macro_input!(item as PropertyValueType);
    struct_syntax.set_trait_name(parse_macro_input!(attr as AttrFormat));

    let ret = quote! {
        #struct_syntax
    };
    proc_macro::TokenStream::from(ret)
}
