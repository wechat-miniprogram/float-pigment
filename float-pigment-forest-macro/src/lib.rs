use quote::quote;
use syn::{parse_macro_input, ItemStruct};

mod style_manager;

#[proc_macro_derive(StyleManagerGetter)]
pub fn style_manager_getter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    style_manager::gen_getter(input)
}

#[proc_macro_derive(StyleManagerSetter)]
pub fn style_manager_setter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    style_manager::gen_setter(input)
}

#[proc_macro_derive(StyleManagerMutation)]
pub fn style_manager_mutation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    style_manager::gen_mutation(input)
}

#[proc_macro_derive(FieldCount)]
pub fn derive_field_count(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;
    let field_count = input.fields.iter().count();

    let output = quote! {
        impl #name  {
            pub(crate) fn field_count(&self) -> usize {
                #field_count
            }
        }
    };

    proc_macro::TokenStream::from(output)
}
