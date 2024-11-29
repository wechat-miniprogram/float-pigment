use crate::*;

#[test]
fn min_width_fixed() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 400px; height: 50px;" expect_width="400"></div>
          </div>
      "#
    )
}

#[test]
fn min_width_percentage() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 50%; height: 50px;" expect_width="300"></div>
          </div>
      "#
    )
}

#[test]
fn min_width_fixed_in_flex() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px; display: flex" expect_width="300">
            <div style="min-width: 100px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

#[test]
fn min_width_percentage_in_flex() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px; display: flex" expect_width="300">
            <div style="min-width: 50%; height: 50px;" expect_width="150"></div>
          </div>
      "#
    )
}

#[test]
fn min_width_fixed_lt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 50px; width: 100px; height: 50px;" expect_width="100"></div>
          </div>
      "#
    )
}

#[test]
fn min_width_fixed_gt_width() {
    assert_xml!(
        r#"
          <div style="width: 300px; height: 100px;" expect_width="300">
            <div style="min-width: 150px; width: 100px; height: 50px;" expect_width="150"></div>
          </div>
      "#
    )
}

#[test]
fn min_height_flex_align_items_center() {
    assert_xml!(
        r#"
          <div style="display:flex;  width: 300px; min-height: 300px; align-items: center;">
            <div style="width: 100px; height: 100px;" expect_top="100"></div>
          </div>
      "#
    )
}

#[test]
fn min_height_flex_justify_content_center() {
    assert_xml!(
        r#"
          <div style="display:flex; flex-direction: column; width: 300px; min-height: 300px; justify-content: center;">
            <div style="width: 100px; height: 100px;" expect_top="100"></div>
          </div>
      "#
    )
}
