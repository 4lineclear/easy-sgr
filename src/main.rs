use std::error::Error;

use easy_sgr::{
    writing::{AdvancedWriter, SGRWriter, StandardWriter},
    Clear::{Clean, Reset},
    ClearKind,
    Color::*,
    ColorKind, EasySGR, SGRString,
    Style::*,
    StyleKind,
};

#[allow(clippy::all)]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Tests starting\n");

    let text1 = format!("{Italic}{RedFg}This should be italic & red!{Reset}");

    let text2 = format!("{}This should be italic & red!{Reset}", Italic.color(RedFg));

    let text3 = format!(
        "{}",
        "This should be italic & red!"
            .to_sgr()
            .style(Italic)
            .color(RedFg)
            .clear(ClearKind::Reset)
    );

    let mut text4 = SGRString::from("This should be italic & red!");
    text4.italic = StyleKind::Place;
    text4.foreground = ColorKind::Red;
    text4.clear = ClearKind::Reset;

    let text4 = format!("{text4}");

    let text5 = {
        let mut writer = StandardWriter::fmt(String::new());

        writer.place_sgr(&Italic.color(RedFg))?;
        writer.write_inner("This should be italic & red!")?;
        writer.inline_sgr(&Reset)?;
        writer.writer.0
    };

    let text6 = {
        let mut writer = AdvancedWriter::fmt(String::new());

        writer.place_sgr(&Italic.color(RedFg))?;
        writer.write_inner("This should be italic & red!, ")?;
        writer.place_sgr(&ClearItalic.color(DefaultFg))?;
        writer.write_inner("This should be normal text!, ")?;
        writer.inline_sgr(&Clean)?;
        writer.write_inner("Back to red & italic text!, ")?;

        writer.writer.writer.0
    };

    dbg!(format!("{text1}"));
    dbg!(format!("{text2}"));
    dbg!(format!("{text3}"));
    dbg!(format!("{text4}"));
    dbg!(format!("{text5}"));
    dbg!(format!("{text6}"));

    println!();

    println!("{text1}");
    println!("{text2}");
    println!("{text3}");
    println!("{text4}");
    println!("{text5}");
    println!("{text6}");

    println!("\nTests complete");

    Ok(())
}
