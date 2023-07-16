// use easy_sgr_macros::{replace_sgr, sgr_test};

fn main() {
    // const TEST: &str = easy_sgr_macros::sgr!(
    //     "Hopefully this works \
    // \x1b[1m\
    // yeah \
    // {{\u{1f604}\
    // }}", 1
    // );
    // dbg!(TEST);
    // println!("{TEST}")
    // let a = "Hopefully this works \u{1b}[1myeah {{😄}}{}";
    // println!(easy_sgr_macros::sgr!(
    //     "Hopefully this works \
    // \x1b[1m\
    // yeah \
    // {{\u{1f604}\
    // }}"
    // ));
    // let a = 1;
    // let test = easy_sgr_macros::sgr!(
    //     "Hopefully this works \
    // \x1b[1m\
    // yeah \
    // {{\u{1f604}\
    // }}{a}"
    // );
    // println!(
    //     easy_sgr_macros::sgr!(
    //         "Hopefully this works \
    //     \x1b[1m\
    //     yeah \
    //     {{\u{1f604}\
    //     }}{a}"
    //     ),
    //     a = a
    // );
    let i = 1;
    // println!(easy_sgr_macros::sgr!("{i}"), i=i);
    easy_sgr_macros::println!(
        "Test\x1b[1m {}\
    ",
        i
    );
    // println!(easy_sgr_macros::sgr!("{i}", i));
    //https://users.rust-lang.org/t/format-with-string-obfuscation/88102/11
}
