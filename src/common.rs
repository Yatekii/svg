use indextree;
use element;
use primitive::*;

pub type Arena<V> = indextree::Arena<element::ElementType<V>>;
pub type Node<V> = indextree::Node<element::ElementType<V>>;
pub type NodeId = indextree::NodeId;