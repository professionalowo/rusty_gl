#[derive(Debug, PartialEq)]
pub struct IntoIter<'a, T> {
    slice: &'a [T],
}

impl<'a, T> IntoIter<'a, T> {
    #[inline]
    pub const fn new(slice: &'a [T]) -> Self {
        Self { slice }
    }
}

impl<'a, T> Iterator for IntoIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(first)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.slice.len();
        (len, Some(len))
    }
}

impl<'a, T> DoubleEndedIterator for IntoIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (last, rest) = self.slice.split_last()?;
        self.slice = rest;
        Some(last)
    }
}

impl<'a, T> ExactSizeIterator for IntoIter<'a, T> {}
