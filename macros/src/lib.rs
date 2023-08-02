//!The proc-macro implementation for the easy-sgr string
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

use parse::{create_raw_string, sgr_string, unwrap_string, UnwrappedLiteral};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

mod parse;

#[proc_macro]
/// Formats data into a string.
#[doc = include_str!("../SYNTAX.md")]
pub fn format(input: TokenStream) -> TokenStream {
    sgr_macro("format", input.into_iter())
}

#[proc_macro]
/// Writes formatted data into a writer.
#[doc = include_str!("../SYNTAX.md")]
pub fn write(input: TokenStream) -> TokenStream {
    sgr_macro("write", input.into_iter())
}

#[proc_macro]
/// Writes formatted data into a writer with a newline appended at the end.
#[doc = include_str!("../SYNTAX.md")]
pub fn writeln(input: TokenStream) -> TokenStream {
    sgr_macro("writeln", input.into_iter())
}

#[proc_macro]
/// Prints formatted data to the standard output.
#[doc = include_str!("../SYNTAX.md")]
pub fn print(input: TokenStream) -> TokenStream {
    sgr_macro("print", input.into_iter())
}

#[proc_macro]
/// Prints formatted data to the standard output with a newline appended at the end.
#[doc = include_str!("../SYNTAX.md")]
pub fn println(input: TokenStream) -> TokenStream {
    sgr_macro("println", input.into_iter())
}

#[proc_macro]
/// Prints formatted data to the standard error.
#[doc = include_str!("../SYNTAX.md")]
pub fn eprint(input: TokenStream) -> TokenStream {
    sgr_macro("eprint", input.into_iter())
}

#[proc_macro]
/// Prints formatted data to the standard error with a newline appended at the end.
#[doc = include_str!("../SYNTAX.md")]
pub fn eprintln(input: TokenStream) -> TokenStream {
    sgr_macro("eprintln", input.into_iter())
}

#[proc_macro]
/// Creates a [`std::fmt::Arguments`] struct for deferred formatting.
#[doc = include_str!("../SYNTAX.md")]
pub fn format_args(input: TokenStream) -> TokenStream {
    sgr_macro("format_args", input.into_iter())
}

#[proc_macro]
/// TODO
#[doc = include_str!("../SYNTAX.md")]
pub fn sgr(input: TokenStream) -> TokenStream {
    let mut tokens = input.clone().into_iter();
    let string_literal = tokens.next();
    if tokens.next().is_some() {
        return create_macro(
            "compile_error",
            Span::mixed_site(),
            TokenTree::Literal(Literal::string("sgr! macro does not accept arguments")).into(),
        );
    }
    match create_literal(string_literal) {
        ParsedLiteral::String(token) => TokenTree::Literal(token).into(),
        ParsedLiteral::RawString(string) => string
            .parse()
            .expect("Raw string parsing failed, should never fail"),
        // compiler will let user know of invalid token
        ParsedLiteral::InvalidToken(token) => create_macro(
            "compile_error",
            token.span(),
            std::format!(r#""Invalid token: {token}""#)
                .parse()
                .expect("Parsing error string failed, should never fail"),
        ),
        ParsedLiteral::InvalidString => input,
        ParsedLiteral::Empty => TokenTree::Literal(Literal::string("")).into(),
    }
}

/// Creates a [`TokenStream`] macro call,
/// meant for `fmt` macros
///
/// # Params
///
/// - `macro_call`: What macro to make
/// - `input`: The [`TokenStream`] to parse
///
/// This may change in the future to just returning the [`TokenStream`]
/// that is inputted in the macro call
fn sgr_macro(macro_call: &str, mut tokens: impl Iterator<Item = TokenTree>) -> TokenStream {
    match create_literal(tokens.next()) {
        ParsedLiteral::String(token) => create_macro(
            macro_call,
            token.span(),
            std::iter::once(TokenTree::Literal(token))
                .chain(tokens)
                .collect(),
        ),
        ParsedLiteral::RawString(string) => {
            // Should not fail
            create_macro(macro_call, Span::mixed_site(), {
                // using FromStr is the only way to return a raw string
                let mut stream: TokenStream = string
                    .parse()
                    .expect("Raw string parsing failed, should never fail");
                stream.extend(tokens);
                stream
            })
        }
        // compiler will let user know of invalid token
        ParsedLiteral::InvalidToken(token) => create_macro(
            macro_call,
            token.span(),
            std::iter::once(token).chain(tokens).collect(),
        ),
        ParsedLiteral::InvalidString => TokenStream::new(),
        ParsedLiteral::Empty => create_macro(macro_call, Span::mixed_site(), TokenStream::new()),
    }
}
enum ParsedLiteral {
    String(Literal),
    RawString(String),
    InvalidToken(TokenTree),
    InvalidString,
    Empty,
}
impl<'a> From<UnwrappedLiteral<'a>> for ParsedLiteral {
    fn from(value: UnwrappedLiteral) -> Self {
        use UnwrappedLiteral::*;

        match value {
            String(s) => {
                sgr_string(s).map_or(Self::InvalidString, |s| Self::String(Literal::string(&s)))
            }
            RawString(s, i) => Self::RawString(create_raw_string(s, i)),
        }
    }
}
fn create_literal(token: Option<TokenTree>) -> ParsedLiteral {
    use ParsedLiteral::*;
    match token {
        Some(TokenTree::Literal(literal)) => unwrap_string(&literal.to_string())
            .map_or_else(|| InvalidToken(TokenTree::Literal(literal)), Into::into),
        Some(t) => InvalidToken(t),
        None => Empty,
    }
}
fn create_macro(macro_call: &str, span: Span, stream: TokenStream) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("std", span)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(macro_call, span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group({
            let mut group = Group::new(Delimiter::Parenthesis, stream);
            group.set_span(span);
            group
        }),
    ]
    .into_iter()
    .collect()
}
