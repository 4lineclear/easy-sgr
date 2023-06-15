use std::io::{stdout, Write};

use easy_ansi::{
    graphics::{ClearKind, ColorKind::*},
    inline::{Color::*, InlineAnsi, Style::*},
    writer::{AnsiWriter, IoWriter},
    ToAnsiString,
};
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

    let mut writer = IoWriter::new(stdout().lock());
    writer.escape().unwrap();
    writer.write_multiple(&[3, 31]).unwrap();
    writer.end().unwrap();
    writer
        .write_all(b"This writing should be italic red, using the IoWriter!\n")
        .unwrap();
    writer.escape().unwrap();
    writer.write_multiple(&[23, 38]).unwrap();
    writer.end().unwrap();
}
