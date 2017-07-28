use std::iter::{Iterator, FromIterator};
use std::sync::Arc;
use std::fmt;

type Link = Arc<Node>;

#[derive(Clone, Debug)]

struct Node {
    data: i32,
    next: List,
}

#[derive(Clone, Debug)]
pub struct List {
    head: Option<Link>,
    tail: Option<Link>,
    size: usize,
}

impl List {
    /// Creates an empty list.
    ///
    /// #Examples
    ///
    /// ```
    /// use immutable::List;
    ///
    /// let list = List::empty();
    ///
    /// assert_eq!(list.len(), 0);
    /// ```
    pub fn empty() -> List {
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
    /// assert_eq!(List::empty().prepend(9).to_string(), "[9]");
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    /// let prepended = list.prepend(0);
    ///
    /// assert_eq!(prepended.to_string(), "[0, 1, 2]");
    /// ```
    pub fn prepend(&self, data: i32) -> List {
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
    pub fn create(data: i32, rest: List) -> List {
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
    /// assert_eq!(List::empty().concat(&List::empty()).to_string(), "[]");
    /// ```
    pub fn concat(&self, right: &Self) -> List {
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
    /// let list2 = List::empty();
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
/// let from_range: List = (1..4).collect();
///
/// assert_eq!(from_range.to_string(), "[1, 2, 3]");
///
/// let from_vec = List::from_iter(vec![4, 5, 6]);
///
/// assert_eq!(from_vec.to_string(), "[4, 5, 6]");
/// ```
impl FromIterator<i32> for List {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        match iter.next() {
            Some(data) => List::create(data, List::from_iter(iter)),
            _ => List::empty(),
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.head {
            Some(ref link) => write!(f, "[{}]", link),
            None => write!(f, "[]"),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.next.head {
            Some(ref link) => write!(f, "{}, {}", self.data, link),
            None => write!(f, "{}", self.data),
        }
    }
}

impl Node {
    fn concat_list(&self, start: &Link, list: &List) -> List {
        let result = match self.next.head {
            Some(ref link) => List::create(self.data, link.concat_list(start, list)),
            None => List::create(self.data, list.clone()),
        };

        result
    }
}
