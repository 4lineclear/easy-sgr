use flc_easy_sgr::{Color::*, ColorKind, EasySGR, SGRString, Style::*, StyleKind};

fn main() {
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
}
