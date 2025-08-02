#[derive(Debug)]
pub struct Range<'a, Item> {
    args: &'a Vec<Item>,
    count: usize,
    min: usize,
    max: usize,
}

impl<Item> Range<'_, Item> {
    pub fn new(args: &Vec<Item>, min: usize, max: usize) -> Range<Item> {
        Range { args, count: args.len(), min, max }
    }

    pub fn to_iter(&self) -> Result<&Vec<Item>, RangeError> {
        if self.count > self.max {
            return Err(RangeError::Over(self.count));
        } else if self.count < self.min {
            return Err(RangeError::Under(self.count));
        }
        Ok(self.args)
    }
}

pub enum RangeError {
    Over(usize),
    Under(usize),
}
