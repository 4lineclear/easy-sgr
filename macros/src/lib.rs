//! The proc-macro implementation for the
//! [easy-sgr](https://crates.io/crates/easy-sgr) crate
//!
#![doc = include_str!("../syntax.md")]
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    missing_docs,
    rustdoc::all,
    future_incompatible
)]
#![warn(missing_debug_implementations)]
#![allow(clippy::enum_glob_use)]

use parse::Error;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::parse::{create_raw_string, sgr_string, unwrap_string, UnwrappedLiteral};

mod parse;
#[cfg(test)]
mod test;

/// Formats data into a string.
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn format(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::Format, input)
}

/// Writes formatted data into a writer.
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::Write, input)
}

/// Writes formatted data into a writer with a newline append
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn writeln(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::Writeln, input)
}

/// Prints formatted data to the standard output.
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::Print, input)
}

/// Prints formatted data to the standard output with a newline
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::Println, input)
}

/// Prints formatted data to the standard error.
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn eprint(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::EPrint, input)
}

/// Prints formatted data to the standard error with a newline
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn eprintln(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::EPrintln, input)
}

/// Creates a [`arguments`](std::fmt::Arguments) struct for d
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::FormatArgs, input)
}

/// Creates a string literal
#[doc = include_str!("../syntax.md")]
#[proc_macro]
pub fn sgr(input: TokenStream) -> TokenStream {
    build_macro(MacroKind::Sgr, input)
}

fn build_macro(kind: MacroKind, input: TokenStream) -> TokenStream {
    let stream = ArgumentBuilder::from_kind(kind).build_from(input);
    match kind {
        MacroKind::Sgr => stream,
        _ => create_macro(kind.name(), Span::mixed_site(), stream),
    }
}

struct ArgumentBuilder {
    kind: MacroKind,
    literal_parser: LiteralParser,
}
impl ArgumentBuilder {
    fn from_kind(kind: MacroKind) -> Self {
        Self {
            literal_parser: if kind == MacroKind::Sgr {
                LiteralParser::MergeCurly
            } else {
                LiteralParser::Standard
            },
            kind,
        }
    }
    fn build_from(self, input: TokenStream) -> TokenStream {
        let tokens = input.into_iter();
        let disassembled_stream = match DisassembledStream::disassemble(self.kind, tokens) {
            Ok(stream) => stream,
            Err(tokens) => return tokens.into_iter().collect(),
        };

        let parsed_literal = match &disassembled_stream.kind {
            StreamKind::Standard(literal) | StreamKind::Writer(_, Some((_, literal))) => {
                unwrap_string(&literal.to_string()).map_or_else(
                    || ParsedLiteral::InvalidToken(TokenTree::Literal(literal.clone())),
                    |unwrapped| self.literal_parser.parse_literal(&unwrapped),
                )
            }
            StreamKind::Writer(_, None) | StreamKind::Empty => ParsedLiteral::Empty,
        };
        match parsed_literal {
            ParsedLiteral::String(literal) => match disassembled_stream.kind {
                StreamKind::Writer(writer, Some((punct, _))) => [
                    TokenTree::from(writer),
                    TokenTree::from(punct),
                    TokenTree::from(literal),
                ]
                .into_iter()
                .chain(disassembled_stream.tokens)
                .collect(),
                StreamKind::Writer(writer, None) => {
                    [TokenTree::from(writer), TokenTree::from(literal)]
                        .into_iter()
                        .collect()
                }
                _ => std::iter::once(TokenTree::from(literal))
                    .chain(disassembled_stream.tokens)
                    .collect(),
            },
            ParsedLiteral::RawString(string) => {
                let mut stream: TokenStream = match disassembled_stream.kind {
                    StreamKind::Writer(writer, Some((punct, _))) => {
                        [TokenTree::from(writer), TokenTree::from(punct)]
                            .into_iter()
                            .collect()
                    }
                    StreamKind::Writer(writer, None) => TokenTree::from(writer).into(),
                    _ => {
                        let mut string = string;
                        string.extend(disassembled_stream.tokens);
                        return string;
                    }
                };
                stream.extend(string.into_iter());
                stream.extend(disassembled_stream.tokens);
                stream
            }
            ParsedLiteral::InvalidToken(token) => std::iter::once(token)
                .chain(disassembled_stream.tokens)
                .collect(),
            ParsedLiteral::InvalidString(e) => e.into(),
            ParsedLiteral::Empty => match disassembled_stream.kind {
                StreamKind::Writer(writer, Some((punct, _))) => {
                    [TokenTree::from(writer), TokenTree::from(punct)]
                        .into_iter()
                        .chain(disassembled_stream.tokens)
                        .collect()
                }
                StreamKind::Writer(writer, None) => {
                    std::iter::once(TokenTree::from(writer)).collect()
                }
                _ if self.kind == MacroKind::Sgr => {
                    compile_error(Span::mixed_site(), "missing string literal")
                }
                _ => TokenStream::new(),
            },
        }
    }
}
enum ParsedLiteral {
    String(Literal),
    RawString(TokenStream),
    InvalidToken(TokenTree),
    InvalidString(Error),
    Empty,
}

enum LiteralParser {
    Standard,
    MergeCurly,
}
impl LiteralParser {
    fn parse_literal(&self, unwrapped: &UnwrappedLiteral) -> ParsedLiteral {
        use UnwrappedLiteral::*;
        let check_curly = |ch| match ch {
            '}' => Some("{}"),
            '{' => Some(if matches!(self, Self::Standard) {
                "{{"
            } else {
                "{"
            }),
            _ => None,
        };
        match unwrapped {
            String(s) => match sgr_string(s, check_curly) {
                Ok(s) => ParsedLiteral::String(Literal::string(&s)),
                Err(e) => ParsedLiteral::InvalidString(e),
            },
            // using FromStr is the only way to return a raw string
            RawString(s, i) => ParsedLiteral::RawString(
                create_raw_string(s, *i)
                    .parse()
                    .expect("Raw string parsing failed, should never fail"),
            ),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum MacroKind {
    EPrint,
    EPrintln,
    Format,
    FormatArgs,
    Print,
    Println,
    Sgr,
    Write,
    Writeln,
}
impl MacroKind {
    const fn name(&self) -> &str {
        use MacroKind::*;
        match self {
            EPrint => "eprint",
            EPrintln => "eprintln",
            Format => "format",
            FormatArgs => "format_args",
            Print => "print",
            Println => "println",
            Sgr => "sgr",
            Write => "write",
            Writeln => "writeln",
        }
    }
}
struct DisassembledStream<I>
where
    I: Iterator<Item = TokenTree>,
{
    kind: StreamKind,
    tokens: I,
}

impl<I> DisassembledStream<I>
where
    I: Iterator<Item = TokenTree>,
{
    fn disassemble(
        kind: MacroKind,
        mut tokens: I,
    ) -> Result<Self, impl Iterator<Item = TokenTree>> {
        Ok(Self {
            kind: match StreamKind::from_kind(kind, &mut tokens) {
                Ok(stream_kind) => stream_kind,
                Err(err) => return Err(err.into_iter().chain(tokens)),
            },
            tokens,
        })
    }
}

enum StreamKind {
    Standard(Literal),
    Writer(Ident, Option<(Punct, Literal)>),
    Empty,
}

impl StreamKind {
    fn from_kind(
        kind: MacroKind,
        tokens: &mut impl Iterator<Item = TokenTree>,
    ) -> Result<Self, Vec<TokenTree>> {
        use MacroKind::*;
        use StreamKind::*;
        let first = tokens.next();
        match kind {
            EPrint | EPrintln | Format | FormatArgs | Print | Println => Ok(match first {
                Some(TokenTree::Literal(literal)) => Standard(literal),
                Some(t) => return Err(vec![t]),
                None => Empty,
            }),
            Write | Writeln => {
                let writer = match first {
                    Some(TokenTree::Ident(ident)) => ident,
                    Some(t) => return Err(vec![t]),
                    None => return Ok(Empty),
                };
                let punct = match tokens.next() {
                    Some(TokenTree::Punct(punct)) => Some(punct),
                    Some(t) => return Err(vec![TokenTree::Ident(writer), t]),
                    None => None,
                };
                let Some(punct) = punct else {
                    return Ok(Writer(writer, None))
                };
                let punct_literal = match tokens.next() {
                    Some(TokenTree::Literal(literal)) => Some((punct, literal)),
                    Some(t) => {
                        return Err(vec![TokenTree::Ident(writer), TokenTree::Punct(punct), t])
                    }
                    None => return Err(vec![TokenTree::Ident(writer), TokenTree::Punct(punct)]),
                };
                Ok(Writer(writer, punct_literal))
            }

            Sgr => match first {
                Some(TokenTree::Literal(literal)) => Ok(Standard(literal)),
                Some(t) => Err(vec![t]),
                None => Ok(Empty),
            },
        }
    }
} // TODO create col_err
pub(crate) fn create_macro(macro_call: &str, span: Span, stream: TokenStream) -> TokenStream {
    let tokens: [TokenTree; 6] = [
        Ident::new("std", span).into(),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new(macro_call, span).into(),
        Punct::new('!', Spacing::Alone).into(),
        Group::new(Delimiter::Parenthesis, stream).into(),
    ];
    tokens.into_iter().collect()
}
pub(crate) fn compile_error(span: Span, message: &str) -> TokenStream {
    create_macro(
        "compile_error",
        span,
        TokenTree::Literal(Literal::string(message)).into(),
    )
}

impl From<Error> for TokenStream {
    fn from(value: Error) -> Self {
        use std::num::IntErrorKind;
        match value {
            Error::ParseInt(e) => compile_error(
                Span::mixed_site(),
                match e.kind() {
                    IntErrorKind::Empty => "cannot parse integer from empty string",
                    IntErrorKind::InvalidDigit => "invalid digit or keyword found",
                    IntErrorKind::PosOverflow => "number too large to fit in u8",
                    IntErrorKind::NegOverflow => "number too small to fit in u8",
                    IntErrorKind::Zero => "number would be zero for non-zero type",
                    _ => return compile_error(Span::mixed_site(), &e.to_string()),
                },
            ),
            Error::MissingBracket => compile_error(Span::mixed_site(), "Missing a close bracket"),
            Error::InvalidColorLen => {
                compile_error(Span::mixed_site(), "Incorrect number of digits found")
            }
            Error::CompilerPassOff => Self::new(),
        }
    }
}
