use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
mod form;
// mod parse;
// mod sgr;

#[proc_macro]
pub fn sgr(input: TokenStream) -> TokenStream {
    match parse_tokens(input) {
        Ok(s) => tokenize(&s),
        Err(error_tokens) => error_tokens,
    }
}

fn parse_tokens(input: TokenStream) -> Result<String, TokenStream> {
    match input.into_iter().next() {
        Some(source) => parse_string(source.to_string()).ok_or_else(|| source.into()),
        None => Err(err()),
    }
}

fn parse_string(string: String) -> Option<String> {
    Some(string)
}

fn tokenize(s: &str) -> TokenStream {
    [TokenTree::Literal(Literal::string(s))]
        .into_iter()
        .collect()
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
