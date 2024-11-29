use float_pigment_css::{
    sheet::{FontDisplay, FontSrc, FontUrl},
    typing::*,
    MediaQueryStatus, StyleSheet, StyleSheetGroup, StyleSheetResource,
};

mod utils;
use utils::*;

#[cfg(all(feature = "serialize_json", feature = "serialize"))]
use float_pigment_css::compile_style_sheet_to_json;
#[cfg(feature = "serialize")]
fn for_each_serialize_format(s: &str, mut f: impl FnMut(StyleSheetGroup)) {
    {
        let mut ssg = StyleSheetGroup::new();
        let ss = StyleSheet::from_str(s);
        ssg.append(ss);
        f(ssg);
    }
    #[cfg(feature = "serialize_json")]
    {
        let mut ssg = StyleSheetGroup::new();
        let buf = compile_style_sheet_to_json("", s);
        let mut resource = StyleSheetResource::new();
        resource.add_json("", buf);
        ssg.append_from_resource(&resource, "", None);
        f(ssg);
    }
    {
        let mut ssg = StyleSheetGroup::new();
        let buf = float_pigment_css::compile_style_sheet_to_bincode("", s);
        let mut resource = StyleSheetResource::new();
        resource.add_bincode("", buf);
        ssg.append_from_resource(&resource, "", None);
        f(ssg);
    }
    {
        let mut ssg = StyleSheetGroup::new();
        let buf = float_pigment_css::compile_style_sheet_to_bincode("", s);
        let ptr = Box::into_raw(buf.into_boxed_slice());
        let mut resource = StyleSheetResource::new();
        unsafe {
            resource.add_bincode_zero_copy("", ptr, move || {
                drop(Box::from_raw(ptr));
            });
        }
        ssg.append_from_resource(&resource, "", None);
        f(ssg);
    }
}

#[test]
fn media_queries() {
    let ss_str = r#"
        @media (width: 800px) {
            .a {
                width: 2px;
                font-family: "xxx";
            }
            @media (height: 600px) {
                .a {
                    height: 3px;
                }
            }
        }
        .a {
            font-size: 4px;
        }
        @media only unknown {
            .a {
                font-size: 5px;
            }
        }
    "#;
    for_each_serialize_format(ss_str, |ssg| {
        let mut media_query_status = MediaQueryStatus::default_screen();
        {
            let node_properties = query_with_media(&ssg, "", "", ["a"], [], &media_query_status);
            assert_eq!(node_properties.width(), Length::Px(2.));
            assert_eq!(node_properties.height(), Length::Px(3.));
            assert_eq!(node_properties.font_size(), Length::Px(4.));
        }
        media_query_status.width = 801.;
        {
            let node_properties = query_with_media(&ssg, "", "", ["a"], [], &media_query_status);
            assert_eq!(node_properties.width(), Length::Undefined);
            assert_eq!(node_properties.height(), Length::Undefined);
            assert_eq!(node_properties.font_size(), Length::Px(4.));
        }
    });
}
#[test]
fn with_parent() {
    let ss_str = r#"
        #b {
            font-size: 2px;
        }
        .a #b {
            font-size: 3px;
        }
        .a > #b {
            font-size: 4px;
        }
        .a div {
            font-size: 5px;
        }
    "#;
    for_each_serialize_format(ss_str, |ssg| {
        let media_query_status = MediaQueryStatus::<f32>::default_screen();
        {
            let node_properties =
                query_list_with_media(&ssg, [query_item("", "b", [""], [])], &media_query_status);
            assert_eq!(node_properties.font_size(), Length::Px(2.));
        }
        {
            let node_properties = query_list_with_media(
                &ssg,
                [query_item("", "", ["a"], []), query_item("", "b", [""], [])],
                &media_query_status,
            );
            assert_eq!(node_properties.font_size(), Length::Px(4.));
        }
        {
            let node_properties = query_list_with_media(
                &ssg,
                [
                    query_item("", "", ["a"], []),
                    query_item("", "", [""], []),
                    query_item("", "b", [""], []),
                ],
                &media_query_status,
            );
            assert_eq!(node_properties.font_size(), Length::Px(3.));
        }
        {
            let node_properties =
                query_list_with_media(&ssg, [query_item("", "", ["a"], [])], &media_query_status);
            assert_eq!(node_properties.font_size(), Length::Undefined);
        }
    });
}

#[test]
fn with_important() {
    let ss_str = r#"
        .a {
          height: 100px !important;
        }
        .a {
          height: 200px;
        }
    "#;
    for_each_serialize_format(ss_str, |ssg| {
        let media_query_status = MediaQueryStatus::<f32>::default_screen();
        {
            let node_properties =
                query_list_with_media(&ssg, [query_item("", "", ["a"], [])], &media_query_status);
            assert_eq!(node_properties.height(), Length::Px(100.0));
        }
    });
}

#[test]
fn font_face() {
    let ss_str = r#"
        @font-face {
            font-family: "hello/world";
            src: local(sans-serif), url("../path/to/font.svg") format("svg");
            font-weight: 200;
            font-style: normal;
            font-display: optional;
        }
        @font-face {
            font-weight: normal;
            font-style: normal;
            font-family: "weui";
            src: url('data:application/octet-stream;base64,AAEAAAALAIAAAwAwR1NVQrD+s+0AAAE4AAAAQk9TLzJAKEx+AAABfAAAAFZjbWFw65cFHQAAAhwAAAJQZ2x5ZvCRR/EAAASUAAAKtGhlYWQLKIN9AAAA4AAAADZoaGVhCCwD+gAAALwAAAAkaG10eEJo//8AA=') format('truetype');
        }
        .a { height:100px; }
    "#;
    for_each_serialize_format(ss_str, |ssg| {
        let ss = ssg.style_sheet(0);
        if let Some(ss) = ss {
            let ss = ss.sheets();
            let ss = ss.first();
            if let Some(ss) = ss {
                let ss_ref = ss.borrow();
                let ff = ss_ref.font_face().first();
                if let Some(ff) = ff {
                    assert_eq!(ff.font_family, FontFamilyName::Title("hello/world".into()));
                    assert_eq!(
                        ff.src,
                        vec![
                            FontSrc::Local(FontFamilyName::SansSerif),
                            FontSrc::Url(FontUrl {
                                url: "../path/to/font.svg".into(),
                                format: Some(vec!["svg".to_string()])
                            })
                        ]
                    );
                    assert_eq!(ff.font_display, Some(FontDisplay::Optional));
                    assert_eq!(ff.font_weight, Some(FontWeightType::Num(Number::F32(200.))));
                    assert_eq!(ff.font_style, Some(FontStyleType::Normal));
                } else {
                    // empty
                }
                let ff = ss_ref.font_face().get(1);
                if let Some(ff) = ff {
                    assert_eq!(ff.font_family, FontFamilyName::Title("weui".into()));
                    assert_eq!(
                        ff.src,
                        vec![FontSrc::Url(FontUrl {
                            url: "data:application/octet-stream;base64,AAEAAAALAIAAAwAwR1NVQrD+s+0AAAE4AAAAQk9TLzJAKEx+AAABfAAAAFZjbWFw65cFHQAAAhwAAAJQZ2x5ZvCRR/EAAASUAAAKtGhlYWQLKIN9AAAA4AAAADZoaGVhCCwD+gAAALwAAAAkaG10eEJo//8AA=".into(),
                            format: Some(vec!["truetype".to_string()])
                        })]
                    );
                    assert_eq!(ff.font_display, None);
                    assert_eq!(ff.font_weight, Some(FontWeightType::Normal));
                    assert_eq!(ff.font_style, Some(FontStyleType::Normal));
                } else {
                    // empty
                }
            } else {
                // empty
            }
        } else {
            // empty
        }
    });
}
