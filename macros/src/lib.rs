use parse::parse_string;
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use sgr::sgrargs;

mod parse;
mod sgr;
#[proc_macro]
pub fn sgr_test(input: TokenStream) -> TokenStream {
    match input.into_iter().next() {
        Some(source) => match sgr_args(&source) {
            Some(output) => output,
            None => source.into(), // rust compiler should take care of errors
        },
        None => err(),
    }
}

/// Input should be a string literal
#[proc_macro]
pub fn replace_sgr(input: TokenStream) -> TokenStream {
    match input.into_iter().next() {
        Some(source) => match replace_sgr_impl(&source) {
            Some(output) => output,
            None => source.into(), // rust compiler should take care of errors
        },
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
                "first item must be a string literal\ncannot be raw and/or byte string",
            ))]
            .into_iter()
            .collect(),
        )),
    ]
    .into_iter()
    .collect()
}
fn replace_sgr_impl(source: &TokenTree) -> Option<TokenStream> {
    Some(
        [TokenTree::Literal(Literal::string(&parse_string(
            source.to_string(),
        )?))]
        .into_iter()
        .collect(),
    )
}

fn sgr_args(source: &TokenTree) -> Option<TokenStream> {
    Some(
        [TokenTree::Literal(Literal::string(
            &sgrargs(&parse_string(source.to_string())?)?.to_string(),
        ))]
        .into_iter()
        .collect(),
    )
}
