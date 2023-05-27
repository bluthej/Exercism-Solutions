/// What should the type of _function be?
pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut output = Vec::with_capacity(input.len());
    for x in input.into_iter() {
        output.push(function(x));
    }
    output
}
