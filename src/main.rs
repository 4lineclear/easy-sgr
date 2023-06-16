use std::{io::{stdout, Write}, error::Error};

use easy_ansi::{
    graphics::ColorKind::*,
    inline::{Color::*, DisplayedAnsi, Style::*},
    write::{AnsiWriter, IoWriter},
    ToAnsiString, write_ansi,
};
fn main() -> Result<(), Box<dyn Error>>{
    // TODO Documentation
    let string = "This is italic and red".to_ansi_string().foreground(Red);

    let test = format!(
        "{Italic}{string}{ClearItalic}, this is just red, {FBlue}\
    and this blue, {BGreen}you can even have backgrounds!{Reset}\nNow back to normal."
    );

    let string2 = "And you can even chain this stuff"
        .foreground(Blue)
        .style(Bold);
    let test2 = format!("{}{string2}", Italic.style(Strikethrough).style(Underline));

    println!("{test}");
    println!("{test2}{Reset}");

    let mut writer = IoWriter::new(stdout().lock());
    
    write_ansi!(writer, FRed, Italic)?;
    
    writer
        .write_all(b"This writing should be italic red, using the IoWriter!\n")
        .unwrap();

    write_ansi!(writer, FDefault, ClearItalic)?;
    Ok(())
}
