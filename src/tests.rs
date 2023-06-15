use crate::{
    graphics::ColorKind::*,
    inline::{Color::*, Style::*},
    AnsiString,
    ClearKind::*,
};

const TEXT: &str = "this is sample text";
const COMPLEX_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

#[test]
fn skip_all() {
    let all_empty = AnsiString::default();
    let just_text = AnsiString::from(TEXT);
    let empty_with_custom = AnsiString::default().custom_clear(2).custom_clear(3);
    let text_with_custom = AnsiString::from(TEXT).custom_clear(2).custom_clear(3);

    assert_eq!(all_empty.to_string(), "");
    assert_eq!(just_text.to_string(), TEXT);
    assert_eq!(empty_with_custom.to_string(), "");
    assert_eq!(text_with_custom.to_string(), TEXT);
}

#[test]
fn skip_places() {
    let just_custom = AnsiString::default()
        .custom_clear(2)
        .custom_clear(3)
        .set_clear(Clean);

    let just_text = AnsiString::from(TEXT);

    let text_with_custom_clears = just_custom.clone().text(TEXT);

    assert_eq!(just_custom.to_string(), "\x1b[2;3m");
    assert_eq!(just_text.to_string(), TEXT);
    assert_eq!(
        text_with_custom_clears.to_string(),
        format!("{TEXT}\x1b[2;3m")
    );
}

#[test]
fn skip_clears() {
    let just_custom = AnsiString::default()
        .custom_place(2)
        .custom_place(3)
        .set_clear(Clean);
    let with_all = AnsiString::from(TEXT)
        .foreground(Red)
        .background(Black)
        .style(Bold)
        .style(Italic)
        .custom_place(2)
        .custom_place(3);

    assert_eq!(just_custom.to_string(), "\x1b[2;3m");
    assert_eq!(with_all.to_string(), format!("\x1b[31;40;1;3;2;3m{TEXT}"));
}

#[test]
fn ansi_string() {
    let lots_of_stuff = AnsiString::from(COMPLEX_TEXT)
        .foreground(Red)
        .background(Black)
        .style(Bold)
        .style(Italic)
        .custom_place(9)
        .custom_clear(29)
        .set_clear(Clean);

    assert_eq!(
        lots_of_stuff.to_string(),
        format!("\x1b[31;40;1;3;9m{COMPLEX_TEXT}\x1b[39;49;22;23;29m")
    );
}

#[test]
fn inline() {
    //TODO Add feature to reduce \x1b[...m
    let lots_of_stuff = format!(
        "{FRed}{BBlack}{Bold}{Italic}\x1b[9m{COMPLEX_TEXT}{FDefault}{BDefault}{ClearBold}{ClearItalic}\x1b[29m"
    );

    assert_eq!(
        lots_of_stuff,
        format!("\x1b[31m\x1b[40m\x1b[1m\x1b[3m\x1b[9m{COMPLEX_TEXT}\x1b[39m\x1b[49m\x1b[22m\x1b[23m\x1b[29m")
    );
}
