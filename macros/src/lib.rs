use parse::{parse_literal, parse_string};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

mod parse;

#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    sgr("println", input)
}

fn sgr(macro_call: &str, input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let token = match tokens.next() {
        Some(TokenTree::Literal(literal)) => {
            TokenTree::Literal(match parse_literal(&literal.to_string()) {
                Some(s) => parse_string(s)
                    .map(|parsed| Literal::string(&parsed))
                    .unwrap_or_else(|| Literal::string(s)),
                None => literal,
            })
        }
        // Some(TokenTree::Literal(s)) => match parse_literal(&s.to_string()) {
        //     Ok(s) => TokenTree::Literal(Literal::string(match parse_string(&s) {
        //         Some(s) => &s,
        //         None => s,
        //     })),
        //     Err(s) => TokenTree::Literal(Literal::string(&s)),
        // },
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
                [token].into_iter().chain(tokens).collect(),
            );
            group.set_span(span);
            group
        }),
    ]
    .into_iter()
    .collect()
}
