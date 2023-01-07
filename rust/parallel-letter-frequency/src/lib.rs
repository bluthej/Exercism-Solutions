use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let freq = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    let n = input.len();
    let (q, r) = (n / worker_count, n % worker_count);
    let mut init = 0;
    for i in 0..worker_count {
        let m = if i < r { q + 1 } else { q };
        let end = init + m;

        let freq = Arc::clone(&freq);
        let list = &input[init..end];
        for word in list.iter() {
            let handle = thread::spawn(move || {
                let mut f = freq.lock().unwrap();
                for c in word.chars() {
                    f.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
                }
            });
            handles.push(handle);
        }

        init += m;
    }

    for handle in handles {
        handle.join().unwrap();
    }
    unimplemented!(
        "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
        input,
        match worker_count {
            1 => "1 worker".to_string(),
            _ => format!("{} workers", worker_count),
        }
    );
    freq.into_inner().unwrap()
}
