use std::iter::{Iterator, FromIterator, IntoIterator};
use List;

pub struct IntoIter<T: Clone> {
    list: List<T>,
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let list = self.list.clone();

        match list.head {
            Some(ref link) => {
                let node = unsafe { &*link.get() };

                self.list = node.next.clone();

                Some(node.data.clone())
            }
            None => None,
        }
    }
}

impl<T: Clone> FromIterator<T> for List<T> {
    /// Crates a List from an Iterator.
    ///
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    /// use std::iter::FromIterator;
    ///
    /// let from_range: List<i32> = (1..4).collect();
    ///
    /// assert_eq!(from_range, purse_list![1, 2, 3]);
    ///
    /// let from_vec = List::from_iter(vec![4, 5, 6]);
    ///
    /// assert_eq!(from_vec, purse_list![4, 5, 6]);
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

impl<T: Clone> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates an `Iterator` from a `List`.
    ///
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    /// use std::iter::IntoIterator;
    ///
    /// let list = purse_list!["a", "b", "c"];
    /// let mut string = String::new();
    ///
    /// for elem in list {
    ///     string += elem;
    /// }
    ///
    /// assert_eq!(string, "abc");
    ///
    /// # }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}
