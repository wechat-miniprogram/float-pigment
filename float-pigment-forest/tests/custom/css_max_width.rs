use crate::*;

#[test]
fn max_width_fixed() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 100px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

#[test]
fn max_width_percentage() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 50%; height: 50px;" expect_width="150"></div>
          </div>
      "#
    )
}

#[test]
fn max_width_fixed_lt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 100px; width: 200px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

#[test]
fn max_width_fixed_gt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 300px; width: 200px; height: 50px;" expect_width="200"></div>
          </div>
      "#
    )
}

#[test]
fn max_width_fixed_lt_child_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 200px; height: 50px;" expect_width="200">
              <div style="width: 300px; height: 50px" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}

#[test]
fn max_width_fixed_gt_child_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="max-width: 400px; height: 50px;" expect_width="300">
              <div style="width: 300px; height: 50px" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}
