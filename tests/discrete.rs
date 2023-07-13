use easy_sgr::Seq;

#[test]
fn seq() {
    assert_eq!("\x1b[", Seq::Esc.to_string());
    assert_eq!("m", Seq::End.to_string());
}

#[cfg(not(feature = "partial"))]
mod normal {
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
            assert_eq!(correct, format!("{style}"))
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
            assert_eq!(correct, format!("{color}"))
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
}

mod from_str {

    use easy_sgr::{Color::*, Seq::*, Style::*};

    #[test]
    fn seq() {
        assert_eq!(Ok(End), "End".parse());
        assert_eq!(Ok(Esc), "Esc".parse());
    }
    #[test]
    fn styles() {
        for (src, style) in [
            ("Reset", Reset),
            ("Bold", Bold),
            ("Dim", Dim),
            ("Italic", Italic),
            ("Underline", Underline),
            ("Blinking", Blinking),
            ("Inverse", Inverse),
            ("Hidden", Hidden),
            ("Strikethrough", Strikethrough),
            ("NotBold", NotBold),
            ("NotDim", NotDim),
            ("NotItalic", NotItalic),
            ("NotUnderline", NotUnderline),
            ("NotBlinking", NotBlinking),
            ("NotInverse", NotInverse),
            ("NotHidden", NotHidden),
            ("NotStrikethrough", NotStrikethrough),
        ] {
            assert_eq!(Ok(style), src.parse())
        }
    }
    #[test]
    fn standard_colors() {
        for (src, color) in [
            ("BlackFg", BlackFg),
            ("RedFg", RedFg),
            ("GreenFg", GreenFg),
            ("YellowFg", YellowFg),
            ("BlueFg", BlueFg),
            ("MagentaFg", MagentaFg),
            ("CyanFg", CyanFg),
            ("WhiteFg", WhiteFg),
            ("DefaultFg", DefaultFg),
            ("BlackBg", BlackBg),
            ("RedBg", RedBg),
            ("GreenBg", GreenBg),
            ("YellowBg", YellowBg),
            ("BlueBg", BlueBg),
            ("MagentaBg", MagentaBg),
            ("CyanBg", CyanBg),
            ("WhiteBg", WhiteBg),
            ("DefaultBg", DefaultBg),
        ] {
            assert_eq!(Ok(color), src.parse())
        }
    }
}

#[cfg(feature = "partial")]
mod partial {
    use easy_sgr::{Color::*, Style::*};
    #[test]
    fn styles() {
        for (correct, style) in [
            ("0", Reset),
            ("1", Bold),
            ("2", Dim),
            ("3", Italic),
            ("4", Underline),
            ("5", Blinking),
            ("7", Inverse),
            ("8", Hidden),
            ("9", Strikethrough),
            ("22", NotBold),
            ("22", NotDim),
            ("23", NotItalic),
            ("24", NotUnderline),
            ("25", NotBlinking),
            ("27", NotInverse),
            ("28", NotHidden),
            ("29", NotStrikethrough),
        ] {
            assert_eq!(correct, format!("{style}"))
        }
    }

    #[test]
    fn standard_colors() {
        for (correct, color) in [
            ("30", BlackFg),
            ("31", RedFg),
            ("32", GreenFg),
            ("33", YellowFg),
            ("34", BlueFg),
            ("35", MagentaFg),
            ("36", CyanFg),
            ("37", WhiteFg),
            ("39", DefaultFg),
            ("40", BlackBg),
            ("41", RedBg),
            ("42", GreenBg),
            ("43", YellowBg),
            ("44", BlueBg),
            ("45", MagentaBg),
            ("46", CyanBg),
            ("47", WhiteBg),
            ("49", DefaultBg),
        ] {
            assert_eq!(correct, format!("{color}"))
        }
    }

    #[test]
    fn byte_color() {
        for i in (0u8..255).step_by(17) {
            assert_eq!(format!("38;2;{i}"), format!("{}", ByteFg(i)));
            assert_eq!(format!("48;2;{i}"), format!("{}", ByteBg(i)));
        }
    }

    #[test]
    fn rgb_color() {
        for i in (0u8..255).step_by(17) {
            for j in (0u8..255).step_by(17) {
                for k in (0u8..255).step_by(17) {
                    assert_eq!(format!("38;5;{i};{j};{k}"), format!("{}", RgbFg(i, j, k)));
                    assert_eq!(format!("48;5;{i};{j};{k}"), format!("{}", RgbBg(i, j, k)));
                }
            }
        }
    }
}
