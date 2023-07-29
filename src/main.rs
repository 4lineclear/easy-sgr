use easy_sgr::println;

fn main() {
    let i = "should be styled";
    println!(
        "\u{1f604} ☀ ☁ ☂ {+Italic+Strikethrough#RedFg#BlackBg#f[0f]},\
        this too { -Italic-Strikethrough}",
        i
    );
    println!("\u{1f604} ☀ ☁ ☂ less styles{#DefaultFg#DefaultBg}");
    println!("\u{1f604} ☀ ☁ ☂ no styles");
    println!(r#""You can even use raw strings!""#);
}
