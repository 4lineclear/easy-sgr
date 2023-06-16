
use crate::{
    inline::{Color, Style},
    Ansi,
};

use StyleKind::*;

#[derive(Debug, Default, Clone)]
pub struct Graphics {
    pub custom_places: Vec<u8>,
    pub custom_clears: Vec<u8>,

    pub foreground: Option<ColorKind>,
    pub background: Option<ColorKind>,

    pub clear: ClearKind,

    pub reset: bool,

    pub bold: StyleKind,
    pub dim: StyleKind,
    pub italic: StyleKind,
    pub underline: StyleKind,
    pub blinking: StyleKind,
    pub inverse: StyleKind,
    pub hidden: StyleKind,
    pub strikethrough: StyleKind,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ClearKind {
    #[default]
    Skip,
    Full,
    Clean,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum ColorKind {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    EightBit(u8),
    Rgb(u8, u8, u8),
    #[default]
    Default,
}
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum StyleKind {
    #[default]
    None,
    Place,
    Clear,
    Both,
}

impl StyleKind {
    #[must_use]
    pub fn shift(&self, other: StyleKind) -> Self {
        match self {
            None => other,
            Place if other == Clear => Both,
            Clear if other == Place => Both,
            _ => *self,
        }
    }
    #[must_use]
    pub fn is_none(&self) -> bool {
        *self == None
    }
}

impl Graphics {
    #[inline]
    #[must_use]
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        use Style::*;

        match style.into() {
            Reset => self.reset = true,

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

    #[inline]
    #[must_use]
    pub fn color(mut self, color: impl Into<crate::inline::Color>) -> Graphics {
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
    #[inline]
    #[must_use]
    pub fn clear(mut self, clear_kind: impl Into<ClearKind>) -> Self {
        self.clear = clear_kind.into();
        self
    }
    #[inline]
    #[must_use]
    pub fn foreground(mut self, color: impl Into<ColorKind>) -> Self {
        self.foreground = Some(color.into());
        self
    }
    #[inline]
    #[must_use]
    pub fn background(mut self, color: impl Into<ColorKind>) -> Self {
        self.background = Some(color.into());
        self
    }
    #[inline]
    #[must_use]
    pub fn custom_place(mut self, code: u8) -> Self {
        self.custom_places.push(code);
        self
    }
    #[inline]
    #[must_use]
    pub fn custom_clear(mut self, code: u8) -> Self {
        self.custom_clears.push(code);
        self
    }
}

impl Ansi for Graphics {
    fn place_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: crate::write::AnsiWriter,
    {
        use ColorKind::*;
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
        writer.write_multiple(&self.custom_places)
    }

    fn clear_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: crate::write::AnsiWriter,
    {
        match self.clear {
            ClearKind::Skip => Ok(()),
            ClearKind::Full => writer.write_code(0),
            ClearKind::Clean => {
                if self.foreground.is_some() {
                    writer.write_code(39)?;
                }
                if self.background.is_some() {
                    writer.write_code(49)?;
                }

                for (kind, place, clear) in [
                    (self.bold, 22, 1),
                    (self.dim, 22, 2),
                    (self.italic, 23, 3),
                    (self.underline, 24, 4),
                    (self.blinking, 25, 5),
                    (self.inverse, 27, 7),
                    (self.hidden, 28, 8),
                    (self.strikethrough, 29, 9),
                ] {
                    match kind {
                        None => (),
                        Place | Both => writer.write_code(place)?,
                        Clear => writer.write_code(clear)?,
                    }
                }
                writer.write_multiple(&self.custom_clears)
            }
        }
    }
    fn no_places(&self) -> bool {
        self.custom_places.is_empty()
            && self.foreground.is_none()
            && self.background.is_none()
            && !self.reset
            && self.bold.is_none()
            && self.dim.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.blinking.is_none()
            && self.inverse.is_none()
            && self.hidden.is_none()
            && self.strikethrough.is_none()
    }

    fn no_clears(&self) -> bool {
        self.clear == ClearKind::Skip
            || (self.custom_clears.is_empty()
                && self.foreground.is_none()
                && self.foreground.is_none()
                && self.bold.is_none()
                && self.dim.is_none()
                && self.italic.is_none()
                && self.underline.is_none()
                && self.blinking.is_none()
                && self.inverse.is_none()
                && self.hidden.is_none()
                && self.strikethrough.is_none())
    }
}
