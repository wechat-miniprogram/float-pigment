use crate::*;

#[test]
fn align_self_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px;">
          <div style="height: 50px; width: 50px; align-self: center" expect_height="50" expect_top="50"></div>
          <div style="height: 50px; width: 50px;" expect_top="0"></div>
          <div style="width: 50px; align-self: center"  expect_height="50" expect_top="50">
            <div style="height: 50px; width: 50px"></div>
          </div>
          <div style="width: 50px;" expect_height="150" expect_top="0">
            <div style="height: 50px; width: 50px" expect_height="50"></div>
          </div>
        </div>
    "#
    )
}

#[test]
fn align_self_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px; align-items: start">
          <div style="width: 50px; align-self: stretch" expect_height="150" expect_top="0">
            <div style="width: 50px; height: 50px;"></div>
          </div>
          <div style="height: 50px; width: 50px;" expect_height="50" expect_top="0"></div>

        </div>
    "#
    )
}

#[test]
fn align_self_self_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px; align-items: center">
          <div style="height: 50px; width: 50px; align-self: start" expect_height="50" expect_top="0"></div>
          <div style="height: 50px; width: 50px;" expect_top="50"></div>
          <div style="width: 50px; align-self: flex-start" expect_height="50" expect_top="0">
            <div style="height: 50px; width: 50px"></div>
          </div>
          <div style="width: 50px; align-self: self-start" expect_height="50" expect_top="0">
            <div style="height: 50px; width: 50px"></div>
          </div>
        </div>
    "#
    )
}

#[test]
fn align_self_self_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; height: 150px; align-items: center">
          <div style="height: 50px; width: 50px; align-self: end" expect_height="50" expect_top="100"></div>
          <div style="height: 50px; width: 50px;" expect_top="50"></div>
          <div style="width: 50px; align-self: flex-end" expect_height="50" expect_top="100">
            <div style="height: 50px; width: 50px"></div>
          </div>
          <div style="width: 50px; align-self: self-end" expect_height="50" expect_top="100">
            <div style="height: 50px; width: 50px"></div>
          </div>
        </div>
    "#
    )
}
