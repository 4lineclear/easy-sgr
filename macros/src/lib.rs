use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

mod form;
mod parse;

const STR_LIT_ERROR: &str = "first item must be a string literal\ncannot be raw and/or byte string";

#[proc_macro]
pub fn sgr(input: TokenStream) -> TokenStream {
    match parse_tokens(input) {
        Ok(s) => tokenize_str(&s),
        Err(error_tokens) => error_tokens,
    }
}
fn parse_tokens(input: TokenStream) -> Result<String, TokenStream> {
    match input.into_iter().next() {
        // return the source TokenTree in the case there is any error
        // any error should be picked up by the rust compiler,
        // as it would be string literal error
        Some(source) => match source {
            TokenTree::Literal(s) => match unquote(&s.to_string()) {
                Some(s) => Ok(parse::parse_string(&mut s.chars())),
                None => Err(err(s.span(), STR_LIT_ERROR)),
            },
            tt => Err(err(tt.span(), STR_LIT_ERROR)),
        },
        None => Err(err(Span::mixed_site(), STR_LIT_ERROR)),
    }
}
fn unquote(string: &str) -> Option<&str> {
    string.strip_prefix('"')?.strip_suffix('"')
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
