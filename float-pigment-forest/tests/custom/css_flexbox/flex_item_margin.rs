use crate::*;

#[test]
fn flex_item_with_margin() {
    assert_xml!(
        r#"
        <div style="height: 100px; display: flex; width: 100px;">
          <div style="width: 30px; margin-left: auto; margin-right: auto" expect_left="10" expect_width="30"></div>
          <div style="width: 30px; margin-left: auto; margin-right: auto" expect_left="60" expect_width="30"></div>
        </div>
    "#
    )
}

#[test]
fn flex_item_with_margin_1() {
    assert_xml!(
        r#"
        <div style="height: 100px; display: flex; width: 100px;">
          <div style="width: 30px; margin-left: auto; margin-right: auto" expect_left="20" expect_width="30"></div>
          <div style="width: 30px;" expect_left="70" expect_width="30"></div>
        </div>
    "#
    )
}
