use indextree;
use element;
use primitive::*;

pub type Arena<V: TransformPrimitive + ColorPrimitive + Clone> = indextree::Arena<element::ElementType<V>>;
pub type Node<V: TransformPrimitive + ColorPrimitive + Clone> = indextree::Node<element::ElementType<V>>;
pub type NodeId = indextree::NodeId;