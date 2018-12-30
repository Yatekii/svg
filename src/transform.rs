use crate::basic_style::BasicStylableElement;
use crate::common::Node;
use crate::common::Arena;
use crate::common::NodeId;
use crate::primitive::TransformPrimitive;
use crate::primitive::ColorPrimitive;
use crate::element::ElementType;

pub struct Extractor<'a, V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    arena: &'a mut Arena<V>,
    root: NodeId
}

impl<'a, V> Extractor<'a, V>
where V: TransformPrimitive + ColorPrimitive + Clone {
    pub fn new(arena: &'a mut Arena<V>, root: NodeId) -> Extractor<V> {
        Extractor {
            arena,
            root
        }
    }

    pub fn query(&mut self, _f: impl FnOnce(&mut dyn BasicStylableElement) -> bool) -> Iter<'_, V> {
        Iter::new(self.arena, self.root)
    } 
}

pub struct Iter<'a, V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    arena: &'a mut Arena<V>,
    root: NodeId,
    returned: bool
}

impl<'a, V> Iter<'a, V>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    pub fn new(arena: &'a mut Arena<V>, root: NodeId) -> Self {
        Iter {
            arena,
            root,
            returned: false
        }
    }
}

impl<'a, V> Iterator for Iter<'a, V>
where V: TransformPrimitive + ColorPrimitive + Clone  {
    type Item = &'a mut Node<V>;

    fn next(&mut self) -> Option<&'a mut Node<V>> {
        if !self.returned { unsafe { self.arena.get_mut(self.root).map(|node| &mut *(node as *mut _)) } } else { None }
    }
}

pub trait Transformer<V>
where V: TransformPrimitive + ColorPrimitive + Clone {
    fn transform(&mut self, f: impl FnOnce(ElementType<V>) -> ElementType<V>);
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Transformer<V> for Node<V> {
    fn transform(&mut self, f: impl FnOnce(ElementType<V>) -> ElementType<V>) {
        // self.data = f(self.data);
        take_mut::take(self, |mut s| {
            s.data = f(s.data);
            s
        })
    }
}