fn main() {
    let i = "should be styled";
    easy_sgr:: println!(
        "\u{1f604} ☀ ☁ ☂ {i+Italic+Strikethrough#RedFg#BlackBg}, this too {}{-Italic-Strikethrough}",
        i
    );
    easy_sgr::println!("\u{1f604} ☀ ☁ ☂ less styles{#DefaultFg#DefaultBg}");
    easy_sgr::println!("\u{1f604} ☀ ☁ ☂ no styles");
}
