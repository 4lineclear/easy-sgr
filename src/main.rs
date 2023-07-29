use easy_sgr::println;

fn main() {
    let green = "This should be green!";
    let normal = "This should be normal!";
    println!("{green#GreenFg&#DefaultFg} {normal}")
    // let i = "should be styled";
    // println!(
    //     "\u{1f604} ☀ ☁ ☂ {+Italic+Strikethrough#RedFg#BlackBg#f[0f]},\
    //     this too { -Italic-Strikethrough}",
    //     i
    // );
    // println!("\u{1f604} ☀ ☁ ☂ less styles{#DefaultFg#DefaultBg}");
    // println!("\u{1f604} ☀ ☁ ☂ no styles");
    // let test = easy_sgr::sgr!("{+Bold}");
    // print!("{test}");
    // println!(r#""You can even use raw strings!""#);
}
