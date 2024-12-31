use proc_macro2::TokenStream;
use quote::*;
use syn::ext::IdentExt;
use syn::parse::*;
use syn::punctuated::Punctuated;
use syn::*;

mod kw {
    syn::custom_keyword!(deprecated);
}

// property list parsing
#[derive(Clone)]
struct PropertyItem {
    index: u32,
    field_name: Ident,
    ref_field_name: Ident,
    set_field_name: Ident,
    #[cfg(debug_assertions)]
    field_name_type: Ident,
    enum_name: Ident,
    css_display_name: LitStr,
    ty: Path,
    inherit: bool,
    default_value_expr: Expr,
    deprecated: bool,
    resolver: Option<Path>,
}

impl Parse for PropertyItem {
    fn parse(input: ParseStream) -> Result<Self> {
        // parse PropertyName
        let index = input.parse::<LitInt>()?.base10_parse()?;
        let enum_name: Ident = input.parse()?;
        let field_name: String = enum_name
            .to_string()
            .chars()
            .enumerate()
            .map(|(index, c)| {
                let mut s = String::new();
                if c.is_ascii_uppercase() {
                    if index > 0 {
                        s.push('_');
                    }
                    s += &c.to_lowercase().to_string();
                } else {
                    s.push(c);
                }
                s
            })
            .collect();
        let set_field_name = Ident::new(&format!("set_{}", field_name), enum_name.span());
        let ref_field_name = Ident::new(&format!("{}_ref", field_name), enum_name.span());
        #[cfg(debug_assertions)]
        let field_name_type = Ident::new(&format!("{}_type", field_name), enum_name.span());
        let field_name = Ident::new(&field_name, enum_name.span());
        input.parse::<Token![:]>()?;

        // parse type and Initial/Inherit
        let ty = input.parse()?;
        input.parse::<Token![as]>()?;
        let initial_or_inherit: Ident = input.parse()?;
        let inherit = match initial_or_inherit.to_string().as_str() {
            "Initial" => false,
            "Inherit" => true,
            _ => Err(Error::new_spanned(
                initial_or_inherit,
                r#"Initial value should be "Initial" or "Inherit""#,
            ))?,
        };

        // parse deprecated value
        let deprecated = input.parse::<kw::deprecated>().is_ok();

        // parse default value
        input.parse::<Token![default]>()?;
        let default_value_expr = input.parse()?;

        // parse extra options
        let mut resolver = None;
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let key: Ident = Ident::parse_any(input)?;
            input.parse::<Token![=]>()?;
            match key.to_string().as_str() {
                "resolver" => {
                    resolver = Some(input.parse()?);
                }
                _ => Err(Error::new_spanned(key, "Unknown option"))?,
            }
        }

        // compute the CSS display name
        let ori_name_string = enum_name.to_string();
        let mut name_string = String::new();
        if ori_name_string.starts_with("Wx") {
            name_string.push('-');
        }
        for (index, c) in ori_name_string.chars().enumerate() {
            if c.is_ascii_uppercase() {
                if index > 0 {
                    name_string.push('-');
                }
                for c in c.to_lowercase() {
                    name_string.push(c);
                }
            } else {
                name_string.push(c);
            }
        }
        let css_display_name = LitStr::new(&name_string, enum_name.span());

        Ok(Self {
            index,
            field_name,
            ref_field_name,
            set_field_name,
            #[cfg(debug_assertions)]
            field_name_type,
            enum_name,
            css_display_name,
            ty,
            inherit,
            default_value_expr,
            deprecated,
            resolver,
        })
    }
}

#[derive(Clone)]
struct PropertiesDefinition {
    trait_name: Path,
    items: Vec<PropertyItem>,
}

impl Parse for PropertiesDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let trait_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let content;
        braced!(content in input);
        let input = content;
        let items_punct: Punctuated<PropertyItem, Token![;]> =
            input.parse_terminated(PropertyItem::parse)?;
        let mut items: Vec<_> = items_punct.into_iter().collect();
        items.sort_by(|a, b| a.index.cmp(&b.index));
        Ok(Self { trait_name, items })
    }
}

impl ToTokens for PropertiesDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // add trait checker to force every value type matching trait
        let trait_name = &self.trait_name;
        tokens.append_all(quote! {
            /// A trait for property global values, such as `initial` `inherit` `unset`.
            pub trait #trait_name {
                /// The type without global values.
                type Inner;

                /// Create a `initial` value.
                fn initial() -> Self;

                /// Create a `inherit` value.
                fn inherit() -> Self;

                /// Create a `unset` value.
                fn unset() -> Self;

                /// Create a `var` value.
                fn var(expr: String) -> Self;

                /// Create a special `var` value with a shorthand property.
                /// 
                /// When parsing a shorthand property with a `var(...)` value,
                /// the splitted properties should be marked this,
                /// and the `shorthand` is the original shorthand propety name.
                fn var_in_shorthand(shorthand: String, expr: String) -> Self;

                /// Convert to `Self::Inner` type if possible.
                fn to_inner_without_global(&self) -> Option<Self::Inner>;

                /// Convert to `Self::Inner` type with default value and specified inherit mode.
                fn to_inner(&self, parent: Option<&Self::Inner>, default_value: Self::Inner, default_inherit: bool) -> Option<Self::Inner>;
            }
        });

        // add NodeProperties struct
        let property_fields: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem { field_name, ty, .. } = item;
                quote! {
                    #field_name : <#ty as #trait_name>::Inner,
                }
            })
            .collect();
        #[cfg(debug_assertions)]
        let property_fields_type: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    field_name_type,
                    ty,
                    ..
                } = item;
                quote! {
                    #field_name_type : #ty,
                }
            })
            .collect();
        #[cfg(not(debug_assertions))]
        let t = quote! {
            /// All properties for an element.
            #[derive(Clone, Debug)]
            pub struct NodeProperties {
                #(#property_fields)*
            }
        };
        #[cfg(debug_assertions)]
        let t = quote! {
            /// All properties for an element.
            ///
            /// Each supported CSS property can be visited with `PROPERTY_NAME()` `PROPERTY_NAME_ref()` `set_PROPERTY_NAME()` `PROPERTY_NAME_type()`. For example:
            ///
            /// * `font_size()` can be used to get `font-size` (with value cloned);
            /// * `font_size_ref()` can be used to get the reference of `font-size`;
            /// * `set_font_size()` can be used to set the `font-size`;
            /// * `font_size_type()` can be used to get `font-size` with global value unresolved.
            #[derive(Clone, Debug)]
            pub struct NodeProperties {
                #(#property_fields)*
                #(#property_fields_type)*
            }
        };
        tokens.append_all(t);
        // add NodeProperties associate functions
        let init_values: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    field_name,
                    inherit,
                    default_value_expr,
                    ..
                } = item;
                if *inherit {
                    quote! {
                      #field_name: if let Some(parent) = parent {
                          parent.#field_name.clone()
                      } else {
                          #default_value_expr
                      },
                    }
                } else {
                    quote! {
                      #field_name: #default_value_expr,
                    }
                }
            })
            .collect();
        #[cfg(debug_assertions)]
        let init_values_type: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    field_name_type,
                    default_value_expr,
                    ..
                } = item;
                quote! {
                  #field_name_type: #default_value_expr.into(),
                }
            })
            .collect();
        let getters: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    field_name,
                    ref_field_name,
                    set_field_name,
                    #[cfg(debug_assertions)]
                    field_name_type,
                    ty,
                    ..
                } = item;
                let ret = quote!(
                    #[allow(missing_docs)]
                    #[inline]
                    pub fn #field_name(&self) -> <#ty as #trait_name>::Inner {
                        self.#field_name.clone()
                    }
                    #[allow(missing_docs)]
                    #[inline]
                    pub fn #ref_field_name(&self) -> &<#ty as #trait_name>::Inner {
                        &self.#field_name
                    }
                    #[allow(missing_docs)]
                    #[inline]
                    pub fn #set_field_name(&mut self, v: <#ty as #trait_name>::Inner) {
                        self.#field_name = v;
                    }
                );
                #[cfg(debug_assertions)]
                let ret = quote!(
                    #ret
                    #[allow(missing_docs)]
                    #[inline]
                    pub fn #field_name_type(&self) -> #ty {
                        let r = self.#field_name_type.clone();
                        r
                    }
                );
                ret
            })
            .collect();
        let property_mergers: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    field_name,
                    enum_name,
                    ty,
                    inherit,
                    default_value_expr,
                    #[cfg(debug_assertions)]
                    field_name_type,
                    resolver,
                    ..
                } = item;
                let resolver = match resolver {
                    Some(x) => quote!(#x),
                    None => quote!(ResolveFontSize::resolve_font_size),
                };
                #[cfg(not(debug_assertions))]
                let ret = quote!(
                    Property::#enum_name(x) => {
                        if let Some(mut x) = <#ty as #trait_name>::to_inner(x, parent.map(|x| &x.#field_name), #default_value_expr, #inherit) {
                            #resolver(&mut x, current_font_size);
                            if self.#field_name != x {
                                self.#field_name = x;
                                return true;
                            }
                        }
                        false
                    }
                );
                #[cfg(debug_assertions)]
                let ret = quote!(
                    Property::#enum_name(x) => {
                        self.#field_name_type = x.clone();
                        if let Some(mut x) = <#ty as #trait_name>::to_inner(x, parent.map(|x| &x.#field_name), #default_value_expr, #inherit) {
                            #resolver(&mut x, current_font_size);
                            if self.#field_name != x {
                                self.#field_name = x;
                                return true;
                            }
                        }
                        false
                    }
                );
                ret
            })
            .collect();
        let name_value_item: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    field_name,
                    css_display_name,
                    ..
                } = item;
                quote!(
                    {
                        let name = #css_display_name;
                        let value = alloc::string::ToString::to_string(&self.#field_name);
                        (name, value)
                    }
                )
            })
            .collect();
        #[cfg(not(debug_assertions))]
        let ret = quote! {
            impl NodeProperties {
                /// Create a new `NodeProperties` with all property defaults.
                // #[inline]
                pub fn new(parent: Option<&NodeProperties>) -> Self {
                    Self {
                        #(#init_values)*
                    }
                }
                #(#getters)*
                /// Merge a property.
                // #[inline]
                pub fn merge_property(&mut self, p: &Property, parent: Option<&NodeProperties>, current_font_size: f32) -> bool {
                    match p {
                        #(#property_mergers)*
                        _ => false,
                    }
                }
                /// Get all property name-value pairs.
                ///
                /// Caution: it is costy and should only used for debugging.
                pub fn to_property_name_value_list(&self) -> Vec<(&'static str, String)> {
                   vec![
                        #(#name_value_item),*
                   ]
                }
            }
        };
        #[cfg(debug_assertions)]
        let ret = quote! {
            impl NodeProperties {
                /// Create a new `NodeProperties` with all property defaults.
                // #[inline]
                pub fn new(parent: Option<&NodeProperties>) -> Self {
                    Self {
                        #(#init_values)*
                        #(#init_values_type)*
                    }
                }
                #(#getters)*
                /// Merge a property.
                // #[inline]
                pub fn merge_property(&mut self, p: &Property, parent: Option<&NodeProperties>, current_font_size: f32) -> bool {
                    match p {
                        #(#property_mergers)*
                        _ => false,
                    }
                }
                /// Get all property name-value pairs.
                ///
                /// Caution: it is costy and should only used for debugging.
                pub fn to_property_name_value_list(&self) -> Vec<(&'static str, String)> {
                    vec![
                         #(#name_value_item),*
                    ]
                }
            }
        };
        tokens.append_all(ret);

        // add property orderer for rule-merging
        let order_property_fields: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem { field_name, .. } = item;
                quote!( #field_name : OrderPropertyField )
            })
            .collect();
        let order_default_values: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem { field_name, .. } = item;
                quote!( #field_name : OrderPropertyField::new() )
            })
            .collect();
        let order_comparer: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    field_name,
                    enum_name,
                    ..
                } = item;
                quote!(
                    Property::#enum_name(_) => {
                        if self.#field_name.weight() == weight {
                            return true;
                        }
                        if self.#field_name.weight() < weight {
                            self.#field_name.set_weight(weight);
                            return true;
                        }
                        return false;
                    }
                )
            })
            .collect();
        tokens.append_all(quote! {
            #[derive(Clone, Debug)]
            pub(crate) struct NodePropertiesOrder {
                #(#order_property_fields),*
            }
            #[derive(Clone, Debug)]
            pub(crate) struct OrderPropertyField {
                weight: u64,
            }
            impl OrderPropertyField {
                pub(crate) fn new() -> Self {
                    Self {
                        weight: 0,
                    }
                }
                pub(crate) fn weight(&self) -> u64 {
                    self.weight
                }
                pub(crate) fn set_weight(&mut self, weight: u64) {
                    self.weight = weight;
                }
            }
            impl NodePropertiesOrder {
                pub(crate) fn new() -> Self {
                    Self {
                        #(#order_default_values),*
                    }
                }
                pub(crate) fn compare_property(&mut self, p: &Property, weight: u64) -> bool {
                    match p {
                        #(#order_comparer),*
                        _ => false,
                    }
                }
            }
        });

        // add main Property enum definition
        let mut cur_index = 1;
        let mut enum_fields = vec![];
        for item in self.items.iter() {
            let PropertyItem {
                index,
                enum_name,
                ty,
                ..
            } = item;
            while cur_index < *index {
                cur_index += 1;
                let invalid_ident =
                    Ident::new(&format!("Invalid{:X}", cur_index), enum_name.span());
                enum_fields.push(quote!(#[serde(rename = "_")] #invalid_ident));
            }
            cur_index += 1;
            enum_fields.push(quote! {
                #enum_name(#ty)
            });
        }
        tokens.append_all(quote! {
            /// The body of a property.
            /// 
            /// Each variant corresponds to the CSS property with the same name.
            #[allow(missing_docs)]
            #[repr(C)]
            #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
            #[cfg_attr(debug_assertions, derive(float_pigment_css_macro::CompatibilityCheckForEnumVariant))]
            pub enum Property {
                /// A property that is not properly parsed and should be skipped in practice.
                Unknown,
                #(#enum_fields),*
            }
        });

        // add main Property enum associate functions
        let name_fields: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    enum_name,
                    css_display_name,
                    ..
                } = item;
                quote! {
                    Self::#enum_name(..) => #css_display_name
                }
            })
            .collect();
        let value_fields_str: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem { enum_name, .. } = item;
                quote! {
                    Self::#enum_name(v) => alloc::string::ToString::to_string(v)
                }
            })
            .collect();
        let deprecated: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    enum_name,
                    deprecated,
                    ..
                } = item;
                let deprecated = *deprecated;
                quote! {
                    Self::#enum_name(v) => #deprecated
                }
            })
            .collect();
        let value_fields: Vec<_> = self
            .items
            .iter()
            .map(|item| {
                let PropertyItem {
                    enum_name,
                    ty,
                    css_display_name,
                    ..
                } = item;
                let field_getter_name = css_display_name.value().replace('-', "_");
                let field_getter = Ident::new(&field_getter_name, css_display_name.span());
                quote! {
                    pub(crate) fn #field_getter(&self) -> Option<#ty> {
                        if let Self::#enum_name(v) = self.clone() {
                            return Some(v);
                        }
                        None
                    }
                }
            })
            .collect();
        tokens.append_all(quote! {
            impl Property {
                pub(crate) fn get_property_name(&self) -> &'static str {
                    match self {
                        #(#name_fields,)*
                        _ => "-wx-unknown",
                    }
                }
                pub(crate) fn get_property_value_string(&self) -> String {
                    match self {
                        #(#value_fields_str,)*
                        _ => "-wx-unknown".to_string(),
                    }
                }
                pub(crate) fn is_deprecated(&self) -> bool {
                    match self {
                        #(#deprecated,)*
                        _ => false,
                    }
                }
                #(#value_fields)*
            }
        });
    }
}

// style sheet parser struct composer
pub(crate) fn property_list(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let style_syntax = parse_macro_input!(tokens as PropertiesDefinition);

    let ret = quote! {
        #style_syntax
    };
    // panic!(proc_macro::TokenStream::from(ret).to_string());
    proc_macro::TokenStream::from(ret)
}
