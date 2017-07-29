use std::iter::{Iterator, FromIterator};
use std::sync::Arc;
use std::fmt;

#[derive(Clone, Debug)]

struct Node<T: Clone> {
    data: T,
    next: List<T>,
}

#[derive(Clone, Debug)]
pub struct List<T: Clone> {
    head: Option<Arc<Node<T>>>,
    tail: Option<Arc<Node<T>>>,
    size: usize,
}

impl<T: Clone> List<T> {
    /// Creates an empty list.
    ///
    /// #Examples
    ///
    /// ```
    /// use immutable::List;
    ///
    /// let list: List<i32> = List::empty();
    ///
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn empty() -> Self {
        List {
            head: None,
            tail: None,
            size: 0,
        }
    }

    /// Creates a list that starts with the given element and continues with the original list
    ///
    /// #Examples
    ///
    /// ```
    /// use immutable::List;
    ///
    /// let empty: List<i32> = List::empty();
    ///
    /// assert_eq!(empty.prepend(9).to_string(), "[9]");
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    /// let prepended = list.prepend(0);
    ///
    /// assert_eq!(prepended.to_string(), "[0, 1, 2]");
    /// ```
    pub fn prepend(&self, data: T) -> Self {
        let node = Node {
            data: data,
            next: List {
                head: self.head.clone(),
                tail: self.tail.clone(),
                size: self.size,
            },
        };

        let head = Some(Arc::new(node));

        List {
            head: head.clone(),
            tail: head,
            size: 1 + self.size,
        }
    }

    /// Creates a list that starts with the original list and ends with the given element
    ///
    /// #Examples
    ///
    /// ```
    /// use immutable::List;
    ///
    /// let empty: List<i32> = List::empty();
    ///
    /// assert_eq!(empty.append(9).to_string(), "[9]");
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    /// let appended = list.append(3);
    ///
    /// assert_eq!(appended.to_string(), "[1, 2, 3]");
    /// ```
    pub fn append(&self, data: T) -> Self {
        self.concat(&List::create(data, List::empty()))
    }

    /// Creates a list from an item and the tail list.
    ///
    /// #Examples
    ///
    /// ```
    /// use immutable::List;
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    ///
    /// assert_eq!(list.to_string(), "[1, 2]");
    /// ```
    pub fn create(data: T, rest: Self) -> Self {
        let tail = rest.tail.clone();
        let size = 1 + rest.size;
        let head = Some(Arc::new(Node {
            data: data,
            next: rest,
        }));

        List {
            head: head,
            tail: tail,
            size: size,
        }
    }

    /// Creates a new list with the elements of the first list followed by the elements of the
    /// second.
    ///
    /// #Examples
    ///
    /// ```
    /// use immutable::List;
    ///
    /// let list1 = List::create(1, List::create(2, List::empty()));
    /// let list2 = List::create(3, List::create(4, List::empty()));
    ///
    /// let concatted = list1.concat(&list2);
    ///
    /// assert_eq!(concatted.to_string(), "[1, 2, 3, 4]");
    ///
    /// let empty: List<()> = List::empty();
    ///
    /// assert_eq!(empty.concat(&empty).len(), 0);
    /// ```
    pub fn concat(&self, right: &Self) -> Self {
        match self.head {
            Some(ref link) => link.concat_list(link, &right),
            None => right.clone(),
        }
    }

    /// Rerives the length of a list.
    ///
    /// #Examples
    ///
    /// ```
    /// use immutable::List;
    ///
    /// let list1 = List::create(1, List::create(2, List::empty()));
    /// let list2 = List::<i32>::empty();
    ///
    /// assert_eq!(list1.len(), 2);
    /// assert_eq!(list2.len(), 0);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }
}

/// #Examples
///
/// ```
/// use immutable::List;
/// use std::iter::FromIterator;
///
/// let from_range: List<i32> = (1..4).collect();
///
/// assert_eq!(from_range.to_string(), "[1, 2, 3]");
///
/// let from_vec = List::from_iter(vec![4, 5, 6]);
///
/// assert_eq!(from_vec.to_string(), "[4, 5, 6]");
/// ```
impl<T: Clone> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        match iter.next() {
            Some(data) => List::create(data, List::from_iter(iter)),
            _ => List::empty(),
        }
    }
}

impl<T: Clone + fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.head {
            Some(ref link) => write!(f, "[{}]", link),
            None => write!(f, "[]"),
        }
    }
}

impl<T: Clone + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.next.head {
            Some(ref link) => write!(f, "{}, {}", self.data, link),
            None => write!(f, "{}", self.data),
        }
    }
}

impl<T: Clone> Node<T> {
    fn concat_list(&self, start: &Arc<Node<T>>, list: &List<T>) -> List<T> {
        match self.next.head {
            Some(ref link) => List::create(self.data.clone(), link.concat_list(start, list)),
            None => List::create(self.data.clone(), list.clone()),
        }
    }
}
