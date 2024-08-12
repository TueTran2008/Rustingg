//!
//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'a> {
    remainer: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(hackstack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainer: hackstack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(delimiter_next) = self.remainer.find(self.delimiter) {
            let token = Some(&self.remainer[..delimiter_next]);
            self.remainer = &self.remainer[delimiter_next + self.delimiter.len()..];
            token
        } else if self.remainer.is_empty() {
            //TODO :Bug
            None
        } else {
            let rest = self.remainer;
            self.remainer = "";
            Some(rest)
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::StrSplit;

    #[test]
    fn it_works() {
        let hackstrack = "a b c d e";
        let letter = StrSplit::new(hackstrack, " ");
        assert!(letter.eq(vec!["a", "b", "c", "d", "e"]));
    }
}
