//!The proc-macro implementation for the easy-sgr string
#![allow(unsafe_code)]
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
use parse::{parse_literal, parse_string};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

#[allow(clippy::module_name_repetitions)]
mod parse;

macro_rules! def_macros {
    ($(#[$attr:meta] $name:ident),*) => {
        $(
            #[$attr]
            #[proc_macro]
            pub fn $name(input: TokenStream) -> TokenStream {
                sgr(stringify!($name), input)
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

fn sgr(macro_call: &str, input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let token = match tokens.next() {
        Some(TokenTree::Literal(literal)) => {
            TokenTree::Literal(parse_literal(&literal.to_string()).map_or(literal, |s| {
                parse_string(s)
                    .map_or_else(|| Literal::string(s), |parsed| Literal::string(&parsed))
            }))
        }
        Some(t) => t,
        None => TokenTree::Literal(Literal::string("")),
    };
    let span = token.span();

    [
        TokenTree::Ident(Ident::new(macro_call, span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group({
            let mut group = Group::new(
                Delimiter::Parenthesis,
                std::iter::once(token).chain(tokens).collect(),
            );
            group.set_span(span);
            group
        }),
    ]
    .into_iter()
    .collect()
}
