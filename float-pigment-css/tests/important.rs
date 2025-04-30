use float_pigment_css::{typing::*, StyleSheet, StyleSheetGroup};

mod utils;
use utils::*;

#[test]
fn important() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
        .a {
            width: 200px;
            height: 100px !important;
        }
        .a {
            height: 200px;
            width: 300px;
        }
        
    "#,
    );
    ssg.append(ss);
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.height(), Length::Px(100.0));
    assert_eq!(np.width(), Length::Px(300.0));
}

#[test]
fn multi_important() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
        #a {
            height: 100px;
        }
        #a {
            height: 200px;
        }
        #a.b {
            height: 300px !important;
            overflow: hidden auto;
        }
        .b {
            height: 200px !important;
        }
        
    "#,
    );
    // println!("{}", ss.serialize_json());
    ssg.append(ss);
    let np = query(&ssg, "", "a", ["b"], []);
    assert_eq!(np.height(), Length::Px(300.0));
}

#[test]
fn error_important() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
        .a {
            height: 200px !important !important;
            height: 100px !important;
        }
    "#,
    );
    // println!("{}", ss.serialize_json());
    ssg.append(ss);
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.height(), Length::Px(100.0));
}

#[test]
fn complex_property_important() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
        .a {
            height: 20px;
            border: 1px solid red !important;
            border: 2px dotted yellow;
            width: 20px;
            margin: 10px !important;
            margin-left: 2px;
            font-family: "hello" !important;
        }
        .a > #b {
            margin-left: 3px !important;
            margin-left: 4px !important;
            font-family: "world!";
        }
        .b {
            font-family: "world"
        }
    "#,
    );
    ssg.append(ss);
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.height(), Length::Px(20.0));
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.border_top_width(), Length::Px(1.0));
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.border_left_style(), BorderStyle::Solid);
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.border_bottom_color(), Color::Specified(255, 0, 0, 255));
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.margin_left(), Length::Px(10.));
    let np = query_list(
        &ssg,
        [
            QueryItem::new().c("a").end(),
            QueryItem::new().id("b").end(),
        ],
    );
    assert_eq!(np.margin_left(), Length::Px(4.));
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(
        np.font_family(),
        FontFamily::Names(vec![FontFamilyName::Title("hello".into())].into())
    );
    let np = query(&ssg, "", "", ["a", "b"], []);
    assert_eq!(
        np.font_family(),
        FontFamily::Names(vec![FontFamilyName::Title("hello".into())].into())
    );
}

#[test]
fn transform_important() {
    test_parse_property!(
        transform,
        "transform",
        "scale(1.0) !important",
        Transform::Series(vec![TransformItem::Scale2D(1.0, 1.0)].into())
    )
}
