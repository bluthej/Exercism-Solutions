use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let freq = Arc::new(Mutex::new(HashMap::new()));

    thread::scope(|s| {
        for i in 0..worker_count {
            let freq = Arc::clone(&freq);
            s.spawn(move || {
                let mut map = HashMap::new();
                for word in input.iter().skip(i).step_by(worker_count) {
                    for c in word.to_lowercase().chars().filter(|c| c.is_alphabetic()) {
                        map.entry(c)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                    }
                }
                let mut freq = freq.lock().unwrap();
                for (k, v) in map.into_iter() {
                    freq.entry(k)
                        .and_modify(|counter| *counter += v)
                        .or_insert(v);
                }
            });
        }
    });

    let res = freq.lock().unwrap();
    res.to_owned()
}
