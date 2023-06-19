use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: Hash + Eq>(HashSet<T>);

impl<T: Clone + Copy + Hash + Eq> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut set = HashSet::with_capacity(input.len());
        for x in input {
            set.insert(*x);
        }
        Self(set)
    }

    pub fn contains(&self, element: &T) -> bool {
        self.0.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.0.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.is_disjoint(&other.0)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0.intersection(&other.0).copied().collect())
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self(self.0.difference(&other.0).copied().collect())
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self(self.0.union(&other.0).copied().collect())
    }
}
