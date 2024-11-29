use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use syn::parse::*;
use syn::parse_macro_input;
use syn::*;

const CACHE_ROOT: &str = "compile_cache";
const EXTENSION: &str = "toml";

fn file_creator(
    folder: &str,
    name: &str,
    extension: &str,
    truncate: bool,
) -> std::result::Result<std::fs::File, std::io::Error> {
    let mut path_buffer = PathBuf::new();
    path_buffer.push(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    path_buffer.push(Path::new(CACHE_ROOT));
    path_buffer.push(Path::new(folder));
    path_buffer.push(Path::new(&format!("{}.{}", name, extension)));
    let mut options = OpenOptions::new();
    let file = options
        .read(true)
        .write(true)
        .truncate(truncate)
        .create(true)
        .open(&path_buffer);
    file
}

fn folder_checker() {
    // check folder
    let mut path_buffer = PathBuf::new();
    path_buffer.push(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    path_buffer.push(Path::new(CACHE_ROOT));
    path_buffer.push("struct");
    std::fs::create_dir_all(&path_buffer).unwrap_or_default();
    path_buffer.pop();
    path_buffer.push("enum");
    std::fs::create_dir(&path_buffer).unwrap_or_default();
    path_buffer.pop();
    path_buffer.push("publish");
    path_buffer.push("enum");
    std::fs::create_dir_all(&path_buffer).unwrap_or_default();
    path_buffer.pop();
    path_buffer.push("struct");
    std::fs::create_dir(&path_buffer).unwrap_or_default();
    path_buffer.pop();
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum EnumCheckMode {
    Full,
    Variant,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CacheItem {
    key: String,
    value: Vec<String>,
}

pub(crate) struct ParseInput {
    next_cache: CacheItem,
    name: String,
    token: TokenStream,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum EnumType {
    Unit,
    Named,
    Unnamed,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct EnumItem {
    typing: EnumType,
    ident: String,
    field: Vec<String>,
}
impl ParseInput {
    fn parse_for_enum(input: ParseStream) -> Result<Self> {
        let origin: ItemEnum = input.parse()?;
        let enum_name = origin.ident.to_string();
        let mut enum_variants = vec![];
        let mut check_variants = proc_macro2::TokenStream::new();
        for variant in origin.variants.into_iter() {
            // enum_variants.push(variant.ident.to_string());
            match &variant.fields {
                Fields::Unit => {
                    let item = EnumItem {
                        typing: EnumType::Unit,
                        ident: variant.ident.to_string(),
                        field: Vec::with_capacity(0),
                    };
                    let str = toml::to_string(&item).unwrap();
                    enum_variants.push(str)
                }
                Fields::Named(n) => {
                    let mut field = Vec::with_capacity(n.named.len());
                    for f in n.named.iter() {
                        let ty = f.ty.clone();
                        let ty_string = ty.to_token_stream().to_string();
                        let token: TokenStream = ty_string.parse().unwrap();
                        check_variants.append_all(quote! {
                            <#token as crate::CompatibilityCheck>::check();
                        });
                        field.push(ty_string);
                    }
                    let item = EnumItem {
                        typing: EnumType::Named,
                        ident: variant.ident.to_string(),
                        field,
                    };
                    let str = toml::to_string(&item).unwrap();
                    enum_variants.push(str)
                }
                Fields::Unnamed(un) => {
                    let mut field = Vec::with_capacity(un.unnamed.len());
                    for f in un.unnamed.iter() {
                        let ty = f.ty.clone();
                        let ty_string = ty.to_token_stream().to_string();
                        let token: TokenStream = ty_string.parse().unwrap();
                        check_variants.append_all(quote! {
                            <#token as crate::CompatibilityCheck>::check();
                        });
                        field.push(ty_string);
                    }
                    let item = EnumItem {
                        typing: EnumType::Unnamed,
                        ident: variant.ident.to_string(),
                        field,
                    };
                    let str = toml::to_string(&item).unwrap();
                    enum_variants.push(str)
                }
            }
        }
        let next_cache = CacheItem {
            key: enum_name.clone(),
            value: enum_variants,
        };
        let ident = origin.ident;
        let generics = origin.generics;
        let has_generics = generics.lt_token.is_some();
        let token = if has_generics {
            let generics_token: proc_macro2::TokenStream = generics
                .params
                .clone()
                .into_iter()
                .map(|p| match p {
                    GenericParam::Type(params) => {
                        let ident = params.ident;
                        quote! {
                            #ident: crate::CompatibilityCheck,
                        }
                    }
                    _ => {
                        quote! {}
                    }
                })
                .collect();
            quote! {
                impl #generics crate::CompatibilityCheck for #ident #generics
                    where #generics_token
                {
                    fn check(){
                        #check_variants
                    }
                }
            }
        } else {
            quote! {
                impl crate::CompatibilityCheck for #ident {
                    fn check(){
                        #check_variants
                    }
                }
            }
        };
        // println!("{:?}", token.to_string());
        Ok(Self {
            name: enum_name,
            next_cache,
            token,
        })
    }
    fn parse_for_struct(input: ParseStream) -> Result<Self> {
        let origin: ItemStruct = input.parse()?;
        let struct_name = origin.ident.to_string();
        let ident = origin.ident;
        let mut struct_fields = vec![];
        let generics = origin.generics;
        let mut check_field = proc_macro2::TokenStream::new();
        for field in origin.fields.iter() {
            let field_value = field.ty.to_token_stream().to_string();
            let __field = field_value.clone();
            struct_fields.push(__field.replace(' ', ""));
            let value = field_value;
            let token: TokenStream = value.parse().unwrap();
            check_field.append_all(quote! {
                <#token as crate::CompatibilityCheck>::check();
            });
        }
        let next_cache = CacheItem {
            key: struct_name.clone(),
            value: struct_fields,
        };
        // check trait
        let has_generics = generics.lt_token.is_some();
        let token = if has_generics {
            let generics_token: proc_macro2::TokenStream = generics
                .params
                .clone()
                .into_iter()
                .map(|p| match p {
                    GenericParam::Type(params) => {
                        let ident = params.ident;
                        quote! {
                            #ident: crate::CompatibilityCheck,
                        }
                    }
                    _ => {
                        quote! {}
                    }
                })
                .collect();
            quote! {
                impl #generics crate::CompatibilityCheck for #ident #generics
                    where #generics_token
                    {
                        fn check() {
                            #check_field
                        }
                    }
            }
        } else {
            quote! {
                impl crate::CompatibilityCheck for #ident {
                    fn check() {
                        #check_field
                    }
                }
            }
        };
        Ok(Self {
            name: struct_name,
            next_cache,
            token,
        })
    }
}

pub(crate) fn check_enum_inner(
    input: proc_macro::TokenStream,
    mod_name: Option<String>,
    mode: EnumCheckMode,
) -> proc_macro::TokenStream {
    folder_checker();
    let p = parse_macro_input!(input with ParseInput::parse_for_enum);
    compare_enum_cache(p, mod_name, mode)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

pub(crate) fn compare_enum_cache(
    input: ParseInput,
    mod_name: Option<String>,
    mode: EnumCheckMode,
) -> Result<TokenStream> {
    let ParseInput {
        next_cache,
        name,
        token,
    } = input;
    if mode == EnumCheckMode::Variant {
        return Ok(token);
    }
    let enum_name = match mod_name {
        Some(n) => {
            format!("{}_{}", n, name)
        }
        None => name,
    };
    // open or create cache
    let mut pb = PathBuf::new();
    pb.push("publish");
    pb.push("enum");
    let folder = pb.to_str().unwrap();
    let file = file_creator(folder, &enum_name, EXTENSION, false);
    if let Ok(file) = file {
        let mut reader = BufReader::new(&file);
        let mut string = String::new();
        reader
            .read_to_string(&mut string)
            .expect("error: read to string");
        let prev_cache = toml::from_str::<CacheItem>(&string);
        if let Ok(prev_cache) = prev_cache {
            if next_cache.value.len() < prev_cache.value.len() {
                return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "[CompatibilityEnumCheck: 1000] enum {:?}, cache_variants_len: {:?}, cur_variants_len: {:?}",
                        enum_name,
                        prev_cache.value.len(),
                        next_cache.value.len()
                    )
                ));
            }
            for (idx, (prev, next)) in prev_cache
                .value
                .iter()
                .zip(next_cache.value.iter())
                .enumerate()
            {
                let next_cache_value_item = toml::from_str::<EnumItem>(next).unwrap();
                let prev_cache_value_item = toml::from_str::<EnumItem>(prev).unwrap();
                if next_cache_value_item.typing != prev_cache_value_item.typing {
                    return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "[CompatibilityEnumCheck: 1001] enum {:?}, cache_variants_type: {:?}, cur_variants_type: {:?}, idx: {:?}",
                        enum_name,
                        prev_cache_value_item.typing,
                        next_cache_value_item.typing,
                        idx
                    )
                ));
                }
                let typing = next_cache_value_item.typing;
                match typing {
                    EnumType::Unit => {
                        if next_cache_value_item.ident != prev_cache_value_item.ident {
                            return Err(Error::new(
                            Span::call_site(),
                            format!(
                                "[CompatibilityEnumCheck: 1002] enum {:?}, cache_variants: {:?}, cur_variants: {:?}, idx: {:?}",
                            enum_name, prev_cache_value_item.ident, next_cache_value_item.ident, idx
                            )
                        ));
                        }
                    }
                    EnumType::Named => {
                        if next_cache_value_item.ident != prev_cache_value_item.ident {
                            return Err(Error::new(
                            Span::call_site(),
                            format!(
                                "[CompatibilityEnumCheck: 1003] enum {:?}, cache_named_variants {:?}, cur_named_variants {:?}",
                                enum_name,
                                prev_cache_value_item.ident,
                                next_cache_value_item.ident
                            )
                        ));
                        }
                        if next_cache_value_item.field.len() < prev_cache_value_item.field.len() {
                            return Err(Error::new(
                            Span::call_site(),
                            format!(
                                "[CompatibilityEnumCheck: 1004] enum {:?}, named_variants {:?}, cache_len: {:?}, cur_len: {:?}",
                            enum_name,
                            prev_cache_value_item.ident,
                            prev_cache_value_item.field.len(),
                            next_cache_value_item.field.len()
                            )
                        ));
                        }
                        for (idx, (prev, next)) in prev_cache_value_item
                            .field
                            .iter()
                            .zip(next_cache_value_item.field.iter())
                            .enumerate()
                        {
                            if next != prev {
                                return Err(Error::new(
                                        Span::call_site(),
                                        format!(
                                            "[CompatibilityEnumCheck: 1005] enum {:?}, named_variants {:?}, cache_variants_type: {:?}, cur_variants_type: {:?}, idx: {:?}",
                                            enum_name,
                                            next_cache_value_item.ident,
                                            next,
                                            prev,
                                            idx
                                        )
                                    ));
                            }
                        }
                    }
                    EnumType::Unnamed => {
                        if next_cache_value_item.ident != prev_cache_value_item.ident {
                            return Err(Error::new(
                            Span::call_site(),
                            format!(
                                "[CompatibilityEnumCheck: 1006] enum {:?}, cache_unnamed_variants {:?}, cur_unnamed_variants {:?}",
                                enum_name,
                                prev_cache_value_item.ident,
                                next_cache_value_item.ident
                            )
                        ));
                        }
                        if next_cache_value_item.field.len() != prev_cache_value_item.field.len() {
                            return Err(Error::new(
                            Span::call_site(),
                            format!(
                                "[CompatibilityEnumCheck: 1007] enum {:?}, unnamed_variants {:?}, cache_len: {:?}, cur_len: {:?}",
                                enum_name,
                                prev_cache_value_item.ident,
                                prev_cache_value_item.field.len(),
                                next_cache_value_item.field.len()
                            )
                        ));
                        }
                        for (idx, (prev, next)) in prev_cache_value_item
                            .field
                            .iter()
                            .zip(next_cache_value_item.field.iter())
                            .enumerate()
                        {
                            if next.replace(' ', "") != prev.replace(' ', "") {
                                return Err(Error::new(
                                Span::call_site(),
                                format!(
                                    "[CompatibilityEnumCheck: 1008] enum {:?}, unnamed_variants {:?}, cache_variants_type: {:?}, cur_variants_type: {:?}, idx: {:?}",
                                    enum_name,
                                    next_cache_value_item.ident,
                                    next,
                                    prev,
                                    idx
                                )
                            ));
                            }
                        }
                    }
                }
            }
        }
    }
    let next_cache_toml = toml::to_string(&next_cache).unwrap();
    let mut file = file_creator("enum", &enum_name, EXTENSION, true).unwrap();
    file.write_all(next_cache_toml.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "[CompatibilityEnumCheck] {:?}.{:?}: write cache error",
                enum_name, EXTENSION
            )
        });
    Ok(token)
}

pub(crate) fn check_enum(
    input: proc_macro::TokenStream,
    mode: EnumCheckMode,
) -> proc_macro::TokenStream {
    check_enum_inner(input, None, mode)
}

pub(crate) fn check_enum_with_mod(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mod_name = attr.to_string();
    let t = item.clone();
    let token = check_enum_inner(t, Some(mod_name), EnumCheckMode::Full);
    let token = proc_macro2::TokenStream::from(token);
    let item: DeriveInput = syn::parse(item).unwrap();
    let ret = quote! {
        #item
        #token
    };
    proc_macro::TokenStream::from(ret)
}
pub fn check_struct_inner(
    input: proc_macro::TokenStream,
    mod_name: Option<String>,
) -> proc_macro::TokenStream {
    folder_checker();
    let p = parse_macro_input!(input with ParseInput::parse_for_struct);
    compare_struct_cache(p, mod_name)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

pub(crate) fn compare_struct_cache(
    input: ParseInput,
    mod_name: Option<String>,
) -> Result<TokenStream> {
    let ParseInput {
        next_cache,
        name,
        token,
    } = input;
    let struct_name = match mod_name {
        Some(n) => format!("{}_{}", n, name),
        None => name,
    };
    // open or create cache
    let mut pb = PathBuf::new();
    pb.push("publish");
    pb.push("struct");
    let folder = pb.to_str().unwrap();
    let file = file_creator(folder, &struct_name, EXTENSION, false);
    if let Ok(file) = file {
        let mut reader = BufReader::new(&file);
        let mut string = String::new();
        reader
            .read_to_string(&mut string)
            .expect("error: read to string");
        let prev_cache = toml::from_str::<CacheItem>(&string);
        if let Ok(prev_cache) = prev_cache {
            if next_cache.value.len() < prev_cache.value.len() {
                return Err(Error::new(
                    Span::call_site(),
                    format!(
                        "[CompatibilityStructCheck: 2000] struct {:?}, cache_fields_len: {:?}, cur_fields_len: {:?}",
                        struct_name,
                        prev_cache.value.len(),
                        next_cache.value.len()
                    )
                ));
            }
            for (idx, (prev, next)) in prev_cache
                .value
                .iter()
                .zip(next_cache.value.iter())
                .enumerate()
            {
                if next != prev {
                    return Err(Error::new(
                        Span::call_site(),
                        format!(
                            "[CompatibilityStructCheck: 2001] struct {:?}, cache_fields: {:?}, cur_fields: {:?}, idx: {:?}",
                            struct_name, prev, next, idx
                        )
                    ));
                }
            }
        }
    }
    let next_cache_toml = toml::to_string(&next_cache).unwrap();
    let mut file = file_creator("struct", &struct_name, EXTENSION, true).unwrap();
    file.write_all(next_cache_toml.as_bytes())
        .unwrap_or_else(|_| {
            panic!(
                "[CompatibilityStructCheck] {:?}.{:?}: write cache error",
                struct_name, EXTENSION
            )
        });
    Ok(token)
}

pub(crate) fn check_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    check_struct_inner(input, None)
}

pub(crate) fn check_struct_with_mod(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mod_name = attr.to_string();
    let t = item.clone();
    let token = check_struct_inner(t, Some(mod_name));
    let token = proc_macro2::TokenStream::from(token);
    let item: DeriveInput = syn::parse(item).unwrap();
    let ret = quote! {
        #item
        #token
    };
    proc_macro::TokenStream::from(ret)
}
