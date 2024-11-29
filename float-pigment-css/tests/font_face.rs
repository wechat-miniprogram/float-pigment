use std::rc::Rc;

use float_pigment_css::{
    sheet::{FontDisplay, FontFace, FontSrc, FontUrl},
    typing::*,
    StyleSheet,
};

#[test]
fn font_face() {
    let s = r#"
      @font-face {
        font-family: sans-serif;
        src: local(sans-serif), url("../path/to/font.svg") format("svg"), url(https://sss.vss);
        font-weight: 200;
        font-style: normal;
        font-display: optional;
        hello: 123;
      }
    "#;
    let ss = StyleSheet::from_str_with_path("/absolute/mod/hello.css", s);
    let ssv = ss.sheets();
    let ss = ssv.first().unwrap().borrow();
    let ff = ss.font_face().first().unwrap();
    let mut font_face = FontFace::new();
    font_face
        .with_font_family(FontFamilyName::SansSerif)
        .with_src(vec![
            FontSrc::Local(FontFamilyName::SansSerif),
            FontSrc::Url(FontUrl {
                url: "absolute/path/to/font.svg".into(),
                format: Some(vec!["svg".into()]),
            }),
            FontSrc::Url(FontUrl {
                url: "https://sss.vss".into(),
                format: None,
            }),
        ])
        .with_font_style(Some(FontStyleType::Normal))
        .with_font_weight(Some(FontWeightType::Num(Number::F32(200.))))
        .with_font_display(Some(FontDisplay::Optional));
    assert_eq!(&Rc::new(font_face), ff);
}

#[test]
fn font_face_1() {
    let s = r#"
    @font-face {
        font-family: 'iconfont6';
        src: url('https://m.elongstatic.com/hotel/h5/wechat-xcx/20180913zhllx/iconfont/iconfont6.ttf?t=20181010') format('truetype'), url('https://m.elongstatic.com/hotel/h5/wechat-xcx/20180913zhllx/iconfont/iconfont6.woff?t=20181010') format('woff');
    }
    .iconfont {
        /*  font-size: 14px;*/
        font-style: normal;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }
    @font-face {
        font-family: 'iconfont3';
        src: url('https://m.elongstatic.com/hotel/h5/wechat-xcx/20220811/iconfont/iconfont3.ttf') format('truetype'),
        url('https://m.elongstatic.com/hotel/h5/wechat-xcx/20220811/iconfont/iconfont3.ttf') format('woff');
    }"#;
    let ss = StyleSheet::from_str_with_path("/absolute/mod/hello.css", s);
    let ssv = ss.sheets();
    let ss = ssv.first().unwrap().borrow();
    let _ff = ss.font_face().first().unwrap();
}

#[test]
fn font_face_url_without_schema() {
    let s = r#"
    @font-face {
        font-family: 'iconfont6';
        src: url("//qq.com");
    }
    "#;

    let ss = StyleSheet::from_str_with_path("/absolute/mod/hello.css", s);
    let ssv = ss.sheets();
    let ss = ssv.first().unwrap().borrow();
    let ff = ss.font_face().first().unwrap();
    assert_eq!(
        ff.src.first(),
        Some(&FontSrc::Url(FontUrl {
            url: "//qq.com".into(),
            format: None,
        }))
    )
}
