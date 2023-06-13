use paste::paste;
use std::fmt::Display;

use crate::{
    color::AnsiColor,
    style::Style,
    writer::{Ansi, AnsiFmt, AnsiWriter},
};

macro_rules! create_graphics {
    ($($field:ident : $type:ty),+) => {

        #[derive(Debug, Default, Clone, Copy)]
        pub struct Graphics {
            $(
                pub $field: $type,
            )*
        }

        paste! {
            impl Graphics {
                $(
                    pub fn [<get_ $field>](&self) -> &$type {
                        &self.$field
                    }
                )*


                $(
                    pub fn [<set_ $field>](&mut self, $field: $type) -> &mut Self{
                        self.$field = $field;

                        self
                    }
                )*
            }


        }
    };
}

create_graphics!(
    foreground: Option<AnsiColor>,
    background: Option<AnsiColor>,
    clear_foreground: bool,
    clear_background: bool,
    reset: bool,
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    blinking: bool,
    inverse: bool,
    hidden: bool,
    strikethrough: bool,
    clear_bold: bool,
    clear_dim: bool,
    clear_italic: bool,
    clear_underline: bool,
    clear_blinking: bool,
    clear_inverse: bool,
    clear_hidden: bool,
    clear_strikethrough: bool,
    skip_reset: bool,
    hard_reset: bool
);

impl Graphics {
    pub fn style(mut self, style: Style) -> Self {
        use Style::*;

        match style {
            Reset => self.reset = true,

            Bold => self.bold = true,
            Dim => self.dim = true,
            Italic => self.italic = true,
            Underline => self.underline = true,
            Blinking => self.blinking = true,
            Inverse => self.inverse = true,
            Hidden => self.hidden = true,
            Strikethrough => self.strikethrough = true,

            ResetBold => self.clear_bold = true,
            ResetDim => self.clear_dim = true,
            ResetItalic => self.clear_italic = true,
            ResetUnderline => self.clear_underline = true,
            ResetBlinking => self.clear_blinking = true,
            ResetInverse => self.clear_inverse = true,
            ResetHidden => self.clear_hidden = true,
            ResetStrikethrough => self.clear_strikethrough = true,
        }
        self
    }

    pub fn and(self, other: Style) -> Self {
        self.style(other)
    }
}

impl Ansi for Graphics {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: crate::writer::AnsiWriter,
    {
        use AnsiColor::*;
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
                EightBit(n) => writer.write_all(&[38, 2, n])?,
                Rgb(r, g, b) => writer.write_all(&[38, 5, r, g, b])?,
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
                EightBit(n) => writer.write_all(&[48, 2, n])?,
                Rgb(r, g, b) => writer.write_all(&[48, 5, r, g, b])?,
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
        Ok(())
    }

    fn reset<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: crate::writer::AnsiWriter,
    {
        if self.skip_reset {
            return Ok(());
        }

        if self.hard_reset {
            return writer.write_code(0);
        }

        if self.foreground.is_some() && self.clear_foreground {
            writer.write_code(39)?;
        }
        if self.background.is_some() && self.clear_background {
            writer.write_code(49)?;
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
                writer.write_code(code)?;
            }
        }
        Ok(())
    }

    fn empty(&self) -> bool {
        self.foreground.is_none()
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
            && !self.hard_reset
            && !self.skip_reset
            && !self.clear_foreground
            && !self.clear_foreground
            && !self.clear_bold
            && !self.clear_dim
            && !self.clear_italic
            && !self.clear_underline
            && !self.clear_blinking
            && !self.clear_inverse
            && !self.clear_hidden
            && !self.clear_strikethrough
    }
}

impl Display for Graphics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut writer = AnsiFmt::new(f);
        writer.escape()?;
        self.write(&mut writer)?;
        writer.escape()
    }
}
