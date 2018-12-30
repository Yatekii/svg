use indextree;
use crate::element;

pub type Arena<V> = indextree::Arena<element::ElementType<V>>;
pub type Node<V> = indextree::Node<element::ElementType<V>>;
pub type NodeId = indextree::NodeId;