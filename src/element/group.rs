use geometry;
use common::*;
use primitive::*;
use element::*;

#[derive(Debug)]
pub struct Group {
    pub local_nodes: Vec<NodeId>,
    pub transform: geometry::Matrix,
}

impl Group {
    pub fn new() -> Group {
        Group {
            local_nodes: Vec::new(),
            transform: Matrix::identity(),
        }
    }

    pub fn wrap<V>(self) -> ElementType<V>
    where
        V: TransformPrimitive + ColorPrimitive + Clone {
        ElementType::Group(self)
    }
}

pub struct GroupBuilder<'a, V>
where
    V: 'a + TransformPrimitive + ColorPrimitive + Clone {
    arena: &'a mut Arena<V>,
    group: Group,
}

impl<'a, V> GroupBuilder<'a, V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {

    pub fn new(arena: &'a mut Arena<V>) -> Self {
        Self {
            arena: arena,
            group: Group::new(),
        }
    }

    pub fn to_root(mut self) -> NodeId {
        let child_nodes: Vec<NodeId> = self.group.local_nodes.drain(..).collect();
        let root_node = self.arena.new_node(self.group.wrap());
        let arena = self.arena;
        child_nodes.into_iter().for_each(|node| root_node.append(node, arena));
        root_node
    }

    pub fn append<I, F>(mut self, f: F) -> Self
    where
        I: Into<ElementType<V>>,
        F: FnOnce(GroupBuilder<'a, V>) -> I {

        // TODO: fix!
        // let mut element = { f(GroupBuilder::new(self.arena)).into() };
        // let child_nodes = if let ElementType::Group(ref mut group) = element {
        //     group.local_nodes.drain(..).collect()
        // } else {
        //     vec![]
        // };
        // let element_node = self.arena.new_node(element);
        // child_nodes.into_iter().for_each(|node| element_node.append(node, self.arena));
        // self.group.local_nodes.push(element_node);
        self
    }

    pub fn finalize(self) -> Group {
        // Unwrap is safe if we assume we never get an invalid NodeId.
        self.group
    }
}

impl<'a, V> From<GroupBuilder<'a, V>> for Group
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    fn from(group_builder: GroupBuilder<'a, V>) -> Self {
        group_builder.group
    }
}

impl<'a, V> From<GroupBuilder<'a, V>> for ElementType<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    fn from(group_builder: GroupBuilder<'a, V>) -> Self {
        ElementType::Group(group_builder.group)
    }
}