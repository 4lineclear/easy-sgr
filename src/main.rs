// use easy_sgr_macros::{replace_sgr, sgr_test};

fn main() {
    const TEST: &'static str = easy_sgr_macros::sgr!("Hopefully this works");
    println!("{TEST}")
}
