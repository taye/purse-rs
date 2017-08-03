use std::sync::{Arc, Weak};
use std::cell::UnsafeCell;
use std::ops::Deref;

use super::node::Node;

pub trait LinkRef<RT>
{
    type DataType: Clone;

    fn new(node: Node<Self::DataType>) -> Self;
    fn get(&self) -> &Node<Self::DataType>;
}

pub trait MutLinkRef<RT>: LinkRef<RT>
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

        LinkRef::get(&node_link);
    }
}
