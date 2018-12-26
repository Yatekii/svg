use basic_style::BasicStylableElement;
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
    V: TransformPrimitive + ColorPrimitive + Clone {
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
        F: FnOnce(GroupBuilder<'_, V>) -> I {

        let mut element = { f(GroupBuilder::new(self.arena)).into() };
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
    impl_basic_style!(self, [fill, fill_ref](fill: Color) {
        self.fill = fill;
    });

    impl_basic_style!(self, [stroke, stroke_ref](stroke: Color) {
        self.stroke = stroke;
    });

    impl_basic_style!(self, [stroke_width, stroke_width_ref](stroke_width: f32) {
        self.stroke_width = stroke_width;
    });

    impl_basic_style!(self, [transform, transform_ref](matrix: Matrix) {
        self.transform = matrix;
    });
}