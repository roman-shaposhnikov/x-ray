use std::{ path::Path };

use crate::{ response::{ Entities, Response, Total } };

use super::{ fan_store::FanStore, module::Module };

pub struct Project<'a> {
    entry: &'a Path,
    fan_store: FanStore,
}

impl Project<'_> {
    pub fn new(entry: &Path) -> Project {
        Project { entry, fan_store: FanStore::new() }
    }

    pub fn fan(&mut self) -> Result<Response, String> {
        if self.entry.is_file() {
            let root_module = Module::new(self.entry, &mut self.fan_store);
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
