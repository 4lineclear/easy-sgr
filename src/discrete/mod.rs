use std::fmt::Display;

use crate::{EasySGR, SGRBuilder, SGRWriter};

/// Implements [`FromStr`](std::str::FromStr) for the [`discrete`](crate::discrete) module
pub mod from_str;

/// An SGR style code's end & escape
///
/// Intended use case is when the `partial` feature is enable
///
/// # Examples
///
/// Using it with `partial` enabled:
///  
///```rust
///use easy_sgr::{Color::*, Seq::*, Style::*};
///
///println!("{Esc}{Bold};{BlueBg}{End}This should be bold & italic!{Esc}{Reset}{End}");
///```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Seq {
    /// The sequence escape string, `\x1b[`
    Esc,
    /// The sequence end string, `m`
    End,
}
impl Display for Seq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Esc => "\x1b[",
            Self::End => "m",
        })
    }
}
/// An SGR style code
///
/// # Examples
///
///```rust
///use easy_sgr::Style::*;
///
///println!(
///"
///{Bold}{Italic}This text is bold & italic
///{Underline}This text is also underline
///{NotUnderline}Now back to bold & italic
///{Reset}And lastly normal text"
///);
///```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Style {
    /// Represents the SGR code `0`
    ///
    /// Resets all(including color & custom codes) to the terminal's default
    Reset,
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
    ///
    /// Is equivalent to [`Style::NotDim`]
    NotBold,
    /// Represents the SGR code `22`
    ///
    /// Is equivalent to [`Style::NotBold`]
    NotDim,
    /// Represents the SGR code `23`
    NotItalic,
    /// Represents the SGR code `24`
    NotUnderline,
    /// Represents the SGR code `25`
    NotBlinking,
    /// Represents the SGR code `27`
    NotInverse,
    /// Represents the SGR code `28`
    NotHidden,
    /// Represents the SGR code `29`
    NotStrikethrough,
}
impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.standard_display(f)
    }
}
impl DiscreteSGR for Style {
    fn write(&self, builder: &mut SGRBuilder) {
        use Style::*;
        builder.write_code(match self {
            Reset => 0,
            Bold => 1,
            Dim => 2,
            Italic => 3,
            Underline => 4,
            Blinking => 5,
            Inverse => 7,
            Hidden => 8,
            Strikethrough => 9,
            NotBold | NotDim => 22,
            NotItalic => 23,
            NotUnderline => 24,
            NotBlinking => 25,
            NotInverse => 27,
            NotHidden => 28,
            NotStrikethrough => 29,
        });
    }
}
/// An SGR color code
///
/// # Examples
///
///```rust
///use easy_sgr::Color::*;
///
///println!("{RedFg}0This text color is red!");
///println!("{BlackBg}And now its background is white");
///println!("{DefaultBg}Now back to just red");
///println!("{DefaultFg}Finally normal text");
///```
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Where `<n>` is an 8 bit color
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
    /// Where `<n>` is an 8 bit color
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
impl DiscreteSGR for Color {
    fn write(&self, builder: &mut SGRBuilder) {
        use Color::*;
        match self {
            BlackFg => builder.write_code(30),
            RedFg => builder.write_code(31),
            GreenFg => builder.write_code(32),
            YellowFg => builder.write_code(33),
            BlueFg => builder.write_code(34),
            MagentaFg => builder.write_code(35),
            CyanFg => builder.write_code(36),
            WhiteFg => builder.write_code(37),
            ByteFg(n) => builder.write_codes(&[38, 2, *n]),
            RgbFg(r, g, b) => builder.write_codes(&[38, 5, *r, *g, *b]),
            DefaultFg => builder.write_code(39),

            BlackBg => builder.write_code(40),
            RedBg => builder.write_code(41),
            GreenBg => builder.write_code(42),
            YellowBg => builder.write_code(43),
            BlueBg => builder.write_code(44),
            MagentaBg => builder.write_code(45),
            CyanBg => builder.write_code(46),
            WhiteBg => builder.write_code(47),
            ByteBg(n) => builder.write_codes(&[48, 2, *n]),
            RgbBg(r, g, b) => builder.write_codes(&[48, 5, *r, *g, *b]),
            DefaultBg => builder.write_code(49),
        }
    }
}
/// Represents SGR sequences that can be used discretely.
///
/// This means it doesn't exist in terms of a [`SGRString`](crate::SGRString),
/// though it can be used in conjunction with one
#[allow(clippy::module_name_repetitions)]
pub trait DiscreteSGR: Sized + Display + EasySGR {
    /// Writes a set of SGR codes to the given [`SGRWriter`]
    ///
    /// Writing is not an IO operation, instead writing
    /// pushes codes to the [`SGRBuilder`]'s buffer
    fn write(&self, writer: &mut SGRBuilder);
    /// Writes an SGR sequence to the given [`Formatter`](std::fmt::Formatter)
    ///
    /// # Errors
    ///
    /// Return an error if writing to the [`Formatter`](std::fmt::Formatter) fails
    #[inline]
    #[cfg(not(feature = "partial"))]
    fn standard_display(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        SGRWriter::from(f).inline_sgr(self)
    }
    /// Writes an SGR sequence to the given [`Formatter`](std::fmt::Formatter)
    ///
    /// Uses [`SGRWriter::partial_sgr`], so the sequence end & escape strings
    /// are not written
    ///
    /// # Errors
    ///
    /// Return an error if writing to the [`Formatter`](std::fmt::Formatter) fails
    #[inline]
    #[cfg(feature = "partial")]
    fn standard_display(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        SGRWriter::from(f).partial_sgr(self)
    }
}
