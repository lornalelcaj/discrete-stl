use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Trace {
    pub values: HashMap<String, Vec<bool>>,
}

impl Trace {
    pub fn new() -> Self {
        Trace {
            values: HashMap::new(),
        }
    }

    pub fn add_signal(&mut self, name: &str, values: Vec<bool>) {
        self.values.insert(name.to_string(), values);
    }

    pub fn value_at(&self, name: &str, time: usize) -> Option<bool> {
        self.values
            .get(name)
            .and_then(|signal| signal.get(time))
            .copied()
    }

    pub fn horizon(&self) -> usize {
        self.values
            .values()
            .map(|signal| signal.len())
            .max()
            .unwrap_or(0)
    }
}