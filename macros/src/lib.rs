use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

mod form;
mod parse;

const STR_LIT_ERROR: &str = "first item must be a string literal\ncannot be raw and/or byte string";

#[proc_macro]
pub fn println(input: TokenStream) -> TokenStream {
    MacroToken::println(Span::call_site(), sgr(input)).tokenize()
}
fn sgr(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    match tokens.next().map(|token| parse_literal(token)) {
        Some(Ok(s)) => tokenize_str(&s).chain(tokens).collect(),
        Some(Err(e)) => e,
        None => tokenize_str("").collect(),
    }
}
fn parse_literal(input: TokenTree) -> Result<String, TokenStream> {
    match input {
        TokenTree::Literal(literal) => match unquote(&literal.to_string()) {
            Some(s) => parse::parse_string(s)
                .ok_or(MacroToken::err(literal.span(), STR_LIT_ERROR).tokenize()),
            None => Err(MacroToken::err(literal.span(), STR_LIT_ERROR).tokenize()),
        },
        tt => Err(MacroToken::err(tt.span(), STR_LIT_ERROR).tokenize()),
    }
}
fn unquote(string: &str) -> Option<&str> {
    string.strip_prefix('"')?.strip_suffix('"')
}
fn tokenize_str(s: &str) -> impl Iterator<Item = TokenTree> {
    [TokenTree::Literal(Literal::string(s))].into_iter()
}
#[derive(Debug)]
struct MacroToken(Ident, Punct, Group);

impl MacroToken {
    fn println(span: Span, stream: impl Into<TokenStream>) -> Self {
        Self(
            Ident::new("println", span),
            Punct::new('!', Spacing::Alone),
            Group::new(Delimiter::Parenthesis, stream.into()),
        )
    }
    fn err(span: Span, message: &str) -> Self {
        Self(
            Ident::new("compile_error", span),
            Punct::new('!', Spacing::Alone),
            Group::new(Delimiter::Parenthesis, tokenize_str(message).collect()),
        )
    }
    fn tokenize(self) -> TokenStream {
        self.iter().collect()
    }
    fn iter(self) -> impl Iterator<Item = TokenTree> {
        [
            TokenTree::Ident(self.0),
            TokenTree::Punct(self.1),
            TokenTree::Group(self.2),
        ]
        .into_iter()
    }
}
