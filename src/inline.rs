use std::fmt::Display;

use crate::{
    graphics::{ColorKind, StyleKind},
    write::{AnsiWriter, FmtWriter},
};
use StyleKind::*;

#[derive(Debug, Clone, Default)]
pub struct InlineGraphics {
    pub custom: Vec<u8>,

    pub reset: bool,

    pub foreground: Option<ColorKind>,
    pub background: Option<ColorKind>,

    pub bold: StyleKind,
    pub dim: StyleKind,
    pub italic: StyleKind,
    pub underline: StyleKind,
    pub blinking: StyleKind,
    pub inverse: StyleKind,
    pub hidden: StyleKind,
    pub strikethrough: StyleKind,
}

impl StyleKind {
    pub fn is_none(&self) -> bool {
        *self == None
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Custom {
    Place(u8),
    Clear(u8),
}

impl Display for InlineGraphics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.custom.is_empty()
            && !self.reset
            && self.foreground.is_none()
            && self.background.is_none()
            && self.bold.is_none()
            && self.dim.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.blinking.is_none()
            && self.inverse.is_none()
            && self.hidden.is_none()
            && self.strikethrough.is_none()
        {
            Ok(())
        } else {
            FmtWriter::new(f).inject_inline(self)
        }
    }
}

impl DisplayedAnsi for InlineGraphics {
    #[inline]
    fn style(mut self, style: impl Into<Style>) -> Self {
        use Style::*;

        match style.into() {
            Reset => self.reset = true,

            Bold => self.bold = Place,
            Dim => self.dim = Place,
            Italic => self.italic = Place,
            Underline => self.underline = Place,
            Blinking => self.blinking = Place,
            Inverse => self.inverse = Place,
            Hidden => self.hidden = Place,
            Strikethrough => self.strikethrough = Place,

            ClearBold => self.bold = Clear,
            ClearDim => self.dim = Clear,
            ClearItalic => self.italic = Clear,
            ClearUnderline => self.underline = Clear,
            ClearBlinking => self.blinking = Clear,
            ClearInverse => self.inverse = Clear,
            ClearHidden => self.hidden = Clear,
            ClearStrikethrough => self.strikethrough = Clear,
        }
        self
    }

    #[inline]
    fn color(mut self, color: impl Into<crate::inline::Color>) -> Self {
        use {Color::*, ColorKind::*};

        match color.into() {
            FBlack => self.foreground = Some(Black),
            FRed => self.foreground = Some(Red),
            FGreen => self.foreground = Some(Green),
            FYellow => self.foreground = Some(Yellow),
            FBlue => self.foreground = Some(Blue),
            FMagenta => self.foreground = Some(Magenta),
            FCyan => self.foreground = Some(Cyan),
            FWhite => self.foreground = Some(White),
            FEightBit(n) => self.foreground = Some(EightBit(n)),
            FRgb(r, g, b) => self.foreground = Some(Rgb(r, g, b)),
            FDefault => self.foreground = Some(Default),

            BBlack => self.background = Some(Black),
            BRed => self.background = Some(Red),
            BGreen => self.background = Some(Green),
            BYellow => self.background = Some(Yellow),
            BBlue => self.background = Some(Blue),
            BMagenta => self.background = Some(Magenta),
            BCyan => self.background = Some(Cyan),
            BWhite => self.background = Some(White),
            BEightBit(n) => self.background = Some(EightBit(n)),
            BRgb(r, g, b) => self.background = Some(Rgb(r, g, b)),
            BDefault => self.background = Some(Default),
        }
        self
    }

    fn custom(mut self, code: impl Into<u8>) -> InlineGraphics {
        self.custom.push(code.into());
        self
    }

    fn write<W: AnsiWriter>(&self, writer: &mut W) -> Result<(), W::Error> {
        use ColorKind::*;
        writer.escape()?;
        if let Some(color) = self.foreground {
            match color {
                Black => writer.write_code(30)?,
                Red => writer.write_code(31)?,
                Green => writer.write_code(32)?,
                Yellow => writer.write_code(33)?,
                Blue => writer.write_code(34)?,
                Magenta => writer.write_code(35)?,
                Cyan => writer.write_code(36)?,
                White => writer.write_code(37)?,
                EightBit(n) => writer.write_multiple(&[38, 2, n])?,
                Rgb(r, g, b) => writer.write_multiple(&[38, 5, r, g, b])?,
                Default => writer.write_code(39)?,
            }
        };
        if let Some(color) = self.background {
            match color {
                Black => writer.write_code(40)?,
                Red => writer.write_code(41)?,
                Green => writer.write_code(42)?,
                Yellow => writer.write_code(43)?,
                Blue => writer.write_code(44)?,
                Magenta => writer.write_code(45)?,
                Cyan => writer.write_code(46)?,
                White => writer.write_code(47)?,
                EightBit(n) => writer.write_multiple(&[48, 2, n])?,
                Rgb(r, g, b) => writer.write_multiple(&[48, 5, r, g, b])?,
                Default => writer.write_code(49)?,
            }
        };
        if self.reset {
            writer.write_code(0)?;
        }
        for (kind, place, clear) in [
            (self.bold, 1, 22),
            (self.dim, 2, 22),
            (self.italic, 3, 23),
            (self.underline, 4, 24),
            (self.blinking, 5, 25),
            (self.inverse, 7, 27),
            (self.hidden, 8, 28),
            (self.strikethrough, 9, 29),
        ] {
            match kind {
                None => (),
                Place | Both => writer.write_code(place)?,
                Clear => writer.write_code(clear)?,
            }
        }
        writer.write_multiple(&self.custom)?;
        writer.end()
    }
}

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
        let mut fmt = FmtWriter::new(f);
        fmt.escape()?;
        fmt.inject_inline(self)?;
        fmt.end()
    }
}

impl DisplayedAnsi for Style {
    fn style(self, style: impl Into<Style>) -> InlineGraphics {
        InlineGraphics::default().style(self).style(style.into())
    }
    fn color(self, color: impl Into<Color>) -> InlineGraphics {
        InlineGraphics::default().style(self).color(color.into())
    }
    fn custom(self, code: impl Into<u8>) -> InlineGraphics {
        InlineGraphics::default().style(self).custom(code.into())
    }
    fn write<W: AnsiWriter>(&self, writer: &mut W) -> Result<(), W::Error> {
        use Style::*;
        writer.write_code(match self {
            Reset => 0,
            Bold => 1,
            Dim => 2,
            Italic => 3,
            Underline => 4,
            Blinking => 5,
            Inverse => 7,
            Hidden => 8,
            Strikethrough => 9,

            ClearBold | ClearDim => 22,
            ClearItalic => 23,
            ClearUnderline => 24,
            ClearBlinking => 25,
            ClearInverse => 27,
            ClearHidden => 28,
            ClearStrikethrough => 29,
        })
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

impl DisplayedAnsi for Color {
    fn style(self, style: impl Into<Style>) -> InlineGraphics {
        InlineGraphics::default().color(self).style(style.into())
    }

    fn color(self, color: impl Into<Color>) -> InlineGraphics {
        InlineGraphics::default().color(self).color(color.into())
    }

    fn custom(self, code: impl Into<u8>) -> InlineGraphics {
        InlineGraphics::default().color(self).custom(code.into())
    }

    fn write<W: AnsiWriter>(&self, writer: &mut W) -> Result<(), W::Error> {
        use Color::*;

        match self {
            FBlack => writer.write_code(30),
            FRed => writer.write_code(31),
            FGreen => writer.write_code(32),
            FYellow => writer.write_code(33),
            FBlue => writer.write_code(34),
            FMagenta => writer.write_code(35),
            FCyan => writer.write_code(36),
            FWhite => writer.write_code(37),
            FEightBit(n) => writer.write_multiple(&[38, 2, *n]),
            FRgb(r, g, b) => writer.write_multiple(&[38, 5, *r, *g, *b]),

            FDefault => writer.write_code(39),
            BBlack => writer.write_code(40),
            BRed => writer.write_code(41),
            BGreen => writer.write_code(42),
            BYellow => writer.write_code(43),
            BBlue => writer.write_code(44),
            BMagenta => writer.write_code(45),
            BCyan => writer.write_code(46),
            BWhite => writer.write_code(47),
            BEightBit(n) => writer.write_multiple(&[48, 2, *n]),
            BRgb(r, g, b) => writer.write_multiple(&[48, 5, *r, *g, *b]),
            BDefault => writer.write_code(49),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = FmtWriter::new(f);
        fmt.escape()?;
        fmt.inject_inline(self)?;
        fmt.end()
    }
}

pub trait DisplayedAnsi: Display {
    #[must_use]
    fn style(self, style: impl Into<Style>) -> InlineGraphics;
    #[must_use]
    fn color(self, color: impl Into<Color>) -> InlineGraphics;
    #[must_use]
    fn custom(self, code: impl Into<u8>) -> InlineGraphics;

    fn write<W: AnsiWriter>(&self, writer: &mut W) -> Result<(), W::Error>;
}

// #[macro_export]
// macro_rules! combine {
//     ($($arg:expr),*) => {{
//         let mut graphics = InlineGraphics {
//             custom: Vec::new(),
//             reset: false,
//             foreground: None,
//             background: None,
//             place_bold: false,
//             place_dim: false,
//             place_italic: false,
//             place_underline: false,
//             place_blinking: false,
//             place_inverse: false,
//             place_hidden: false,
//             place_strikethrough: false,
//             clear_bold: false,
//             clear_dim: false,
//             clear_italic: false,
//             clear_underline: false,
//             clear_blinking: false,
//             clear_inverse: false,
//             clear_hidden: false,
//             clear_strikethrough: false,
//         };
//         $(
//             match $arg {
//                 Reset => graphics.reset = true,

//                 Bold => graphics.place_bold = true,
//                 Dim => graphics.place_dim = true,
//                 Italic => graphics.place_italic = true,
//                 Underline => graphics.place_underline = true,
//                 Blinking => graphics.place_blinking = true,
//                 Inverse => graphics.place_inverse = true,
//                 Hidden => graphics.place_hidden = true,
//                 Strikethrough => graphics.place_strikethrough = true,

//                 ClearBold => graphics.clear_bold = true,
//                 ClearDim => graphics.clear_dim = true,
//                 ClearItalic => graphics.clear_italic = true,
//                 ClearUnderline => graphics.clear_underline = true,
//                 ClearBlinking => graphics.clear_blinking = true,
//                 ClearInverse => graphics.clear_inverse = true,
//                 ClearHidden => graphics.clear_hidden = true,
//                 ClearStrikethrough => graphics.clear_strikethrough = true,
//             }
//         )*
//         graphics

//     }};
// }
