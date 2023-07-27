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
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

use crate::parse::parse_raw_string;

#[allow(clippy::module_name_repetitions)]
mod parse;

macro_rules! def_macros {
    ($(#[$attr:meta] $name:ident),*) => {
        $(
            #[$attr]
            #[proc_macro]
            pub fn $name(input: TokenStream) -> TokenStream {
                sgr_macro(stringify!($name), input)
            }
        )*
    };
}

def_macros!(
    ///
    format,
    ///
    write,
    ///
    writeln,
    ///
    print,
    ///
    println,
    ///
    eprint,
    ///
    eprintln,
    ///
    format_args
);

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
    use UnwrappedLiteral::*;
    let mut tokens = input.into_iter();
    let literal = match tokens.next() {
        Some(TokenTree::Literal(literal)) => {
            TokenTree::Literal(match unwrap_string(&literal.to_string()) {
                Some(String(s)) => Literal::string(&parse_string(s)),
                Some(RawString(s, i)) => Literal::string(&parse_raw_string(s, i)),
                None => literal,
            })
        }
        Some(t) => t,
        None => TokenTree::Literal(Literal::string("")),
    };
    let span = literal.span();
    let stream = std::iter::once(literal).chain(tokens).collect();
    [
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
