use std::fmt::{Debug, Display};

use crate::{
    writing::{SGRWriter, StandardWriter},
    Clear,
};

use self::inline::{Color, Style};

/// Implements whats supposed to be used inline of a string literal
pub mod inline;

/// A String encapsulating SGR codes
///
/// SGR codes are applied when the [`Display`] trait is used,
/// or when the [`SGRString::place_all`] or [`SGRString::clean_all`]
/// functions are called.
#[derive(Default, Debug)]
pub struct SGRString {
    /// The actual text
    pub text: String,
    /// The SGR grahics
    pub graphics: Graphics,
}
impl SGRString {
    /// Writes all contained SGR codes to the given [`SGRWriter`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn place_all<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        if self.no_places() {
            return Ok(());
        }
        writer.escape()?;
        if self.graphics.reset {
            writer.write_code(0)?;
        }
        self.place_colors(writer)?;
        self.place_styles(writer)?;
        self.place_custom(writer)?;
        writer.end()
    }
    /// Writes SGR color codes to the given [`SGRWriter`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn place_colors<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        use ColorKind::*;
        match self.graphics.foreground {
            Black => writer.write_code(30)?,
            Red => writer.write_code(31)?,
            Green => writer.write_code(32)?,
            Yellow => writer.write_code(33)?,
            Blue => writer.write_code(34)?,
            Magenta => writer.write_code(35)?,
            Cyan => writer.write_code(36)?,
            White => writer.write_code(37)?,
            Byte(n) => writer.write_multiple(&[38, 2, n])?,
            Rgb(r, g, b) => writer.write_multiple(&[38, 5, r, g, b])?,
            Default => writer.write_code(39)?,
            ColorKind::None => (),
        }
        match self.graphics.background {
            Black => writer.write_code(40)?,
            Red => writer.write_code(41)?,
            Green => writer.write_code(42)?,
            Yellow => writer.write_code(43)?,
            Blue => writer.write_code(44)?,
            Magenta => writer.write_code(45)?,
            Cyan => writer.write_code(46)?,
            White => writer.write_code(47)?,
            Byte(n) => writer.write_multiple(&[48, 2, n])?,
            Rgb(r, g, b) => writer.write_multiple(&[48, 5, r, g, b])?,
            Default => writer.write_code(49)?,
            ColorKind::None => (),
        }
        Ok(())
    }
    /// Writes SGR style codes to the given [`SGRWriter`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn place_styles<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        use StyleKind::*;
        for (kind, place, clear) in [
            (&self.graphics.bold, 1, 22),
            (&self.graphics.dim, 2, 22),
            (&self.graphics.italic, 3, 23),
            (&self.graphics.underline, 4, 24),
            (&self.graphics.blinking, 5, 25),
            (&self.graphics.inverse, 7, 27),
            (&self.graphics.hidden, 8, 28),
            (&self.graphics.strikethrough, 9, 29),
        ] {
            match kind {
                None => (),
                Place => writer.write_code(place)?,
                Clean => writer.write_code(clear)?,
            }
        }
        Ok(())
    }
    /// Writes custom SGR codes to the given [`SGRWriter`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn place_custom<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        writer.write_multiple(&self.graphics.custom_places)
    }
    /// Writes the contained SGR codes to the given [`SGRWriter`]
    ///
    /// Reverses the effects of [`SGRString::place_all`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn clean_all<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        match self.graphics.clear {
            ClearKind::Reset => {
                writer.escape()?;
                writer.write_code(0)?;
                writer.end()
            }
            ClearKind::Clean if !self.no_clears() => {
                writer.escape()?;
                self.clean_colors(writer)?;
                self.clean_styles(writer)?;
                self.clean_custom(writer)?;
                writer.end()
            }
            _ => Ok(()),
        }
    }
    /// Writes SGR color codes to the given [`SGRWriter`]
    ///
    /// Reverses the effects of [`SGRString::place_colors`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn clean_colors<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        if self.graphics.foreground != ColorKind::None {
            writer.write_code(39)?;
        }
        if self.graphics.background != ColorKind::None {
            writer.write_code(49)?;
        }
        Ok(())
    }
    /// Writes SGR style codes to the given [`SGRWriter`]
    ///
    /// Reverses the effects of [`SGRString::place_styles`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn clean_styles<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        for (kind, place, clear) in [
            (&self.graphics.bold, 22, 1),
            (&self.graphics.dim, 22, 2),
            (&self.graphics.italic, 23, 3),
            (&self.graphics.underline, 24, 4),
            (&self.graphics.blinking, 25, 5),
            (&self.graphics.inverse, 27, 7),
            (&self.graphics.hidden, 28, 8),
            (&self.graphics.strikethrough, 29, 9),
        ] {
            match kind {
                StyleKind::None => (),
                StyleKind::Place => writer.write_code(place)?,
                StyleKind::Clean => writer.write_code(clear)?,
            }
        }
        Ok(())
    }
    /// Writes SGR codes to the given [`SGRWriter`]
    ///
    /// Reverses the effects of [`SGRString::place_custom`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn clean_custom<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: SGRWriter,
    {
        writer.write_multiple(&self.graphics.custom_cleans)
    }
    /// Checks if any SGR codes should be written
    ///
    /// Is used to prevent an empty SGR sequence, `\x1b[m`, which
    /// would be interpreted by most terminals as `\x1b[0m`, resetting
    /// all.
    ///
    /// # Returns
    ///
    /// true if there are no SGR codes to write, else false
    ///
    #[must_use]
    pub fn no_places(&self) -> bool {
        self.graphics.custom_places.is_empty()
            && self.graphics.foreground == ColorKind::None
            && self.graphics.background == ColorKind::None
            && !self.graphics.reset
            && self.graphics.bold == StyleKind::None
            && self.graphics.dim == StyleKind::None
            && self.graphics.italic == StyleKind::None
            && self.graphics.underline == StyleKind::None
            && self.graphics.blinking == StyleKind::None
            && self.graphics.inverse == StyleKind::None
            && self.graphics.hidden == StyleKind::None
            && self.graphics.strikethrough == StyleKind::None
    }
    /// Checks if any SGR codes should be written
    ///
    /// Is used to prevent an empty SGR sequence, `\x1b[m`, which
    /// would be interpreted by most terminals as `\x1b[0m`, resetting
    /// all.
    ///
    /// # Returns
    ///
    /// true if there are no SGR codes to write, else false
    ///
    #[must_use]
    pub fn no_clears(&self) -> bool {
        self.graphics.clear == ClearKind::None
            || (self.graphics.custom_cleans.is_empty()
                && self.graphics.foreground == ColorKind::None
                && self.graphics.background == ColorKind::None
                && self.graphics.bold == StyleKind::None
                && self.graphics.dim == StyleKind::None
                && self.graphics.italic == StyleKind::None
                && self.graphics.underline == StyleKind::None
                && self.graphics.blinking == StyleKind::None
                && self.graphics.inverse == StyleKind::None
                && self.graphics.hidden == StyleKind::None
                && self.graphics.strikethrough == StyleKind::None)
    }
}
impl From<Clear> for SGRString {
    fn from(value: Clear) -> Self {
        Self::default().clear(value)
    }
}
impl From<Color> for SGRString {
    fn from(value: Color) -> Self {
        Self::default().color(value)
    }
}
impl From<Style> for SGRString {
    fn from(value: Style) -> Self {
        Self::default().style(value)
    }
}
impl From<&str> for SGRString {
    fn from(value: &str) -> Self {
        Self {
            text: String::from(value),
            ..Default::default()
        }
    }
}
impl From<String> for SGRString {
    fn from(value: String) -> Self {
        Self {
            text: value,
            ..Default::default()
        }
    }
}
impl From<&String> for SGRString {
    fn from(value: &String) -> Self {
        Self {
            text: String::from(value),
            ..Default::default()
        }
    }
}
impl Display for SGRString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = StandardWriter::fmt(f);
        fmt.place_sgr(self)?;
        fmt.write_inner(&self.text)?;
        fmt.clean_sgr(self)
    }
}
/// The graphics portion of a [`SGRString`]
#[derive(Debug, Default)]
pub struct Graphics {
    /// The type of clean to apply after the string
    ///
    /// By default [`ClearKind::None`], meaning nothing is done
    pub clear: ClearKind,

    /// Any custom codes added
    ///
    /// These codes are written before the string when
    /// the [`Display`] trait is called
    pub custom_places: Vec<u8>,
    /// Any custom codes added
    ///
    /// These codes are written after the string when
    /// the [`Display`] trait is called
    pub custom_cleans: Vec<u8>,

    /// The color of the foreground
    ///
    /// By default [`ColorKind::None`], meaning nothing is applied.
    /// This differs from [`ColorKind::Default`], where the default SGR
    /// code for foreground is applied.
    pub foreground: ColorKind,
    /// The color of the background
    ///
    /// By default [`ColorKind::None`], meaning nothing is applied.
    /// This differs from [`ColorKind::Default`], where the default SGR
    /// code for background is applied.
    pub background: ColorKind,

    /// Determines whether the clear code `0` is to be applied to the beggining
    ///
    /// Not be confused with [`SGRString.clear`], this only has effect on [`SGRString::place_all`]
    pub reset: bool,
    /// Refer to [`StyleKind`]
    pub bold: StyleKind,
    /// Refer to [`StyleKind`]
    pub dim: StyleKind,
    /// Refer to [`StyleKind`]
    pub italic: StyleKind,
    /// Refer to [`StyleKind`]
    pub underline: StyleKind,
    /// Refer to [`StyleKind`]
    pub blinking: StyleKind,
    /// Refer to [`StyleKind`]
    pub inverse: StyleKind,
    /// Refer to [`StyleKind`]
    pub hidden: StyleKind,
    /// Refer to [`StyleKind`]
    pub strikethrough: StyleKind,
}
/// The type of clear to apply
#[derive(Debug, Default, PartialEq, Eq)]
pub enum ClearKind {
    /// Do nothing
    #[default]
    None,
    /// Apply the reset all code
    Reset,
    /// Applies a reversing effect to everything.
    /// This is dependant on where its used
    Clean,
}
impl From<Clear> for ClearKind {
    fn from(value: Clear) -> Self {
        match value {
            Clear::Reset => Self::Reset,
            Clear::Clean => Self::Clean,
        }
    }
}
/// Component of [`SGRString`]; the type of style to apply
#[derive(Debug, Default, PartialEq, Eq)]
pub enum StyleKind {
    /// Do nothing
    #[default]
    None,
    /// Apply the style
    Place,
    /// Apply what reverses the style
    Clean,
}
/// Component of [`SGRString`]; the type of color
#[derive(Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ColorKind {
    /// Does nothing
    #[default]
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u8),
    Rgb(u8, u8, u8),
    /// Applies the default `SGR` color
    Default,
}
impl<I: Into<SGRString>> EasySGR for I {}
/// Allows for chaining SGR code types
///
/// Methods return a [`SGRString`]
#[allow(missing_docs)]
pub trait EasySGR: Into<SGRString> {
    /// Turns self into [`SGRString`]
    ///
    /// Equivalant to calling
    ///```rust
    /// # use easy_sgr::SGRString;
    /// # pub trait EasySGR: Into<SGRString> {
    /// # fn to_sgr(self) -> SGRString {
    ///Into::<SGRString>::into(self)
    /// # }
    /// # }
    ///```
    #[must_use]
    #[inline]
    fn to_sgr(self) -> SGRString {
        self.into()
    }
    /// Sets the plaintext of the returned [`SGRString`]  
    #[must_use]
    #[inline]
    fn text(self, text: impl Into<String>) -> SGRString {
        SGRString {
            text: text.into(),
            ..self.into()
        }
    }
    /// Adds a style to the returned [`SGRString`]  
    #[must_use]
    #[inline]
    fn style(self, style: impl Into<inline::Style>) -> SGRString {
        use Style::*;
        use StyleKind::*;

        let mut this = self.into();
        match style.into() {
            Bold => this.graphics.bold = Place,
            Dim => this.graphics.dim = Place,
            Italic => this.graphics.italic = Place,
            Underline => this.graphics.underline = Place,
            Blinking => this.graphics.blinking = Place,
            Inverse => this.graphics.inverse = Place,
            Hidden => this.graphics.hidden = Place,
            Strikethrough => this.graphics.strikethrough = Place,

            ClearBold => this.graphics.bold = Clean,
            ClearDim => this.graphics.dim = Clean,
            ClearItalic => this.graphics.italic = Clean,
            ClearUnderline => this.graphics.underline = Clean,
            ClearBlinking => this.graphics.blinking = Clean,
            ClearInverse => this.graphics.inverse = Clean,
            ClearHidden => this.graphics.hidden = Clean,
            ClearStrikethrough => this.graphics.strikethrough = Clean,
        }
        this
    }
    /// Adds a color(foreground or background) to the returned [`SGRString`]  
    #[must_use]
    #[inline]
    fn color(self, color: impl Into<inline::Color>) -> SGRString {
        use {Color::*, ColorKind::*};

        let mut this = self.into();
        match color.into() {
            BlackFg => this.graphics.foreground = Black,
            RedFg => this.graphics.foreground = Red,
            GreenFg => this.graphics.foreground = Green,
            YellowFg => this.graphics.foreground = Yellow,
            BlueFg => this.graphics.foreground = Blue,
            MagentaFg => this.graphics.foreground = Magenta,
            CyanFg => this.graphics.foreground = Cyan,
            WhiteFg => this.graphics.foreground = White,
            ByteFg(n) => this.graphics.foreground = Byte(n),
            RgbFg(r, g, b) => this.graphics.foreground = Rgb(r, g, b),
            DefaultFg => this.graphics.foreground = Default,

            BlackBg => this.graphics.background = Black,
            RedBg => this.graphics.background = Red,
            GreenBg => this.graphics.background = Green,
            YellowBg => this.graphics.background = Yellow,
            BlueBg => this.graphics.background = Blue,
            MagentaBg => this.graphics.background = Magenta,
            CyanBg => this.graphics.background = Cyan,
            WhiteBg => this.graphics.background = White,
            ByteBg(n) => this.graphics.background = Byte(n),
            RgbBg(r, g, b) => this.graphics.background = Rgb(r, g, b),
            DefaultBg => this.graphics.background = Default,
        }
        this
    }
    /// Adds a custom code to the returned [`SGRString`]
    ///
    /// Adds to the [`clear`](SGRString#structfield.custom_places)
    #[must_use]
    #[inline]
    fn custom(self, code: impl Into<u8>) -> SGRString {
        let mut this = self.into();
        this.graphics.custom_places.push(code.into());
        this
    }
    #[must_use]
    #[inline]
    fn clear(self, clear: impl Into<ClearKind>) -> SGRString {
        let mut this = self.into();
        this.graphics.clear = clear.into();
        this
    }
    #[must_use]
    #[inline]
    fn custom_place(self, code: impl Into<u8>) -> SGRString {
        let mut this = self.into();
        this.graphics.custom_places.push(code.into());
        this
    }
    #[must_use]
    #[inline]
    fn custom_clean(self, code: impl Into<u8>) -> SGRString {
        let mut this = self.into();
        this.graphics.custom_cleans.push(code.into());
        this
    }
}
