use crate::*;

#[test]
fn content_box() {
    assert_xml!(
        r#"
        <div style="width: 200px; height: 200px; padding-left: 20px; padding-top: 30px;" expect_width="220" expect_height="230">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="20"></div>
        </div>
    "#
    )
}

#[test]
fn border_box() {
    assert_xml!(
        r#"
        <div style="box-sizing: border-box; width: 200px; height: 200px; padding-left: 20px; padding-top: 30px;" expect_width="200" expect_height="200">
            <div style="width: 50px; height: 50px;" expect_top="30" expect_left="20"></div>
        </div>
    "#
    )
}
