#[derive(Debug, Default)]
pub struct Counter {
    counter: usize,
}

impl Counter {
    pub fn next(&mut self) -> usize {
        self.counter += 1;
        self.counter - 1
    }

    pub const fn get(&self) -> usize {
        self.counter
    }
}
