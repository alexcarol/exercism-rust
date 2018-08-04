use std::collections::HashMap;

#[allow(unused_variables)]

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School { grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if  !self.grades.contains_key(&grade) {
            self.grades.insert(grade, vec![]);
        }

        let grades = self.grades.get_mut(&grade).unwrap();
        grades.push(String::from(student));
        grades.sort();

        ()
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut last: u32 = 123412;
        let mut grades = self.grades
            .keys()
            .map(|grade| *grade)
            .filter(|grade| {
                let equals_last = last == *grade;
                last = *grade;
                !equals_last
            })
            .collect::<Vec<u32>>();

        grades.sort();

        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).cloned()
    }
}
