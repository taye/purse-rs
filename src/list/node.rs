use std::sync::{Arc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::UnsafeCell;
use std::fmt;

use List;

use super::link::{ArcUnsafeNode, NodeCell};

pub type Link<T> = Option<ArcUnsafeNode<T>>;
pub type WeakLink<T> = Option<Weak<UnsafeCell<Node<T>>>>;

pub fn new_link<T>(node: Node<T>) -> Link<T> {
    Some(Arc::new(UnsafeCell::new(node)))
}

pub fn get_unwrapped_link_node<T>(link: &ArcUnsafeNode<T>) -> &Node<T> {
    unsafe { &*link.get() }
}

pub fn get_unwrapped_link_node_mut<T>(link: &ArcUnsafeNode<T>) -> &mut Node<T> {
    (*link).get_node_mut().unwrap()
}

pub fn get_link_node<T>(link: &Link<T>) -> &Node<T> {
    unsafe { &*link.as_ref().unwrap().get() }
}

#[derive(Clone)]
pub struct Node<T> {
    pub data: T,
    pub next: List<T>,
    pub mutating: Arc<AtomicBool>,
}

pub struct NodeMutator<'a, T: 'a> {
    pub node: &'a mut Node<T>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: List<T>) -> Self {
        Node {
            data: data,
            next: next,
            mutating: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.data,
            _ => {
                assert!(self.next.size > 0);

                let node = &get_link_node(&self.next.head);

                node.index(index - 1)
            }
        }
    }

    pub fn concat_list(&self, start: &Link<T>, list: &List<T>) -> List<T>
    where
        T: Clone,
    {
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

    pub fn get_mutator(&mut self) -> Option<NodeMutator<T>> {
        let already_mutating = self.mutating.compare_and_swap(
            false,
            true,
            Ordering::Relaxed,
        );

        if already_mutating {
            None
        } else {
            Some(NodeMutator { node: self })
        }
    }
}

impl<'a, T> Drop for NodeMutator<'a, T> {
    fn drop(&mut self) {
        self.node.mutating.store(false, Ordering::Relaxed);
    }
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.next.head.as_ref().map_or(
            write!(f, "{:?}", self.data),
            |ref link| {
                write!(f, "{:?}, {:?}", self.data, get_unwrapped_link_node(&link))
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_mutator() {
        let mut node = Node::new("ed".to_string(), List::empty());

        {
            let node_mutator = node.get_mutator().unwrap();

            assert_eq!(node_mutator.node.data, "ed");
            assert!(node_mutator.node.mutating.load(Ordering::Relaxed));

            node_mutator.node.data += "it";

            // node_mutator will be dropped and the node marked as no longer mutating
        }

        assert!(!node.mutating.load(Ordering::Relaxed));
        assert_eq!(node.data, "edit");
    }
}
