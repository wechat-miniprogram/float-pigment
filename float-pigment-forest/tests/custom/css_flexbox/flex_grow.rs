use crate::*;

#[test]
fn flex_grow_0_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; height: 10px;" expect_width="0"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="300"></div>
        </div>
    "#
    )
}

#[test]
fn flex_grow_1_1() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 10px;" expect_width="150"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="150"></div>
        </div>
    "#
    )
}

#[test]
fn flex_grow_1_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 1; height: 10px;" expect_width="100"></div>
          <div style="flex-grow: 2; height: 10px;" expect_width="200"></div>
        </div>
    "#
    )
}

#[test]
fn flex_grow_0_1_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; height: 10px;" expect_width="0"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="100"></div>
          <div style="flex-grow: 2; height: 10px;" expect_width="200"></div>
        </div>
    "#
    )
}

#[test]
fn flex_grow_0_width_1_2() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 30px; height: 10px;" expect_width="30"></div>
          <div style="flex-grow: 1; height: 10px;" expect_width="90"></div>
          <div style="flex-grow: 2; height: 10px;" expect_width="180"></div>
        </div>
    "#
    )
}

#[test]
fn flex_grow_0_width() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px;">
          <div style="flex-grow: 0; width: 10px; height: 10px;" expect_width="10"></div>
          <div style="flex-grow: 0; width: 20px; height: 10px;" expect_width="20"></div>
          <div style="flex-grow: 0; height: 10px;" expect_width="0"></div>
        </div>
    "#
    )
}
