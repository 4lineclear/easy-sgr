use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

enum ParseError {
    MissingStrLit,
}

#[proc_macro]
pub fn replace_sgr(input: TokenStream) -> TokenStream {
    match replace_sgr_impl(input) {
        Ok(tokens) => tokens,
        Err(e) => [
            TokenTree::Ident(Ident::new("compile_error", Span::mixed_site())),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
            TokenTree::Group(Group::new(
                Delimiter::Parenthesis,
                [TokenTree::Literal(Literal::string(match e {
                    ParseError::MissingStrLit => "Missing String literal!",
                }))]
                .into_iter()
                .collect(),
            )),
        ]
        .into_iter()
        .collect(),
    }
}

fn replace_sgr_impl(input: TokenStream) -> Result<TokenStream, ParseError> {
    Ok([TokenTree::Literal(Literal::string(
        &input
            .into_iter()
            .next()
            .map(|x| match x {
                proc_macro::TokenTree::Literal(literal) => Ok(literal.to_string()),
                _ => Err(ParseError::MissingStrLit),
            })
            .ok_or(ParseError::MissingStrLit)??,
    ))]
    .into_iter()
    .collect())
}
