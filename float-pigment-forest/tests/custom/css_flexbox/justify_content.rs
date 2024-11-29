use crate::*;

#[test]
fn justify_content_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: start">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-start">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_center() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: center">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="100"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="150"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: end">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="200"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="250"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: flex-end">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="200"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="250"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_left() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: left">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="50"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_right() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 300px; justify-content: right">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="200"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_left="250"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_space_between() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; justify-content: space-between">
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="0"></div>
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="80"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_space_around() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 120px; justify-content: space-around">
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="20"></div>
            <div style="height: 50px; width: 20px;" expect_width="20" expect_height="50" expect_left="80"></div>
        </div>
    "#
    )
}

#[test]
fn justify_content_space_evenly() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 170px; justify-content: space-evenly">
            <div style="height: 50px; width: 40px;" expect_width="40" expect_height="50" expect_left="30"></div>
            <div style="height: 50px; width: 40px;" expect_width="40" expect_height="50" expect_left="100"></div>
        </div>
    "#
    )
}
