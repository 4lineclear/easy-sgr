use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

mod form;
mod parse;

const STR_LIT_ERROR: &str = "first item must be a string literal\ncannot be raw and/or byte string";

#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    if input.is_empty() {
        [
            TokenTree::Ident(Ident::new("println", Span::mixed_site())),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, tokenize_str(""))),
        ]
        .into_iter()
        .collect()
    } else {
        [
            TokenTree::Ident(Ident::new("println", Span::mixed_site())),
            TokenTree::Punct(Punct::new('!', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, sgr(input))),
        ]
        .into_iter()
        .collect()
    }
}
fn sgr(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    match parse_first(tokens.next()) {
        Ok(s) => tokenize_all(&s, tokens),
        Err(error_tokens) => error_tokens,
    }
}
fn parse_first(input: Option<TokenTree>) -> Result<String, TokenStream> {
    match input {
        // return the source TokenTree in the case there is any error
        // any error should be picked up by the rust compiler,
        // as it would be string literal error
        Some(source) => match source {
            TokenTree::Literal(literal) => match unquote(&literal.to_string()) {
                Some(s) => {
                    parse::parse_string(&mut s.chars()).ok_or(err(literal.span(), STR_LIT_ERROR))
                }
                None => Err(err(literal.span(), STR_LIT_ERROR)),
            },
            tt => Err(err(tt.span(), STR_LIT_ERROR)),
        },
        None => Err(err(Span::mixed_site(), STR_LIT_ERROR)),
    }
}
fn unquote(string: &str) -> Option<&str> {
    string.strip_prefix('"')?.strip_suffix('"')
}
fn tokenize_all(s: &str, tokens: impl Iterator<Item = TokenTree>) -> TokenStream {
    [TokenTree::Literal(Literal::string(s))]
        .into_iter()
        .chain(
            tokens, // .flat_map(|tt| {
                   // [
                   //     TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                   //     tt.clone(),
                   //     TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                   //     tt.clone(),
                   // ]})
        )
        .collect()
}
fn tokenize_str(s: &str) -> TokenStream {
    [TokenTree::Literal(Literal::string(s))]
        .into_iter()
        .collect()
}
/// Returns a compile error with the inputted span & message
fn err(span: Span, message: &str) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group({
            let mut group = Group::new(Delimiter::Parenthesis, tokenize_str(message));
            group.set_span(span);
            group
        }),
    ]
    .into_iter()
    .collect()
}
