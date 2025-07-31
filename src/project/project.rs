use std::{ path::Path };

use crate::response::{ Entities, Response, Total };

use super::module::Module;

pub struct Project<'a> {
    entry: &'a Path,
}

impl Project<'_> {
    pub fn new(entry: &Path) -> Project {
        Project { entry }
    }

    pub fn fan(&self) -> Result<Response, String> {
        if self.entry.is_file() {
            let root_module = Module::new(self.entry);
        }

        Ok(Response {
            total: Total {
                modules: 0,
                classes: 0,
            },
            entities: Entities {
                modules: vec![],
                classes: vec![],
            },
        })
    }
}
