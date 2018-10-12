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
use lyon::tessellation::{ StrokeVertex, FillVertex, VertexConstructor };

use geometry::Matrix;
use color::Color;
use primitive::*;

#[derive(Debug)]
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

pub trait ElementUpdate<V, Ctor>
where
    V: TransformPrimitive + ColorPrimitive + Clone,
    Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy
{
    fn is_dirty(&self) -> bool;
    fn make_dirty(&mut self);
    fn tesselate(&mut self, ctor: Ctor);
    fn set_group_transform(&mut self, transform: &Matrix);
    fn set_local_transform(&mut self, transform: &Matrix);
}

pub trait BasicStylableElement {
    fn fill(self, fill: Color) -> Self;
    fn stroke(self, stroke: Color) -> Self;
    fn stroke_width(self, width: f32) -> Self;
}