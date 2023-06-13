use std::fmt::Display;

use crate::{
    writer::{AnsiFmt, AnsiWriter},
};

#[derive(Debug, Default, Clone, Copy)]
pub enum AnsiColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    EightBit(u8),
    Rgb(u8, u8, u8),
    #[default]
    Default,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    FBlack,
    FRed,
    FGreen,
    FYellow,
    FBlue,
    FMagenta,
    FCyan,
    FWhite,
    FEightBit(u8),
    Frgb(u8, u8, u8),
    FDefault,

    BBlack,
    BRed,
    BGreen,
    BYellow,
    BBlue,
    BMagenta,
    BCyan,
    BWhite,
    BEightBit(u8),
    Brgb(u8, u8, u8),
    BDefault,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;
        let mut fmt = AnsiFmt::new(f);
        fmt.escape()?;
        match self {
            FBlack => fmt.write_code(30),
            FRed => fmt.write_code(31),
            FGreen => fmt.write_code(32),
            FYellow => fmt.write_code(33),
            FBlue => fmt.write_code(34),
            FMagenta => fmt.write_code(35),
            FCyan => fmt.write_code(36),
            FWhite => fmt.write_code(37),
            FEightBit(n) => fmt.write_all(&[38, 2, *n]),
            Frgb(r, g, b) => fmt.write_all(&[38, 5, *r, *g, *b]),
            FDefault => fmt.write_code(39),
            BBlack => fmt.write_code(40),
            BRed => fmt.write_code(41),
            BGreen => fmt.write_code(42),
            BYellow => fmt.write_code(43),
            BBlue => fmt.write_code(44),
            BMagenta => fmt.write_code(45),
            BCyan => fmt.write_code(46),
            BWhite => fmt.write_code(47),
            BEightBit(n) => fmt.write_all(&[48, 2, *n]),
            Brgb(r, g, b) => fmt.write_all(&[48, 5, *r, *g, *b]),
            BDefault => fmt.write_code(49),
        }?;

        fmt.end()
    }
}
