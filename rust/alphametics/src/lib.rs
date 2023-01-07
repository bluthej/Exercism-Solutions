use std::{
    collections::{HashMap, HashSet},
    iter,
};

use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (lhs, rhs) = input.split_once(" == ").unwrap();
    let addends: Vec<&str> = lhs.split(" + ").collect();

    let letters: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();

    let first_letters: HashSet<char> = addends
        .iter()
        .chain(iter::once(&rhs))
        .filter_map(|s| if s.len() > 1 { s.chars().next() } else { None })
        .collect();

    let mut sol: HashMap<char, usize> = letters
        .clone()
        .into_iter()
        .zip((0..letters.len()).into_iter())
        .collect();

    let arrs = (0..=9).permutations(letters.len()).unique();
    'arrangements: for arr in arrs {
        let mut digits = arr.into_iter();
        for &letter in &letters {
            sol.insert(letter, digits.next().unwrap());
        }
        if first_letters.iter().any(|c| sol.get(&c).unwrap() == &0) {
            continue 'arrangements;
        }
        let mut vlhs = 0;
        for addend in &addends {
            vlhs += addend
                .chars()
                .fold(0, |acc, c| 10 * acc + sol.get(&c).unwrap());
        }
        let vrhs = rhs
            .chars()
            .fold(0, |acc, c| 10 * acc + sol.get(&c).unwrap());
        if vlhs == vrhs {
            let sol: HashMap<char, u8> = sol.iter().map(|(&k, &v)| (k, v as u8)).collect();
            return Some(sol);
        }
    }
    return None;
}
