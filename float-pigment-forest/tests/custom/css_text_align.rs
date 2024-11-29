use crate::*;

#[test]
fn text_align_1() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: center">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="50"></div>
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="150"></div>
        </div>
    "#
    )
}

#[test]
fn text_align_2() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: end">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="100"></div>
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="200"></div>
        </div>
    "#
    )
}

#[test]
fn text_align_3() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: start">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="0"></div>
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="100"></div>
        </div>
    "#
    )
}

#[test]
fn text_align_4() {
    assert_xml!(
        r#"
        <div style="width: 300px; text-align: center">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="100"></div>
          <div style="width: 100px; text-align: center">
            <div style="display: inline-block; width: 50px; height: 30px;" expect_left="25"></div>
          </div>
        </div>
    "#
    )
}

#[test]
fn text_align_5() {
    assert_xml!(
        r#"
        <div style="width: 20px; text-align: center">
          <div style="display: inline-block; width: 100px; height: 30px;" expect_left="0"></div>
        </div>
    "#
    )
}
