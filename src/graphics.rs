use crate::{
    inline::{Color, Style},
    Ansi,
};

#[derive(Debug, Default, Clone)]
pub struct Graphics {
    pub custom_places: Vec<u8>,
    pub custom_clears: Vec<u8>,

    pub foreground: Option<ColorKind>,
    pub background: Option<ColorKind>,

    pub clear: ClearKind,

    pub reset: bool,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blinking: bool,
    pub inverse: bool,
    pub hidden: bool,
    pub strikethrough: bool,
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

impl Graphics {
    #[inline]
    #[must_use]
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        use Style::*;

        match style.into() {
            Reset => self.reset = true,

            Bold => self.bold = true,
            Dim => self.dim = true,
            Italic => self.italic = true,
            Underline => self.underline = true,
            Blinking => self.blinking = true,
            Inverse => self.inverse = true,
            Hidden => self.hidden = true,
            Strikethrough => self.strikethrough = true,

            ClearBold => self.bold = false,
            ClearDim => self.dim = false,
            ClearItalic => self.italic = false,
            ClearUnderline => self.underline = false,
            ClearBlinking => self.blinking = false,
            ClearInverse => self.inverse = false,
            ClearHidden => self.hidden = false,
            ClearStrikethrough => self.strikethrough = false,
        }
        self
    }

    #[inline]
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

        for (should_write, code) in [
            (self.reset, 0),
            (self.bold, 1),
            (self.dim, 2),
            (self.italic, 3),
            (self.underline, 4),
            (self.blinking, 5),
            (self.inverse, 7),
            (self.hidden, 8),
            (self.strikethrough, 9),
        ] {
            if should_write {
                writer.write_code(code)?;
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
                for (should_write, code) in [
                    (self.bold, 22),
                    (self.dim, 22),
                    (self.italic, 23),
                    (self.underline, 24),
                    (self.blinking, 25),
                    (self.inverse, 27),
                    (self.hidden, 28),
                    (self.strikethrough, 29),
                ] {
                    if should_write {
                        writer.write_code(code)?;
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
            && !self.bold
            && !self.dim
            && !self.italic
            && !self.underline
            && !self.blinking
            && !self.inverse
            && !self.hidden
            && !self.strikethrough
    }

    fn no_clears(&self) -> bool {
        self.clear == ClearKind::Skip
            || (self.custom_clears.is_empty()
                && self.foreground.is_none()
                && self.foreground.is_none()
                && !self.bold
                && !self.dim
                && !self.italic
                && !self.underline
                && !self.blinking
                && !self.inverse
                && !self.hidden
                && !self.strikethrough)
    }
}
