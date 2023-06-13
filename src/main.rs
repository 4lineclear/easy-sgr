use easy_ansi::{color::AnsiColor::*, color::Color::*, style::Style::*, ToAnsiString};

// TODO create writer with defaults
fn main() {
    let string = "This is italic and red"
        .to_ansi_string()
        .skip_reset()
        .foreground(Red);

    let test = format!(
        "{Italic}{string}{ResetItalic}, this is just red, {FBlue}\
    and this blue, {BGreen}you can even have backgrounds!{Reset}\nNow back to normal."
    );

    let string2 = "And you can even chain this stuff"
        .foreground(Blue)
        .style(Bold);
    let test2 = format!("{}{string2}", Italic.and(Strikethrough).and(Underline));

    println!("{test}");
    println!("{test2}");
    dbg!(&test);
    dbg!(&test2);
}
