use crate::*;

#[test]
fn width_fixed() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_width="100"> </div>
    "#
    )
}

#[test]
fn width_percentage() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 100px;" expect_width="100">
          <div style="width: 50%; height: 100px;" expect_width="50"></div>
        </div>
    "#
    )
}

#[test]
fn width_auto() {
    assert_xml!(
        r#"
        <div style="width: 100px; height: 200px;" expect_width="100">
          <div style="width: auto; height: 100px;" expect_width="100">
            <div style="width: 300px; height: 100px;" expect_width="300"> </div>
          </div>
          <div style="width: auto; height: 100px;" expect_width="100">
            <div style="width: 50%; height: 100px;" expect_width="50"> </div>
          </div>
        </div>
    "#
    )
}
