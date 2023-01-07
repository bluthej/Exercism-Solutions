pub fn find<T: Ord, U: AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    if Some(&key) < array.first() || array.last() < Some(&key) {
        return None;
    }
    let n = array.len() / 2;
    let (left, right) = array.split_at(n);
    if key < array[n] {
        return find(left, key);
    } else if array[n] < key {
        return find(right, key).map(|m| m + n);
    }
    Some(n)
}
