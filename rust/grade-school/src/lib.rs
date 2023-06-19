use std::collections::HashMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
pub struct School(HashMap<u32, Vec<String>>);

#[allow(clippy::new_without_default)]
impl School {
    pub fn new() -> School {
        Self(HashMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0
            .entry(grade)
            .and_modify(|e| {
                let (Ok(pos) | Err(pos)) = e.binary_search(&student.to_string());
                e.insert(pos, student.to_string())
            })
            .or_insert(vec![student.to_string()]);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.0.keys().copied().collect();
        grades.sort();
        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.0.get(&grade).map_or(Vec::new(), Vec::clone)
    }
}
