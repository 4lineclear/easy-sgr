use easy_sgr::Style::*;

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
