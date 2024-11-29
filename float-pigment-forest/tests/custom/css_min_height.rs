use crate::*;

#[test]
fn min_height_fixed() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="100">
              <div style="min-height: 100px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn min_height_fixed_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="100">
              <div style="min-height: 100px; height: 10px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn min_height_fixed_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div expect_height="100">
              <div style="min-height: 10px; height: 100px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn min_height_percentage() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="min-height: 50%;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn min_height_percentage_gt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="min-height: 50%; height: 10px;" expect_height="50"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn min_height_percentage_lt_height() {
    assert_xml!(
        r#"
        <div style="height: 300px;" expect_height="300">
            <div style="height: 100px;" expect_height="100">
              <div style="min-height: 50%; height: 100px;" expect_height="100"></div>
            </div>
        </div>
    "#
    )
}
