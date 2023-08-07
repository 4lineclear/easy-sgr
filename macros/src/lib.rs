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
use proc_macro::{
    token_stream::IntoIter, Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream,
    TokenTree,
};

use crate::parse::{create_raw_string, sgr_string, unwrap_string, UnwrappedLiteral};

mod parse;
#[cfg(test)]
mod test;
// TODO fix spans
macro_rules! def_macros {
    ($($(#[$docs:meta])* $name:ident : $kind:ident),+) => {
        $(
            $(#[$docs])*
            #[doc = include_str!("../keywords.md")]
            #[proc_macro]
            pub fn $name(input: TokenStream) -> TokenStream {
                build_macro::<{matches!(MacroKind::$kind, MacroKind::Sgr)}>(MacroKind::$kind, input)
            }
        )+
    };
}
// TODO turn this into a crate maybe
macro_rules! build_stream {
    ($($unit:expr),*) => {{
        let mut stream = TokenStream::new();
        $(
            $unit.extend_from_self(&mut stream);
        )*
        stream
    }};
}
def_macros!(
    /// Creates a String using interpolation of runtime expressions,
    /// SGR keywords substituted.
    ///
    /// # Examples
    format : Format,
    /// Writes formatted data into a buffer,
    /// SGR keywords substituted.
    ///
    /// # Examples
    write : Write,
    /// Write formatted data into a buffer, with a newline appended,
    /// SGR keywords substituted.
    ///
    /// # Examples
    writeln : Writeln,
    /// Prints to the standard output,
    /// SGR keywords substituted.
    ///
    /// # Examples
    print : Print,
    /// Prints to the standard output, with a newline,
    /// SGR keywords substituted.
    ///
    /// # Examples
    println : Println,
    /// Prints to the standard error,
    /// SGR keywords substituted.
    ///
    /// # Examples
    eprint : EPrint,
    /// Prints to the standard error, with a newline,
    /// SGR keywords substituted.
    ///
    /// # Examples
    eprintln : EPrintln,
    /// Constructs parameters for the other string-formatting macros,
    /// SGR keywords substituted.
    ///
    /// # Examples
    format_args : FormatArgs,
    /// Creates a string literal,
    /// SGR keywords substituted.
    ///
    /// # Examples
    sgr : Sgr
);

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
fn build_macro<const MERGE_CURLY: bool>(kind: MacroKind, input: TokenStream) -> TokenStream {
    let stream = build_args::<MERGE_CURLY>(kind, input);
    match kind {
        MacroKind::Sgr => stream,
        _ => create_macro(kind.name(), Span::mixed_site(), stream),
    }
}
fn build_args<const MERGE_CURLY: bool>(kind: MacroKind, input: TokenStream) -> TokenStream {
    let tokens = input.into_iter();
    let stream = match DisassembledStream::disassemble(kind, tokens) {
        Ok(stream) => stream,
        Err(tokens) => return build_stream!(tokens),
    };

    let parsed_literal = match &stream.kind {
        StreamKind::Standard(literal) | StreamKind::Writer(_, Some((_, literal))) => {
            unwrap_string(&literal.to_string()).map_or_else(
                || ParsedLiteral::InvalidToken(TokenTree::Literal(literal.clone())),
                |unwrapped| parse_literal::<MERGE_CURLY>(&unwrapped),
            )
        }
        StreamKind::Writer(_, None) | StreamKind::Empty => ParsedLiteral::Empty,
    };
    match parsed_literal {
        ParsedLiteral::String(literal) => match stream.kind {
            StreamKind::Writer(writer, Some((punct, _))) => {
                build_stream!(writer, punct, literal, stream.tokens)
            }
            StreamKind::Writer(writer, None) => build_stream!(writer, literal, stream.tokens),
            _ => build_stream!(literal, stream.tokens),
        },
        ParsedLiteral::RawString(string) => {
            build_stream!(
                match stream.kind {
                    StreamKind::Writer(writer, Some((punct, _))) => build_stream!(writer, punct),
                    StreamKind::Writer(writer, None) => TokenTree::from(writer).into(),
                    _ => TokenStream::new(),
                },
                string,
                stream.tokens
            )
        }
        ParsedLiteral::InvalidToken(token) => build_stream!(token, stream.tokens),
        ParsedLiteral::InvalidString(e) => e.into(),
        ParsedLiteral::Empty => match stream.kind {
            StreamKind::Writer(writer, Some((punct, _))) => {
                build_stream!(writer, punct, stream.tokens)
            }
            StreamKind::Writer(writer, None) => build_stream!(writer),
            _ if kind == MacroKind::Sgr => {
                compile_error(Span::mixed_site(), "missing string literal")
            }
            _ => TokenStream::new(),
        },
    }
}
enum ParsedLiteral {
    String(Literal),
    RawString(TokenStream),
    InvalidToken(TokenTree),
    InvalidString(Error),
    Empty,
}
fn parse_literal<const MERGE_CURLY: bool>(unwrapped: &UnwrappedLiteral) -> ParsedLiteral {
    use UnwrappedLiteral::*;
    let check_curly = |ch| match ch {
        '}' => Some("{}"),
        '{' => Some(if MERGE_CURLY { "{" } else { "{{" }),
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
struct DisassembledStream {
    kind: StreamKind,
    tokens: IntoIter,
}
impl DisassembledStream {
    fn disassemble(kind: MacroKind, mut tokens: IntoIter) -> Result<Self, IntoIter> {
        Ok(Self {
            kind: match StreamKind::from_kind(kind, &mut tokens) {
                Ok(stream_kind) => stream_kind,
                Err(err) => return Err(build_stream!(err, tokens).into_iter()),
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
    build_stream!(
        Ident::new("std", span),
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new(macro_call, span),
        Punct::new('!', Spacing::Alone),
        Group::new(Delimiter::Parenthesis, stream)
    )
}
pub(crate) fn compile_error(span: Span, message: &str) -> TokenStream {
    create_macro(
        "compile_error",
        span,
        build_stream!(Literal::string(message)),
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
trait StreamUnit {
    fn extend_from_self(self, stream: &mut TokenStream);
}
trait ToTree {
    fn to_tree(self) -> TokenTree;
}
macro_rules! to_tree {
    ($($name:ident),*) => {
        $(
            impl ToTree for $name {
                fn to_tree(self) -> TokenTree {
                    self.into()
                }
            }
        )*
    };
}
to_tree!(Group, Ident, Punct, Literal, TokenTree);

impl<T: ToTree> StreamUnit for T {
    fn extend_from_self(self, stream: &mut TokenStream) {
        stream.extend(Some(self.to_tree()));
    }
}
impl StreamUnit for TokenStream {
    fn extend_from_self(self, stream: &mut TokenStream) {
        stream.extend(self);
    }
}
impl StreamUnit for Vec<TokenTree> {
    fn extend_from_self(self, stream: &mut TokenStream) {
        stream.extend(self);
    }
}
impl StreamUnit for IntoIter {
    fn extend_from_self(self, stream: &mut TokenStream) {
        stream.extend(self);
    }
}
