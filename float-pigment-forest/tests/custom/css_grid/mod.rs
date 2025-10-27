use crate::*;

#[test]
fn grid() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto; grid-template-rows: 30px 40px;">
          <div expect_height="30" expect_left="0">header1</div>
          <div expect_height="30" expect_left="250">header2</div>
          <div expect_height="30" expect_left="350">header3</div>
          <div expect_height="40" expect_left="0">content1</div>
          <div style="width: 23px; height: 23px" expect_height="23" expect_left="250">content2</div>
          <div expect_height="40" expect_left="350">content3</div>
          <div expect_height="32" expect_left="0">content4</div>
          <div expect_height="32" expect_left="250">content5</div>
          <div expect_height="32" expect_left="350">content6</div>
        </div>
    "#,
        true
    )
}

#[test]
fn grid_1() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto;" expect_height="32">
          <div expect_width="250" expect_height="32">header1</div>
          <div expect_width="100" expect_height="32">header2</div>
          <div expect_width="250" expect_height="32">header3</div>
        </div>
    "#,
        true
    )
}

#[test]
fn grid_2() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto;" expect_height="300">
          <div expect_width="250" expect_height="300">header1</div>
          <div expect_width="100">
            <div style="height: 300px;" expect_height="300"></div>
          </div>
          <div expect_width="250" expect_height="300">header3</div>
        </div>
    "#,
        true
    )
}

#[test]
fn grid_3() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 600px; grid-template-columns: auto 100px auto;" expect_height="316">
          <div expect_width="250" expect_height="300">header1</div>
          <div expect_width="100">
            <div style="height: 300px;" expect_height="300"></div>
          </div>
          <div expect_width="250" expect_height="300">header3</div>
          <div expect_width="250" expect_height="16">foote</div>
        </div>
    "#,
        true
    )
}

#[test]
fn grid_item_with_margin() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 100px 100px" >
          <div style="margin-top: 10px; margin-left: 10px; width: 50px; height: 50px;" expect_top="10" expect_left="10"></div>
          <div style="margin-top: 10px; margin-left: 10px; width: 50px; height: 50px;" expect_top="10" expect_left="110"></div>
        </div>
    "#,
        true
    )
}

#[test]
fn grid_item_with_border() {
    assert_xml!(
        r#"
        <div style="display: grid; width: 300px; grid-template-columns: 100px 100px" >
          <div style="width: 50px; padding: 10px; border-bottom: 1px solid black;" expect_height="101" expect_width="70"></div>
          <div style="width: 50px; height: 100px; border-bottom: 1px solid black;" expect_height="101" expect_width="50"></div>
        </div>
    "#,
        true
    )
}
