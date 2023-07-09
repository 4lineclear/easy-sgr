use easy_sgr::{Color::*, Style::*};

#[test]
fn styles() {
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
        assert_eq!(correct, &format!("{style}"))
    }
}

#[test]
fn standard_colors() {
    for (correct, color) in [
        ("\x1b[30m", BlackFg),
        ("\x1b[31m", RedFg),
        ("\x1b[32m", GreenFg),
        ("\x1b[33m", YellowFg),
        ("\x1b[34m", BlueFg),
        ("\x1b[35m", MagentaFg),
        ("\x1b[36m", CyanFg),
        ("\x1b[37m", WhiteFg),
        ("\x1b[39m", DefaultFg),
        ("\x1b[40m", BlackBg),
        ("\x1b[41m", RedBg),
        ("\x1b[42m", GreenBg),
        ("\x1b[43m", YellowBg),
        ("\x1b[44m", BlueBg),
        ("\x1b[45m", MagentaBg),
        ("\x1b[46m", CyanBg),
        ("\x1b[47m", WhiteBg),
        ("\x1b[49m", DefaultBg),
    ] {
        assert_eq!(correct, &format!("{color}"))
    }
}

#[test]
fn byte_color() {
    for i in (0u8..255).step_by(17) {
        assert_eq!(format!("\x1b[38;2;{i}m"), format!("{}", ByteFg(i)));
        assert_eq!(format!("\x1b[48;2;{i}m"), format!("{}", ByteBg(i)));
    }
}

#[test]
fn rgb_color() {
    for i in (0u8..255).step_by(17) {
        for j in (0u8..255).step_by(17) {
            for k in (0u8..255).step_by(17) {
                assert_eq!(
                    format!("\x1b[38;5;{i};{j};{k}m"),
                    format!("{}", RgbFg(i, j, k))
                );
                assert_eq!(
                    format!("\x1b[48;5;{i};{j};{k}m"),
                    format!("{}", RgbBg(i, j, k))
                );
            }
        }
    }
}
