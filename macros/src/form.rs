//TODO Consider a 'skip_if' type
#[derive(Clone)]
pub(crate) struct Transform<I, F> {
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

pub(crate) trait ToTransform<I, F> {
    fn transform(self, f: F) -> Transform<I, F>
    where
        I: Iterator,
        F: FnMut(&mut I) -> Option<I::Item>;
}
impl<I, F> ToTransform<I, F> for I {
    fn transform(self, f: F) -> Transform<I, F>
    where
        I: Iterator,
        F: FnMut(&mut I) -> Option<I::Item>,
    {
        Transform { iter: self, f }
    }
}

#[derive(Clone)]
pub(crate) struct Mapform<I, F> {
    iter: I,
    f: F,
}
impl<I, F, N> Iterator for Mapform<I, F>
where
    I: Iterator,
    F: FnMut(&mut I) -> Option<N>,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.iter)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub(crate) trait ToMapform<I, F, N>
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
