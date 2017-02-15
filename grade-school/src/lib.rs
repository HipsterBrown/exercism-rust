use std::collections::BTreeMap;
use std::vec;

type GradeLevel = vec::Vec<String>;

pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        School { 
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, level: u32, name: &str) {
        let grade = self.grades.entry(level).or_insert(GradeLevel::new()); 
        grade.push(name.to_string());
        grade.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect::<Vec<u32>>()
    }

    pub fn grade(&self, level: u32) -> Option<&GradeLevel> {
        self.grades.get(&level)
    }
}
