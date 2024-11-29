use crate::*;

#[test]
fn flex_direction_row() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>

        </div>
    "#
    )
}

#[test]
fn flex_direction_row_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row-reverse;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>

        </div>
    "#
    )
}

#[test]
fn flex_direction_column() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: column; height: 100px;"  expect_width="100" expect_height="100">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="0"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="50"></div>
        </div>
    "#
    )
}

#[test]
fn flex_direction_column_reverse() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: column-reverse; height: 100px;"  expect_width="100" expect_height="100">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="50"></div>
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="0" expect_top="0"></div>
        </div>
    "#
    )
}

#[test]
fn flex_direction_row_with_parent_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row; padding-left: 10px; padding-right: 10px; box-sizing: border-box;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="10"></div>
        </div>
      "#
    )
}

#[test]
fn flex_direction_row_reverse_with_parent_padding() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; flex-direction: row-reverse; padding-left: 10px; padding-right: 10px; box-sizing: border-box;"  expect_width="100" expect_height="50">
          <div style="width: 50px; height: 50px;" expect_width="50" expect_height="50" expect_left="40"></div>
        </div>
      "#
    )
}
