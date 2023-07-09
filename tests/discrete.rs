use easy_sgr::{Color::*, Style::*};

#[test]
fn styles() {
    assert_eq!("\x1b[0m", &format!("{Reset}"));
    assert_eq!("\x1b[1m", &format!("{Bold}"));
    assert_eq!("\x1b[2m", &format!("{Dim}"));
    assert_eq!("\x1b[3m", &format!("{Italic}"));
    assert_eq!("\x1b[4m", &format!("{Underline}"));
    assert_eq!("\x1b[5m", &format!("{Blinking}"));
    assert_eq!("\x1b[7m", &format!("{Inverse}"));
    assert_eq!("\x1b[8m", &format!("{Hidden}"));
    assert_eq!("\x1b[9m", &format!("{Strikethrough}"));
    assert_eq!("\x1b[22m", &format!("{NotBold}"));
    assert_eq!("\x1b[22m", &format!("{NotDim}"));
    assert_eq!("\x1b[23m", &format!("{NotItalic}"));
    assert_eq!("\x1b[24m", &format!("{NotUnderline}"));
    assert_eq!("\x1b[25m", &format!("{NotBlinking}"));
    assert_eq!("\x1b[27m", &format!("{NotInverse}"));
    assert_eq!("\x1b[28m", &format!("{NotHidden}"));
    assert_eq!("\x1b[29m", &format!("{NotStrikethrough}"));
}

#[test]
fn standard_colors() {
    assert_eq!("\x1b[30m", format!("{BlackFg}"));
    assert_eq!("\x1b[31m", format!("{RedFg}"));
    assert_eq!("\x1b[32m", format!("{GreenFg}"));
    assert_eq!("\x1b[33m", format!("{YellowFg}"));
    assert_eq!("\x1b[34m", format!("{BlueFg}"));
    assert_eq!("\x1b[35m", format!("{MagentaFg}"));
    assert_eq!("\x1b[36m", format!("{CyanFg}"));
    assert_eq!("\x1b[37m", format!("{WhiteFg}"));
    assert_eq!("\x1b[39m", format!("{DefaultFg}"));

    assert_eq!("\x1b[40m", format!("{BlackBg}"));
    assert_eq!("\x1b[41m", format!("{RedBg}"));
    assert_eq!("\x1b[42m", format!("{GreenBg}"));
    assert_eq!("\x1b[43m", format!("{YellowBg}"));
    assert_eq!("\x1b[44m", format!("{BlueBg}"));
    assert_eq!("\x1b[45m", format!("{MagentaBg}"));
    assert_eq!("\x1b[46m", format!("{CyanBg}"));
    assert_eq!("\x1b[47m", format!("{WhiteBg}"));
    assert_eq!("\x1b[49m", format!("{DefaultBg}"));
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
