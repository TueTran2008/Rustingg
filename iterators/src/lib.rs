fn flatten<O>(iter: O) -> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    <O as Iterator>::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner_iter) = &mut self.inner_iter {
                if let Some(inner_item) = inner_iter.next() {
                    return Some(inner_item);
                } else {
                    self.inner_iter = None;
                }
            } else {
                let next_inner_iter = self.outer.next()?.into_iter();
                self.inner_iter = Some(next_inner_iter);
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator + Iterator,
    <O as Iterator>::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner_iter {
                if let Some(inner_item) = inner_iter.next_back() {
                    return Some(inner_item);
                } else {
                    self.inner_iter = None;
                }
            } else {
                let outer = self.outer.next_back()?.into_iter();
                self.inner_iter = Some(outer);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }
    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        );
    }
    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }
    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }
    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }

    #[test]
    fn reserve() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }
    #[test]
    fn revserve_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]].into_iter())
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }
}
