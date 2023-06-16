use std::fmt::Display;

use crate::{
    graphics::{ColorKind, Graphics, StyleKind},
    write::{AnsiWriter, FmtWriter},
    Ansi,
};

impl Display for Graphics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.no_clears() && self.no_places() {
            Ok(())
        } else {
            FmtWriter::new(f).inject_inline(self)
        }
    }
}

impl DisplayedAnsi for Graphics {
    fn style(mut self, style: impl Into<Style>) -> Self {
        use Style::*;

        match style.into() {
            Reset => self.reset = self.reset.shift(StyleKind::Place),

            Bold => self.bold = self.bold.shift(StyleKind::Place),
            Dim => self.dim = self.dim.shift(StyleKind::Place),
            Italic => self.italic = self.italic.shift(StyleKind::Place),
            Underline => self.underline = self.underline.shift(StyleKind::Place),
            Blinking => self.blinking = self.blinking.shift(StyleKind::Place),
            Inverse => self.inverse = self.inverse.shift(StyleKind::Place),
            Hidden => self.hidden = self.hidden.shift(StyleKind::Place),
            Strikethrough => self.strikethrough = self.strikethrough.shift(StyleKind::Place),

            ClearBold => self.bold = self.bold.shift(StyleKind::Clear),
            ClearDim => self.dim = self.dim.shift(StyleKind::Clear),
            ClearItalic => self.italic = self.italic.shift(StyleKind::Clear),
            ClearUnderline => self.underline = self.underline.shift(StyleKind::Clear),
            ClearBlinking => self.blinking = self.blinking.shift(StyleKind::Clear),
            ClearInverse => self.inverse = self.inverse.shift(StyleKind::Clear),
            ClearHidden => self.hidden = self.hidden.shift(StyleKind::Clear),
            ClearStrikethrough => self.strikethrough = self.strikethrough.shift(StyleKind::Clear),
        }
        self
    }
    fn color(mut self, color: impl Into<crate::inline::Color>) -> Graphics {
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
    fn custom_place(mut self, code: impl Into<u8>) -> Graphics {
        self.custom_places.push(code.into());
        self
    }

    fn custom_clear(mut self, code: impl Into<u8>) -> Graphics {
        self.custom_clears.push(code.into());
        self
    }

    fn write<W: AnsiWriter>(&self, writer: &mut W) -> Result<(), W::Error> {
        writer.escape()?;
        self.place_ansi(writer)?;
        self.clear_ansi(writer)?;
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

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = FmtWriter::new(f);
        fmt.escape()?;
        fmt.inject_inline(self)?;
        fmt.end()
    }
}

impl DisplayedAnsi for Style {
    fn style(self, style: impl Into<Style>) -> Graphics {
        Graphics::default().style(self).style(style.into())
    }
    fn color(self, color: impl Into<Color>) -> Graphics {
        Graphics::default().style(self).color(color.into())
    }
    fn custom_place(self, code: impl Into<u8>) -> Graphics {
        Graphics::default().style(self).custom_place(code)
    }

    fn custom_clear(self, code: impl Into<u8>) -> Graphics {
        Graphics::default().style(self).custom_clear(code)
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
    fn style(self, style: impl Into<Style>) -> Graphics {
        Graphics::default().color(self).style(style.into())
    }

    fn color(self, color: impl Into<Color>) -> Graphics {
        Graphics::default().color(self).color(color.into())
    }
    fn custom_place(self, code: impl Into<u8>) -> Graphics {
        Graphics::default().color(self).custom_place(code)
    }

    fn custom_clear(self, code: impl Into<u8>) -> Graphics {
        Graphics::default().color(self).custom_clear(code)
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
    fn style(self, style: impl Into<Style>) -> Graphics;
    #[must_use]
    fn color(self, color: impl Into<Color>) -> Graphics;
    #[must_use]
    fn custom_place(self, code: impl Into<u8>) -> Graphics;
    #[must_use]
    fn custom_clear(self, code: impl Into<u8>) -> Graphics;

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
