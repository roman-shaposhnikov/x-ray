use std::path::Path;

pub struct File<'a> {
    path: &'a Path,
}

impl<'a> File<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
    }
}

impl File<'_> {
    pub fn content(&self) -> String {
        std::fs::read_to_string(self.path).unwrap()
    }
}
