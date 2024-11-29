#![recursion_limit = "128"]
// #![feature(proc_macro_span)]
// #![feature(path_file_prefix)]

mod compatibility_check;
mod property_list;
mod resolve_font_size;
mod style_syntax;
mod value_type;

#[proc_macro]
pub fn property_list(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    property_list::property_list(tokens)
}

#[proc_macro]
pub fn property_value_format(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    style_syntax::property_value_format(tokens)
}

#[proc_macro_attribute]
pub fn property_value_type(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    value_type::property_value_type(attr, item)
}

#[proc_macro_derive(ResolveFontSize, attributes(resolve_font_size))]
pub fn resolve_font_size(attr: proc_macro::TokenStream) -> proc_macro::TokenStream {
    resolve_font_size::derive_resolve_font_size(attr)
}

#[proc_macro_derive(CompatibilityEnumCheck)]
pub fn compatibility_check_enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    compatibility_check::check_enum(input, compatibility_check::EnumCheckMode::Full)
}

#[proc_macro_derive(CompatibilityCheckForEnumVariant)]
pub fn compatibility_check_for_enum_variant_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    compatibility_check::check_enum(input, compatibility_check::EnumCheckMode::Variant)
}

#[proc_macro_derive(CompatibilityStructCheck)]
pub fn compatibility_check_struct_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    compatibility_check::check_struct(input)
}

#[proc_macro_attribute]
pub fn compatibility_enum_check(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    compatibility_check::check_enum_with_mod(attr, item)
}

#[proc_macro_attribute]
pub fn compatibility_struct_check(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    compatibility_check::check_struct_with_mod(attr, item)
}
