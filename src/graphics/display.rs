// use std::fmt::Display;

use std::fmt::{Display, Write};

use crate::writing::{AnsiWriter, FmtWriter};

use super::{AnsiString, ColorKind, StyleKind, ToAnsiString};

pub trait InlineAnsi: Sized + Display + Into<AnsiString> + ToAnsiString {
    #[must_use]
    fn text(self, text: impl Into<String>) -> AnsiString {
        self.into().text(text)
    }
    #[must_use]
    fn style(self, style: impl Into<Style>) -> AnsiString {
        self.into().style(style)
    }
    #[must_use]
    fn color(self, color: impl Into<Color>) -> AnsiString {
        self.into().color(color)
    }
    #[must_use]
    fn custom(self, code: impl Into<u8>) -> AnsiString {
        self.into().custom(code)
    }
    // TODO link 'Escapes' and 'ends'
    /// Writes a set of ansi codes to given [`AnsiWriter`]
    /// 
    /// Escapes and ends the sequence
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the given write fails
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter;
    /// Writes to the given [`Formatter`](std::fmt::Formatter) the ANSI sequence
    ///
    /// # Errors
    ///
    /// Return an error if writing to the [`Formatter`](std::fmt::Formatter) fails
    fn standard_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = FmtWriter::new(f);
        fmt.escape()?;
        fmt.inline_ansi(self)?;
        fmt.end()
    }
}
impl From<Color> for AnsiString {
    fn from(value: Color) -> Self {
        Self::default().color(value)
    }
}

impl From<Style> for AnsiString {
    fn from(value: Style) -> Self {
        Self::default().style(value)
    }
}

impl Display for AnsiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.standard_display(f)
    }
}

impl InlineAnsi for AnsiString {
    fn text(mut self, text: impl Into<String>) -> AnsiString {
        self.text = text.into();
        self
    }
    fn style(mut self, style: impl Into<Style>) -> AnsiString {
        use Style::*;
        use StyleKind::*;

        match style.into() {
            Reset => self.reset = Place,
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

    fn color(mut self, color: impl Into<Color>) -> AnsiString {
        use {Color::*, ColorKind::*};

        match color.into() {
            FgBlack => self.foreground = Black,
            FgRed => self.foreground = Red,
            FgGreen => self.foreground = Green,
            FgYellow => self.foreground = Yellow,
            FgBlue => self.foreground = Blue,
            FgMagenta => self.foreground = Magenta,
            FgCyan => self.foreground = Cyan,
            FgWhite => self.foreground = White,
            FgEightBit(n) => self.foreground = EightBit(n),
            FgRgb(r, g, b) => self.foreground = Rgb(r, g, b),
            FgDefault => self.foreground = Default,

            BgBlack => self.background = Black,
            BgRed => self.background = Red,
            BgGreen => self.background = Green,
            BgYellow => self.background = Yellow,
            BgBlue => self.background = Blue,
            BgMagenta => self.background = Magenta,
            BgCyan => self.background = Cyan,
            BgWhite => self.background = White,
            BgEightBit(n) => self.background = EightBit(n),
            BgRgb(r, g, b) => self.background = Rgb(r, g, b),
            BgDefault => self.background = Default,
        }
        self
    }

    fn custom(mut self, code: impl Into<u8>) -> AnsiString {
        self.custom_places.push(code.into());
        self
    }

    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter,
    {
        writer.place_ansi(self)?;
        writer.write_inner(&*self.text)
    }

    fn standard_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = FmtWriter::new(f);
        fmt.place_ansi(self)?;
        fmt.write_str(&self.text)?;
        fmt.clean_ansi(self)
    }
}

#[derive(Debug, Clone)]
pub enum Style {
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
        self.standard_display(f)
    }
}

impl InlineAnsi for Style {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter,
    {
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

#[derive(Debug, Clone)]
pub enum Color {
    FgBlack,
    FgRed,
    FgGreen,
    FgYellow,
    FgBlue,
    FgMagenta,
    FgCyan,
    FgWhite,
    FgEightBit(u8),
    FgRgb(u8, u8, u8),
    FgDefault,

    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    BgEightBit(u8),
    BgRgb(u8, u8, u8),
    BgDefault,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.standard_display(f)
    }
}

impl InlineAnsi for Color {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter,
    {
        use Color::*;
        match self {
            FgBlack => writer.write_code(30),
            FgRed => writer.write_code(31),
            FgGreen => writer.write_code(32),
            FgYellow => writer.write_code(33),
            FgBlue => writer.write_code(34),
            FgMagenta => writer.write_code(35),
            FgCyan => writer.write_code(36),
            FgWhite => writer.write_code(37),
            FgEightBit(n) => writer.write_multiple(&[38, 2, *n]),
            FgRgb(r, g, b) => writer.write_multiple(&[38, 5, *r, *g, *b]),
            FgDefault => writer.write_code(39),

            BgBlack => writer.write_code(40),
            BgRed => writer.write_code(41),
            BgGreen => writer.write_code(42),
            BgYellow => writer.write_code(43),
            BgBlue => writer.write_code(44),
            BgMagenta => writer.write_code(45),
            BgCyan => writer.write_code(46),
            BgWhite => writer.write_code(47),
            BgEightBit(n) => writer.write_multiple(&[48, 2, *n]),
            BgRgb(r, g, b) => writer.write_multiple(&[48, 5, *r, *g, *b]),
            BgDefault => writer.write_code(49),
        }
    }
}
