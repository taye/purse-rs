use std::sync::{Arc, Weak};
use std::cell::UnsafeCell;

use super::node::Node;

pub trait LinkRef<RN>
{
    type DataType: Clone;

    fn new(node: Node<Self::DataType>) -> Self;
    fn get(&self) -> &Node<Self::DataType>;
}

pub trait MutLinkRef<RN>: LinkRef<RN>
{
    fn get_mut(&self) -> &mut Node<Self::DataType>;
}

impl<T: Clone> LinkRef<Arc<Node<T>>> for Arc<Node<T>>
{
    type DataType = T;

    fn new(node: Node<T>) -> Self {
        Arc::new(node)
    }

    fn get(&self) -> &Node<T> {
        &*self
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
}
