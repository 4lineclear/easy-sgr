use easy_sgr_macros as esm;

fn main() {
    let i = "should be styled,";
    esm::println!(
        "{i+Italic+Strikethrough#RedFg#BlackBg} this too\0 {}\
    ",
        i
    );
    esm::println!("{-Italic-Strikethrough}less styles");
    esm::println!("{#DefaultFg#DefaultBg}no styles");
}
