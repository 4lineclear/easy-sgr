use easy_sgr_macros as esm;

fn main() {
    let i = "should be styled,";
    esm::println!(
        "\u{1f604}☀ ☁ ☂{i+Italic+Strikethrough#RedFg#BlackBg} this too\0 {}\
    \u{1f604}",
        i
    );
    esm::println!("{-Italic-Strikethrough}less styles");
    esm::println!("{#DefaultFg#DefaultBg}no styles");
}
