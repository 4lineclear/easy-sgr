use std::error::Error;

use easy_sgr::{
    CleanKind, Color::*, ColorKind, EasySGR, SGRString, SGRWriter, StandardWriter, Style::*,
    StyleKind,
};

#[allow(clippy::all)]
fn main() -> Result<(), Box<dyn Error>> {
    println!("{GreenFg}Tests starting{Reset}\n");

    let text1 = format!("{Italic}{RedFg}This should be italic & red!{Reset}");

    let text2 = format!("{}This should be italic & red!{Reset}", Italic.color(RedFg));

    let text3 = format!(
        "{}",
        "This should be italic & red!"
            .to_sgr()
            .style(Italic)
            .color(RedFg)
            .clean(CleanKind::Reset)
    );

    let mut text4 = SGRString::from("This should be italic & red!");
    text4.italic = StyleKind::Place;
    text4.foreground = ColorKind::Red;
    text4.clean = CleanKind::Reset;

    let text4 = format!("{text4}");

    let text5 = {
        let mut writer = StandardWriter::fmt(String::new());

        writer.sgr(&Italic.color(RedFg))?;
        writer.write_inner("This should be italic & red!")?;
        writer.sgr(&Reset)?;
        writer.writer.0
    };

    dbg!(format!("{text1}"));
    dbg!(format!("{text2}"));
    dbg!(format!("{text3}"));
    dbg!(format!("{text4}"));
    dbg!(format!("{text5}"));

    println!();

    println!("{text1}");
    println!("{text2}");
    println!("{text3}");
    println!("{text4}");
    println!("{text5}");

    println!("\n{GreenFg}Tests complete{Reset}");

    Ok(())
}
