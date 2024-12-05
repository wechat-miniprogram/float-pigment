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
        <div style="display: flex; width: 100px; column-gap: 10px; flex-wrap: wrap;">
          <div style="width: 100px; height: 30px; flex-shrink: 0" expect_width="100"></div>
          <div style="width: 100px; height: 30px; flex-shrink: 0" expect_width="100" expect_top="40"></div>
        </div>
    "#
    )
}

#[test]
fn row_gap_in_flex_row_box() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; row-gap: 10px;">
          <div style="height: 10px; flex: 1" expect_width="45"></div>
          <div style="height: 10px; flex: 1" expect_width="45" expect_left="55"></div>
        </div>
    "#
    )
}

#[test]
fn column_gap_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 50px; column-gap: 10px; flex-wrap: wrap; align-content: flex-start">
          <div style="width: 30px; height: 30px; flex-shrink: 0" expect_height="30"></div>
          <div style="width: 30px; height: 30px; flex-shrink: 0" expect_height="30" expect_left="40"></div>
        </div>
    "#
    )
}

#[test]
fn row_gap_in_flex_column_box() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; width: 100px; height: 100px; row-gap: 10px;">
          <div style="width: 100px; height: 30px;" expect_width="100"></div>
          <div style="width: 100px; height: 30px;" expect_width="100" expect_top="40"></div>
        </div>
    "#
    )
}
