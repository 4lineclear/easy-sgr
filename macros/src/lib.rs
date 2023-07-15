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

#[derive(Clone)]
struct Transform<I, F> {
    iter: I,
    f: F,
}
impl<I, F> Iterator for Transform<I, F>
where
    I: Iterator,
    F: FnMut(&mut I) -> Option<I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.iter)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

trait ToTransform<I, F>
where
    I: Iterator,
    F: FnMut(&mut I) -> Option<I::Item>,
{
    fn transform(self, f: F) -> Transform<I, F>;
}
impl<I, F> ToTransform<I, F> for I
where
    I: Iterator,
    F: FnMut(&mut I) -> Option<I::Item>,
{
    fn transform(self, f: F) -> Transform<I, F> {
        Transform { iter: self, f }
    }
}

#[derive(Clone)]
struct Mapform<I, F> {
    iter: I,
    f: F,
}
impl<I, F, N> Iterator for Mapform<I, F>
where
    I: Iterator,
    F: FnMut(&mut I) -> N,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.f)(&mut self.iter))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

trait ToMapform<I, F, N>
where
    F: FnMut(&mut I) -> N,
{
    fn mapform(self, f: F) -> Mapform<I, F>;
}
impl<I, F, N> ToMapform<I, F, N> for I
where
    F: FnMut(&mut I) -> N,
{
    fn mapform(self, f: F) -> Mapform<I, F> {
        Mapform { iter: self, f }
    }
}
