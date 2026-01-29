use crate::*;

#[test]
fn gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; gap: 10px;">
          <div style="height: 10px; flex: 1" expect_width="45"></div>
          <div style="height: 10px; flex: 1" expect_width="45" expect_left="55"></div>
        </div>
    "#
    )
}

#[test]
fn column_gap_in_flex_row_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: row; width: 100px; height: 100px; column-gap: 10px;">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="30"></div>
        </div>
    "#
    )
}

#[test]
fn column_gap_with_percentage_in_flex_row_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: row; width: 200px; height: 100px; column-gap: 10%; align-items: flex-start">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="40"></div>
        </div>
    "#
    )
}

#[test]
fn column_gap_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 20px; column-gap: 10px; flex-wrap: wrap;">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="55"></div>
        </div>
    "#
    )
}

#[test]
fn column_gap_in_flex_column_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 20px; column-gap: 10px; flex-wrap: wrap; align-content: flex-start">
          <div style="width: 20px; height: 20px;" expect_left="0"></div>
          <div style="width: 20px; height: 20px;" expect_left="30"></div>
        </div>
    "#
    )
}

#[test]
fn column_gap_with_percentage_in_flex_column_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 20px; column-gap: 10%; flex-wrap: wrap; align-content: flex-start">
          <div style="width: 20px; height: 20px; flex-shrink: 0" expect_left="0"></div>
          <div style="width: 20px; height: 20px; flex-shrink: 0" expect_left="30"></div>
        </div>
    "#
    )
}

#[test]
fn row_gap_in_flex_row_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction:row; width: 100px; height: 100px; row-gap: 10px; flex-wrap: wrap;">
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="50"  expect_top="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="55"></div>
          <div style="height: 10px; width: 50px;" expect_left="50" expect_top="55"></div>
        </div>
    "#
    )
}

#[test]
fn row_gap_in_flex_row_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction:row; width: 100px; height: 100px; row-gap: 10px; flex-wrap: wrap; align-content: flex-start;">
          <div style="height: 10px; width: 50px;" expect_left="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="50"></div>
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="20"></div>
          <div style="height: 10px; width: 50px;" expect_left="50" expect_top="20"></div>
        </div>
    "#
    )
}

#[test]
fn row_gap_with_percentage_in_flex_row_box_with_align_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction:row; width: 100px; height: 100px; row-gap: 10%; flex-wrap: wrap; align-content: flex-start;">
          <div style="height: 10px; width: 50px;" expect_left="0"></div>
          <div style="height: 10px; width: 50px;" expect_left="50"></div>
          <div style="height: 10px; width: 50px;" expect_left="0" expect_top="20"></div>
          <div style="height: 10px; width: 50px;" expect_left="50" expect_top="20"></div>
        </div>
    "#
    )
}

#[test]
fn row_gap_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 100px; row-gap: 10px;">
          <div style="width: 100px; height: 30px;"></div>
          <div style="width: 100px; height: 30px;"expect_top="40"></div>
        </div>
    "#
    )
}

#[test]
fn row_gap_with_percentage_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 100px; row-gap: 10%;">
          <div style="width: 100px; height: 30px;"></div>
          <div style="width: 100px; height: 30px;"expect_top="40"></div>
        </div>
    "#
    )
}

#[test]
fn flex_item_with_gap_should_shrink_to_fit() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; height: 50px; flex-direction: column; gap: 10px;" expect_height="50">
          <div style="height: 30px;" expect_top="0" expect_height="20"></div>
          <div style="height: 30px;" expect_top="30" expect_height="20"></div>
        </div>
    "#
    )
}
