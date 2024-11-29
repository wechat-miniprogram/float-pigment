use float_pigment_css::{StyleSheet, StyleSheetGroup};

#[test]
fn test_keyframes() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
        @keyframes hello {
          0% {
            top: 0;
          }
          50% {
            top: 30px;
            left: 20px;
          }
          50% {
            top: 10px;
          }
          100% {
            top: 0;
          }
        }
        @keyframes box-ani_2 {
            120% {
                margin-left: 0px;
            }
            80% {
                margin-left: 200px;
            }
        }
        @keyframes mymove_3 {
          0% {
          }
          to {
            margin-left: 200px;
          }
        }
        @keyframes mymove_4 {
            20% {
              margin-left: 20px;
            }
            80% {
              margin-left: 200px;
            }
        }
      "#,
    );
    println!("{:?}", &ss);
    ssg.append(ss);
}
