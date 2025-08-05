#[derive(Debug)]
pub struct Response {
    pub total: Total,
    pub entities: Entities,
}

#[derive(Debug)]
pub struct Total {
    pub modules: u32,
    pub classes: u32,
}

#[derive(Debug)]
pub struct Entities {
    pub modules: Vec<Entity>,
    pub classes: Vec<Entity>,
}

#[derive(Debug)]
pub struct Entity {
    pub id: String,
    pub fan: Fan,
}

#[derive(Debug)]
pub struct Fan {
    pub in_: u32,
    pub out: u32,
}

pub enum FanKey {
    In,
    Out,
}

impl Fan {
    pub fn increase(&mut self, key: &FanKey) {
        match key {
            FanKey::In => {
                self.in_ += 1;
            }
            FanKey::Out => {
                self.out += 1;
            }
        }
    }
}
