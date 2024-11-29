use float_pigment_css::{typing::*, StyleSheetGroup};

use crate::utils::query;

pub fn padding_case_assert(ssg: StyleSheetGroup) {
    let np = query(&ssg, "", "", ["intro"], []);
    println!("{:?}", ssg.style_sheet(0));
    assert_eq!(np.padding_left(), Length::Px(24.));
    assert_eq!(np.padding_right(), Length::Px(24.));
    assert_eq!(np.padding_top(), Length::Px(0.));
    assert_eq!(np.padding_bottom(), Length::Px(0.));
    assert_eq!(np.margin_left(), Length::Px(50.));
    assert_eq!(np.margin_right(), Length::Px(50.));
    assert_eq!(np.margin_top(), Length::Px(50.));
    assert_eq!(np.margin_bottom(), Length::Px(50.));
}
