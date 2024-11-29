use crate::*;

#[test]
fn relative_left_top_fixed() {
    assert_xml!(
        r#"
          <div style="height: 100px; width: 100px;">
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="10"></div>
            <div style="position: relative; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="60"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_size_fixed() {
    assert_xml!(
        r#"
          <div expect_height="0">
            <div style="position: absolute; height: 50px; width: 50px; left: 10px; top: 10px;" expect_left="10" expect_top="10"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_size_percentage() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10%; width: 10%; left: 10px; top: 10px;" expect_height="20" expect_width="10" expect_left="10" expect_top="10"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_left_top_fixed() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; left: 10px; top: 10px; " expect_left="10" expect_top="10"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_right_bottom_fixed() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; right: 10px; bottom: 10px; " expect_left="80" expect_top="180"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_left_top_percentage() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; left: 50%; top: 50%;" expect_left="50" expect_top="100"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_right_bottom_percentage() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="position: absolute; height: 10px; width: 10px; right: 50%; bottom: 50%;" expect_left="40" expect_top="90"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_zero() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 200px;" expect_height="200" expect_width="100">
            <div style="height: 10px;"></div>
            <div style="position: absolute; left: 0; top: 0; bottom: 0; right: 0;" expect_width="100" expect_height="200" expect_top="0"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_in_flex() {
    assert_xml!(
        r#"
          <div style="width: 100px; height: 100px; display: flex;">
            <div style="flex-grow: 1;" expect_width="50" expect_left="0"></div>
            <div style="flex-grow: 1; position: absolute; left: 20px; top: 20px; width: 10px; height: 10px;" expect_top="20" expect_left="20"></div>
            <div style="flex-grow: 1;" expect_width="50" expect_left="50"></div>
          </div>
      "#
    )
}

#[test]
fn fixed_with_margin() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; margin-left: 100px; margin-top: 100px" expect_left="100" expect_top="100" expect_width="32" expect_height="16">hello</div>
        </div>
    "#
    )
}

#[test]
fn fixed_with_specified_left_right() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; left: 100px; right: 100px;" expect_left="100" expect_width="175" expect_height="16">hello</div>
        </div>
    "#
    )
}

#[test]
fn fixed_with_specified_left_right_and_margin() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; left: 25px; margin-left: 25px; right: 50px; height: 50px;" expect_left="100" expect_width="275" expect_height="50" expect_top="0" expect_left="50"></div>
        </div>
    "#
    )
}

#[test]
fn fixed_with_specified_top_bottom() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; top: 100px; bottom: 100px;" expect_top="100" expect_height=550" expect_width="0">hello</div>
        </div>
    "#
    )
}

#[test]
fn fixed_with_specified_width() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; width: 100px; height: 100px; margin-left: 100px; margin-right: 100px;" expect_top="0"  expect_left="100" expect_height=16" expect_width="100"></div>
        </div>
    "#
    )
}

#[test]
fn fixed_with_percentage_top() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; width: 100px; height: 100px; top: 50%" expect_top="375"></div>
        </div>
    "#
    )
}

#[test]
fn fixed_in_flex_container() {
    assert_xml!(
        r#"
        <div>
          <div style="display: flex; flex-direction: column; width: 300px; height: 300px; align-items: center">
            <div style="position: fixed; width: 100px; height: 100px; top: 0px;" expect_top="0"  expect_left="100" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; flex-direction: column; width: 300px; height: 300px; align-items: flex-start">
            <div style="position: fixed; width: 100px; height: 100px; top: 0px;" expect_top="0"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; flex-direction: column; width: 300px; height: 300px; align-items: flex-end">
            <div style="position: fixed; width: 100px; height: 100px; top: 0px;" expect_top="0"  expect_left="200" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; width: 300px; height: 300px; align-items: flex-start">
            <div style="position: fixed; width: 100px; height: 100px;" expect_top="0"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; width: 300px; height: 300px; align-items: flex-end">
            <div style="position: fixed; width: 100px; height: 100px;" expect_top="200"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
          <div style="display: flex; width: 300px; height: 300px; align-items: center">
            <div style="position: fixed; width: 100px; height: 100px;" expect_top="100"  expect_left="0" expect_height="100" expect_width="100"></div>
          </div>
        </div>
       
    "#
    )
}

#[test]
fn fixed_with_specified_top_bottom_left_right() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; top: 0px; bottom: 0px; left: 0px; right: 0px;" expect_top="0" expect_left="0" expect_width="375" expect_height="750">hello</div>
          <div style="position: fixed; top: 100px; bottom: 100px; left: 0px; right: 0px;" expect_top="100" expect_left="0" expect_width="375" expect_height="550">hello</div>
        </div>
    "#
    );
}

#[test]
fn fixed_complex() {
    assert_xml!(
        r#"
        <div>
          <div style="position: fixed; left: 30px; right: 30px; margin-left: 10px; margin-right: 10px;" expect_top="0"  expect_left="40" expect_height=16" expect_width="295"></div>
        </div>
    "#
    )
}

#[test]
fn absolute_flex_align_items() {
    assert_xml!(
        r#"
          <div style="display: flex; box-sizing: border-box; width: 100%;height: 40px; border: 2px solid #07c160; align-items: center;">
            <div style="position: absolute; right: 0; width: 60px; height: 100%; align-items: center;" expect_top="2" expect_height="36"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_in_border_box() {
    assert_xml!(
        r#"
          <div style="box-sizing: border-box; width: 200px; height: 100px; border: 2px solid #07c160;">
            <div style="position: absolute; left: 10px; height: 100%; right: 10px;" expect_top="2" expect_height="96" expect_left="12" expect_width="176" expect_right="12"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_flex_align_items_1() {
    assert_xml!(
        r#"
          <div style="display:flex; align-items:center; padding-top:20px; width:300px; height: 68px">
            <div style="width:113px; height:32px" expect_top="38"></div>
            <div style="position:absolute; left:60px; width:60px; height:48px" expect_left="60" expect_top="30"></div>
          </div>
      "#
    )
}

#[test]
fn absolute_in_inline_1() {
    assert_xml!(
        r#"
          <div style="width: 200px;" expect_width="200" expect_height="0">
            <div style="display: inline" expect_width="0" expect_height="0">
              <div style="position: absolute; width: 100%; height: 20px;" expect_height="20" expect_width="0"></div>
              <div style="position: absolute; width: 100%; height: 100%;" expect_height="0" expect_width="0"></div>
              <div style="position: absolute; width: 30px; height: 100%;" expect_height="0" expect_width="30"></div>
            </div>
          </div>
      "#
    )
}
#[test]
fn absolute_in_inline_2() {
    assert_xml!(
        r#"
          <div style="width: 200px;" expect_width="200" expect_height="100">
            <div style="display: inline-block" expect_width="0" expect_height="0">
              <div style="position: absolute; width: 100%; height: 20px;" expect_height="20" expect_width="0"></div>
              <div style="position: absolute; width: 100%; height: 100%;" expect_height="0" expect_width="0"></div>
              <div style="position: absolute; width: 30px; height: 100%;" expect_height="0" expect_width="30"></div>            
            </div>
            <div style="display: inline-block; width: 100px; height: 100px; " expect_width="100" expect_height="100">
              <div style="position: absolute; width: 100%; height: 20px;" expect_height="20" expect_width="100"></div>
              <div style="position: absolute; width: 100%; height: 100%;" expect_height="100" expect_width="100"></div>
              <div style="position: absolute; width: 30px; height: 100%;" expect_height="100" expect_width="30"></div>            
            </div>
          </div>
      "#
    )
}

#[test]
fn relative_in_inline_1() {
    assert_xml!(
        r#"
          <div style="width: 200px; position: absolute" expect_width="200" expect_height="600">
            <div style="display: inline" expect_width="300" expect_height="300">
              <div style="display: flex; position: relative; width: 300px; height: 300px;" expect_height="300" expect_width="300"></div>
            </div>
            <div style="display: inline" expect_width="300" expect_height="300">
              <div style="display: flex; position: relative; left: 100px; top: 100px; width: 300px; height: 300px;" expect_left="100" expect_top="100" expect_height="300" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}

#[test]
fn inline_as_relative() {
    assert_xml!(
        r#"
          <div style="width: 200px; position: absolute" expect_width="200" expect_height="600">
            <div style="display: inline; position: relative; top: 100px;" expect_width="300" expect_height="300" expect_top="100">
              <div style="display: flex; position: relative; width: 300px; height: 300px;" expect_height="300" expect_width="300"></div>
            </div>
            <div style="display: inline; position: relative; top: 100px;" expect_width="300" expect_height="300" expect_top="400">
              <div style="display: flex; position: relative; left: 100px; top: 100px; width: 300px; height: 300px;" expect_left="100" expect_top="100" expect_height="300" expect_width="300"></div>
            </div>
          </div>
      "#
    )
}
