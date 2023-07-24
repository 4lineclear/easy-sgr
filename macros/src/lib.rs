use parse::{parse_literal, parse_string};
use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};

mod parse;

macro_rules! def_macros {
    ($($name:ident),*) => {
        $(
            #[proc_macro]
            pub fn $name(input: TokenStream) -> TokenStream {
                sgr(stringify!($name), input)
            }
        )*
    };
}

def_macros!(
    format,
    write,
    writeln,
    print,
    println,
    eprint,
    eprintln,
    format_args
);

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
