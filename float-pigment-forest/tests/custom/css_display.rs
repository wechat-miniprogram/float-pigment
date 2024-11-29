use crate::*;

#[test]
fn display_block() {
    assert_xml!(
        r#"
            <div style="height: 80px;">
                <div>
                    <div style="height: 40px; width: 200px; margin-right: 100px;" expect_width="200"></div>
                    <div style="height: 40px;" expect_width="375" expect_top="40"></div>
                </div>
            </div>
        "#
    )
}

#[test]
fn display_none() {
    assert_xml!(
        r#"
            <div style="height: 300px;" expect_height="300">
                <div style="display: none; width: 100px; height: 100px;" expect_top="0" expect_height="0" expect_width="0"></div>
                <div style="width: 100px; height: 100px;" expect_height="100" expect_width="100" expect_top="0"></div>
            </div>
        "#
    )
}

#[test]
fn display_flex() {
    assert_xml!(
        r#"
            <div style="height: 300px;" expect_height="300">
               <div style="display: flex; width="100px" expect_height="50">
                    <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50"></div>
                    <div style="height: 50px; width: 30px; flex-grow: 1;" expect_width="50" expect_height="50"></div>
               </div>
            </div>
        "#
    )
}
