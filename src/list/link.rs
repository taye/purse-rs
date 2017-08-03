use std::sync::{Arc, Weak};
use std::cell::UnsafeCell;

use super::node::Node;

pub type ArcUnsafeNode<T> = Arc<UnsafeCell<Node<T>>>;

pub trait LinkRef {
    type Data;

    fn get(&self) -> &Node<Self::Data>;
    fn get_mut(&self) -> Option<&mut Node<Self::Data>>;
}

impl<T> LinkRef for Arc<Node<T>> {
    type Data = T;

    fn get(&self) -> &Node<T> {
        &*self
    }

    fn get_mut(&self) -> Option<&mut Node<T>> {
        None
    }
}

impl<T> LinkRef for ArcUnsafeNode<T> {
    type Data = T;

    fn get(&self) -> &Node<T> {
        unsafe { &*(**self).get() }
    }

    fn get_mut(&self) -> Option<&mut Node<T>> {
        Some(unsafe { &mut *(**self).get() })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use list::list::List;

    #[test]
    fn arc() {
        let node = Node::new(0, List::empty());
        let node_link = Arc::new(node);

        let node = node_link.get();

        assert_eq!(node.data, 0);
    }

    #[test]
    fn arc_unsafe_cell() {
        let node = Node::new("mut".to_string(), List::empty());
        let node_link = Arc::new(UnsafeCell::new(node));

        let mut_node = node_link.get_mut().unwrap();

        assert_eq!(mut_node.data, "mut");

        mut_node.data += "ate";

        let node = node_link.get();

        assert_eq!(node.data, "mutate");
    }
}
