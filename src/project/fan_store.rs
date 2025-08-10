use std::collections::HashMap;

use crate::response::{ Fan, FanKey };

pub struct FanStore {
    modules: HashMap<String, Fan>,
}

impl FanStore {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn add_module_dependency(&mut self, mod_id: String, dep_id: String) {
        self.increase_module_fan(mod_id, &FanKey::Out);
        self.increase_module_fan(dep_id, &FanKey::In);
    }

    fn increase_module_fan(&mut self, mod_id: String, key: &FanKey) {
        self.modules
            .entry(mod_id)
            .and_modify(|fan| fan.increase(key))
            .or_insert(match key {
                FanKey::In => Fan { in_: 1, out: 0 },
                FanKey::Out => Fan { in_: 0, out: 1 },
            });
    }
}
