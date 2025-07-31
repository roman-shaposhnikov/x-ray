use crate::response::{ Fan };

pub trait Entity {
    fn fan(&self) -> Fan;
}
