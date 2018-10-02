use geometry;
use common::*;
use primitive::*;
use element::*;

pub struct Group {
    pub transform: geometry::Matrix,
}

impl Group {
    pub fn new() -> Group {
        Group {
            transform: Matrix::identity(),
        }
    }
}

pub struct GroupBuilder<'a, V, Ctor>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    arena: &'a mut Arena<V>,
    current_node: NodeId,
    ctor: Ctor,
    group: Group,
}

impl<'a, V, Ctor> GroupBuilder<'a, V, Ctor>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    pub fn new(arena: &'a mut Arena<V>, ctor: Ctor) -> Self {
        let node = arena.new_node(ElementType::None);
        Self {
            arena: arena,
            current_node: node,
            ctor: ctor,
            group: Group::new(),
        }
    }

    pub fn commit(self) -> NodeId {
        // Unwrap is safe if we assume we never get an invalid NodeId.
        self.arena.get_mut(self.current_node).unwrap().data = ElementType::Group(self.group);
        self.current_node
    }

    pub fn append(mut self, element: ElementType<V>) -> Self {
        self.current_node.append(self.arena.new_node(element), &mut self.arena);
        self
    }
}