use std::default::Default;

use easy_sgr::{CleanKind, ColorKind, SGRString, StyleKind};

#[test]
fn skips() {
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
fn fully_loaded() {
    assert_eq!(
        "\x1b[0;31;41;1;2;3;4;5;7;8;9;100mtest\x1b[39;49;22;22;23;24;25;27;28;29;100m",
        SGRString {
            text: "test".to_string(),
            clean: CleanKind::Reverse,
            custom_places: vec![100],
            custom_cleans: vec![100],
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
    assert_eq!(
        "\x1b[0;31;41;22;22;23;24;25;27;28;29;100mtest\x1b[39;49;1;2;3;4;5;7;8;9;100m",
        SGRString {
            text: "test".to_string(),
            clean: CleanKind::Reverse,
            custom_places: vec![100],
            custom_cleans: vec![100],
            foreground: ColorKind::Red,
            background: ColorKind::Red,
            reset: true,
            bold: StyleKind::Clean,
            dim: StyleKind::Clean,
            italic: StyleKind::Clean,
            underline: StyleKind::Clean,
            blinking: StyleKind::Clean,
            inverse: StyleKind::Clean,
            hidden: StyleKind::Clean,
            strikethrough: StyleKind::Clean
        }
        .to_string()
    );
}

#[test]
fn colors() {
    for (correct, color) in [
        ("", ColorKind::None),
        ("\x1b[30;40m", ColorKind::Black),
        ("\x1b[31;41m", ColorKind::Red),
        ("\x1b[32;42m", ColorKind::Green),
        ("\x1b[33;43m", ColorKind::Yellow),
        ("\x1b[34;44m", ColorKind::Blue),
        ("\x1b[35;45m", ColorKind::Magenta),
        ("\x1b[36;46m", ColorKind::Cyan),
        ("\x1b[37;47m", ColorKind::White),
        ("\x1b[38;2;208;48;2;208m", ColorKind::Byte(208)),
        (
            "\x1b[38;5;208;208;208;48;5;208;208;208m",
            ColorKind::Rgb(208, 208, 208),
        ),
        ("\x1b[39;49m", ColorKind::Default),
    ] {
        assert_eq!(
            correct,
            SGRString {
                foreground: color.clone(),
                background: color,
                ..Default::default()
            }
            .to_string()
        )
    }
}
