// use std::fmt::Display;

use std::fmt::{Display, Write};

use crate::writing::{AnsiWriter, FmtWriter};

use super::{AnsiString, ColorKind, StyleKind, ToAnsiString};

/// Allows for chaining types that implement it
///
/// Works by calling
/// ```ignore
/// self.into().<graphic>(<graphic>)
/// ```
/// Where `<graphic>` is a type of ANSI graphics code such as [`Style`] or [`Color`]
pub trait InlineAnsi: Sized + Display + Into<AnsiString> + ToAnsiString {
    /// Sets the plaintext of the returned [`AnsiString`]  
    #[must_use]
    fn text(self, text: impl Into<String>) -> AnsiString {
        self.into().text(text)
    }
    /// Adds a style to the returned [`AnsiString`]  
    #[must_use]
    fn style(self, style: impl Into<Style>) -> AnsiString {
        self.into().style(style)
    }
    /// Adds a color(foreground or background) to the returned [`AnsiString`]  
    #[must_use]
    fn color(self, color: impl Into<Color>) -> AnsiString {
        self.into().color(color)
    }
    /// Adds a custom code to the returned [`AnsiString`]  
    #[must_use]
    fn custom(self, code: impl Into<u8>) -> AnsiString {
        self.into().custom(code)
    }
    // TODO link 'Escapes' and 'ends'
    /// Writes a set of ansi codes to the given [`AnsiWriter`]
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

            ClearBoldDim => self.bold = Clear,
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
            Fg8Bit(n) => self.foreground = EightBit(n),
            FgRGB(r, g, b) => self.foreground = RGB(r, g, b),
            FgDefault => self.foreground = Default,

            BgBlack => self.background = Black,
            BgRed => self.background = Red,
            BgGreen => self.background = Green,
            BgYellow => self.background = Yellow,
            BgBlue => self.background = Blue,
            BgMagenta => self.background = Magenta,
            BgCyan => self.background = Cyan,
            BgWhite => self.background = White,
            Bg8Bit(n) => self.background = EightBit(n),
            BgRGB(r, g, b) => self.background = RGB(r, g, b),
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

/// A set of ANSI code sequences
#[derive(Debug, Clone)]
pub enum Style {
    /// Represents the ANSI code `0`
    Reset = 0,
    /// Represents the ANSI code `1`
    Bold = 1,
    /// Represents the ANSI code `2`
    Dim = 2,
    /// Represents the ANSI code `3`
    Italic = 3,
    /// Represents the ANSI code `4`
    Underline = 4,
    /// Represents the ANSI code `5`
    Blinking = 5,
    /// Represents the ANSI code `7`
    Inverse = 7,
    /// Represents the ANSI code `8`
    Hidden = 8,
    /// Represents the ANSI code `9`
    Strikethrough = 9,
    /// Represents the ANSI code `2`
    ///
    /// Works for either Bold and Dim(for most terminals)
    ClearBoldDim = 22,
    /// Represents the ANSI code `3`
    ClearItalic = 23,
    /// Represents the ANSI code `4`
    ClearUnderline = 24,
    /// Represents the ANSI code `5`
    ClearBlinking = 25,
    /// Represents the ANSI code `7`
    ClearInverse = 27,
    /// Represents the ANSI code `8`
    ClearHidden = 28,
    /// Represents the ANSI code `9`
    ClearStrikethrough = 29,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.standard_display(f)
    }
}

impl InlineAnsi for Style {
    /// Writes a set of ansi codes to given [`AnsiWriter`]
    ///
    /// Escapes and ends the sequence
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the given write fails
    ///
    /// # Safety
    ///
    /// Uses unsafe pointer casting to reliably access the discriminant
    /// See [here](https://doc.rust-lang.org/reference/items/enumerations.html#pointer-casting)
    ///
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter,
    {
        writer.write_code(unsafe { *(self as *const Self as *const u8) })
    }
}
/// A ANSI color code
#[derive(Debug, Clone)]
pub enum Color {
    /// Represents the ANSI code `30`
    FgBlack,
    /// Represents the ANSI code `31`
    FgRed,
    /// Represents the ANSI code `32`
    FgGreen,
    /// Represents the ANSI code `33`
    FgYellow,
    /// Represents the ANSI code `34`
    FgBlue,
    /// Represents the ANSI code `35`
    FgMagenta,
    /// Represents the ANSI code `36`
    FgCyan,
    /// Represents the ANSI code `37`
    FgWhite,
    /// Represents the ANSI codes `38;2;<n>`
    ///
    /// Where `<n>` is specified in use
    Fg8Bit(u8),
    /// Represents the ANSI codes `38;2;<n1>;<n2>;<n3>`
    ///
    /// Where `<n1>`,`<n2>`,`<n3>` are specified in use
    FgRGB(u8, u8, u8),
    /// Represents the ANSI code `39`
    FgDefault,

    /// Represents the ANSI code `40`
    BgBlack,
    /// Represents the ANSI code `41`
    BgRed,
    /// Represents the ANSI code `42`
    BgGreen,
    /// Represents the ANSI code `43`
    BgYellow,
    /// Represents the ANSI code `44`
    BgBlue,
    /// Represents the ANSI code `45`
    BgMagenta,
    /// Represents the ANSI code `46`
    BgCyan,
    /// Represents the ANSI code `47`
    BgWhite,
    /// Represents the ANSI codes `48;2;<n>`
    ///
    /// Where `<n>` is specified in use
    Bg8Bit(u8),
    /// Represents the ANSI codes `38;2;<n1>;<n2>;<n3>`
    ///
    /// Where `<n1>`,`<n2>`,`<n3>` are specified in use
    BgRGB(u8, u8, u8),
    /// Represents the ANSI code `49`
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
            Fg8Bit(n) => writer.write_multiple(&[38, 2, *n]),
            FgRGB(r, g, b) => writer.write_multiple(&[38, 5, *r, *g, *b]),
            FgDefault => writer.write_code(39),

            BgBlack => writer.write_code(40),
            BgRed => writer.write_code(41),
            BgGreen => writer.write_code(42),
            BgYellow => writer.write_code(43),
            BgBlue => writer.write_code(44),
            BgMagenta => writer.write_code(45),
            BgCyan => writer.write_code(46),
            BgWhite => writer.write_code(47),
            Bg8Bit(n) => writer.write_multiple(&[48, 2, *n]),
            BgRGB(r, g, b) => writer.write_multiple(&[48, 5, *r, *g, *b]),
            BgDefault => writer.write_code(49),
        }
    }
}
