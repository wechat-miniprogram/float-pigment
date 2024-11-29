use float_pigment_css::{property::Property, StyleSheet, StyleSheetResource};

mod utils;

#[test]
fn warning_test() {
    let _ = StyleSheet::from_str(
        r#"
          @font-face {
            hello: 100px;
          }
          .a {
            height: 100pp;
            width: 10px 10px;
          }
        "#,
    );
}

#[test]
fn parser_hooks_property() {
    struct Hooks {}

    impl float_pigment_css::parser::hooks::Hooks for Hooks {
        fn parsed_property(
            &mut self,
            _ctx: &mut float_pigment_css::parser::hooks::ParserHooksContext,
            p: &mut float_pigment_css::property::Property,
        ) {
            if let Property::Color(_) = p {
                // empty
            } else {
                panic!("");
            }
        }
    }

    let mut ssr = StyleSheetResource::new();
    let hooks = Hooks {};
    ssr.add_source_with_hooks("a", ".a { color: red }", Some(Box::new(hooks)));
}
