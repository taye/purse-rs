use std::iter::{Iterator, FromIterator};
use List;

impl<T: Clone> FromIterator<T> for List<T> {
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate immutable;
    /// # fn main() {
    /// use immutable::List;
    /// use std::iter::FromIterator;
    ///
    /// let from_range: List<i32> = (1..4).collect();
    ///
    /// assert_eq!(from_range, list![1, 2, 3]);
    ///
    /// let from_vec = List::from_iter(vec![4, 5, 6]);
    ///
    /// assert_eq!(from_vec, list![4, 5, 6]);
    /// # }
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        match iter.next() {
            Some(data) => List::create(data, List::from_iter(iter)),
            _ => List::empty(),
        }
    }
}
