use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        School { 
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, level: u32, name: &str) {
        unimplemented!();
        // self.grades.entry(level).or_insert(Vec::new()).push(name); 
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    pub fn grade(&self, level: u32) -> Iterator<Item=Vec<String>> {
        unimplemented!();
    }
}
