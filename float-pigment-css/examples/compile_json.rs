fn main() {
    // let style_text = r#"
    //     .my-class {
    //         color: #abc;
    //     }
    //     @media (max-width: 800px) {
    //         .my-class {
    //             color: #def;
    //         }
    //     }
    //     @media only screen and (max-width: 900px), not screen and (min-width: 400px) {
    //       .my-class {
    //         color: 123
    //       }
    //     }
    // "#;
    let _st = r#"
    #a {
      height: 100px;
    }
    .b {
      height: 200px !important;
      width: 400px;
    }
    .b {
      height: 300px;
      overflow: hidden auto;
      transition-property: overflow, all, width;
    }
    "#;
    // #[cfg(debug_assertions)]
    // println!(
    //     "{}",
    //     float_pigment_css::compile_style_sheet_to_json("text", st)
    // );
}
