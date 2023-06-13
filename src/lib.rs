use std::fmt::{Display, Write};

use graphics::Graphics;
use writer::{Ansi, AnsiFmt};

pub mod color;
pub mod graphics;
pub mod style;
pub mod writer;

pub const ESCAPE: &'static str = "\x1b[";
pub const END: char = 'm';
pub const RESET: &str = "\x1b[0m";
// TODO name: https://en.wikipedia.org/wiki/ANSI_escape_code#SGR
#[derive(Debug, Default, Clone)]
pub struct AnsiString {
    pub text: String,
    pub graphics: Graphics,
    pub custom: Custom,
}

impl AnsiString {
    pub fn hard_reset(mut self) -> Self {
        self.graphics.hard_reset = true;
        self
    }
    pub fn skip_reset(mut self) -> Self {
        self.graphics.skip_reset = true;
        self
    }
    pub fn style(mut self, style: impl Into<style::Style>) -> Self {
        self.graphics = self.graphics.style(style.into());
        self
    }
    pub fn foreground(mut self, color: impl Into<color::AnsiColor>) -> Self {
        self.graphics.foreground = Some(color.into());
        self
    }
    pub fn background(mut self, color: impl Into<color::AnsiColor>) -> Self {
        self.graphics.background = Some(color.into());
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
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: writer::AnsiWriter,
    {
        writer.escape()?;
        self.graphics.write(writer)?;
        self.custom.write(writer)?;
        writer.end()
    }

    fn reset<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: writer::AnsiWriter,
    {
        writer.escape()?;
        self.graphics.reset(writer)?;
        self.custom.reset(writer)?;
        writer.end()
    }

    fn empty(&self) -> bool {
        self.graphics.empty() && self.custom.empty()
    }
}

impl Display for AnsiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.empty() {
            true => f.write_str(&self.text),
            false => {
                let mut writer = AnsiFmt::new(f);
                self.write(&mut writer)?;
                writer.write_str(&self.text)?;
                self.reset(&mut writer)
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Custom {
    pub writes: Vec<u8>,
    pub resets: Vec<u8>,
}

impl Ansi for Custom {
    fn write<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: writer::AnsiWriter,
    {
        writer.write_all(&self.writes)
    }

    fn reset<W>(&self, writer: &mut W) -> Result<(), W::Error>
    where
        W: writer::AnsiWriter,
    {
        writer.write_all(&self.resets)
    }

    fn empty(&self) -> bool {
        self.writes.is_empty() && self.resets.is_empty()
    }
}

pub trait ToAnsiString {
    fn to_ansi_string(self) -> AnsiString;
    fn skip_reset(self) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().skip_reset()
    }
    fn style(self, style: impl Into<style::Style>) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().style(style)
    }
    fn foreground(self, color: impl Into<color::AnsiColor>) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().foreground(color)
    }
    fn background(self, color: impl Into<color::AnsiColor>) -> AnsiString
    where
        Self: Sized,
    {
        self.to_ansi_string().background(color)
    }
}

macro_rules! impl_to_ansi_string {
    ($($t:ty),+) => {
        $(
            impl ToAnsiString for $t {
                fn to_ansi_string(self) -> AnsiString {
                    AnsiString::from(self)
                }
            }
        )*
    };
}

impl_to_ansi_string!(&str, String, &String);

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
