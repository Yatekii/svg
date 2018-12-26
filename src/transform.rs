use basic_style::BasicStylableElement;
use common::Node;
use common::Arena;
use common::NodeId;
use primitive::TransformPrimitive;
use primitive::ColorPrimitive;
use element::ElementType;

pub struct Extractor<'a, V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    arena: &'a mut Arena<V>,
    root: NodeId
}

pub struct Iter<'a, V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    arena: Option<&'a mut Arena<V>>,
    root: NodeId,
    returned: bool
}

impl<'a, V> Iter<'a, V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    pub fn new(arena: &'a mut Arena<V>, root: NodeId) -> Self {
        Iter {
            arena: Some(arena),
            root,
            returned: false
        }
    }
}

impl<'a, V> Extractor<'a, V>
where V: TransformPrimitive + ColorPrimitive + Clone {
    fn query(&mut self, f: impl FnOnce(&mut dyn BasicStylableElement)) -> Iter<'_, V> {
        Iter::new(self.arena, self.root)
    } 
}

impl<'a, V> Iterator for Iter<'a, V>
where V: TransformPrimitive + ColorPrimitive + Clone  {
    type Item = &'a mut Node<V>;

    fn next(&mut self) -> Option<&'a mut Node<V>> {
        if !self.returned {
            return self.arena.take().unwrap().get_mut(self.root);
        }
        None
    }
}

pub trait Transformer {
    fn transform(&mut self, f: impl FnOnce(&mut dyn BasicStylableElement));
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Transformer for Node<V> {
    fn transform(&mut self, f: impl FnOnce(&mut dyn BasicStylableElement)) {
        match &mut self.data {
            ElementType::Circle(ref mut c) => f(c),
            ElementType::Line(ref mut c) => f(c),
            ElementType::Path(ref mut c) => f(c),
            ElementType::Rect(ref mut c) => f(c),
            ElementType::Group(ref mut c) => f(c),
        }
    }
}