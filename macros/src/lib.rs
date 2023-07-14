// use std::num::ParseIntError;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn replace_sgr(input: TokenStream) -> TokenStream {
    match replace_sgr_impl(input) {
        Some(tokens) => tokens,
        None => err(),
    }
}
fn err() -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", Span::mixed_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(
                "requires a string literal\ncannot be raw and/or byte string",
            ))]
            .into_iter()
            .collect(),
        )),
    ]
    .into_iter()
    .collect()
}
fn replace_sgr_impl(input: TokenStream) -> Option<TokenStream> {
    Some(
        [TokenTree::Literal(Literal::string(&remove_escapes(
            input
                .into_iter()
                .next()
                .map(|x| match x {
                    proc_macro::TokenTree::Literal(literal) => Some(literal.to_string()),
                    _ => None,
                })??
                .strip_suffix('"')?
                .strip_prefix('"')?,
        )?))]
        .into_iter()
        .collect(),
    )
}
fn remove_escapes(src: &str) -> Option<String> {
    let mut s = String::with_capacity(src.len());
    let mut chars = src.chars();
    while let Some(c) = chars.next() {
        if let '\\' = c {
            if let Some(c_next) = chars.next() {
                match c_next {
                    //quote escapes
                    '\'' => s.push('\''),
                    '"' => s.push('"'),
                    //ascii escapes
                    'x' => s.push(parse_7bit(&mut chars)?),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    '\\' => s.push('\\'),
                    '\0' => s.push('\0'),
                    //unicode escape
                    'u' => s.push(parse_24bit(&mut chars)?),
                    //whitespace ignore
                    '\n' => {
                        for c in chars.by_ref() {
                            let (' ' | '\n' | '\r' | '\t') = c else {
                                s.push(c);
                                break;
                            };
                        }
                    }
                    _ => (),
                }
            }
        } else {
            s.push(c)
        }
    }
    Some(s)
}
fn parse_7bit(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    let mut src = String::with_capacity(2);
    src.push(chars.next()?);
    src.push(chars.next()?);

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
fn parse_24bit(chars: &mut impl Iterator<Item = char>) -> Option<char> {
    chars.next()?;
    let src: String = chars.take_while(|&c| c != '}').collect();

    char::from_u32(u32::from_str_radix(&src, 16).ok()?)
}
