#[derive(Debug)]
pub struct Counter {
    counter: usize,
}

impl Default for Counter {
   fn default() -> Self {
        Self { counter: 0 }
    }
}

impl Counter {
    pub fn next(&mut self) -> usize {
        self.counter += 1;
        self.counter - 1
    }

    pub fn get(&self) -> usize {
        self.counter
    }
}
