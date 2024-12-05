use crate::*;

#[test]
fn flex_gap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; gap: 10px;">
          <div style="height: 10px; flex: 1" expect_width="45"></div>
          <div style="height: 10px; flex: 1" expect_width="45" expect_left="55"></div>
        </div>
    "#
    )
}
