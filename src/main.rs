// use easy_sgr_macros::{replace_sgr, sgr_test};

fn main() {
    let i = "should be styled,";
    easy_sgr_macros::println!(
        "{i+Italic-Bold#RedFg#BlackBg} this too\0 {}\
    ",
        i
    );
}
