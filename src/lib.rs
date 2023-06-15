#![deny(clippy::all, clippy::cargo)]
use std::fmt::{Display, Write};

use graphics::{ClearKind, Graphics};
use write::FmtWriter;

pub mod graphics;
pub mod inline;
pub mod parser;
pub mod write;

#[cfg(test)]
mod tests;

pub const ESCAPE: &str = "\x1b[";
pub const END: char = 'm';
pub const RESET: &str = "\x1b[0m";

#[derive(Debug, Default, Clone)]
pub struct AnsiString {
    pub text: String,
    pub graphics: Graphics,
}

impl AnsiString {
    #[inline]
    #[must_use]
    pub fn style(mut self, style: impl Into<inline::Style>) -> Self {
        self.graphics = self.graphics.style(style);
        self
    }
    #[inline]
    #[must_use]
    pub fn set_clear(mut self, clear_kind: impl Into<ClearKind>) -> Self {
        self.graphics = self.graphics.clear(clear_kind);
        self
    }
    #[inline]
    #[must_use]
    pub fn foreground(mut self, color: impl Into<graphics::ColorKind>) -> Self {
        self.graphics = self.graphics.foreground(color);
        self
    }
    #[inline]
    #[must_use]
    pub fn background(mut self, color: impl Into<graphics::ColorKind>) -> Self {
        self.graphics = self.graphics.background(color);
        self
    }
    #[inline]
    #[must_use]
    pub fn custom_place(mut self, code: u8) -> Self {
        self.graphics = self.graphics.custom_place(code);
        self
    }
    #[inline]
    #[must_use]
    pub fn custom_clear(mut self, code: u8) -> Self {
        self.graphics = self.graphics.custom_clear(code);
        self
    }
    #[inline]
    #[must_use]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
}

impl From<&str> for AnsiString {
    fn from(text: &str) -> Self {
        Self {
            text: String::from(text),
            ..Default::default()
        }
    }
}

impl From<String> for AnsiString {
    fn from(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl From<&String> for AnsiString {
    fn from(text: &String) -> Self {
        Self {
            text: String::from(text),
            ..Default::default()
        }
    }
}

impl Ansi for AnsiString {
    fn place_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: write::AnsiWriter,
    {
        if self.no_places() {
            Ok(())
        } else {
            writer.escape()?;
            self.graphics.place_ansi(writer)?;
            writer.end()
        }
    }

    fn clear_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: write::AnsiWriter,
    {
        if self.no_clears() {
            Ok(())
        } else {
            writer.escape()?;
            self.graphics.clear_ansi(writer)?;
            writer.end()
        }
    }

    fn no_places(&self) -> bool {
        self.graphics.no_places()
    }

    fn no_clears(&self) -> bool {
        self.graphics.no_clears()
    }
}

impl Display for AnsiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut writer = FmtWriter::new(f);
        self.place_ansi(&mut writer)?;
        writer.write_str(&self.text)?;
        self.clear_ansi(&mut writer)
    }
}

pub trait ToAnsiString {
    fn to_ansi_string(self) -> AnsiString;

    fn foreground(self, color: impl Into<graphics::ColorKind>) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().foreground(color)
    }
    fn background(self, color: impl Into<graphics::ColorKind>) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().background(color)
    }
}

impl ToAnsiString for &str {
    fn to_ansi_string(self) -> AnsiString {
        AnsiString::from(self)
    }
}
impl ToAnsiString for String {
    fn to_ansi_string(self) -> AnsiString {
        AnsiString::from(self)
    }
}
impl ToAnsiString for &String {
    fn to_ansi_string(self) -> AnsiString {
        AnsiString::from(self)
    }
}

pub trait Ansi {
    fn place_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: write::AnsiWriter;
    fn clear_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: write::AnsiWriter;
    fn no_places(&self) -> bool;
    fn no_clears(&self) -> bool;
}
