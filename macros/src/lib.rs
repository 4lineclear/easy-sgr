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

/// Contains strictly string parsing implementation
mod parse;
#[cfg(test)]
mod test;
/// Defines the exported `proc_macros`
macro_rules! def_macros {
    ($($(#[$docs:meta])* $name:ident : $kind:ident),+) => {
        $(
            $(#[$docs])*
            #[doc = include_str!("../keywords.md")]
            #[proc_macro]
            pub fn $name(input: TokenStream) -> TokenStream {
                build_macro(MacroKind::$kind, input)
            }
        )+
    };
}
/// Builds a [`TokenStream`] according to the inputted
/// items
///
/// Expects items to implement [`StreamUnit`]
///
/// Specifying `from_trees` expects all given types
/// to implement `Into<TokenTree>` instead
macro_rules! build_stream {
    ($first:expr $(,$unit:expr)+) => {{
        let mut stream = $first.to_stream();
        $(
            $unit.extend_from_self(&mut stream);
        )*
        stream
    }};
    (from_trees $($unit:expr),*) => {{
        [
            $(TokenTree::from($unit)),*
        ].into_iter().map(TokenTree::from).collect()
    }};
    ([$first:expr]) => {{
        $first.into();
    }};
    ($unit:expr) => {{
        $unit.to_stream()
    }};
    () => {
        TokenStream::new()
    };
}
// #[cfg(feature = "alias")]
// pub(crate) mod user_keyword {
//     use std::{
//         collections::HashMap,
//         sync::{Mutex, OnceLock},
//     };
//     static KEYWORDS: OnceLock<Mutex<HashMap<&str, &str>>> = OnceLock::new();
// }
// /// will create keywords aliases in the future
// ///
// #[proc_macro]
// #[cfg(feature = "alias")]
// pub fn sgr_alias(input: TokenStream) -> TokenStream {
//     let mut tokens = input.into_iter();
//     let ret = match tokens.next() {
//         Some(TokenTree::Literal(s)) => (),
//         Some(t) => return create_macro(
//             "compile_error",
//             t.span(),
//             r#""Invalid token found""#
//                 .parse()
//                 .expect("Parsing error string failed, should never fail"),
//         ),
//         None => todo!(),
//     };
//     unimplemented!()
// }
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
/// The type of macro
///
/// Is used to differentiate how to go about parsing
/// the inputted [`TokenStream`] and the string [`Literal`]
/// contained(if any)
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
    /// Returns the name of the macro variant,
    /// or in the case of [`MacroKind::Sgr`] returning an empty string.
    const fn name(&self) -> &str {
        use MacroKind::*;
        match self {
            EPrint => "eprint",
            EPrintln => "eprintln",
            Format => "format",
            FormatArgs => "format_args",
            Print => "print",
            Println => "println",
            Sgr => "",
            Write => "write",
            Writeln => "writeln",
        }
    }
}
/// Builds a macro according to the given [`MacroKind`] and [`TokenStream`],
/// or an error found while parsing.
///
/// # Errors
///
/// All returned `TokenStreams` should all be valid,
/// but may be used to indicate an error.
///
/// This may come in several forms:
/// - A [`compile_error`] with an appropriate message
/// - An empty [`TokenStream`], which the compiler will catch an report the error
/// - The normal expected macro call, with the error als being reported by the compiler
///
/// There are two ways that an error is 'handed off' to the compiler,
/// the first is when an incorrect string literal is found, but parsing is not possible,
/// the second is when an incorrect string literal is found, but parsing may be possible.
///
/// The first first type of hand off is indicated by the use of [`Error::CompilerHandOff`]
/// and is used for escape errors. These are not possible to parse, and the compiler will
/// readily find them.
///
/// The second type concerns variable captures (`"{...}"`). These types of errors are
/// not as easily found by the compiler: they are checked after macros are parsed. This
/// means that parsing an empty [`TokenStream`] like in the other method may lead to the
/// error not being reported. Therefore a string will still be returned to some capacity,
/// which will then be picked up by the compiler to report the relevant error.
fn build_macro(kind: MacroKind, input: TokenStream) -> TokenStream {
    match kind {
        MacroKind::Sgr => match build_args::<true>(kind, input) {
            Ok(tokens) | Err(tokens) => tokens,
        },
        _ => match build_args::<false>(kind, input) {
            Ok(tokens) | Err(tokens) => create_macro(kind.name(), Span::mixed_site(), tokens),
        },
    }
}
/// Builds the arguments that should be within the returned macro call
///
/// In the case of [`MacroKind::Sgr`] this would just be a string literal,
/// else some kind of error.
///
/// # Errors
///
/// See [`StreamParts::from_parts`]
fn build_args<const MERGE_CURLY: bool>(
    kind: MacroKind,
    input: TokenStream,
) -> Result<TokenStream, TokenStream> {
    let tokens = input.into_iter();
    let stream = match StreamParts::from_parts(kind, tokens) {
        Ok(stream) => stream,
        Err(tokens) => return Err(tokens),
    };

    let (literal, parsed_literal) = match &stream.kind {
        StreamKind::Standard(literal) | StreamKind::Writer(_, Some((_, literal))) => (
            Some(literal),
            unwrap_string(&literal.to_string()).map_or_else(
                || ParsedLiteral::InvalidToken(TokenTree::from(literal.clone())),
                |unwrapped| ParsedLiteral::parse::<MERGE_CURLY>(&unwrapped),
            ),
        ),
        StreamKind::Writer(_, None) | StreamKind::Empty => (None, ParsedLiteral::Empty),
    };
    Ok(match parsed_literal {
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
                    StreamKind::Writer(writer, None) => build_stream!(writer),
                    _ => build_stream!(),
                },
                string,
                stream.tokens
            )
        }
        ParsedLiteral::InvalidToken(token) => build_stream!(token, stream.tokens),
        ParsedLiteral::InvalidString(e) => return Err(e.into_stream(literal)),
        ParsedLiteral::Empty => match stream.kind {
            StreamKind::Writer(writer, Some((punct, _))) => {
                build_stream!(writer, punct, stream.tokens)
            }
            StreamKind::Writer(writer, None) => build_stream!(writer),
            _ if kind == MacroKind::Sgr => {
                return Err(compile_error(Span::mixed_site(), "missing string literal"))
            }
            _ => build_stream!(),
        },
    })
}
enum ParsedLiteral {
    String(Literal),
    RawString(TokenStream),
    InvalidToken(TokenTree),
    InvalidString(Error),
    Empty,
}
impl ParsedLiteral {
    /// Parses the given [`UnwrappedLiteral`] into a [`ParsedLiteral`]
    ///
    /// A constant bool is used to indicate whether to merge open curly brackets.
    /// This means wether `{{` should be turned into `{` or `{{`
    fn parse<const MERGE_CURLY: bool>(unwrapped: &UnwrappedLiteral) -> Self {
        use UnwrappedLiteral::*;
        let check_curly = |ch| match ch {
            '}' => Some("{}"),
            '{' => Some(if MERGE_CURLY { "{" } else { "{{" }),
            _ => None,
        };
        match unwrapped {
            String(s) => match sgr_string(s, check_curly) {
                Ok(s) => Self::String(Literal::string(&s)),
                Err(e) => Self::InvalidString(e),
            },
            // using FromStr is the only way to return a raw string
            RawString(s, i) => Self::RawString(
                create_raw_string(s, *i)
                    .parse()
                    .expect("Raw string parsing failed, should never fail"),
            ),
        }
    }
}
/// A [`TokenStream`] split up into the needed parts
struct StreamParts {
    /// The type of stream needed depending on the inputted [`MacroKind`]
    kind: StreamKind,
    /// The remaining tokens
    tokens: IntoIter,
}
impl StreamParts {
    /// Disassembles the given tokens
    ///
    /// Tokens are turned into a [`StreamKind`] depending on the given [`MacroKind`],
    /// the remaining tokens are left within `tokens`
    ///
    /// # Errors
    ///
    /// An `Err(TokenStream)` is returned when the inputted tokens are invalid
    /// in the context of the given [`MacroKind`]. This [`TokenStream`] is made
    /// up of the `TokenTrees` of the inputted tokens
    fn from_parts(kind: MacroKind, mut tokens: IntoIter) -> Result<Self, TokenStream> {
        Ok(Self {
            kind: match StreamKind::from_kind(kind, &mut tokens) {
                Ok(stream_kind) => stream_kind,
                Err(err) => return Err(build_stream!(err, tokens)),
            },
            tokens,
        })
    }
}

/// The parts a of a [`TokenStream`]
///
/// Additional `TokenTrees` should be found within [`StreamParts`]
enum StreamKind {
    /// For one of
    /// `EPrint | EPrintln | Format | FormatArgs | Print | Println | Sgr`
    Standard(Literal),
    /// For one of `Write | Writeln`
    ///
    /// `1` will be `None` when either the [`Punct`] & [`Literal`] are not found
    Writer(Ident, Option<(Punct, Literal)>),
    /// For all variants of [`MacroKind`]
    Empty,
}

impl StreamKind {
    /// Creates a [`StreamKind`] from a given [`MacroKind`]
    /// and iterator of [`TokenTree`]
    ///
    /// # Errors
    ///
    /// An `Err(TokenStream)` is returned when the inputted tokens are invalid
    /// in the context of the given [`MacroKind`]. This [`TokenStream`] is made
    /// up of the `TokenTrees` collected up until that point, any tokens remaining
    /// in `tokens` are ignored
    fn from_kind(kind: MacroKind, tokens: &mut IntoIter) -> Result<Self, TokenStream> {
        use MacroKind::*;
        use StreamKind::*;
        let first = tokens.next();
        match kind {
            EPrint | EPrintln | Format | FormatArgs | Print | Println | Sgr => match first {
                Some(TokenTree::Literal(literal)) => Ok(Standard(literal)),
                Some(t) => Err(build_stream!(t)),
                None => Ok(Empty),
            },
            Write | Writeln => {
                let writer = match first {
                    Some(TokenTree::Ident(writer)) => writer,
                    Some(t) => return Err(build_stream!(t)),
                    None => return Ok(Empty),
                };
                let punct = match tokens.next() {
                    Some(TokenTree::Punct(punct)) => punct,
                    Some(t) => return Err(build_stream!(writer, t)),
                    None => return Ok(Writer(writer, None)),
                };
                match tokens.next() {
                    Some(TokenTree::Literal(literal)) => Ok(Writer(writer, Some((punct, literal)))),
                    Some(t) => Err(build_stream!(writer, punct, t)),
                    None => Err(build_stream!(writer, punct)),
                }
            }
        }
    }
}
/// creates a [`TokenStream`] of a [`std`] macro
/// with the given [`Span`] & stream (used within a [`Group`])
pub(crate) fn create_macro(macro_call: &str, span: Span, stream: TokenStream) -> TokenStream {
    build_stream!( from_trees
        Ident::new("std", span),
        Punct::new(':', Spacing::Joint),
        Punct::new(':', Spacing::Alone),
        Ident::new(macro_call, span),
        Punct::new('!', Spacing::Alone),
        Group::new(Delimiter::Parenthesis, stream)
    )
}
/// creates a [`TokenStream`] of a [`std::compile_error`]
/// with the given [`Span`] & message
pub(crate) fn compile_error(span: Span, message: &str) -> TokenStream {
    create_macro(
        "compile_error",
        span,
        build_stream!(Literal::string(message)),
    )
}
impl Error {
    /// Turns self into a [`TokenStream`] of a [`compile_error`]
    /// which informs the user of an error
    ///
    /// May return an empty [`TokenStream`] when the error will be reported
    /// by the compiler itself
    fn into_stream(self, literal: Option<&Literal>) -> TokenStream {
        use std::num::IntErrorKind::*;
        use Error::*;
        let span = literal.map_or_else(Span::mixed_site, Literal::span);
        match self {
            ParseInt(e) => compile_error(
                span,
                match e.kind() {
                    Empty => "cannot parse integer from empty string",
                    InvalidDigit => "invalid digit or keyword found",
                    PosOverflow => "number too large to fit in u8",
                    NegOverflow => "number too small to fit in u8",
                    Zero => "number would be zero for non-zero type",
                    _ => return compile_error(span, &e.to_string()),
                },
            ),
            MissingBracket => compile_error(span, "Missing a close bracket"),
            InvalidColorLen => compile_error(span, "Incorrect number of digits found"),
            CompilerPassOff => {
                literal.map_or_else(|| build_stream!(), |literal| build_stream!(literal.clone()))
            }
        }
    }
}
/// A unit of a [`TokenStream`]
///
/// Used to append to and build `TokenStreams`
trait StreamUnit {
    /// Adds self onto a [`TokenStream`]
    fn extend_from_self(self, stream: &mut TokenStream);
    /// Turns self onto a [`TokenStream`]
    fn to_stream(self) -> TokenStream;
}
/// Used to implement [`StreamUnit`] for [`TokenTree`] types
/// ([`Group`], [`Ident`], [`Punct`], [`Literal`])
///
/// Used to avoid
/// '`upstream crates may add a new impl of trait in future versions`'
/// error
trait ToTree {
    fn to_tree(self) -> TokenTree;
}
/// Utility macro to declare [`ToTree`]
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
    fn to_stream(self) -> TokenStream {
        self.to_tree().into()
    }
}
impl StreamUnit for TokenStream {
    fn extend_from_self(self, stream: &mut TokenStream) {
        stream.extend(self);
    }
    fn to_stream(self) -> TokenStream {
        self
    }
}
impl StreamUnit for IntoIter {
    fn extend_from_self(self, stream: &mut TokenStream) {
        stream.extend(self);
    }
    fn to_stream(self) -> TokenStream {
        self.collect()
    }
}
