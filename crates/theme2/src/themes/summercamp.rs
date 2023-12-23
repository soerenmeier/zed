// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn summercamp() -> UserThemeFamily {
    UserThemeFamily {
        name: "Summercamp".into(),
        author: "Zed Industries".into(),
        themes: vec![UserTheme {
            name: "Summercamp".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border: Some(rgba(0x29251bff).into()),
                    border_variant: Some(rgba(0x221e15ff).into()),
                    elevated_surface_background: Some(rgba(0x2a261cff).into()),
                    background: Some(rgba(0x2a261cff).into()),
                    panel_background: Some(rgba(0x231f16ff).into()),
                    element_hover: Some(rgba(0x312d2180).into()),
                    element_selected: Some(rgba(0x39342780).into()),
                    text: Some(rgba(0xf8f5deff).into()),
                    text_muted: Some(rgba(0x3d382aff).into()),
                    text_placeholder: Some(rgba(0x3d382aff).into()),
                    text_disabled: Some(rgba(0xf8f5deff).into()),
                    text_accent: Some(rgba(0x499befff).into()),
                    status_bar_background: Some(rgba(0x2a261cff).into()),
                    title_bar_background: Some(rgba(0x2a261cff).into()),
                    toolbar_background: Some(rgba(0x1c1810ff).into()),
                    tab_bar_background: Some(rgba(0x231f16ff).into()),
                    tab_inactive_background: Some(rgba(0x231f16ff).into()),
                    tab_active_background: Some(rgba(0x1c1810ff).into()),
                    scrollbar_thumb_background: Some(rgba(0xf8f5de4d).into()),
                    scrollbar_thumb_hover_background: Some(rgba(0xf8f5de4d).into()),
                    scrollbar_thumb_border: Some(rgba(0x221e15ff).into()),
                    scrollbar_track_border: Some(rgba(0x221e15ff).into()),
                    editor_foreground: Some(rgba(0xf8f5deff).into()),
                    editor_background: Some(rgba(0x1c1810ff).into()),
                    editor_gutter_background: Some(rgba(0x1c1810ff).into()),
                    editor_line_number: Some(rgba(0xf8f5de59).into()),
                    editor_active_line_number: Some(rgba(0xf8f5deff).into()),
                    editor_wrap_guide: Some(rgba(0xf8f5de0d).into()),
                    editor_active_wrap_guide: Some(rgba(0xf8f5de1a).into()),
                    terminal_background: Some(rgba(0x1c1810ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x3b3627ff).into()),
                    terminal_ansi_bright_red: Some(rgba(0x7f2724ff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x28842cff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0x8c9a10ff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x234b7fff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0x88487eff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x298462ff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xf8f5deff).into()),
                    terminal_ansi_black: Some(rgba(0x1c1810ff).into()),
                    terminal_ansi_red: Some(rgba(0xe35142ff).into()),
                    terminal_ansi_green: Some(rgba(0x5dea5aff).into()),
                    terminal_ansi_yellow: Some(rgba(0xf1fe29ff).into()),
                    terminal_ansi_blue: Some(rgba(0x499befff).into()),
                    terminal_ansi_magenta: Some(rgba(0xf59be6ff).into()),
                    terminal_ansi_cyan: Some(rgba(0x5beabcff).into()),
                    terminal_ansi_white: Some(rgba(0xf8f5deff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    created: Some(rgba(0x5dea5aff).into()),
                    deleted: Some(rgba(0xb93f36ff).into()),
                    error: Some(rgba(0xe35142ff).into()),
                    modified: Some(rgba(0xf1fe29ff).into()),
                    success: Some(rgba(0xf8f5deff).into()),
                    warning: Some(rgba(0xf1fe29ff).into()),
                    ..Default::default()
                },
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        (
                            "attribute".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "boolean".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x777259ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment.doc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x777259ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constructor".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "embedded".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis.strong".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "enum".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "function".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf1fe29ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "hint".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x246e61ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "keyword".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "label".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_text".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_uri".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "number".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5dea5aff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "operator".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "predictive".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x79434bff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "preproc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "primary".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "property".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.bracket".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.delimiter".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.list_marker".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xbfbb9bff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.escape".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x777259ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.regex".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special.symbol".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "tag".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "text.literal".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfaa11dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "title".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "type".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x5beabcff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf8f5deff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x499befff).into()),
                                ..Default::default()
                            },
                        ),
                    ],
                }),
            },
        }],
    }
}
