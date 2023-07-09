use std::default::Default;

use easy_sgr::{CleanKind, Color::*, ColorKind, EasySGR, SGRString, Style::*, StyleKind};

#[test]
fn general() {
    assert_eq!("", SGRString::default().to_string());
    assert_eq!(
        "test\x1b[0m",
        String::from("test").clean(CleanKind::Reset).to_string()
    );
    assert_eq!(
        "",
        (&String::default()).clean(CleanKind::Reverse).to_string()
    );
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

#[test]
fn easy_sgr() {
    assert_eq!("test", "test".to_sgr().to_string());
    assert_eq!("test", "".text("test").to_string());
    assert_eq!("\x1b[100mtest", "test".custom(100).to_string());
    assert_eq!("\x1b[100mtest", "test".custom_place(100).to_string());
    assert_eq!("test\x1b[100m", "test".custom_clean(100).to_string());

    assert_eq!("\x1b[31;3m", Italic.color(RedFg).to_string());
    assert_eq!("\x1b[31;3m", RedFg.style(Italic).to_string());
}

#[test]
fn easy_sgr_style() {
    for (correct, style) in [
        ("\x1b[0m", Reset),
        ("\x1b[1m", Bold),
        ("\x1b[2m", Dim),
        ("\x1b[3m", Italic),
        ("\x1b[4m", Underline),
        ("\x1b[5m", Blinking),
        ("\x1b[7m", Inverse),
        ("\x1b[8m", Hidden),
        ("\x1b[9m", Strikethrough),
        ("\x1b[22m", NotBold),
        ("\x1b[22m", NotDim),
        ("\x1b[23m", NotItalic),
        ("\x1b[24m", NotUnderline),
        ("\x1b[25m", NotBlinking),
        ("\x1b[27m", NotInverse),
        ("\x1b[28m", NotHidden),
        ("\x1b[29m", NotStrikethrough),
    ] {
        assert_eq!(correct, "".style(style).to_string());
    }
}

#[test]
fn easy_sgr_color() {
    for (correct, color) in [
        ("\x1b[30m", BlackFg),
        ("\x1b[31m", RedFg),
        ("\x1b[32m", GreenFg),
        ("\x1b[33m", YellowFg),
        ("\x1b[34m", BlueFg),
        ("\x1b[35m", MagentaFg),
        ("\x1b[36m", CyanFg),
        ("\x1b[37m", WhiteFg),
        ("\x1b[38;2;208m", ByteFg(208)),
        ("\x1b[38;5;208;208;208m", RgbFg(208, 208, 208)),
        ("\x1b[39m", DefaultFg),
        ("\x1b[40m", BlackBg),
        ("\x1b[41m", RedBg),
        ("\x1b[42m", GreenBg),
        ("\x1b[43m", YellowBg),
        ("\x1b[44m", BlueBg),
        ("\x1b[45m", MagentaBg),
        ("\x1b[46m", CyanBg),
        ("\x1b[47m", WhiteBg),
        ("\x1b[48;2;208m", ByteBg(208)),
        ("\x1b[48;5;208;208;208m", RgbBg(208, 208, 208)),
        ("\x1b[49m", DefaultBg),
    ] {
        assert_eq!(correct, "".color(color).to_string())
    }
}
