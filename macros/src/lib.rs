//!The proc-macro implementation for the easy-sgr string
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    missing_docs,
    rustdoc::all
)]
#![warn(missing_debug_implementations)]
#![allow(clippy::enum_glob_use)]

use parse::{parse_string, unwrap_string, UnwrappedLiteral};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::parse::parse_raw_string;

#[allow(clippy::module_name_repetitions)]
mod parse;

macro_rules! def_macros {
    ($($(#[$attr:meta])* $name:ident),*) => {
        $(
            $(#[$attr])*
            #[proc_macro]
            pub fn $name(input: TokenStream) -> TokenStream {
                sgr_macro(stringify!($name), input)
            }
        )*
    };
}

def_macros!(
    /// Formats data into a string.
    ///
    /// This macro is an extension to [`std::format`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::format`]
    /// - [`std::fmt`]
    format,
    /// Writes formatted data into a writer.
    ///
    /// This macro is an extension to [`std::write`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::write`]
    /// - [`std::fmt`]
    write,
    /// Writes formatted data into a writer with a newline appended at the end.
    ///
    /// This macro is an extension to [`std::writeln`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::writeln`]
    /// - [`std::fmt`]
    writeln,
    /// Prints formatted data to the standard output.
    ///
    /// This macro is an extension to [`std::print`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::print`]
    /// - [`std::fmt`]
    print,
    /// Prints formatted data to the standard output with a newline appended at the end.
    ///
    /// This macro is an extension to [`std::println`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::println`]
    /// - [`std::fmt`]
    println,
    /// Writes formatted data to the standard error.
    ///
    /// This macro is an extension to [`std::eprint`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::eprint`]
    /// - [`std::fmt`]
    eprint,
    /// Writes formatted data to the standard error with a newline appended at the end.
    ///
    /// This macro is an extension to [`std::eprintln`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::eprintln`]
    /// - [`std::fmt`]
    eprintln,
    /// Creates a [`std::fmt::Arguments`](https://doc.rust-lang.org/std/fmt/struct.Arguments.html)
    /// struct for deferred formatting.
    ///
    /// This macro is an extension to [`std::format_args`],
    /// it contains the ability switch certain keywords out with SGR escapes
    ///
    /// # See also
    /// - [`easy_sgr`](https://docs.rs/easy-sgr/latest/easy_sgr/)
    /// - [`std::format_args`]
    /// - [`std::fmt`]
    format_args
);

#[proc_macro]
/// TODO
pub fn sgr(input: TokenStream) -> TokenStream {
    let mut tokens = input.clone().into_iter();
    let first = tokens.next();
    if tokens.next().is_some() {
        return create_macro(
            "compile error",
            Span::mixed_site(),
            TokenTree::Literal(Literal::string("This macro does not accept arguments")).into(),
        );
    }
    match create_literal(first) {
        ParsedLiteral::String(token) => TokenTree::Literal(token).into(),
        ParsedLiteral::RawString(string) => string
            .parse()
            .expect("Raw string parsing failed, should never fail"),
        // compiler will let user know of invalid token
        ParsedLiteral::InvalidToken(token, s) => {
            let span = token.span();
            [
                std::iter::once(token).collect(),
                create_macro(
                    "compile_error",
                    span,
                    s.parse()
                        .expect("Parsing string into TokenStream failed, should never fail"),
                ),
            ]
            .into_iter()
            .collect()
        }
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
fn sgr_macro(macro_call: &str, input: TokenStream) -> TokenStream {
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
        ParsedLiteral::InvalidToken(token, s) => {
            let span = token.span();
            [
                create_macro(
                    macro_call,
                    span,
                    std::iter::once(token).chain(tokens).collect(),
                ),
                create_macro(
                    "compile_error",
                    span,
                    s.parse()
                        .expect("Parsing string into TokenStream failed, should never fail"),
                ),
            ]
            .into_iter()
            .collect()
        }
        ParsedLiteral::InvalidString => TokenStream::new(),
        ParsedLiteral::Empty => create_macro(macro_call, Span::mixed_site(), TokenStream::new()),
    }
}
enum ParsedLiteral {
    String(Literal),
    RawString(String),
    InvalidToken(TokenTree, &'static str),
    InvalidString,
    Empty,
}
impl<'a> From<UnwrappedLiteral<'a>> for ParsedLiteral {
    fn from(value: UnwrappedLiteral) -> Self {
        use UnwrappedLiteral::*;

        match value {
            String(s) => {
                parse_string(s).map_or(Self::InvalidString, |s| Self::String(Literal::string(&s)))
            }
            RawString(s, i) => Self::RawString(parse_raw_string(s, i)),
        }
    }
}
fn create_literal(token: Option<TokenTree>) -> ParsedLiteral {
    use ParsedLiteral::*;
    match token {
        Some(TokenTree::Literal(literal)) => unwrap_string(&literal.to_string()).map_or_else(
            || InvalidToken(TokenTree::Literal(literal), "Invalid string found"),
            Into::into,
        ),
        Some(t) => InvalidToken(t, "Non string literal token found"),
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
