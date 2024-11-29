use crate::*;

#[test]
fn flex_start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: flex-start; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="0" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="100" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn start() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: start; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="0" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="50" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="100" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn flex_end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: flex-end; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="450" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="500" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="550" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn end() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: end; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="450" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="500" expect_left="0"></div>
            <div style="height: 50px; width: 50px;" expect_width="50" expect_height="50" expect_top="550" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn center() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: center; width: 50px; height: 500px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="100" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="200" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="300" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn space_between() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: space-between; width: 50px; height: 500px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="0" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="400" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn space_around() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: space-around; width: 50px; height: 600px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="100" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="400" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn space_evenly() {
    assert_xml!(
        r#"
        <div style="display: flex; align-content: space-evenly; width: 50px; height: 500px; flex-wrap: wrap;">
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="100" expect_left="0"></div>
            <div style="height: 100px; width: 50px;" expect_width="50" expect_height="100" expect_top="300" expect_left="0"></div>
        </div>
    "#
    )
}

#[test]
fn flex_end_with_wrap() {
    assert_xml!(
        r#"
        <div style="display: flex; width: 100px; height: 100px; flex-wrap: wrap; flex-direction: column; align-content: flex-end;">
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="0"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="10"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="20"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="30"></div>
            <div style="width: 50px; height: 10px;" expect_left="50" expect_top="40"></div>
        </div>
    "#
    )
}
