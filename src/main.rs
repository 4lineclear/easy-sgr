use std::error::Error;

use easy_sgr::Color;

fn main() -> Result<(), Box<dyn Error>> {
    let a: Result<Color, _> = "RgbFg(1,2,3)".parse();
    dbg!(a)?;
    Ok(())
}
