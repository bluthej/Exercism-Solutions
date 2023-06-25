// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::{fmt::Display, ops::Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<S: Display>(matcher: impl Fn(T) -> bool + 'static, subs: S) -> Matcher<T> {
        Self {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Display + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(self, matcher: Matcher<T>) -> Self {
        let mut matchers = self.matchers;
        matchers.push(matcher);
        Self { matchers }
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        // unimplemented!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        iter.map(move |item| {
            let mut answer = String::new();
            for matcher in &self.matchers {
                if (matcher.matcher)(item) {
                    answer.push_str(&matcher.subs);
                }
            }
            if answer.is_empty() {
                return item.to_string();
            }
            answer
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + Copy + Rem + From<u8>,
    <T as std::ops::Rem>::Output: PartialEq + From<u8>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3u8.into() == 0u8.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5u8.into() == 0u8.into(), "buzz"))
}
