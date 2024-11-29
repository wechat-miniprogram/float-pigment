use crate::*;

#[test]
fn height_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_height="100"></div>
    "#
    )
}

#[test]
fn height_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_height="100">
          <div style="height: 50%" expect_height="50"></div>
        </div>
    "#
    )
}

#[test]
fn height_auto() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_height="100">
          <div style="height: auto" expect_height="50">
            <div style="height: 50px" expect_height="50"></div>
          </div>
          <div style="height: auto" expect_height="0">
            <div style="height: 50%" expect_height="0"></div>
          </div>
        </div>
    "#
    )
}
