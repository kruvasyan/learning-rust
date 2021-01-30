//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit {}

impl StrSplit {
    pub fn new(haystack: &str, delimiter: &str) -> Self {}
}


impl Iterator for StrSplit {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    for letter in StrSplit::new(haystack, " ") {
        // a
        // b
        // c
        // d
        // e
    }
}