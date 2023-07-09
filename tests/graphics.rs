use std::default::Default;

use easy_sgr::{CleanKind, ColorKind, SGRString, StyleKind};

#[test]
fn skip_escapes() {
    assert_eq!("", SGRString::default().to_string());
    assert_eq!("test", SGRString::from("test").to_string());
    assert_eq!(
        "\x1b[31;1m",
        SGRString {
            bold: StyleKind::Place,
            foreground: ColorKind::Red,
            ..Default::default()
        }
        .to_string()
    );
    assert_eq!(
        "\x1b[31;1mtest",
        SGRString {
            text: String::from("test"),
            bold: StyleKind::Place,
            foreground: ColorKind::Red,
            ..Default::default()
        }
        .to_string()
    );
}
#[test]
fn all_place() {
    assert_eq!(
        "\x1b[0;31;41;1;2;3;4;5;7;8;9m\x1b[39;49;22;22;23;24;25;27;28;29m",
        SGRString {
            text: String::new(),
            clean: CleanKind::Reverse,
            custom_places: Vec::new(),
            custom_cleans: Vec::new(),
            foreground: ColorKind::Red,
            background: ColorKind::Red,
            reset: true,
            bold: StyleKind::Place,
            dim: StyleKind::Place,
            italic: StyleKind::Place,
            underline: StyleKind::Place,
            blinking: StyleKind::Place,
            inverse: StyleKind::Place,
            hidden: StyleKind::Place,
            strikethrough: StyleKind::Place
        }
        .to_string()
    );
}
