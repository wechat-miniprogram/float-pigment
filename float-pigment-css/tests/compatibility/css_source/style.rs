use float_pigment_css::{typing::*, StyleSheetGroup};

use crate::utils::query;

pub fn style_css_assert(ssg: StyleSheetGroup) {
    let np = query(&ssg, "wx-button", "", [""], []);
    assert_eq!(np.position(), Position::Relative);
    assert_eq!(np.margin_left(), Length::Auto);
    assert_eq!(np.margin_right(), Length::Auto);
    assert_eq!(np.padding_left(), Length::Px(14.));
    assert_eq!(np.padding_right(), Length::Px(14.));
    assert_eq!(np.font_size(), Length::Px(18.));
    assert_eq!(np.text_align(), TextAlign::Center);
    assert_eq!(np.flex_direction(), FlexDirection::Row);
    assert_eq!(np.justify_content(), JustifyContent::Center);
    assert_eq!(np.line_height(), LineHeight::Num(Number::F32(2.555_555_6)));
    assert_eq!(
        np.border_top_left_radius(),
        BorderRadius::Pos(Length::Px(5.), Length::Px(5.))
    );
    assert_eq!(np.border_left_width(), Length::Px(0.5));
    assert_eq!(np.border_bottom_width(), Length::Px(0.5));
    assert_eq!(np.border_right_style(), BorderStyle::Solid);
    assert_eq!(np.border_top_style(), BorderStyle::Solid);
    assert_eq!(np.border_left_color(), Color::Specified(0, 0, 0, 51));
    assert_eq!(np.border_bottom_color(), Color::Specified(0, 0, 0, 51));
    assert_eq!(np.overflow_x(), Overflow::Hidden);
    assert_eq!(np.overflow_y(), Overflow::Hidden);
    assert_eq!(np.color(), Color::Specified(0, 0, 0, 255));

    // TODO
}
