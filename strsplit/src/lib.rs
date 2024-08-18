use std::usize;

//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainer: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(hackstack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainer: Some(hackstack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_str(&self, remainer: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainer) = self.remainer {
            if let Some((deli_start, deli_stop)) = self.delimiter.find_str(remainer) {
                let token = &remainer[..deli_start];
                *remainer = &remainer[deli_stop..];
                println!("debug start stop {deli_start} {deli_stop}");
                Some(token)
            } else {
                self.remainer.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_str(&self, remainer: &str) -> Option<(usize, usize)> {
        remainer.find(self).map(|start| (start, start + self.len()))
    }
}
impl Delimiter for char {
    fn find_str(&self, remainer: &str) -> Option<(usize, usize)> {
        remainer
            .char_indices()
            .find(|(_index, x)| x == self)
            .map(|(index, _x)| (index, index + self.len_utf8()))
    }
}
pub fn until_char(input: &str, deli: char) -> &'_ str {
    //let deli_str = &format!("{deli}");
    //println!("Debug {deli_str}");
    StrSplit::new(input, deli).next().unwrap()
}

#[cfg(test)]
mod tests {

    use crate::{until_char, StrSplit};

    #[test]
    fn it_works() {
        let hackstrack = "a b c d e";
        let letter = StrSplit::new(hackstrack, " ");
        assert!(letter.eq(vec!["a", "b", "c", "d", "e"]));
    }
    #[test]
    fn tail_test() {
        let hackstrack = "a b c d ";
        let letter = StrSplit::new(hackstrack, " ");
        assert!(letter.eq(vec!["a", "b", "c", "d", ""]));
    }
    #[test]
    fn until_test() {
        let haystack = "a b c d e";
        let c = 'c';
        let test = until_char(haystack, c);
        println!("{test}");
        assert_eq!("a b ", test);
    }
}
