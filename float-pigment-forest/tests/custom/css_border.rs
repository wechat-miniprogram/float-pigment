use crate::*;

#[test]
fn border_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; box-sizing: content-box; border: 10px;" expect_height="30" expect_width="30"></div>
    "#
    )
}

#[test]
fn border_fixed_border_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; box-sizing: border-box; border: 1px;" expect_height="10" expect_width="10"></div>
    "#
    )
}

#[test]
fn border_percentage_content_box() {
    assert_xml!(
        r#"
        <div style="width: 300px; height: 200px;">
          <div style="height: 10px; width: 10px; border: 10%;" expect_height="70" expect_width="70"></div>
        </div>
    "#
    )
}

#[test]
fn border_left_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-left: 20px;" expect_width="30"></div>
    "#
    )
}

#[test]
fn border_right_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-right: 20px;" expect_width="30"></div>
    "#
    )
}

#[test]
fn border_top_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-top: 20px;" expect_height="30"></div>
    "#
    )
}

#[test]
fn border_bottom_fixed_content_box() {
    assert_xml!(
        r#"
        <div style="height: 10px; width: 10px; border-bottom: 20px;" expect_height="30"></div>
    "#
    )
}
