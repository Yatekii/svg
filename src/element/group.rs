use geometry;
use common::*;
use primitive::*;
use element::*;

#[derive(Debug)]
pub struct Group {
    pub local_nodes: Vec<NodeId>,
    pub transform: geometry::Matrix,
    pub fill: Color,
    pub stroke: Color,
    pub stroke_width: f32,
}

impl Group {
    pub fn new() -> Group {
        Group {
            local_nodes: Vec::new(),
            transform: Matrix::identity(),
            fill: Color::black(),
            stroke: Color::none(),
            stroke_width: 1.0,
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

    pub fn append<F>(mut self, f: F) -> Self
    where
        F: FnOnce(GroupBuilder<V>) -> ElementType<V> + Sized
    {
        let mut element = f(GroupBuilder::new(self.arena));
        let child_nodes = if let ElementType::Group(ref mut group) = element {
            group.local_nodes.drain(..).collect()
        } else {
            vec![]
        };
        let element_node = self.arena.new_node(element);
        child_nodes.into_iter().for_each(|node| element_node.append(node, self.arena));
        self.group.local_nodes.push(element_node);
        self
    }

    pub fn map<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Group) -> Group + Sized
    {
        self.group = f(self.group);
        self
    }

    pub fn finalize(self) -> Group {
        self.group
    }
}

impl BasicStylableElement for Group
{
    fn fill(mut self, fill: Color) -> Self {
        self.fill = fill;
        self
    }

    fn stroke(mut self, stroke: Color) -> Self {
        self.stroke = stroke;
        self
    }

    fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width;
        self
    }

    fn transform(mut self, matrix: Matrix) -> Self {
        self.transform = matrix;
        self
    }
}