use std::fmt::{Debug, Display, Write};

use crate::writing::{FmtWriter, SGRWriter};

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
        if self.reset {
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
        match self.foreground {
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
        match self.background {
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
            (&self.bold, 1, 22),
            (&self.dim, 2, 22),
            (&self.italic, 3, 23),
            (&self.underline, 4, 24),
            (&self.blinking, 5, 25),
            (&self.inverse, 7, 27),
            (&self.hidden, 8, 28),
            (&self.strikethrough, 9, 29),
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
        writer.write_multiple(&self.custom_places)
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
        match self.clear {
            ClearKind::Full => {
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
        if self.foreground != ColorKind::None {
            writer.write_code(39)?;
        }
        if self.background != ColorKind::None {
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
            (&self.bold, 22, 1),
            (&self.dim, 22, 2),
            (&self.italic, 23, 3),
            (&self.underline, 24, 4),
            (&self.blinking, 25, 5),
            (&self.inverse, 27, 7),
            (&self.hidden, 28, 8),
            (&self.strikethrough, 29, 9),
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
        writer.write_multiple(&self.custom_cleans)
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
        self.custom_places.is_empty()
            && self.foreground == ColorKind::None
            && self.background == ColorKind::None
            && !self.reset
            && self.bold == StyleKind::None
            && self.dim == StyleKind::None
            && self.italic == StyleKind::None
            && self.underline == StyleKind::None
            && self.blinking == StyleKind::None
            && self.inverse == StyleKind::None
            && self.hidden == StyleKind::None
            && self.strikethrough == StyleKind::None
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
        self.clear == ClearKind::None
            || (self.custom_cleans.is_empty()
                && self.foreground == ColorKind::None
                && self.background == ColorKind::None
                && self.bold == StyleKind::None
                && self.dim == StyleKind::None
                && self.italic == StyleKind::None
                && self.underline == StyleKind::None
                && self.blinking == StyleKind::None
                && self.inverse == StyleKind::None
                && self.hidden == StyleKind::None
                && self.strikethrough == StyleKind::None)
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
        let mut fmt = FmtWriter::new(f);
        fmt.place_sgr(self)?;
        fmt.write_str(&self.text)?;
        fmt.clean_sgr(self)
    }
}

/// The type of clear to apply
#[derive(Debug, Default, PartialEq, Eq)]
pub enum ClearKind {
    /// Do nothing
    #[default]
    None,
    /// Full apply the reset all code
    Full,
    /// Applies a reversing effect to everything.
    /// This is dependant on where its used
    Clean,
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

impl<I: Into<SGRString> + Debug> EasySGR for I {}
/// Allows for chaining types that implement it
///
/// Inner workins are:
/// ```ignore
/// self.into().<graphic>(<graphic>)
/// ```
/// Where `<graphic>` is a type of SGR graphics code such as [`Style`] or [`Color`]
#[allow(missing_docs)]
pub trait EasySGR: Into<SGRString> + Debug {
    /// Turns self into [`SGRString`]
    ///
    /// Equivalant to calling
    ///```ignore
    /// Into::<SGRString>::into(self)
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
            ..Default::default()
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
            Reset => this.reset = true,
            Bold => this.bold = Place,
            Dim => this.dim = Place,
            Italic => this.italic = Place,
            Underline => this.underline = Place,
            Blinking => this.blinking = Place,
            Inverse => this.inverse = Place,
            Hidden => this.hidden = Place,
            Strikethrough => this.strikethrough = Place,

            ClearBold => this.bold = Clean,
            ClearDim => this.dim = Clean,
            ClearItalic => this.italic = Clean,
            ClearUnderline => this.underline = Clean,
            ClearBlinking => this.blinking = Clean,
            ClearInverse => this.inverse = Clean,
            ClearHidden => this.hidden = Clean,
            ClearStrikethrough => this.strikethrough = Clean,
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
            BlackFg => this.foreground = Black,
            RedFg => this.foreground = Red,
            GreenFg => this.foreground = Green,
            YellowFg => this.foreground = Yellow,
            BlueFg => this.foreground = Blue,
            MagentaFg => this.foreground = Magenta,
            CyanFg => this.foreground = Cyan,
            WhiteFg => this.foreground = White,
            ByteFg(n) => this.foreground = Byte(n),
            RgbFg(r, g, b) => this.foreground = Rgb(r, g, b),
            DefaultFg => this.foreground = Default,

            BlackBg => this.background = Black,
            RedBg => this.background = Red,
            GreenBg => this.background = Green,
            YellowBg => this.background = Yellow,
            BlueBg => this.background = Blue,
            MagentaBg => this.background = Magenta,
            CyanBg => this.background = Cyan,
            WhiteBg => this.background = White,
            ByteBg(n) => this.background = Byte(n),
            RgbBg(r, g, b) => this.background = Rgb(r, g, b),
            DefaultBg => this.background = Default,
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
        this.custom_places.push(code.into());
        this
    }
    #[must_use]
    #[inline]
    fn clear(self, clear: impl Into<ClearKind>) -> SGRString {
        let mut this = self.into();
        this.clear = clear.into();
        this
    }
    #[must_use]
    #[inline]
    fn custom_place(self, code: impl Into<u8>) -> SGRString {
        let mut this = self.into();
        this.custom_places.push(code.into());
        this
    }
    #[must_use]
    #[inline]
    fn custom_clean(self, code: impl Into<u8>) -> SGRString {
        let mut this = self.into();
        this.custom_cleans.push(code.into());
        this
    }
    #[must_use]
    #[inline]
    fn foreground(self, color: impl Into<ColorKind>) -> SGRString {
        let mut this = self.into();
        this.foreground = color.into();
        this
    }
    #[must_use]
    #[inline]
    fn background(self, color: impl Into<ColorKind>) -> SGRString {
        let mut this = self.into();
        this.background = color.into();
        this
    }
    #[must_use]
    #[inline]
    fn reset(self, reset: bool) -> SGRString {
        let mut this = self.into();
        this.reset = reset;
        this
    }
    #[must_use]
    #[inline]
    fn bold(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.bold = style;
        this
    }
    #[must_use]
    #[inline]
    fn dim(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.dim = style;
        this
    }
    #[must_use]
    #[inline]
    fn italic(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.italic = style;
        this
    }
    #[must_use]
    #[inline]
    fn underline(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.underline = style;
        this
    }
    #[must_use]
    #[inline]
    fn blinking(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.blinking = style;
        this
    }
    #[must_use]
    #[inline]
    fn inverse(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.inverse = style;
        this
    }
    #[must_use]
    #[inline]
    fn hidden(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.hidden = style;
        this
    }
    #[must_use]
    #[inline]
    fn strikethrough(self, style: StyleKind) -> SGRString {
        let mut this = self.into();
        this.strikethrough = style;
        this
    }
}
