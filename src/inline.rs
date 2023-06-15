use std::fmt::Display;

use crate::{
    graphics::ColorKind,
    writer::{AnsiFmt, AnsiWriter},
};

#[derive(Debug, Clone, Default)]
pub struct InlineGraphics {
    custom: Vec<u8>,

    reset: bool,

    foreground: Option<ColorKind>,
    background: Option<ColorKind>,

    place_bold: bool,
    place_dim: bool,
    place_italic: bool,
    place_underline: bool,
    place_blinking: bool,
    place_inverse: bool,
    place_hidden: bool,
    place_strikethrough: bool,

    clear_bold: bool,
    clear_dim: bool,
    clear_italic: bool,
    clear_underline: bool,
    clear_blinking: bool,
    clear_inverse: bool,
    clear_hidden: bool,
    clear_strikethrough: bool,
}

impl Display for InlineGraphics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.custom.is_empty()
            && !self.reset
            && self.foreground.is_none()
            && self.background.is_none()
            && !self.place_bold
            && !self.place_dim
            && !self.place_italic
            && !self.place_underline
            && !self.place_blinking
            && !self.place_inverse
            && !self.place_hidden
            && !self.place_strikethrough
            && !self.clear_bold
            && !self.clear_dim
            && !self.clear_italic
            && !self.clear_underline
            && !self.clear_blinking
            && !self.clear_inverse
            && !self.clear_hidden
            && !self.clear_strikethrough
        {
            true => Ok(()),
            false => {
                use ColorKind::*;
                let mut fmt = AnsiFmt::new(f);

                fmt.escape()?;
                if let Some(color) = self.foreground {
                    match color {
                        Black => fmt.write_code(30)?,
                        Red => fmt.write_code(31)?,
                        Green => fmt.write_code(32)?,
                        Yellow => fmt.write_code(33)?,
                        Blue => fmt.write_code(34)?,
                        Magenta => fmt.write_code(35)?,
                        Cyan => fmt.write_code(36)?,
                        White => fmt.write_code(37)?,
                        EightBit(n) => fmt.write_all(&[38, 2, n])?,
                        Rgb(r, g, b) => fmt.write_all(&[38, 5, r, g, b])?,
                        Default => fmt.write_code(39)?,
                    }
                };
                if let Some(color) = self.background {
                    match color {
                        Black => fmt.write_code(40)?,
                        Red => fmt.write_code(41)?,
                        Green => fmt.write_code(42)?,
                        Yellow => fmt.write_code(43)?,
                        Blue => fmt.write_code(44)?,
                        Magenta => fmt.write_code(45)?,
                        Cyan => fmt.write_code(46)?,
                        White => fmt.write_code(47)?,
                        EightBit(n) => fmt.write_all(&[48, 2, n])?,
                        Rgb(r, g, b) => fmt.write_all(&[48, 5, r, g, b])?,
                        Default => fmt.write_code(49)?,
                    }
                };

                for (should_write, code) in [
                    (self.reset, 0),
                    (self.place_bold, 1),
                    (self.place_dim, 2),
                    (self.place_italic, 3),
                    (self.place_underline, 4),
                    (self.place_blinking, 5),
                    (self.place_inverse, 7),
                    (self.place_hidden, 8),
                    (self.place_strikethrough, 9),
                ] {
                    if should_write {
                        fmt.write_code(code)?;
                    }
                }
                for (should_write, code) in [
                    (self.clear_bold, 22),
                    (self.clear_dim, 22),
                    (self.clear_italic, 23),
                    (self.clear_underline, 24),
                    (self.clear_blinking, 25),
                    (self.clear_inverse, 27),
                    (self.clear_hidden, 28),
                    (self.clear_strikethrough, 29),
                ] {
                    if should_write {
                        fmt.write_code(code)?;
                    }
                }
                fmt.write_all(&self.custom)?;
                fmt.end()
            }
        }
    }
}

impl InlineAnsi for InlineGraphics {
    #[inline]
    fn style(mut self, style: impl Into<Style>) -> Self {
        use Style::*;

        match style.into() {
            Reset => self.reset = true,

            Bold => self.place_bold = true,
            Dim => self.place_dim = true,
            Italic => self.place_italic = true,
            Underline => self.place_underline = true,
            Blinking => self.place_blinking = true,
            Inverse => self.place_inverse = true,
            Hidden => self.place_hidden = true,
            Strikethrough => self.place_strikethrough = true,

            ClearBold => self.clear_bold = true,
            ClearDim => self.clear_dim = true,
            ClearItalic => self.clear_italic = true,
            ClearUnderline => self.clear_underline = true,
            ClearBlinking => self.clear_blinking = true,
            ClearInverse => self.clear_inverse = true,
            ClearHidden => self.clear_hidden = true,
            ClearStrikethrough => self.clear_strikethrough = true,
        }
        self
    }

    #[inline]
    fn color(mut self, color: impl Into<crate::inline::Color>) -> Self {
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

    fn custom(mut self, code: impl Into<u8>) -> InlineGraphics {
        self.custom.push(code.into());
        self
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

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Style::*;
        let mut fmt = AnsiFmt::new(f);
        fmt.escape()?;

        fmt.write_code(match self {
            Reset => 0,
            Bold => 1,
            Dim => 2,
            Italic => 3,
            Underline => 4,
            Blinking => 5,
            Inverse => 7,
            Hidden => 8,
            Strikethrough => 9,

            ClearBold => 22,
            ClearDim => 22,
            ClearItalic => 23,
            ClearUnderline => 24,
            ClearBlinking => 25,
            ClearInverse => 27,
            ClearHidden => 28,
            ClearStrikethrough => 29,
        })?;
        fmt.end()
    }
}

impl InlineAnsi for Style {
    fn style(self, style: impl Into<Style>) -> InlineGraphics {
        InlineGraphics::default().style(self).style(style.into())
    }

    fn color(self, color: impl Into<Color>) -> InlineGraphics {
        InlineGraphics::default().style(self).color(color.into())
    }

    fn custom(self, code: impl Into<u8>) -> InlineGraphics {
        InlineGraphics::default().style(self).custom(code.into())
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

impl InlineAnsi for Color {
    fn style(self, style: impl Into<Style>) -> InlineGraphics {
        InlineGraphics::default().color(self).style(style.into())
    }

    fn color(self, color: impl Into<Color>) -> InlineGraphics {
        InlineGraphics::default().color(self).color(color.into())
    }

    fn custom(self, code: impl Into<u8>) -> InlineGraphics {
        InlineGraphics::default().color(self).custom(code.into())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Color::*;
        let mut fmt = AnsiFmt::new(f);
        fmt.escape()?;

        match self {
            FBlack => fmt.write_code(30),
            FRed => fmt.write_code(31),
            FGreen => fmt.write_code(32),
            FYellow => fmt.write_code(33),
            FBlue => fmt.write_code(34),
            FMagenta => fmt.write_code(35),
            FCyan => fmt.write_code(36),
            FWhite => fmt.write_code(37),
            FEightBit(n) => fmt.write_all(&[38, 2, *n]),
            FRgb(r, g, b) => fmt.write_all(&[38, 5, *r, *g, *b]),

            FDefault => fmt.write_code(39),
            BBlack => fmt.write_code(40),
            BRed => fmt.write_code(41),
            BGreen => fmt.write_code(42),
            BYellow => fmt.write_code(43),
            BBlue => fmt.write_code(44),
            BMagenta => fmt.write_code(45),
            BCyan => fmt.write_code(46),
            BWhite => fmt.write_code(47),
            BEightBit(n) => fmt.write_all(&[48, 2, *n]),
            BRgb(r, g, b) => fmt.write_all(&[48, 5, *r, *g, *b]),
            BDefault => fmt.write_code(49),
        }?;

        fmt.end()
    }
}

pub trait InlineAnsi: Display {
    fn style(self, style: impl Into<Style>) -> InlineGraphics;
    fn color(self, color: impl Into<Color>) -> InlineGraphics;
    fn custom(self, code: impl Into<u8>) -> InlineGraphics;
}
