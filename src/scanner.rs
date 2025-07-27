use std::path::Path;

pub struct Scanner<'a> {
    root: &'a Path,
}

impl Scanner<'_> {
    pub fn new(root: &Path) -> Scanner {
        Scanner { root }
    }

    pub fn scan(&self) {}
}
