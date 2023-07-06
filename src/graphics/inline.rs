// use std::fmt::Display;

use std::fmt::Display;

use crate::writing::{CapableWriter, StandardWriter};

use super::EasySGR;

/// Represents SGR sequences that can be used Inline.
#[allow(clippy::module_name_repetitions)]
pub trait InlineSGR: Sized + Display + EasySGR {
    // TODO link 'Escapes' and 'ends'
    /// Writes a set of SGR codes to the given [`StandardWriter`]
    ///
    /// Escapes and ends the sequence
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the given write fails
    ///
    fn write<W>(&self, writer: &mut StandardWriter<W>) -> Result<(), W::Error>
    where
        W: CapableWriter;
    /// Writes to the given [`Formatter`](std::fmt::Formatter) the SGR sequence
    ///
    /// # Errors
    ///
    /// Return an error if writing to the [`Formatter`](std::fmt::Formatter) fails
    ///
    #[inline]
    fn standard_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        StandardWriter::fmt(f).inline_sgr(self)
    }
}

#[derive(Debug)]
/// A type of clear
pub enum Clear {
    /// Clears all
    Reset,
    /// Resets to previous style smartly,
    /// when used with advanced writer
    Clean,
}
impl Display for Clear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.standard_display(f)
    }
}
impl InlineSGR for Clear {
    fn write<W>(&self, writer: &mut StandardWriter<W>) -> Result<(), W::Error>
    where
        W: CapableWriter,
    {
        match self {
            Clear::Reset => writer.write_code(0),
            Clear::Clean => todo!(),
        }
    }
}
/// A set of SGR code sequences
#[derive(Debug)]
pub enum Style {
    /// Represents the SGR code `1`
    Bold,
    /// Represents the SGR code `2`
    Dim,
    /// Represents the SGR code `3`
    Italic,
    /// Represents the SGR code `4`
    Underline,
    /// Represents the SGR code `5`
    Blinking,
    /// Represents the SGR code `7`
    Inverse,
    /// Represents the SGR code `8`
    Hidden,
    /// Represents the SGR code `9`
    Strikethrough,
    /// Represents the SGR code `22`
    ClearBold,
    /// Represents the SGR code `22`
    ClearDim,
    /// Represents the SGR code `23`
    ClearItalic,
    /// Represents the SGR code `24`
    ClearUnderline,
    /// Represents the SGR code `25`
    ClearBlinking,
    /// Represents the SGR code `27`
    ClearInverse,
    /// Represents the SGR code `28`
    ClearHidden,
    /// Represents the SGR code `29`
    ClearStrikethrough,
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.standard_display(f)
    }
}

impl InlineSGR for Style {
    /// Writes a set of SGR codes to given [`StandardWriter`]
    ///
    /// Escapes and ends the sequence
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the given write fails
    ///
    fn write<W>(&self, writer: &mut StandardWriter<W>) -> Result<(), W::Error>
    where
        W: CapableWriter,
    {
        use Style::*;
        writer.write_code(match self {
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
/// A SGR color code
#[derive(Debug)]
pub enum Color {
    /// Represents the SGR code `30`
    BlackFg,
    /// Represents the SGR code `31`
    RedFg,
    /// Represents the SGR code `32`
    GreenFg,
    /// Represents the SGR code `33`
    YellowFg,
    /// Represents the SGR code `34`
    BlueFg,
    /// Represents the SGR code `35`
    MagentaFg,
    /// Represents the SGR code `36`
    CyanFg,
    /// Represents the SGR code `37`
    WhiteFg,
    /// Represents the SGR codes `38;2;<n>`
    ///
    /// Where `<n>` is 8 bit colors
    ByteFg(u8),
    /// Represents the SGR codes `38;2;<n1>;<n2>;<n3>`
    ///
    /// Where `<n1>`,`<n2>`,`<n3>` are 8 bit colors
    RgbFg(u8, u8, u8),
    /// Represents the SGR code `39`
    DefaultFg,

    /// Represents the SGR code `40`
    BlackBg,
    /// Represents the SGR code `41`
    RedBg,
    /// Represents the SGR code `42`
    GreenBg,
    /// Represents the SGR code `43`
    YellowBg,
    /// Represents the SGR code `44`
    BlueBg,
    /// Represents the SGR code `45`
    MagentaBg,
    /// Represents the SGR code `46`
    CyanBg,
    /// Represents the SGR code `47`
    WhiteBg,
    /// Represents the SGR codes `48;2;<n>`
    ///
    /// Where `<n>` is 8 bit colors
    ByteBg(u8),
    /// Represents the SGR codes `38;2;<n1>;<n2>;<n3>`
    ///
    /// Where `<n1>`,`<n2>`,`<n3>` are 8 bit colors
    RgbBg(u8, u8, u8),
    /// Represents the SGR code `49`
    DefaultBg,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.standard_display(f)
    }
}

impl InlineSGR for Color {
    fn write<W>(&self, writer: &mut StandardWriter<W>) -> Result<(), W::Error>
    where
        W: CapableWriter,
    {
        use Color::*;
        match self {
            BlackFg => writer.write_code(30),
            RedFg => writer.write_code(31),
            GreenFg => writer.write_code(32),
            YellowFg => writer.write_code(33),
            BlueFg => writer.write_code(34),
            MagentaFg => writer.write_code(35),
            CyanFg => writer.write_code(36),
            WhiteFg => writer.write_code(37),
            ByteFg(n) => writer.write_multiple(&[38, 2, *n]),
            RgbFg(r, g, b) => writer.write_multiple(&[38, 5, *r, *g, *b]),
            DefaultFg => writer.write_code(39),

            BlackBg => writer.write_code(40),
            RedBg => writer.write_code(41),
            GreenBg => writer.write_code(42),
            YellowBg => writer.write_code(43),
            BlueBg => writer.write_code(44),
            MagentaBg => writer.write_code(45),
            CyanBg => writer.write_code(46),
            WhiteBg => writer.write_code(47),
            ByteBg(n) => writer.write_multiple(&[48, 2, *n]),
            RgbBg(r, g, b) => writer.write_multiple(&[48, 5, *r, *g, *b]),
            DefaultBg => writer.write_code(49),
        }
    }
}
