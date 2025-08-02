use std::path::Path;

pub struct File<'a> {
    path: &'a Path,
}

impl File<'_> {
    pub fn new(path: &Path) -> File {
        File { path }
    }

    pub fn content(&self) -> String {
        std::fs::read_to_string(self.path).unwrap()
    }
}
