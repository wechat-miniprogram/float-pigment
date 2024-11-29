use float_pigment_css::{typing::Length, StyleSheet, StyleSheetGroup};

mod utils;
use utils::*;

#[test]
fn ignore_error_property_segment() {
    let ss = StyleSheet::from_str(
        r#"
        .a {
            height: 200px;
            width: 300px 500px 8000px;
            margin-top: 100px
            margin-left: 400px;
            width: 20xx;
            width: 200px;
            
        }
    "#,
    );
    let mut ssg = StyleSheetGroup::new();
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.height(), Length::Px(200.));
    assert_eq!(node_properties.width(), Length::Px(200.));
    assert_ne!(node_properties.margin_left(), Length::Px(400.));
    assert_ne!(node_properties.margin_top(), Length::Px(100.))
}

#[test]
fn ignore_error_segment() {
    let ss = StyleSheet::from_str(
        r#"
        .a {
            height: 200px;
            width: 300px 500px 8000px;
            margin-top: 100px
            margin-left: 400px;
            width: 20xx;;
            width: 200px;
        }
        .b {{

        }}
        .c { height: 100px }
        .d {
            {(())}asdasdadasdadafffsdd >>> ,, .. ## 
            a@%$@^@%24
            .e { height: 200px }
        }    
        .f { height: 300px }
        .g {
            height: 200px;
            asdasfafafafasdadfasda;
            width: 300px;
        }    
    "#,
    );
    let mut ssg = StyleSheetGroup::new();
    assert_eq!(ss.rules_count(Some(0)).unwrap(), 4u32);
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.height(), Length::Px(200.));
    assert_eq!(node_properties.width(), Length::Px(200.));
    assert_ne!(node_properties.margin_left(), Length::Px(400.));
    assert_ne!(node_properties.margin_top(), Length::Px(100.));

    let node_properties = query(&ssg, "", "", ["c"], []);
    assert_eq!(node_properties.height(), Length::Px(100.));

    let node_properties = query(&ssg, "", "", ["f"], []);
    assert_eq!(node_properties.height(), Length::Px(300.));
}
