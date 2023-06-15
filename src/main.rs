use easy_ansi::{
    graphics::{ClearKind, ColorKind::*},
    inline::{Color::*, InlineAnsi, Style::*},
    ToAnsiString,
};

// TODO create writer with defaults
fn main() {
    let string = "This is italic and red".to_ansi_string().foreground(Red);

    let test = format!(
        "{Italic}{string}{ClearItalic}, this is just red, {FBlue}\
    and this blue, {BGreen}you can even have backgrounds!{Reset}\nNow back to normal."
    );

    let string2 = "And you can even chain this stuff"
        .foreground(Blue)
        .style(Bold)
        .set_clear(ClearKind::Full);
    let test2 = format!("{}{string2}", Italic.style(Strikethrough).style(Underline));

    println!("{test}");
    println!("{test2}");
    dbg!(&test);
    dbg!(&test2);
}
