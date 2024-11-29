use crate::*;

#[test]
fn max_height_fixed_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="50">
              <div style="max-height: 100px; height: 50px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn max_height_fixed_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="50">
              <div style="max-height: 50px; height: 100px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn max_height_percentage_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="max-height: 50%; height: 100px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn max_height_percentage_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="max-height: 50%; height: 20px;" expect_height="20"></div>
            </div>
        </div>
    "#
    )
}
