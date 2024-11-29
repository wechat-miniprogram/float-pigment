use float_pigment_css::{typing::*, StyleSheetGroup};

use crate::utils::query;

pub fn base_css_assert(ssg: StyleSheetGroup) {
    // assert
    let np = query(&ssg, "", "", ["a"], []);
    assert_eq!(np.height(), Length::Px(100.));
    assert_eq!(np.padding_left(), Length::Px(10.));
    assert_eq!(np.aspect_ratio(), AspectRatio::Auto);
    assert_eq!(np.margin_left(), Length::Px(10.));
    assert_eq!(np.margin_top(), Length::Px(10.));
    assert_eq!(np.margin_right(), Length::Px(10.));
    assert_eq!(np.margin_bottom(), Length::Px(10.));

    let np = query(&ssg, "", "", ["b"], []);
    assert_eq!(np.width(), Length::Px(100.));
    assert_eq!(np.border_left_width(), Length::Px(1.));
    assert_eq!(np.border_top_width(), Length::Px(1.));
    assert_eq!(np.border_right_width(), Length::Px(1.));
    assert_eq!(np.border_bottom_width(), Length::Px(1.));
    assert_eq!(np.border_left_color(), Color::Specified(0, 0, 0, 255));
    assert_eq!(np.border_top_color(), Color::Specified(0, 0, 0, 255));
    assert_eq!(np.border_right_color(), Color::Specified(0, 0, 0, 255));
    assert_eq!(np.border_bottom_color(), Color::Specified(0, 0, 0, 255));
    assert_eq!(np.border_left_style(), BorderStyle::Solid);
    assert_eq!(np.border_top_style(), BorderStyle::Solid);
    assert_eq!(np.border_right_style(), BorderStyle::Solid);
    assert_eq!(np.border_bottom_style(), BorderStyle::Solid);
    assert_eq!(
        np.background_position_x(),
        BackgroundPosition::List(
            vec![BackgroundPositionItem::Value(
                BackgroundPositionValue::Left(Length::Px(10.))
            )]
            .into()
        )
    )
}
