use crate::*;

#[test]
fn align_items() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; height: 100px; width: 100px;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="25" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_stretch() {
    assert_xml!(
        r#"
        <div style="display: flex; margin-top: 100px;">
          <div style="flex: 33; height: 80px;"></div>
          <div style="flex: 33;" expect_height="80" expect_width="125"></div>
          <div style="flex: 33;" expect_height="80" expect_width="125">
            <div style="width: 100%; height: 100%; expect_height="80" expect_width="125"></div>
          </div>
        </div>
    "#
    )
}

#[test]
fn align_items_start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: start; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="0" ></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="0"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="0"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: flex-start; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="0" ></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="0"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="0"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_center() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="10"></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="20"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="25"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: end; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="20"></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="40"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="50"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: flex-end; height: 100px;" expect_height="100">
          <div style="flex: 33; height: 80px;" expect_height="80" expect_top="20"></div>
          <div style="flex: 33; height: 60px" expect_height="60" expect_width="125" expect_top="40"></div>
          <div style="flex: 33; height: 50px;" expect_height="50" expect_width="125" expect_top="50"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="0">xxx</div>
            <div style="height: 10px; width: 10px;" expect_top="6"></div>
            <div expect_top="0">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline_1() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="4">xxx</div>
            <div style="height: 20px; width: 10px;" expect_top="0"></div>
            <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline_2() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="4">xxx</div>
            <div style="display: flex; height: 20px; width: 10px;" expect_top="0"></div>
            <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline_3() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="4">xxx</div>
            <div style="display: inline-block; height: 20px; width: 10px;" expect_top="0"></div>
            <div expect_top="4">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline_4() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="0">xxx</div>
            <div style="display: flex; height: 10px; width: 10px;" expect_top="6"></div>
            <div expect_top="0">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline_5() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="0">xxx</div>
            <div style="display: inline-block; height: 10px; width: 10px;" expect_top="6"></div>
            <div expect_top="0">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline_margin_top() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div expect_top="10">xxx</div>
            <div style="margin-top: 10px; height: 10px; width: 10px;" expect_top="10">x x x</div>
            <div expect_top="10">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_baseline_max_margin_top() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: baseline">
            <div style="margin-top: 20px;"expect_top="20">xxx</div>
            <div style="margin-top: 10px; height: 10px; width: 10px;" expect_top="20">xxx</div>
            <div expect_top="20">xxx</div>
        </div>
    "#
    )
}

#[test]
fn align_items_center_with_min_height() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; min-height: 60px; height: 10px;" expect_height="60">
            <div style="height: 10px; width: 10px;" expect_top="25"></div>
            <div style="height: 20px; width: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_center_with_max_height() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; max-height: 60px; height: 100px;" expect_height="60">
            <div style="height: 10px; width: 10px;" expect_top="25"></div>
            <div style="height: 20px; width: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_center_with_min_width() {
    assert_xml!(
        r#"
        <div style="width: 10px;">
            <div style="display: flex; flex-direction: column; align-items: center; height: 100px; min-width: 100px;" expect_width="100">
                <div style="height: 10px; width: 10px;" expect_left="45"></div>
                <div style="height: 20px; width: 20px;" expect_left="40"></div>
            </div>
        </div>
    "#
    )
}

#[test]
fn align_items_center_with_max_width() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; align-items: center; height: 100px; max-width: 100px;" expect_width="100">
            <div style="height: 10px; width: 10px;" expect_left="45"></div>
            <div style="height: 20px; width: 20px;" expect_left="40"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_center_with_min_max_limit() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; min-height: 60px; height: 30px; max-height: 300px;" expect_height="60">
            <div style="height: 10px; width: 10px;" expect_top="25"></div>
            <div style="height: 20px; width: 10px;" expect_top="20"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_center_with_min_max_limit_2() {
    assert_xml!(
        r#"
        <div style="display: flex; flex-direction: column; align-items: center; height: 100px; min-width: 100px; width: 30px; max-width: 300px;" expect_width="100">
            <div style="height: 10px; width: 10px;" expect_left="45"></div>
            <div style="height: 20px; width: 20px;" expect_left="40"></div>
        </div>
    "#
    )
}

#[test]
fn align_items_center_with_align_content_flex_start_and_min_height() {
    assert_xml!(
        r#"
        <div style="display: flex; align-items: center; min-height: 100px; align-content: flex-start" expect_height="100" expect_top="0">
            <div style="display: flex; height: 10px; width: 10px;" expect_top="45"></div>
            <div style="display: flex; height: 20px; width: 20px;" expect_top="40"></div>
        </div>
    "#,
        true
    )
}

#[test]
fn align_items_center_with_align_content_flex_start_with_min_height_with_wrap() {
    assert_xml!(
        r#"
            <div style="display: flex; align-items: center; min-height: 100px; align-content: flex-start; flex-wrap: wrap" expect_height="100" expect_top="0">
                <div style="display: flex; height: 10px; width: 10px;" expect_top="5"></div>
                <div style="display: flex; height: 20px; width: 20px;" expect_top="0"></div>
            </div>
        "#,
        true
    )
}
