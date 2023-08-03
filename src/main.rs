use easy_sgr::{println, sgr, write};
use std::fmt::Write;
fn main() {
    let green = "This should be green!";
    let normal = "This should be normal!";
    let styled = "should be styled";
    let bold = sgr!("{[bold]}");
    let mut written_to = String::new();
    write!(
        written_to,
        "{[!bold]}And, you can even use the {[green]}write!{[]} and {[green]}writeln!{[]} functions!"
    )
    .unwrap();

    println!("{normal}");
    println!("\u{1f604} ☀ ☁ ☂ {[green]}{green}{[default]} {normal}");
    println!(
        "\u{1f604} ☀ ☁ ☂ {[italic strike red on-black]} {styled}, \
        this too {}{[!italic !strike #0f]}",
        styled
    );
    println!("\u{1f604} ☀ ☁ ☂ now the text is white!{[default on-default]}");
    println!("\u{1f604} ☀ ☁ ☂ no styles");
    print!("{bold}");
    println!(r#""You can even use raw strings! Though this just gets returned as is""#);
    println!("{written_to}")
}
