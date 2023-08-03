//! The proc-macro implementation for the easy-sgr
//!
//! ## Syntax
//!
//! See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
//!
//!
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

/// Formats data into a string.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn format(input: TokenStream) -> TokenStream {
    standard_sgr_macro("format", input)
}

/// Writes formatted data into a writer.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn write(input: TokenStream) -> TokenStream {
    write_sgr_macro("write", input)
}

/// Writes formatted data into a writer with a newline appended at the end.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn writeln(input: TokenStream) -> TokenStream {
    write_sgr_macro("writeln", input)
}

/// Prints formatted data to the standard output.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    standard_sgr_macro("print", input)
}

/// Prints formatted data to the standard output with a newline appended at the end.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    standard_sgr_macro("println", input)
}

/// Prints formatted data to the standard error.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn eprint(input: TokenStream) -> TokenStream {
    standard_sgr_macro("eprint", input)
}

/// Prints formatted data to the standard error with a newline appended at the end.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn eprintln(input: TokenStream) -> TokenStream {
    standard_sgr_macro("eprintln", input)
}

/// Creates a [`arguments`](std::fmt::Arguments) struct for deferred formatting.
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn format_args(input: TokenStream) -> TokenStream {
    standard_sgr_macro("format_args", input)
}

/// Creates a string literal
///
/// SGR keywords are switched out with their code counterparts
///
/// # Syntax
///
/// See [easy-sgr](https://docs.rs/easy-sgr/0.0.8/easy_sgr/#macros)
///
#[proc_macro]
pub fn sgr(input: TokenStream) -> TokenStream {
    let mut tokens = input.clone().into_iter();
    let string_literal = tokens.next();
    if tokens.next().is_some() {
        create_macro(
            "compile_error",
            Span::mixed_site(),
            TokenTree::Literal(Literal::string("sgr! does not accept arguments")).into(),
        )
    } else {
        match create_literal(string_literal) {
            ParsedLiteral::String(token) => TokenTree::Literal(token).into(),
            ParsedLiteral::RawString(string) => string
                .parse()
                .expect("Raw string parsing failed, should never fail"),
            // need to manually tell the user that the token is incorrect
            ParsedLiteral::InvalidToken(token) => create_macro(
                "compile_error",
                token.span(),
                r#""sgr! only accepts a format string argument""#
                    .parse()
                    .expect("Parsing error string failed, should never fail"),
            ),
            ParsedLiteral::InvalidString => input,
            ParsedLiteral::Empty => TokenTree::Literal(Literal::string("")).into(),
        }
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
fn standard_sgr_macro(macro_call: &str, input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
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
/// similar to [`standard_sgr_macro`], except
/// the first token is expected to be a writer's ident
fn write_sgr_macro(macro_call: &str, input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let writer = tokens.next().expect("Missing writer");
    let (next, punct) = match tokens.next() {
        Some(TokenTree::Punct(p)) => (tokens.next(), Some(p)),
        Some(t) => (Some(t), None),
        None => (None, None),
    };
    match create_literal(next) {
        ParsedLiteral::String(token) => create_macro(
            macro_call,
            token.span(),
            match punct {
                Some(ident) => [writer, TokenTree::Punct(ident), TokenTree::Literal(token)]
                    .into_iter()
                    .chain(tokens)
                    .collect(),
                None => [writer, TokenTree::Literal(token)]
                    .into_iter()
                    .chain(tokens)
                    .collect(),
            },
        ),
        ParsedLiteral::RawString(string) => {
            // Should not fail
            create_macro(macro_call, Span::mixed_site(), {
                // using FromStr is the only way to return a raw string
                let mut stream = match punct {
                    Some(punct) => [writer, TokenTree::Punct(punct)].into_iter().collect(),
                    None => TokenStream::from(writer),
                };

                let str_lit: TokenStream = string
                    .parse()
                    .expect("Raw string parsing failed, should never fail");
                stream.extend(str_lit.into_iter());
                stream.extend(tokens);
                stream
            })
        }
        // compiler will let user know of invalid token
        ParsedLiteral::InvalidToken(token) => create_macro(
            macro_call,
            token.span(),
            match punct {
                Some(ident) => [writer, TokenTree::Punct(ident), token]
                    .into_iter()
                    .chain(tokens)
                    .collect(),
                None => [writer, token].into_iter().chain(tokens).collect(),
            },
        ),
        ParsedLiteral::InvalidString => TokenStream::new(),
        ParsedLiteral::Empty => create_macro(
            macro_call,
            Span::mixed_site(),
            match punct {
                Some(ident) => [writer, TokenTree::Punct(ident)]
                    .into_iter()
                    .chain(tokens)
                    .collect(),
                None => std::iter::once(writer).chain(tokens).collect(),
            },
        ),
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
