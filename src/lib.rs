use std::fmt::{Display, Write};

use graphics::{Graphics, ClearKind};
use writer::{Ansi, AnsiFmt};

pub mod color;
pub mod graphics;
pub mod style;
pub mod writer;

pub const ESCAPE: &'static str = "\x1b[";
pub const END: char = 'm';
pub const RESET: &str = "\x1b[0m";

#[derive(Debug, Default, Clone)]
pub struct AnsiString {
    pub text: String,
    pub graphics: Graphics,
}

impl AnsiString {
    #[inline]
    pub fn style(mut self, style: impl Into<style::Style>) -> Self {
        self.graphics = self.graphics.style(style);
        self
    }
    #[inline]
    pub fn set_clear(mut self, clear_kind: impl Into<ClearKind>) -> Self {
        self.graphics = self.graphics.set_clear(clear_kind);
        self
    }
    #[inline]
    pub fn foreground(mut self, color: impl Into<color::ColorKind>) -> Self {
        self.graphics = self.graphics.foreground(color);
        self
    }
    #[inline]
    pub fn background(mut self, color: impl Into<color::ColorKind>) -> Self {
        self.graphics = self.graphics.background(color);
        self
    }
    #[inline]
    pub fn place_custom(mut self, code: u8) -> Self {
        self.graphics = self.graphics.place_custom(code);
        self
    }
    #[inline]
    pub fn clear_custom(mut self, code: u8) -> Self {
        self.graphics = self.graphics.clear_custom(code);
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
        W: writer::AnsiWriter,
    {
        match self.no_places() {
            true => Ok(()),
            false => {
                writer.escape()?;
                self.graphics.place_ansi(writer)?;
                writer.end()
            }
        }
    }

    fn clear_ansi<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: writer::AnsiWriter,
    {
        match self.no_clears() {
            true => Ok(()),
            false => {
                writer.escape()?;
                self.graphics.clear_ansi(writer)?;
                writer.end()
            }
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
        let mut writer = AnsiFmt::new(f);
        self.place_ansi(&mut writer)?;
        writer.write_str(&self.text)?;
        self.clear_ansi(&mut writer)
    }
}

pub trait ToAnsiString {
    fn to_ansi_string(self) -> AnsiString;

    fn set_clear(self, clear_kind: impl Into<ClearKind>) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().set_clear(clear_kind)
    }
    fn foreground(self, color: impl Into<color::ColorKind>) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().foreground(color)
    }
    fn background(self, color: impl Into<color::ColorKind>) -> AnsiString
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

// #[derive(Debug, Default, Clone)]
// pub struct AnsiAggregate {
//     pub strings: Vec<AnsiString>,
// }

// pub fn parse<'a>(src: &[u8]) -> Result<AnsiAggregate, ParseError> {
//     let mut code_buffer = 0u8;

//     let mut start = 0;

//     let mut search_type = SearchType::FindEscape1;
//     let mut codes = Vec::new();

//     let mut ansi_aggregate = AnsiAggregate::default();

//     for (i, b) in src.into_iter().enumerate() {
//         match search_type {
//             SearchType::FindEscape1 if *b == b'\x1b' => search_type = SearchType::FindEscape2,
//             SearchType::FindEscape2 if *b == b'[' => search_type = SearchType::Escaped,
//             SearchType::Escaped => match *b {
//                 b'0'..=b'9' => {
//                     code_buffer = match code_buffer.checked_add(*b - b'0') {
//                         Some(n) => n,
//                         None => return Err(ParseError::Overflow),
//                     };
//                 }
//                 b';' => {
//                     codes.push(code_buffer);
//                     code_buffer = 0;
//                 }
//                 b'm' => {
//                     codes.push(code_buffer);

//                     ansi_aggregate
//                         .strings
//                         .push(string_from_parts(&src[start..i], &codes));

//                     code_buffer = 0;
//                     start = i + 1;
//                     search_type = SearchType::FindEscape1;
//                 }
//                 _ => return Err(ParseError::NonDigit),
//             },
//             _ => (),
//         }
//     }
//     Ok(AnsiAggregate::default())
// }

// enum SearchType {
//     FindEscape1,
//     FindEscape2,
//     Escaped,
// }

// fn string_from_parts(string: &[u8], codes: &[u8]) -> AnsiString {
//     let mut graphics = Graphics::default();
//     for code in codes {
//         match code {
//             0 => graphics.reset = true,
//             1 => graphics.bold = true,
//             2 => graphics.dim = true,
//             3 => graphics.italic = true,
//             4 => graphics.underline = true,
//             5 => graphics.blinking = true,
//             7 => graphics.inverse = true,
//             8 => graphics.hidden = true,
//             9 => graphics.strikethrough = true,
//             _ => ()
//         }
//     }
//     AnsiString::default()
// }

// #[derive(Debug, Error)]
// pub enum ParseError {
//     #[error("Overflowed, number too high")]
//     Overflow,
//     #[error("Not a digit")]
//     NonDigit,
// }

// #[inline]
// pub fn parse_u8(src: &[u8]) -> Option<u8> {
//     match src.len() {
//         1 => match src[0] {
//             b'0'..=b'9' => Some(src[0] - b'0'),
//             _ => return None,
//         },
//         2 => match (src[0], src[1]) {
//             (b'0'..=b'9', b'0'..=b'9') => Some((src[0] - b'0') * 10 + (src[1] - b'0')),
//             _ => return None,
//         },
//         3 => match (src[0], src[1], src[2]) {
//             (b'0', b'0'..=b'9', b'0'..=b'9') => Some((src[1] - b'0') * 10 + (src[2] - b'0')),
//             (b'1', b'0'..=b'9', b'0'..=b'9') => Some(100 + (src[1] - b'0') * 10 + (src[2] - b'0')),
//             (b'2', b'0'..=b'4', b'0'..=b'9') => Some(200 + (src[1] - b'0') * 10 + (src[2] - b'0')),
//             (b'2', b'5', b'0'..=b'5') => Some(250 + (src[2] - b'0')),
//             _ => return None,
//         },
//         0 | _ => None,
//     }
// }
