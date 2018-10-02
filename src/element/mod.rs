pub mod circle;
pub mod line;
pub mod path;
pub mod rect;
pub mod group;

use vertex_data::VertexData;
pub use self::circle::Circle;
pub use self::line::Line;
pub use self::path::Path;
pub use self::rect::Rect;
pub use self::group::Group;

use self::circle::CircleBuilder;
use self::rect::RectBuilder;
use self::group::GroupBuilder;

use lyon::tessellation::{ StrokeVertex, FillVertex, VertexConstructor };

use geometry::Matrix;
use color::Color;
use primitive::*;
use common::*;

pub enum ElementType<V: TransformPrimitive + ColorPrimitive + Clone> {
    Circle(circle::Circle<V>),
    Line(line::Line<V>),
    Path(path::Path<V>),
    Rect(rect::Rect<V>),
    Group(group::Group),
    None,
}

pub trait Element<V: TransformPrimitive + ColorPrimitive + Clone> {
    fn get_vertex_data(&self) -> &VertexData<V>;
    fn get_local_tranform(&self) -> &Matrix;
    fn get_group_tranform(&self) -> &Matrix;
    
}

pub trait ElementUpdate {
    fn set_group_transform(&mut self, transform: &Matrix);
    fn set_local_transform(&mut self, transform: &Matrix);
    fn set_color(&mut self, color: &Color);
}

pub struct ElementBuilder<'a, V, Ctor>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    arena: &'a mut Arena<V>,
    ctor: Ctor
}

impl<'a, V, Ctor> ElementBuilder<'a, V, Ctor>
where
    V: TransformPrimitive + ColorPrimitive + Clone {
    pub fn new(arena: &'a mut Arena<V>, ctor: Ctor) -> Self {
        ElementBuilder {
            arena: arena,
            ctor: ctor,
        }
    }

    pub fn circle(&self) -> CircleBuilder<V, Ctor>
    where Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy {
        CircleBuilder::new(self.ctor)
    }

    // pub fn line(&self) -> Line {
    //     self
    // }

    // pub fn path(&self) -> Path {
    //     self
    // }

    pub fn rect(&self) -> RectBuilder<V, Ctor>
    where
        Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy {
        RectBuilder::new(self.ctor)
    }
    
    pub fn group(&mut self) -> GroupBuilder<V, Ctor>
    where
        Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy {
        GroupBuilder::new(self.arena, self.ctor)
    }
}