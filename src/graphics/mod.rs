use std::ops::Deref;

use crate::writing::AnsiWriter;

pub mod display;

/// A String with added ANSI codes
#[derive(Default, Debug)]
pub struct AnsiString {
    pub text: String,
    pub clear: ClearKind,

    pub custom_places: Vec<u8>,
    pub custom_cleans: Vec<u8>,

    pub foreground: ColorKind,
    pub background: ColorKind,

    pub reset: StyleKind,
    pub bold: StyleKind,
    pub dim: StyleKind,
    pub italic: StyleKind,
    pub underline: StyleKind,
    pub blinking: StyleKind,
    pub inverse: StyleKind,
    pub hidden: StyleKind,
    pub strikethrough: StyleKind,
}

impl AnsiString {
    /// Writes the contained ANSI codes to the given [`AnsiWriter`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn place<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter,
    {
        use ColorKind::*;
        use StyleKind::*;
        if self.no_places() {
            return Ok(());
        }
        writer.escape()?;
        match self.foreground {
            Black => writer.write_code(30)?,
            Red => writer.write_code(31)?,
            Green => writer.write_code(32)?,
            Yellow => writer.write_code(33)?,
            Blue => writer.write_code(34)?,
            Magenta => writer.write_code(35)?,
            Cyan => writer.write_code(36)?,
            White => writer.write_code(37)?,
            EightBit(n) => writer.write_multiple(&[38, 2, n])?,
            RGB(r, g, b) => writer.write_multiple(&[38, 5, r, g, b])?,
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
            EightBit(n) => writer.write_multiple(&[48, 2, n])?,
            RGB(r, g, b) => writer.write_multiple(&[48, 5, r, g, b])?,
            Default => writer.write_code(49)?,
            ColorKind::None => (),
        }

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
                StyleKind::None => (),
                Place => writer.write_code(place)?,
                Clear => writer.write_code(clear)?,
            }
        }
        writer.write_multiple(&self.custom_places)?;
        writer.end()
    }
    /// Writes the contained ANSI codes to the given [`AnsiWriter`]
    ///
    /// Reverses the effects of [`AnsiString::place`], depending on [`clear`](#structfield.clear)
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails
    pub fn clean<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: AnsiWriter,
    {
        match self.clear {
            ClearKind::Full => {
                writer.escape()?;
                writer.write_code(0)?;
                writer.end()
            }
            ClearKind::Clean if !self.no_clears() => {
                writer.escape()?;
                if self.foreground != ColorKind::None {
                    writer.write_code(39)?;
                }
                if self.background != ColorKind::None {
                    writer.write_code(49)?;
                }

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
                        StyleKind::Clear => writer.write_code(clear)?,
                    }
                }
                writer.write_multiple(&self.custom_cleans)?;
                writer.end()
            }
            _ => Ok(()),
        }
    }
    /// Checks if any ANSI codes should be written
    ///
    /// Is used to prevent an empty ANSI sequence, `\x1b[m`, which
    /// would be interpreted by most terminals as `\x1b[0m`, resetting
    /// all.
    ///
    /// # Returns
    ///
    /// true if there are no ANSI codes to write, else false
    ///
    #[must_use]
    pub fn no_places(&self) -> bool {
        self.custom_places.is_empty()
            && self.foreground == ColorKind::None
            && self.background == ColorKind::None
            && self.reset == StyleKind::None
            && self.bold == StyleKind::None
            && self.dim == StyleKind::None
            && self.italic == StyleKind::None
            && self.underline == StyleKind::None
            && self.blinking == StyleKind::None
            && self.inverse == StyleKind::None
            && self.hidden == StyleKind::None
            && self.strikethrough == StyleKind::None
    }
    /// Checks if any ANSI codes should be written
    ///
    /// Is used to prevent an empty ANSI sequence, `\x1b[m`, which
    /// would be interpreted by most terminals as `\x1b[0m`, resetting
    /// all.
    ///
    /// # Returns
    ///
    /// true if there are no ANSI codes to write, else false
    ///
    #[must_use]
    pub fn no_clears(&self) -> bool {
        self.clear == ClearKind::Skip
            || (self.custom_cleans.is_empty()
                && self.foreground == ColorKind::None
                && self.background == ColorKind::None
                && self.reset == StyleKind::None
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

/// A set of methods to turn a type into [`AnsiString`]
///
/// All types that implement `Into<AnsiString>` implement this as well through
/// the blanket implmentation:
///
/// ```ignore
/// impl<I: Into<AnsiString>> ToAnsiString for I {}
/// ```
pub trait ToAnsiString: Into<AnsiString> {
    /// Turns self into [`AnsiString`]
    ///
    /// Equivalant to calling
    ///```ignore
    /// Into::<AnsiString>::into(self)
    ///```
    #[must_use]
    #[inline(always)]
    fn to_ansi_string(self) -> AnsiString {
        self.into()
    }
    #[must_use]
    fn clear(self, clear: impl Into<ClearKind>) -> AnsiString {
        let mut this = self.into();
        this.clear = clear.into();
        this
    }
    #[must_use]
    fn custom_place(self, code: impl Into<u8>) -> AnsiString {
        let mut this = self.into();
        this.custom_places.push(code.into());
        this
    }
    #[must_use]
    fn custom_clean(self, code: impl Into<u8>) -> AnsiString {
        let mut this = self.into();
        this.custom_cleans.push(code.into());
        this
    }
    #[must_use]
    fn foreground(self, color: impl Into<ColorKind>) -> AnsiString {
        let mut this = self.into();
        this.foreground = color.into();
        this
    }
    #[must_use]
    fn background(self, color: impl Into<ColorKind>) -> AnsiString {
        let mut this = self.into();
        this.background = color.into();
        this
    }
    #[must_use]
    fn reset(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.reset = style;
        this
    }
    #[must_use]
    fn bold(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.bold = style;
        this
    }
    #[must_use]
    fn dim(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.dim = style;
        this
    }
    #[must_use]
    fn italic(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.italic = style;
        this
    }
    #[must_use]
    fn underline(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.underline = style;
        this
    }
    #[must_use]
    fn blinking(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.blinking = style;
        this
    }
    #[must_use]
    fn inverse(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.inverse = style;
        this
    }
    #[must_use]
    fn hidden(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.hidden = style;
        this
    }
    #[must_use]
    fn strikethrough(self, style: StyleKind) -> AnsiString {
        let mut this = self.into();
        this.strikethrough = style;
        this
    }
}

impl<I: Into<AnsiString>> ToAnsiString for I {}

impl From<&str> for AnsiString {
    fn from(value: &str) -> Self {
        Self {
            text: String::from(value),
            ..Default::default()
        }
    }
}
impl From<String> for AnsiString {
    fn from(value: String) -> Self {
        Self {
            text: value,
            ..Default::default()
        }
    }
}
impl From<&String> for AnsiString {
    fn from(value: &String) -> Self {
        Self {
            text: String::from(value),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ClearKind {
    #[default]
    Skip,
    Full,
    Clean,
}
#[derive(Debug, Default, PartialEq, Eq)]
pub enum StyleKind {
    #[default]
    None,
    Place,
    Clear,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ColorKind {
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
    EightBit(u8),
    RGB(u8, u8, u8),
    Default,
}
