#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use std::cmp::Ordering;

    let (bigger, smaller, res) = match first_list.len().cmp(&second_list.len()) {
        Ordering::Greater => (first_list, second_list, Comparison::Superlist),
        Ordering::Equal => (first_list, second_list, Comparison::Equal),
        Ordering::Less => (second_list, first_list, Comparison::Sublist),
    };

    if smaller.is_empty()
        || bigger
            .windows(smaller.len())
            .any(|window| window == smaller)
    {
        return res;
    }

    Comparison::Unequal
}
