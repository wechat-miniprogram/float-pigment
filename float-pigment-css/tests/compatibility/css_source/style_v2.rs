use float_pigment_css::{typing::*, StyleSheetGroup};

use crate::utils::query;

pub fn style_v2_css_assert(ssg: StyleSheetGroup) {
    let np = query(&ssg, "wx-button", "", [""], []);
    assert_eq!(np.padding_top(), Length::Px(8.));
    assert_eq!(np.padding_right(), Length::Px(24.));
    assert_eq!(
        np.border_top_left_radius(),
        BorderRadius::Pos(Length::Px(4.), Length::Px(4.))
    );
    assert_eq!(np.font_size(), Length::Px(17.));
    assert_eq!(np.font_weight(), FontWeight::Num(Number::F32(700.)));

    // TODO
}
