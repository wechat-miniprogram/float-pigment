use crate::*;

#[test]
fn padding_fixed() {
    assert_xml!(
        r#"
        <div style="height: 10px; padding: 10px 20px 30px 40px;" expect_height="50">
            <div style="height: 100%" expect_height="10" expect_width="315" expect_left="40"></div>
        </div>
    "#
    )
}

#[test]
fn padding_left_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-left: 20px; width: 10px; height: 10px;" expect_width="30"></div>
        </div>
    "#
    )
}

#[test]
fn padding_right_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-right: 20px; box-sizing: content-box; width: 10px; height: 10px;" expect_width="30"></div>
        </div>
    "#
    )
}

#[test]
fn padding_top_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-top: 20px; box-sizing: content-box; width: 10px; height: 10px;" expect_width="10" expect_height="30"></div>
        </div>
    "#
    )
}

#[test]
fn padding_bottom_fixed() {
    assert_xml!(
        r#"
        <div>
            <div style="padding-top: 20px; box-sizing: content-box; width: 10px; height: 10px;" expect_width="10" expect_height="30"></div>
        </div>
    "#
    )
}

#[test]
fn padding_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;">
            <div style="padding: 10%" expect_height="20"></div>
        </div>
    "#
    )
}
