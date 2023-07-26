#[test]
fn styles() {
    assert_eq!("\x1b[0m", easy_sgr::format!("{+Reset}"));
    assert_eq!("\x1b[1m", easy_sgr::format!("{+Bold}"));
    assert_eq!("\x1b[2m", easy_sgr::format!("{+Dim}"));
    assert_eq!("\x1b[3m", easy_sgr::format!("{+Italic}"));
    assert_eq!("\x1b[4m", easy_sgr::format!("{+Underline}"));
    assert_eq!("\x1b[5m", easy_sgr::format!("{+Blinking}"));
    assert_eq!("\x1b[7m", easy_sgr::format!("{+Inverse}"));
    assert_eq!("\x1b[8m", easy_sgr::format!("{+Hidden}"));
    assert_eq!("\x1b[9m", easy_sgr::format!("{+Strikethrough}"));
    assert_eq!("\x1b[22m", easy_sgr::format!("{-Bold}"));
    assert_eq!("\x1b[22m", easy_sgr::format!("{-Dim}"));
    assert_eq!("\x1b[23m", easy_sgr::format!("{-Italic}"));
    assert_eq!("\x1b[24m", easy_sgr::format!("{-Underline}"));
    assert_eq!("\x1b[25m", easy_sgr::format!("{-Blinking}"));
    assert_eq!("\x1b[27m", easy_sgr::format!("{-Inverse}"));
    assert_eq!("\x1b[28m", easy_sgr::format!("{-Hidden}"));
    assert_eq!("\x1b[29m", easy_sgr::format!("{-Strikethrough}"));
}
#[test]
fn standard_colors() {
    assert_eq!("\x1b[30m", easy_sgr::format!("{#BlackFg}"));
    assert_eq!("\x1b[31m", easy_sgr::format!("{#RedFg}"));
    assert_eq!("\x1b[32m", easy_sgr::format!("{#GreenFg}"));
    assert_eq!("\x1b[33m", easy_sgr::format!("{#YellowFg}"));
    assert_eq!("\x1b[34m", easy_sgr::format!("{#BlueFg}"));
    assert_eq!("\x1b[35m", easy_sgr::format!("{#MagentaFg}"));
    assert_eq!("\x1b[36m", easy_sgr::format!("{#CyanFg}"));
    assert_eq!("\x1b[37m", easy_sgr::format!("{#WhiteFg}"));
    assert_eq!("\x1b[39m", easy_sgr::format!("{#DefaultFg}"));
    assert_eq!("\x1b[40m", easy_sgr::format!("{#BlackBg}"));
    assert_eq!("\x1b[41m", easy_sgr::format!("{#RedBg}"));
    assert_eq!("\x1b[42m", easy_sgr::format!("{#GreenBg}"));
    assert_eq!("\x1b[43m", easy_sgr::format!("{#YellowBg}"));
    assert_eq!("\x1b[44m", easy_sgr::format!("{#BlueBg}"));
    assert_eq!("\x1b[45m", easy_sgr::format!("{#MagentaBg}"));
    assert_eq!("\x1b[46m", easy_sgr::format!("{#CyanBg}"));
    assert_eq!("\x1b[47m", easy_sgr::format!("{#WhiteBg}"));
    assert_eq!("\x1b[49m", easy_sgr::format!("{#DefaultBg}"));
}
