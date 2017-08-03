use std::sync::{Arc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;
use std::ops::Index;
use std::fmt;

type Link<T> = Option<Arc<UnsafeCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<UnsafeCell<Node<T>>>>;

fn new_link<T: Clone>(node: Node<T>) -> Link<T> {
    Some(Arc::new(UnsafeCell::new(node)))
}

fn get_unwrapped_link_node<T: Clone>(link: &Arc<UnsafeCell<Node<T>>>) -> &Node<T> {
    get_unwrapped_link_node_mut(link)
}

fn get_unwrapped_link_node_mut<T: Clone>(link: &Arc<UnsafeCell<Node<T>>>) -> &mut Node<T> {
    unsafe { &mut *link.get() }
}

fn get_link_node<T: Clone>(link: &Link<T>) -> &Node<T> {
    get_unwrapped_link_node(link.as_ref().unwrap())
}

#[derive(Clone)]
pub(super) struct Node<T: Clone> {
    pub(super) data: T,
    pub(super) next: List<T>,
    mutating: Arc<AtomicBool>,
}

/// A persistent singly linked list of elements.
///
/// Examples
///
/// ```
/// use purse::List;
///
/// let letters = List::create("o", List::create("n", List::create("e", List::empty())));
/// let word = letters.into_iter().fold(String::new(), |acc, x| acc + x);
///
/// assert_eq!(word, "one");
/// ```
///
/// ```
/// #[macro_use] extern crate purse;
/// fn main() {
///     use purse::List;
///     use std::iter::FromIterator;
///
///     let list1 = purse_list![0, 1];
///     let list2: List<List<i32>> = std::iter::repeat(list1.clone()).take(2).collect();
///
///     assert_eq!(list2, purse_list![purse_list![0, 1], purse_list![0, 1]]);
///     assert_eq!(list2[1], list1);
/// }
/// ```
#[derive(Clone, Default)]
pub struct List<T: Clone> {
    pub(super) head: Link<T>,
    pub(super) tail: WeakLink<T>,
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
    /// assert_eq!(empty.prepend(9), purse_list![9]);
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    /// let prepended = list.prepend(0);
    ///
    /// assert_eq!(prepended, purse_list![0, 1, 2]);
    /// # assert_eq!(prepended.last(), Some(&2));
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
            mutating: Arc::new(AtomicBool::new(false)),
        };

        let head = new_link(node);

        List {
            head: head.clone(),
            tail: self.tail.clone(),
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
    /// assert_eq!(empty.append(9), purse_list![9]);
    ///
    /// let list = List::create(1, List::create(2, List::empty()));
    /// let appended = list.append(3);
    ///
    /// assert_eq!(appended, purse_list![1, 2, 3]);
    /// # }
    /// ```
    pub fn append(self, data: T) -> Self {
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
    /// assert_eq!(list, purse_list![1, 2]);
    /// # assert_eq!(list.last(), Some(&2));
    /// # }
    /// ```
    pub fn create(data: T, rest: Self) -> Self {
        let tail = rest.tail.clone();
        let size = 1 + rest.size;
        let head = new_link(Node {
            data: data,
            next: rest,
            mutating: Arc::new(AtomicBool::new(false)),
        });

        List {
            head: head.clone(),
            tail: tail.or(head.as_ref().map(Arc::downgrade)),
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
    /// let list3 = list2.clone();
    ///
    /// let concat_1_2 = list1.concat(&list2);
    /// let concat_2_3 = list2.concat(&list3);
    ///
    /// assert_eq!(concat_1_2, purse_list![1, 2, 3, 4]);
    /// assert_eq!(concat_2_3, purse_list![3, 4, 3, 4]);
    ///
    /// let empty: List<()> = List::empty();
    ///
    /// assert_eq!(empty.clone().concat(&empty).len(), 0);
    /// # }
    /// ```
    pub fn concat(self, right: &Self) -> Self {
        let do_immut = || List::concat_immut(&self.head, right);

        self.head.as_ref().map_or(right.clone(), |link| {
            // if the right list is empty, return this same list cloned
            if right.size == 0 {
                return self.clone();
            }

            // If another list has this link as its head, concat immutably
            if Arc::strong_count(link) != 1 {
                return do_immut();
            }

            let node = get_unwrapped_link_node_mut(link);
            let already_mutating = node.mutating.compare_and_swap(
                false,
                true,
                Ordering::Relaxed,
            );

            // if another list is modifying this link, concat immutably
            if already_mutating {
                do_immut()
            } else {
                let mut list = self.clone();

                list.concat_mut(right);
                node.mutating.store(false, Ordering::Relaxed);

                list
            }
        })
    }

    pub(super) fn concat_immut(link: &Link<T>, right: &Self) -> Self {
        let node = get_unwrapped_link_node(link.as_ref().unwrap());

        node.concat_list(link, &right)
    }

    // Add the elements of a list to an existing list by mutating its fields recursively.
    pub(super) fn concat_mut(&mut self, right: &Self) {
        let head = match self.head.clone() {
            Some(link) => {
                get_unwrapped_link_node_mut(&link).next.concat_mut(right);

                Some(link)
            }
            None => right.head.clone(),
        };

        if let Some(ref link) = self.tail {
            let tail = link.upgrade().unwrap();
            let tail_node = get_unwrapped_link_node_mut(&tail);

            tail_node.next = right.clone();
        };

        self.head = head.or(right.head.clone());
        self.tail = right.tail.clone();
        self.size += right.size;
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

    fn get_link_data(link: &Link<T>) -> Option<&T> {
        link.as_ref().map(|link_cell| {
            &get_unwrapped_link_node(&link_cell).data
        })
    }

    /// Returns a reference to the first element of the list or None if it's empty.
    ///
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let list1 = purse_list![1, 2, 3];
    /// let list2 = purse_list!['x', 'y', 'z'];
    ///
    /// assert_eq!(list1.first().unwrap(), &1);
    /// assert_eq!(list2.first().unwrap(), &'x');
    /// # }
    /// ```
    pub fn first(&self) -> Option<&T> {
        List::get_link_data(&self.head)
    }

    /// Returns a reference to the last element of the list or None if it's empty.
    ///
    /// #Examples
    ///
    /// ```
    /// # #[macro_use] extern crate purse;
    /// # fn main() {
    /// use purse::List;
    ///
    /// let list1 = purse_list![1, 2, 3];
    /// let list2 = purse_list!['x', 'y', 'z'];
    ///
    /// assert_eq!(list1.last().unwrap(), &3);
    /// assert_eq!(list2.last().unwrap(), &'z');
    /// # }
    /// ```
    pub fn last(&self) -> Option<&T> {
        self.tail.as_ref().map(|weak| unsafe {
            &(*weak.upgrade().unwrap().get()).data
        })
    }
}

/// Elements of a list may be accessed by index.
///
/// #Examples
///
/// ```
/// # #[macro_use] extern crate purse;
/// # fn main() {
/// let list = purse_list![1, 2, 3];
///
/// assert_eq!(list[0], 1);
/// assert_eq!(list[2], 3);
/// # }
/// ```
impl<T: Clone> Index<usize> for List<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index >= self.size {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.size,
                index
            );
        }

        &get_link_node(&self.head).index(index)
    }
}

impl<T: Clone> Node<T> {
    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.data,
            _ => {
                assert!(self.next.size > 0);

                let node = &get_link_node(&self.next.head);

                node.index(index - 1)
            }
        }
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
    /// let list1 = List::create(1, List::create(2, List::empty()));
    /// let list2 = List::create(1, List::create(2, List::empty()));
    /// let list3 = list2.clone().concat(&List::empty());
    ///
    /// assert!(list1 == list2);
    /// assert!(list1 == list3);
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
                let self_head = get_unwrapped_link_node(&self_head);
                let other_head = get_unwrapped_link_node(&other_head);

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
        self.head.as_ref().map_or(write!(f, "[]"), |ref link| {
            write!(f, "[{:?}]", get_unwrapped_link_node(&link))
        })
    }
}

impl<T: Clone + fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.next.head.as_ref().map_or(
            write!(f, "{:?}", self.data),
            |ref link| {
                write!(f, "{:?}, {:?}", self.data, get_unwrapped_link_node(&link))
            },
        )
    }
}

impl<T: Clone> Node<T> {
    fn concat_list(&self, start: &Link<T>, list: &List<T>) -> List<T> {
        self.next.head.as_ref().map_or(
            List::create(
                self.data.clone(),
                list.clone(),
            ),
            |link| {
                let node = get_unwrapped_link_node(&link);

                List::create(self.data.clone(), node.concat_list(start, list))
            },
        )
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
/// let list = purse_list![1, 2, 3];
///
/// assert_eq!(list, List::create(1, List::create(2, List::create(3, List::empty()))));
/// # }
/// ```
#[macro_export]
macro_rules! purse_list {
    [] => { $crate::List::empty() };

    [ $head:expr ] => {
        $crate::List::create($head, $crate::List::empty())
    };

    [ $head:expr, $($rest:expr),* ] => {
        $crate::List::create($head, purse_list![$($rest),*])
    };
}
