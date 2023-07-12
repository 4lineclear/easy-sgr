use std::error::Error;

use easy_sgr::{Color, EasySGR, SGRWriter, StandardWriter, Style};

#[test]
fn sgr_writer() -> Result<(), Box<dyn Error>> {
    let mut w = StandardWriter::from(String::new());
    w.write_inner("test")?;
    w.sgr(&Color::RedFg.style(Style::Italic))?;
    w.sgr(&Style::Bold)?;

    assert_eq!("test\x1b[31;3m\x1b[1m", w.writer.0);
    Ok(())
}

#[test]
fn sgr_builder() -> Result<(), Box<dyn Error>> {
    let mut w = StandardWriter::from(String::new());

    w.escape().end()?;
    assert_eq!("", w.writer.0);

    let mut builder = w.escape();
    builder.write_code(0);
    builder.write_codes(&[1, 2]);
    builder.chain_code(3).chain_codes(&[4, 5]).end()?;

    assert_eq!("\x1b[0;1;2;3;4;5m", w.writer.0);
    Ok(())
}
