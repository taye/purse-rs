use std::sync::Arc;
use std::cell::UnsafeCell;

use super::node::Node;

pub type ArcUnsafeNode<T> = Arc<UnsafeCell<Node<T>>>;

pub trait NodeCell<T> {
    fn get_node(&self) -> &Node<T>;
    fn get_node_mut(&self) -> Option<&mut Node<T>>;
}

impl<T> NodeCell<T> for Node<T> {
    fn get_node(&self) -> &Node<T> {
        self
    }

    fn get_node_mut(&self) -> Option<&mut Node<T>> {
        None
    }
}

impl<T> NodeCell<T> for UnsafeCell<Node<T>> {
    fn get_node(&self) -> &Node<T> {
        unsafe { &*self.get() }
    }

    fn get_node_mut(&self) -> Option<&mut Node<T>> {
        Some(unsafe { &mut *self.get() })
    }
}

impl<T> NodeCell<T> for AsRef<NodeCell<T>> {
    #[allow(unconditional_recursion)]
    fn get_node(&self) -> &Node<T> {
        (*self).get_node()
    }

    #[allow(unconditional_recursion)]
    fn get_node_mut(&self) -> Option<&mut Node<T>> {
        (*self).get_node_mut()
    }
}

#[cfg(test)]
mod test {
    use super::Node;
    use list::list::*;
    use super::*;
    use std::rc::Rc;

    #[test]
    fn arc() {
        let node = Node::new(0, List::empty());
        let node_link = Arc::new(node);

        let node = node_link.get_node();

        assert_eq!(node.data, 0);
    }

    #[test]
    fn arc_unsafe_cell() {
        let node = Node::new("mut".to_string(), List::empty());
        let node_link = Arc::new(UnsafeCell::new(node));

        let mut_node = node_link.get_node_mut().unwrap();

        assert_eq!(mut_node.data, "mut");

        mut_node.data += "ate";

        let node = node_link.get_node();

        assert_eq!(node.data, "mutate");
    }

    #[test]
    fn rc() {
        let node = Node::new(0, List::empty());
        let node_link = Rc::new(node);

        let node = node_link.get_node();

        assert_eq!(node.data, 0);
    }
}
