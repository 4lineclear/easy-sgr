use easy_sgr::{println, sgr};

fn main() {
    let green = "This should be green!";
    let normal = "This should be normal!";
    let styled = "should be styled";
    let bold = sgr!("{+Bold}");

    let _ = "{+[italic,strike]} ";

    println!("{normal}");

    println!("\u{1f604} ☀ ☁ ☂ {#GreenFg&green#DefaultFg} {normal}");
    println!(
        "\u{1f604} ☀ ☁ ☂ {styled+Italic+Strike#RedFg#BlackBg}, \
        this too { -Italic-Strike#f[0f]}",
        styled
    );
    println!("\u{1f604} ☀ ☁ ☂ now the text is white!{#DefaultFg#DefaultBg}");
    println!("\u{1f604} ☀ ☁ ☂ no styles");
    print!("{bold}");
    println!(r#""You can even use raw strings! Though this just gets returned as is""#);
}
