use std::sync::Arc;
use std::fmt;

#[derive(Clone)]
pub(super) struct Node<T: Clone> {
    pub(super) data: T,
    pub(super) next: List<T>,
}

#[derive(Clone)]
pub struct List<T: Clone> {
    pub(super) head: Option<Arc<Node<T>>>,
    pub(super) tail: Option<Arc<Node<T>>>,
    pub(super) size: usize,
}

impl<T: Clone> List<T> {
    /// Creates an empty list.
    ///
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let list: List<i32> = List::empty();
    ///
    /// assert_eq!(list.len(), 0);
    /// # }
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
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let empty: List<i32> = List::empty();
    ///
    /// assert_eq!(empty.prepend(9), list![9]);
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    /// let prepended = list.prepend(0);
    ///
    /// assert_eq!(prepended, list![0, 1, 2]);
    /// # }
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
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let empty: List<i32> = List::empty();
    ///
    /// assert_eq!(empty.append(9), list![9]);
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    /// let appended = list.append(3);
    ///
    /// assert_eq!(appended, list![1, 2, 3]);
    /// # }
    /// ```
    pub fn append(&self, data: T) -> Self {
        self.concat(&List::create(data, List::empty()))
    }

    /// Creates a list from an item and the tail list.
    ///
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    ///
    /// assert_eq!(list, list![1, 2]);
    /// # }
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
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let list1 = List::create(1, List::create(2, List::empty()));
    /// let list2 = List::create(3, List::create(4, List::empty()));
    ///
    /// let concatted = list1.concat(&list2);
    ///
    /// assert_eq!(concatted, list![1, 2, 3, 4]);
    ///
    /// let empty: List<()> = List::empty();
    ///
    /// assert_eq!(empty.concat(&empty).len(), 0);
    /// # }
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
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let list1 = List::create(1, List::create(2, List::empty()));
    /// let list2 = List::<i32>::empty();
    ///
    /// assert_eq!(list1.len(), 2);
    /// assert_eq!(list2.len(), 0);
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T> PartialEq for List<T>
where
    T: Clone + PartialEq,
{
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let list1: List<i32> = List::create(1, List::create(2, List::empty()));
    /// let list2: List<i32> = List::create(1, List::create(2, List::empty()));
    ///
    /// assert!(list1 == list2);
    /// assert!(List::<i32>::empty() == List::<i32>::empty());
    /// assert!(list1 != List::<i32>::empty());
    /// # }
    /// ```
    fn eq(&self, other: &Self) -> bool {
        // different lengths
        if self.size != other.size {
            return false;
        };

        match (&self.head, &other.head) {
            // both empty
            (&None, &None) => true,
            (&Some(ref self_head), &Some(ref other_head)) => {
                self_head.data == other_head.data && self_head.next == other_head.next
            }
            _ => false,
        }
    }
}

impl<T> Eq for List<T>
where
    T: Clone + PartialEq,
{
}

impl<T: Clone + fmt::Debug> fmt::Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.head {
            Some(ref link) => write!(f, "[{:?}]", link),
            None => write!(f, "[:"),
        }
    }
}

impl<T: Clone + fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.next.head {
            Some(ref link) => write!(f, "{:?}, {:?}", self.data, link),
            None => write!(f, "{:?}", self.data),
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

/// Macro for convenient list creation
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate purse;
/// # fn main() {
/// use purse::List;
/// let list = list![1, 2, 3];
///
/// assert_eq!(list, List::create(1, List::create(2, List::create(3, List::empty()))));
/// # }
/// ```
#[macro_export]
macro_rules! list {
    [] => { $crate::List::empty() };

    [ $head:expr ] => {
        $crate::List::create($head, $crate::List::empty())
    };

    [ $head:expr, $($rest:expr),* ] => {
        $crate::List::create($head, list![$($rest),*])
    };
}