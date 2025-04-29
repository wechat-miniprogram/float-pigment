use crate::*;

#[test]
fn text_with_font_size() {
    assert_xml!(
        r#"
        <div style="font-size: 30px;" expect_height="30">
          XX
        </div>
    "#,
        true
    )
}

#[test]
fn text_with_font_size_2() {
    assert_xml!(
        r#"
        <div style="font-size: 30px;" expect_height="30">
          <div style="display: inline" expect_width="60">XX</div>
        </div>
    "#,
        true
    )
}
