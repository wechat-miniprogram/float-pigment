use float_pigment_css::{typing::*, StyleSheet, StyleSheetGroup};

mod utils;
use utils::*;

#[test]
fn initial() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
            .a {
                display: none;
                color: red;
            }
            .b {
                display: initial;
                color: initial;
            }
        ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.display(), Display::None);
    assert_eq!(node_properties.color(), Color::Specified(255, 0, 0, 255));
    let child_node_properties = query_list(
        &ssg,
        [query_item("", "", ["a"], []), query_item("", "", ["b"], [])],
    );
    assert_eq!(child_node_properties.display(), Display::Inline);
    assert_eq!(
        child_node_properties.color(),
        Color::Specified(0, 0, 0, 255)
    );
}

#[test]
fn inherit() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
            .a {
                display: none;
                color: red;
            }
            .b {
                display: inherit;
                color: inherit;
            }
        ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.display(), Display::None);
    assert_eq!(node_properties.color(), Color::Specified(255, 0, 0, 255));
    let child_node_properties = query_list_with_parent(
        &ssg,
        [query_item("", "", ["a"], []), query_item("", "", ["b"], [])],
        &node_properties,
    );
    assert_eq!(child_node_properties.display(), Display::None);
    assert_eq!(
        child_node_properties.color(),
        Color::Specified(255, 0, 0, 255)
    );
}

#[test]
fn unset() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
            .a {
                display: none;
                color: red;
            }
            .b {
                display: unset;
                color: unset;
            }
        ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.display(), Display::None);
    assert_eq!(node_properties.color(), Color::Specified(255, 0, 0, 255));
    let child_node_properties = query_list_with_parent(
        &ssg,
        [query_item("", "", ["a"], []), query_item("", "", ["b"], [])],
        &node_properties,
    );
    assert_eq!(child_node_properties.display(), Display::Inline);
    assert_eq!(
        child_node_properties.color(),
        Color::Specified(255, 0, 0, 255)
    );
}
