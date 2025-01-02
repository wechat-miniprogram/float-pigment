use float_pigment_css::{
    property::*, query::MatchedRuleList, typing::*, MediaQueryStatus, StyleQuery, StyleSheet,
    StyleSheetGroup,
};

mod utils;
use utils::*;

#[test]
fn stringify() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
            .a, .b { opacity: 0.5; }
            .a { opacity: 0.5; }
            .a.a2 { opacity: 0.5; }
            #d.a { opacity: 0.5; }
            c.a { opacity: 0.5; }
            c#d { opacity: 0.5; }
            c { opacity: 0.5; }
            #d { opacity: 0.5; }
        "#,
    );
    ssg.append(ss);
    let media_query_status = MediaQueryStatus::<f32>::default_screen();
    fn merge_rule_strings(mut matched_rules: MatchedRuleList) -> String {
        matched_rules.rules.sort();
        let strings = matched_rules
            .rules
            .into_iter()
            .map(|matched_rule| matched_rule.rule.get_selector_string())
            .collect::<Box<[String]>>();
        strings.join("|")
    }
    let classes = vec![("a".into(), None), ("a2".into(), None)];
    let query = StyleQuery::single(None, None, None, "", "", &classes);
    let matched_rules = ssg.query_matched_rules(&[query], &media_query_status);
    assert_eq!(merge_rule_strings(matched_rules), ".a, .b|.a|.a.a2");
    let classes = vec![("a".into(), None)];
    let query = StyleQuery::single(None, None, None, "", "d", &classes);
    let matched_rules = ssg.query_matched_rules(&[query], &media_query_status);
    assert_eq!(merge_rule_strings(matched_rules), ".a, .b|.a|#d|#d.a");
    let query = StyleQuery::single(None, None, None, "c", "d", &classes);
    let matched_rules = ssg.query_matched_rules(&[query], &media_query_status);
    assert_eq!(
        merge_rule_strings(matched_rules),
        "c|.a, .b|.a|c.a|#d|c#d|#d.a"
    );
}

#[test]
fn rule_stringify() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
            .a { display: block; }
            @media all {
                .b { display: block; }
            }
        "#,
    );
    ssg.append(ss);
    let media_query_status = MediaQueryStatus::<f32>::default_screen();
    let classes = vec![("a".into(), None)];
    let query = StyleQuery::single(None, None, None, "", "", &classes);
    let matched_rules = ssg.query_matched_rules(&[query], &media_query_status);
    assert_eq!(
        matched_rules.rules[0].rule.to_string(),
        ".a { display: block; }"
    );
    let classes = vec![("b".into(), None)];
    let query = StyleQuery::single(None, None, None, "", "", &classes);
    let matched_rules = ssg.query_matched_rules(&[query], &media_query_status);
    assert_eq!(
        matched_rules.rules[0].rule.to_string(),
        "@media all { .b { display: block; } }"
    );
}

#[test]
fn multi_classes() {
    let ssg = style_sheets([r#"
            .a { width: 1px }
            .a.b { width: 2px }
        "#]);
    {
        let classes = vec![("a".into(), None)];
        let query = StyleQuery::single(None, None, None, "", "", &classes);
        let mut node_properties = NodeProperties::new(None);
        ssg.query_single(
            &query,
            &MediaQueryStatus::<f32>::default_screen(),
            &mut node_properties,
        );
        assert_eq!(node_properties.width(), Length::Px(1.));
    }
    let node_properties = query(&ssg, "", "", ["b"], []);
    assert_eq!(node_properties.width(), Length::Undefined);
    let node_properties = query(&ssg, "", "", ["a", "b"], []);
    assert_eq!(node_properties.width(), Length::Px(2.));
    let node_properties = query(&ssg, "", "", ["a", "c"], []);
    assert_eq!(node_properties.width(), Length::Px(1.));
}

#[test]
fn id_with_class() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
        #a { width: 1px }
        #a.b { width: 2px }
        .b { width: 3px }
        .a { width: 4px }
    ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "a", [""], []);
    assert_eq!(node_properties.width(), Length::Px(1.));
    let node_properties = query(&ssg, "", "", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(3.));
    let node_properties = query(&ssg, "", "a", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(2.));
    let node_properties = query(&ssg, "", "a", ["a"], []);
    assert_eq!(node_properties.width(), Length::Px(1.));
}

#[test]
fn tag_name_id_with_class() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
        .b { width: 0 }
        div { width: 1px }
        div#a { width: 2px }
        div#c.b { width: 3px }
        div.b { width: 4px }
        span { width: 5px }
    ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "span", "", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(0.));
    let node_properties = query(&ssg, "div", "", [""], []);
    assert_eq!(node_properties.width(), Length::Px(1.));
    let node_properties = query(&ssg, "div", "a", [""], []);
    assert_eq!(node_properties.width(), Length::Px(2.));
    let node_properties = query(&ssg, "div", "", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(4.));
    let node_properties = query(&ssg, "div", "a", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(2.));
    let node_properties = query(&ssg, "div", "c", [""], []);
    assert_eq!(node_properties.width(), Length::Px(1.));
    let node_properties = query(&ssg, "div", "c", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(3.));
}

#[test]
fn parent() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
        .a > .b { width: 1px }
        .b { width: 2px }
        .a { height: 3px }
    ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.width(), Length::Undefined);
    assert_eq!(node_properties.height(), Length::Px(3.));
    let node_properties = query(&ssg, "", "", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(2.));
    assert_eq!(node_properties.height(), Length::Undefined);
    let node_properties = query_list(
        &ssg,
        [query_item("", "", ["a"], []), query_item("", "", ["b"], [])],
    );
    assert_eq!(node_properties.width(), Length::Px(1.));
    assert_eq!(node_properties.height(), Length::Undefined);
    let node_properties = query_list(
        &ssg,
        [
            query_item("", "", ["a"], []),
            query_item("", "", ["c"], []),
            query_item("", "", ["b"], []),
        ],
    );
    assert_eq!(node_properties.width(), Length::Px(2.));
    assert_eq!(node_properties.height(), Length::Undefined);
}

#[test]
fn next_sibling() {
    let ss = StyleSheet::from_str(
        "
            .a + .b {
                height: 100px;
            }
        ",
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(".a + .b", rule.get_selector_string())
    }
}

#[test]
fn subsequent_sibling() {
    let ss = StyleSheet::from_str(
        "
            .a ~ .b {
                height: 100px;
            }
        ",
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(".a ~ .b", rule.get_selector_string())
    }
}

#[test]
fn is_not_terminated() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
        .b { height: 100px}
        .a, .b, { height: 200px }
        .c > { height: 200px }
    ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["b"], []);
    assert_eq!(node_properties.height(), Length::Px(100.));
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_ne!(node_properties.height(), Length::Px(200.));
    let node_properties = query(&ssg, "", "", ["c"], []);
    assert_ne!(node_properties.height(), Length::Px(300.));
}

#[test]
fn ignore_error_selector() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
        .a > { height: 100px}
        .b { height: 200px }
        . { height: 300px }
        , { height: 200px; }
        > { height: 100px }
        >>a {height: 200px}
        .c { height: 400px }
        .d, ,.b { height: 300px; }
    ",
    );
    assert_eq!(ss.rules_count(Some(0)).unwrap(), 2u32);
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_ne!(node_properties.height(), Length::Px(100.));
    let node_properties = query(&ssg, "", "", ["b"], []);
    assert_eq!(node_properties.height(), Length::Px(200.));
    let node_properties = query(&ssg, "", "", ["c"], []);
    assert_eq!(node_properties.height(), Length::Px(400.));
    let node_properties = query(&ssg, "", "", ["d"], []);
    assert_ne!(node_properties.height(), Length::Px(300.));
}

#[test]
fn attribute_selector() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
        a[title] { height: 100px }
        .b { height: 200px }
        a[class~="logo"] { width: 300px }
        .c { height: 400px !important; }
    "#,
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "a", "", [""], []);
    assert_eq!(node_properties.height(), Length::Undefined);
    assert_eq!(node_properties.width(), Length::Undefined);
    let node_properties = query(&ssg, "a", "", [""], [("title".into(), "".into()), ("class".into(), "logo1 logo2".into())]);
    assert_eq!(node_properties.height(), Length::Px(100.));
    assert_eq!(node_properties.width(), Length::Undefined);
    let node_properties = query(&ssg, "a", "", [""], [("title".into(), "".into()), ("class".into(), "logo1 logo logo2".into())]);
    assert_eq!(node_properties.height(), Length::Px(100.));
    assert_eq!(node_properties.width(), Length::Px(300.));
    let node_properties = query(&ssg, "a", "", ["b"], [("title".into(), "".into())]);
    assert_eq!(node_properties.height(), Length::Px(100.));
    let node_properties = query(&ssg, "", "", [], [("title".into(), "".into())]);
    assert_eq!(node_properties.height(), Length::Undefined);
}

#[test]
fn ignore_number_id_selector() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        "
        #2 { height: 200px }
        .a, #3 { height: 300px }
    ",
    );
    ssg.append(ss);
    let node_properties = query(&ssg, "", "2", [""], []);
    assert_ne!(node_properties.height(), Length::Px(200.));
    let node_properties = query(&ssg, "", "3", [""], []);
    assert_ne!(node_properties.height(), Length::Px(300.));
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_ne!(node_properties.height(), Length::Px(300.));
}

#[test]
fn ignore_empty_selector() {
    let mut ssg = StyleSheetGroup::new();
    let ss = StyleSheet::from_str(
        r#"
        .a { height: 200px }
        {
            height: 200px;
        }
        .b { width: 200px }
    "#,
    );
    assert_eq!(ss.rules_count(Some(0)).unwrap(), 2u32);
    ssg.append(ss);
    let node_properties = query(&ssg, "", "", ["a"], []);
    assert_eq!(node_properties.height(), Length::Px(200.));

    let node_properties = query(&ssg, "", "", ["b"], []);
    assert_eq!(node_properties.width(), Length::Px(200.));
}

#[test]
fn universal_selector() {
    let ss = StyleSheet::from_str(
        r#"
        * {
            height: 200px;
            color: red;
        }
        .a {
            height: 300px;
        }
        .b > * {
            color: blue;
        }
        .b > .c {
            color: green;
        }
        .a * {
            height: 100px;
        }
        .e* {
            color: blue;
        }
        *.e {
            color: yellow;
        }
    "#,
    );
    let mut ssg = StyleSheetGroup::new();
    ssg.append(ss);
    let node_properties = query_list(&ssg, [query_item("", "", [""], [])]);
    assert_eq!(node_properties.height(), Length::Px(200.));
    assert_eq!(node_properties.color(), Color::Specified(255, 0, 0, 255));
    let node_properties = query_list(
        &ssg,
        [query_item("", "", ["b"], []), query_item("", "", [""], [])],
    );
    assert_eq!(node_properties.color(), Color::Specified(0, 0, 255, 255));
    let node_properties = query_list(
        &ssg,
        [query_item("", "", ["b"], []), query_item("", "", ["c"], [])],
    );
    assert_eq!(node_properties.color(), Color::Specified(0, 128, 0, 255));
    let node_properties = query_list(&ssg, [query_item("", "", ["b"], [])]);
    assert_eq!(node_properties.color(), Color::Specified(255, 0, 0, 255));
    let node_properties = query_list(
        &ssg,
        [query_item("", "", ["a"], []), query_item("", "", [""], [])],
    );
    assert_eq!(node_properties.height(), Length::Px(100.));
    let node_properties = query_list(&ssg, [query_item("", "", ["e"], [])]);
    assert_eq!(node_properties.color(), Color::Specified(255, 255, 0, 255));
}

#[test]
fn host_selector() {
    let ss = StyleSheet::from_str(
        r#"
            :host {
                height: 10px;
            }
            .a, :host {
                height: 20px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ":host")
    }
    let rule = ss.get_rule(1);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a, :host")
    }
}

#[test]
fn pseudo_classes_selector() {
    let ss = StyleSheet::from_str(
        r#"
            div:first-child {
                height: 10px;
            }
            span: first-child {
                height: 20px;
            }
            hello :first-child {
                height: 30px;
            }
            world : first-child {
                height: 40px;
            }
            world: {
                height: 50px;
            }
            div:first-child .foo {
                background: magenta;
            }
        "#,
    );
    println!("{:?}", ss);
}

#[test]
fn pseudo_classes_nth_of_type() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-of-type(2n + 3) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-of-type(2n + 3)")
    }
}

#[test]
fn pseudo_classes_nth_of_type_odd() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-of-type(odd) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-of-type(2n + 1)")
    }
}

#[test]
fn pseudo_classes_nth_of_type_even() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-of-type(even) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-of-type(2n + 0)")
    }
}

#[test]
fn pseudo_classes_nth_child() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-child(2n + 3) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-child(2n + 3)")
    }
}

#[test]
fn pseudo_classes_nth_child_of_selectors() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-child(2n + 3 of .a) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-child(2n + 3 of .a)")
    }
}

#[test]
fn pseudo_classes_nth_child_of_multi_selectors() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-child(2n + 3 of .a, div.b) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(
            rule.get_selector_string(),
            "div:nth-child(2n + 3 of .a,div.b)"
        )
    }
}
#[test]
fn pseudo_classes_nth_child_a_is_zero() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-child(7) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-child(0n + 7)")
    }
}

#[test]
fn pseudo_classes_nth_child_b_is_zero() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-child(3n) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-child(3n + 0)")
    }
}

#[test]
fn pseudo_classes_nth_child_even() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-child(even) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-child(2n + 0)")
    }
}

#[test]
fn pseudo_classes_nth_child_odd() {
    let ss = StyleSheet::from_str(
        r#"
            div:nth-child(odd) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:nth-child(2n + 1)")
    }
}

#[test]
fn pseudo_classes_nth_child_not_prefix() {
    let ss = StyleSheet::from_str(
        r#"
            :nth-child(odd) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ":nth-child(2n + 1)")
    }
}

#[test]
fn pseudo_classes_nth_child_of_nth_child_not_prefix() {
    let ss = StyleSheet::from_str(
        r#"
            :nth-child(odd of :nth-child(even)) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(
            rule.get_selector_string(),
            ":nth-child(2n + 1 of :nth-child(2n + 0))"
        )
    }
}

#[test]
fn pseudo_classes_first_child() {
    let ss = StyleSheet::from_str(
        r#"
            div:first-child {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:first-child")
    }
}

#[test]
fn pseudo_classes_last_child() {
    let ss = StyleSheet::from_str(
        r#"
            div:last-child {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:last-child")
    }
}

#[test]
fn pseudo_classes_only_child() {
    let ss = StyleSheet::from_str(
        r#"
            div:only-child {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div:only-child")
    }
}

#[test]
fn pseudo_classes_selector_empty() {
    let ss = StyleSheet::from_str(
        r#"
            div:empty {
                height: 10px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, "div:empty")
    }
}

#[test]
fn pseudo_classes_selector_not() {
    let ss = StyleSheet::from_str(
        r#"
            div:not(.a) {
                height: 10px;
            }
            div:not(.a.b) {
                height: 10px;
            }
            div:not(.a.b, #c) {
                height: 10px;
            }
            div:not(.a):not(.b) {
                height: 10px;
            }
            div:not(:first-child) {
                height: 10px;
            }
            div:not(#theid).class:not(.fail).test#theid#theid {
                height: 10px;
            }
            .foo:not(.a) .b {
                height: 100px;
            }
            .foo:not(.a) > .b {
                height: 100px;
            }
            .foo:not(.a):not(.b) > .c {
                height: 100px;
            }
            .foo:not(.a):not(.b), .c {
                height: 100px;
            }
            .a > .foo:not(.b) {
                height: 100px;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, "div:not(.a)")
    }
    let rule = ss.get_rule(1);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, "div:not(.a.b)")
    }
    let rule = ss.get_rule(2);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, "div:not(.a.b, #c)")
    }
    let rule = ss.get_rule(3);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, "div:not(.a, .b)")
    }
    let rule = ss.get_rule(4);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, "div:not(:first-child)")
    }
    let rule = ss.get_rule(5);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, "div#theid.class.test:not(#theid, .fail)")
    }
    let rule = ss.get_rule(6);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str: String = rule.get_selector_string();
        assert_eq!(selector_str, ".foo:not(.a) .b");
    }
    let rule = ss.get_rule(7);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, ".foo:not(.a) > .b");
    }
    let rule = ss.get_rule(8);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, ".foo:not(.a, .b) > .c");
    }
    let rule = ss.get_rule(9);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, ".foo:not(.a, .b), .c");
    }
    let rule = ss.get_rule(10);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        let selector_str = rule.get_selector_string();
        assert_eq!(selector_str, ".a > .foo:not(.b)");
    }
}

#[test]
fn pseudo_elements_selector() {
    let ss = StyleSheet::from_str(
        r#"
            div::before {
                height: 10px;
                content: "123";
            }
            div::before {
                height: 10px;
                content: url("123");
            }
            div::before {
                height: 10px;
                content: none;
            }
            div::before {
                height: 10px;
                content: normal;
            }
            span:: after {
                height: 20px;
            }
            hello ::after {
                height: 30px;
            }
            world :: after {
                height: 40px;
            }
            world : : after{
                height: 50px;
            }
            div::before .foo {
                height: 60px;
            }
            div:first-child .a::before {
                height: 70px;
            }
            .foo::after {
                content: "abc";
                display: flex;
                background: yellow;
            }
          
            .foo::before {
                content: "def";
                display: flex;
                background: blue;
            }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), "div::before")
    }
}

#[test]
fn pseudo_elements_selector_test() {
    let ss = StyleSheet::from_str(
        r#"
        .foo::before {
            content: '123';
            background: red;
          }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".foo::before")
    }
}

#[test]
fn attr_set_selector_test() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a[foo]")
    }
}

#[test]
fn attr_exact_selector_test() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo="bar"] {
            height: 10px
        }
        .a[foo=bar] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a[foo=\"bar\"]")
    }
    let rule = ss.get_rule(1);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a[foo=\"bar\"]")
    }
}

#[test]
fn attr_selector_error_test() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo="aaa" ddddd] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_none());
}

#[test]
fn attr_set_with_flags_test() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo i] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_none());
}

#[test]
fn attr_exact_with_flags_test() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo=bar i] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a[foo=\"bar\" i]")
    }
}

#[test]
fn attr_selector_as_child() {
    let ss = StyleSheet::from_str(
        r#"
        .b > .a[foo=bar i] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".b > .a[foo=\"bar\" i]")
    }
}

#[test]
fn attr_selector_as_parent() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo=bar i] > .b {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a[foo=\"bar\" i] > .b")
    }
}

#[test]
fn attr_selector_as_next_sibling() {
    let ss = StyleSheet::from_str(
        r#"
        .b ~ .a[foo=bar i]  {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".b ~ .a[foo=\"bar\" i]")
    }
}

#[test]
fn attr_selector_as_prev_sibling() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo=bar i] ~ .b[foo] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a[foo=\"bar\" i] ~ .b[foo]")
    }
}

#[test]
fn multi_attr_selector() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo=bar][hello] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(rule.get_selector_string(), ".a[foo=\"bar\"][hello]")
    }
}

#[test]
fn multi_attr_selector_2() {
    let ss = StyleSheet::from_str(
        r#"
        .a[foo=bar i][hello] ~ .b[foo] {
            height: 10px
        }
        "#,
    );
    let rule = ss.get_rule(0);
    assert!(rule.is_some());
    if let Some(rule) = rule {
        assert_eq!(
            rule.get_selector_string(),
            ".a[foo=\"bar\" i][hello] ~ .b[foo]"
        )
    }
}
