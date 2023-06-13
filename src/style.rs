use std::fmt::Write;

use crate::{ESCAPE, END, graphics::Graphics};

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
    ResetBold,
    ResetDim,
    ResetItalic,
    ResetUnderline,
    ResetBlinking,
    ResetInverse,
    ResetHidden,
    ResetStrikethrough,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(ESCAPE)?;
        f.write_str(&self.code().to_string())?;
        f.write_char(END)
    }
}

impl Style {
    pub fn code(&self) -> u8 {
        use Style::*;
        match self {
            Reset => 0,
            Bold => 1,
            Dim => 2,
            Italic => 3,
            Underline => 4,
            Blinking => 5,
            Inverse => 7,
            Hidden => 8,
            Strikethrough => 9,
            ResetBold => 22,
            ResetDim => 22,
            ResetItalic => 23,
            ResetUnderline => 24,
            ResetBlinking => 25,
            ResetInverse => 27,
            ResetHidden => 28,
            ResetStrikethrough => 29,
        }
    }
    pub fn and(self, other: Self) -> Graphics {
        Graphics::default().style(self).style(other)
    }
}
