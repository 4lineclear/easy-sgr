use easy_sgr_macros as esm;

fn main() {
    let i = "should be styled";

    esm::println!(
        "\u{1f604} ☀ ☁ ☂ {i+Italic+Strikethrough#RedFg#BlackBg}, this too {}{-Italic-Strikethrough}",
        i
    );
    esm::println!("\u{1f604} ☀ ☁ ☂ less styles{#DefaultFg#DefaultBg}");
    esm::println!("\u{1f604} ☀ ☁ ☂ no styles");
}
