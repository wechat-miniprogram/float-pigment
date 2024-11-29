use crate::*;

#[test]
fn flex_shrink_0_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px;">
          <div style="flex-shrink: 0; height: 100px; width: 200px;" expect_width="200"></div>
          <div style="flex-shrink: 1; height: 100px; width: 100px;" expect_width="0"></div>
        </div>
    "#
    )
}

#[test]
fn flex_shrink_1_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; height: 100px; width: 200px;" expect_width="80"></div>
          <div style="flex-shrink: 1; height: 100px; width: 300px;" expect_width="120"></div>
        </div>
    "#
    )
}

#[test]
fn flex_shrink_1_0_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 200px;">
          <div style="flex-shrink: 1; height: 100px; width: 200px;" expect_width="120"></div>
          <div style="flex-shrink: 0; height: 100px; width: 20px;" expect_width="20"></div>
          <div style="flex-shrink: 2; height: 100px; width: 300px;" expect_width="60"></div>
        </div>
    "#
    )
}
