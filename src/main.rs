use std::io::{stdout, Write};

use flc_easy_sgr::{
    writing::{IoWriter, SGRWriter},
    Clear::Reset,
    Color::*,
    ColorKind, EasySGR, SGRString,
    Style::*,
    StyleKind,
};

#[allow(clippy::all)]
fn main() {
    println!("Tests starting\n");
    println!("{Italic}{RedFg}This should be italic & red!{Reset}");

    println!("{}This should be italic & red!{Reset}", Italic.color(RedFg));

    let text = "This should be italic & red!"
        .to_sgr()
        .style(Italic)
        .color(RedFg);
    println!("{text}");

    let mut text = SGRString::from("This should be italic & red!");
    text.italic = StyleKind::Place;
    text.foreground = ColorKind::Red;
    println!("{text}");

    let mut writer = IoWriter::new(stdout().lock());
    writer.place_sgr(&Italic.color(RedFg)).unwrap();
    writer.write(b"This should be italic & red!").unwrap();
    writer.inline_sgr(&Reset).unwrap();
    println!("\n\nTests complete")
}
