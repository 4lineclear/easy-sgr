use std::fmt::Display;

use crate::{
    graphics::Graphics,
    writer::{AnsiFmt, AnsiWriter},
};

#[derive(Debug, Default, Clone, Copy)]
pub enum Style {
    #[default]
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Inverse,
    Hidden,
    Strikethrough,

    ClearBold,
    ClearDim,
    ClearItalic,
    ClearUnderline,
    ClearBlinking,
    ClearInverse,
    ClearHidden,
    ClearStrikethrough,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Style::*;
        let mut fmt = AnsiFmt::new(f);
        fmt.escape()?;

        fmt.write_code(match self {
            Reset => 0,
            Bold => 1,
            Dim => 2,
            Italic => 3,
            Underline => 4,
            Blinking => 5,
            Inverse => 7,
            Hidden => 8,
            Strikethrough => 9,

            ClearBold => 22,
            ClearDim => 22,
            ClearItalic => 23,
            ClearUnderline => 24,
            ClearBlinking => 25,
            ClearInverse => 27,
            ClearHidden => 28,
            ClearStrikethrough => 29,
        })?;
        fmt.end()
    }
}

impl Style {
    pub fn style(self, other: Self) -> Graphics {
        Graphics::default().style(self).style(other)
    }
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
    FRgb(u8, u8, u8),
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
    BRgb(u8, u8, u8),
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
            FRgb(r, g, b) => fmt.write_all(&[38, 5, *r, *g, *b]),

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
            BRgb(r, g, b) => fmt.write_all(&[48, 5, *r, *g, *b]),
            BDefault => fmt.write_code(49),
        }?;

        fmt.end()
    }
}
