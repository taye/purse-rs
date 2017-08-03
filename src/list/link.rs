use std::sync::{Arc, Weak};
use std::cell::UnsafeCell;

use super::node::Node;

pub type ArcUnsafeNode<T> = Arc<UnsafeCell<Node<T>>>;

pub trait LinkRef<RN>
{
    type DataType: Clone;

    fn get(&self) -> &Node<Self::DataType>;
}

pub trait LinkRefMut<RN>: LinkRef<RN>
{
    fn get_mut(&self) -> &mut Node<Self::DataType>;
}

impl<T: Clone> LinkRef<Arc<Node<T>>> for Arc<Node<T>>
{
    type DataType = T;

    fn get(&self) -> &Node<T> {
        &*self
    }
}

impl<T> LinkRef<ArcUnsafeNode<T>> for ArcUnsafeNode<T>
where T: Clone
{
    type DataType = T;

    fn get(&self) -> &Node<T> {
        unsafe { &*(**self).get() }
    }
}

impl<T> LinkRefMut<ArcUnsafeNode<T>> for ArcUnsafeNode<T>
where
    T: Clone,
{
    fn get_mut(&self) -> &mut Node<T> {
        unsafe { &mut *(**self).get() }
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

        let mut_node = node_link.get_mut();

        assert_eq!(mut_node.data, "mut");

        mut_node.data += "ate";

        let node = node_link.get();

        assert_eq!(node.data, "mutate");
    }
}
