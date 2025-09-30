use super::StbiPtr;

#[derive(Debug, PartialEq)]
pub struct IntoIter<'a, T> {
    index: usize,
    inner: &'a StbiPtr<T>,
}

impl<'a, T> IntoIter<'a, T> {
    #[inline]
    pub const fn new(inner: &'a StbiPtr<T>) -> Self {
        Self { index: 0, inner }
    }
}

impl<'a, T> Iterator for IntoIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.len {
            None
        } else {
            let item = &self.inner[self.index];
            self.index += 1;
            Some(item)
        }
    }
}
