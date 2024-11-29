use crate::*;

#[test]
fn order() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 2;  width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 3;  width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 4;  width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

#[test]
fn order_1() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 1;  width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 3;  width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 4;  width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

#[test]
fn order_2() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 1;  width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 2;  width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 2;  width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

#[test]
fn order_3() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 4; width: 10px; height: 10px;" expect_left="90"></div>
            <div style="order: 3; width: 20px; height: 10px;" expect_left="70"></div>
            <div style="order: 2; width: 30px; height: 10px;" expect_left="40"></div>
            <div style="order: 1; width: 40px; height: 10px;" expect_left="0"></div>
          </div>
      "#
    )
}

#[test]
fn order_4() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: -100; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 0; width: 20px; height: 10px;" expect_left="10"></div>
            <div style="order: 1; width: 30px; height: 10px;" expect_left="30"></div>
            <div style="order: 100; width: 40px; height: 10px;" expect_left="60"></div>
          </div>
      "#
    )
}

#[test]
fn order_5() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 3;  width: 20px; height: 10px;" expect_left="90"></div>
            <div style="order: 2;  width: 30px; height: 10px;" expect_left="10"></div>
            <div style="order: 4;  width: 40px; height: 10px;" expect_left="110"></div>
            <div style="order: 2;  width: 50px; height: 10px;" expect_left="40"></div>
          </div>
      "#
    )
}

#[test]
fn order_6() {
    assert_xml!(
        r#"
          <div style="display: flex;">
            <div style="order: 1; width: 10px; height: 10px;" expect_left="0"></div>
            <div style="order: 3; position: fixed; width: 20px; height: 10px; left: 0px; height: 0px;" expect_left="0"></div>
            <div style="order: 2; position: absolute; width: 30px; height: 10px; left: 0px; height: 0px;" expect_left="0"></div>
            <div style="order: 4; width: 40px; height: 10px;" expect_left="60"></div>
            <div style="order: 2; width: 50px; height: 10px;" expect_left="10"></div>
          </div>
      "#
    )
}
