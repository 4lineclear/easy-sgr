use std::{error::Error, fmt::Display, num::ParseIntError, str::FromStr};

use crate::{Color, Seq, Style};

impl FromStr for Seq {
    type Err = ParseSeqError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Esc" => Ok(Self::Esc),
            "End" => Ok(Self::End),
            _ => Err(ParseSeqError),
        }
    }
}
/// An error encountered while trying to parse a string into a [`Seq`]
#[derive(Debug, PartialEq, Eq)]
pub struct ParseSeqError;
impl FromStr for Style {
    type Err = ParseStyleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Reset" => Ok(Self::Reset),
            "Bold" => Ok(Self::Bold),
            "Dim" => Ok(Self::Dim),
            "Italic" => Ok(Self::Italic),
            "Underline" => Ok(Self::Underline),
            "Blinking" => Ok(Self::Blinking),
            "Inverse" => Ok(Self::Inverse),
            "Hidden" => Ok(Self::Hidden),
            "Strikethrough" => Ok(Self::Strikethrough),
            "NotBold" => Ok(Self::NotBold),
            "NotDim" => Ok(Self::NotDim),
            "NotItalic" => Ok(Self::NotItalic),
            "NotUnderline" => Ok(Self::NotUnderline),
            "NotBlinking" => Ok(Self::NotBlinking),
            "NotInverse" => Ok(Self::NotInverse),
            "NotHidden" => Ok(Self::NotHidden),
            "NotStrikethrough" => Ok(Self::NotStrikethrough),
            _ => Err(ParseStyleError),
        }
    }
}
/// An error encountered while trying to parse a string into a [`Style`]
#[derive(Debug, PartialEq, Eq)]
pub struct ParseStyleError;
impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Color::*;
        match s {
            "BlackFg" => Ok(BlackFg),
            "RedFg" => Ok(RedFg),
            "GreenFg" => Ok(GreenFg),
            "YellowFg" => Ok(YellowFg),
            "BlueFg" => Ok(BlueFg),
            "MagentaFg" => Ok(MagentaFg),
            "CyanFg" => Ok(CyanFg),
            "WhiteFg" => Ok(WhiteFg),
            "DefaultFg" => Ok(DefaultFg),
            "BlackBg" => Ok(BlackBg),
            "RedBg" => Ok(RedBg),
            "GreenBg" => Ok(GreenBg),
            "YellowBg" => Ok(YellowBg),
            "BlueBg" => Ok(BlueBg),
            "MagentaBg" => Ok(MagentaBg),
            "CyanBg" => Ok(CyanBg),
            "WhiteBg" => Ok(WhiteBg),
            "DefaultBg" => Ok(DefaultBg),
            _ => match s.get(..5) {
                Some("RgbFg") => {
                    let parts = resolve_rgb(s)?;
                    Ok(RgbFg(parts.0, parts.1, parts.2))
                }
                Some("RgbBg") => {
                    let parts = resolve_rgb(s)?;
                    Ok(RgbBg(parts.0, parts.1, parts.2))
                }
                Some(_) => match s.get(..6) {
                    Some("ByteFg") => Ok(ByteFg(resolve_byte(s)?)),
                    Some("ByteBg") => Ok(ByteBg(resolve_byte(s)?)),
                    _ => Err(ParseColorError::Invalid(s.to_string())),
                },
                None => Err(ParseColorError::Invalid(s.to_string())),
            },
        }
    }
}
/// An error encountered while trying to parse a string into a [`Color`]
#[derive(Debug, PartialEq, Eq)]
pub enum ParseColorError {
    /// A string that is completely invalid
    Invalid(String),
    /// Missing the number
    ///
    /// i.e. `ByteFg` or `RgbBg`
    MissingNum(String),
    /// Brace Error
    ///
    /// i.e. `ByteFg20)` or `ByteFg(20`
    Brace(String),
    /// Int parsing error
    ///
    /// See [`ParseIntError`]
    ParseIntError(ParseIntError),
    /// Wrong number of integers
    ///
    /// i.e. `RgbFg(20)`, `RgbFg(20,30)` or `RgbFg(20,30,40,50)`
    Len(usize),
}
impl Display for ParseColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(s) => write!(f, "Invalid string: {s}"),
            Self::MissingNum(s) => write!(f, "Missing number: {s}"),
            Self::Brace(s) => {
                write!(f, "Missing braces: {s}")
            }
            Self::ParseIntError(e) => write!(f, "Error parsing int: {e}"),
            Self::Len(n) => {
                write!(f, "Found wrong number of items in string: {n}. Needed 2")
            }
        }
    }
}
impl Error for ParseColorError {}
fn resolve_byte(s: &str) -> Result<u8, ParseColorError> {
    s.get(6..)
        .ok_or_else(|| ParseColorError::MissingNum(s.to_string()))
        .and_then(|src| match src.len() {
            0 => Err(ParseColorError::MissingNum(s.to_string())),
            _ => Ok(src),
        })?
        .strip_prefix('(')
        .ok_or_else(|| ParseColorError::Brace(s.to_string()))?
        .strip_suffix(')')
        .ok_or_else(|| ParseColorError::Brace(s.to_string()))?
        .parse()
        .map_err(ParseColorError::ParseIntError)
}
fn resolve_rgb(s: &str) -> Result<(u8, u8, u8), ParseColorError> {
    let parts: Vec<u8> = s
        .get(5..)
        .ok_or_else(|| ParseColorError::MissingNum(s.to_string()))
        .and_then(|src| match src.len() {
            0 => Err(ParseColorError::MissingNum(s.to_string())),
            _ => Ok(src),
        })?
        .strip_prefix('(')
        .ok_or_else(|| ParseColorError::Brace(s.to_string()))?
        .strip_suffix(')')
        .ok_or_else(|| ParseColorError::Brace(s.to_string()))?
        .split(',')
        .flat_map(|s| s.parse().map_err(ParseColorError::ParseIntError))
        .collect();

    match &parts[..] {
        &[n1, n2, n3] => Ok((n1, n2, n3)),
        _ => Err(ParseColorError::Len(parts.len())),
    }
}
