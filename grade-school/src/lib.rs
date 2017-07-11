use std::collections::HashSet;


pub struct School {
    grades: HashSet<u32>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashSet::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        (self.grades.into_iter().collect::<Vec<u32>>()).to_vec()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        unimplemented!()
    }
}
