use super::*;


    // 0xa5 0xa6
    #[test]
    fn grid_template() {
        test_parse_property!(
            grid_template_rows,
            "grid-template-rows",
            "100px 1fr 200px",
            GridTemplate::TrackList(
                vec![
                    TrackListItem::TrackSize(TrackSize::Length(Length::Px(100.))),
                    TrackListItem::TrackSize(TrackSize::Fr(1.)),
                    TrackListItem::TrackSize(TrackSize::Length(Length::Px(200.)))
                ]
                .into()
            )
        );
        test_parse_property!(
            grid_template_rows,
            "grid-template-rows",
            "[hello] 1fr [world] 200px [foo bar]",
            GridTemplate::TrackList(
                vec![
                    TrackListItem::LineNames(vec!["hello".to_string().into()].into()),
                    TrackListItem::TrackSize(TrackSize::Fr(1.)),
                    TrackListItem::LineNames(vec!["world".to_string().into()].into()),
                    TrackListItem::TrackSize(TrackSize::Length(Length::Px(200.))),
                    TrackListItem::LineNames(
                        vec!["foo".to_string().into(), "bar".to_string().into()].into()
                    ),
                ]
                .into()
            )
        );
        test_parse_property!(
            grid_template_rows,
            "grid-template-rows",
            "auto auto auto",
            GridTemplate::TrackList(
                vec![
                    TrackListItem::TrackSize(TrackSize::Length(Length::Auto)),
                    TrackListItem::TrackSize(TrackSize::Length(Length::Auto)),
                    TrackListItem::TrackSize(TrackSize::Length(Length::Auto)),
                ]
                .into()
            )
        );
        test_parse_property!(
            grid_template_columns,
            "grid-template-columns",
            "auto auto",
            GridTemplate::TrackList(
                vec![
                    TrackListItem::TrackSize(TrackSize::Length(Length::Auto)),
                    TrackListItem::TrackSize(TrackSize::Length(Length::Auto)),
                ]
                .into()
            )
        );
    }

    // 0xa7
    #[test]
    fn grid_auto_flow() {
        test_parse_property!(grid_auto_flow, "grid-auto-flow", "row", GridAutoFlow::Row);
        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "column",
            GridAutoFlow::Column
        );
        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "row dense",
            GridAutoFlow::RowDense
        );
        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "column dense",
            GridAutoFlow::ColumnDense
        );
        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "dense",
            GridAutoFlow::RowDense
        );

        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "dense row",
            GridAutoFlow::RowDense
        );
        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "dense column",
            GridAutoFlow::ColumnDense
        );
        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "row row",
            GridAutoFlow::Row
        );
        test_parse_property!(
            grid_auto_flow,
            "grid-auto-flow",
            "column column",
            GridAutoFlow::Row
        );
    }

    // 0xa9 0xaa
    #[test]
    fn grid_auto_rows_columns() {
        // grid-auto-rows: illegal
        test_parse_property!(
            grid_auto_rows,
            "grid-auto-rows",
            "illegal",
            GridAuto::List(vec![TrackSize::Length(Length::Auto)].into())
        );
        // grid-auto-rows: single track size
        test_parse_property!(
            grid_auto_rows,
            "grid-auto-rows",
            "100px",
            GridAuto::List(vec![TrackSize::Length(Length::Px(100.))].into())
        );
        // grid-auto-rows: multiple track sizes
        test_parse_property!(
            grid_auto_rows,
            "grid-auto-rows",
            "100px 1fr",
            GridAuto::List(vec![TrackSize::Length(Length::Px(100.)), TrackSize::Fr(1.)].into())
        );
        // grid-auto-rows: auto
        test_parse_property!(
            grid_auto_rows,
            "grid-auto-rows",
            "auto",
            GridAuto::List(vec![TrackSize::Length(Length::Auto)].into())
        );
        // grid-auto-rows: multiple with auto
        test_parse_property!(
            grid_auto_rows,
            "grid-auto-rows",
            "auto 100px auto",
            GridAuto::List(
                vec![
                    TrackSize::Length(Length::Auto),
                    TrackSize::Length(Length::Px(100.)),
                    TrackSize::Length(Length::Auto)
                ]
                .into()
            )
        );
        // grid-auto-rows: min-content / max-content
        test_parse_property!(
            grid_auto_rows,
            "grid-auto-rows",
            "min-content",
            GridAuto::List(vec![TrackSize::MinContent].into())
        );
        test_parse_property!(
            grid_auto_rows,
            "grid-auto-rows",
            "max-content",
            GridAuto::List(vec![TrackSize::MaxContent].into())
        );
        // grid-auto-columns: illegal
        test_parse_property!(
            grid_auto_columns,
            "grid-auto-columns",
            "illegal",
            GridAuto::List(vec![TrackSize::Length(Length::Auto)].into())
        );
        // grid-auto-columns: single track size
        test_parse_property!(
            grid_auto_columns,
            "grid-auto-columns",
            "200px",
            GridAuto::List(vec![TrackSize::Length(Length::Px(200.))].into())
        );
        // grid-auto-columns: multiple track sizes
        test_parse_property!(
            grid_auto_columns,
            "grid-auto-columns",
            "1fr 2fr",
            GridAuto::List(vec![TrackSize::Fr(1.), TrackSize::Fr(2.)].into())
        );
        // grid-auto-columns: percentage
        test_parse_property!(
            grid_auto_columns,
            "grid-auto-columns",
            "50%",
            GridAuto::List(vec![TrackSize::Length(Length::Ratio(0.5))].into())
        );
    }
