// use easy_sgr_macros::{replace_sgr, sgr_test};

fn main() {
    const TEST: &str = easy_sgr_macros::sgr!(
        "Hopefully this works \
    \x1b[1m\
    yeah\
    {{!\
    }}"
    );
    dbg!(TEST);
    println!("{TEST}")
}
