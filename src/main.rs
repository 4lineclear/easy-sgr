use easy_sgr_macros::replace_sgr;

fn main() {
    let test = replace_sgr!("Test");
    println!("{test}");
}
