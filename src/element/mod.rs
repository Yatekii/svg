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
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> From<circle::Circle<V>> for ElementType<V> {
    fn from(circle: circle::Circle<V>) -> Self {
        ElementType::Circle(circle)
    }
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> From<line::Line<V>> for ElementType<V> {
    fn from(line: line::Line<V>) -> Self {
        ElementType::Line(line)
    }
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> From<path::Path<V>> for ElementType<V> {
    fn from(path: path::Path<V>) -> Self {
        ElementType::Path(path)
    }
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> From<rect::Rect<V>> for ElementType<V> {
    fn from(rect: rect::Rect<V>) -> Self {
        ElementType::Rect(rect)
    }
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> From<group::Group> for ElementType<V> {
    fn from(group: group::Group) -> Self {
        ElementType::Group(group)
    }
}

pub trait Element<V: TransformPrimitive + ColorPrimitive + Clone> {
    fn get_vertex_data(&self) -> &VertexData<V>;
    fn get_local_tranform(&self) -> &Matrix;
    fn get_group_tranform(&self) -> &Matrix;
    
}

pub trait ElementUpdate<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone
{
    fn is_dirty(&self) -> bool;
    fn make_dirty(&mut self);
    fn get_vertex_data(&self) -> &VertexData<V>;
    fn get_vertex_data_mut(&mut self) -> &mut VertexData<V>;
    fn tesselate<Ctor>(&mut self, ctor: Ctor) where Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy;
}

pub trait BasicStylableElement {
    fn fill(self, fill: Color) -> Self;
    fn stroke(self, stroke: Color) -> Self;
    fn stroke_width(self, width: f32) -> Self;
    fn transform(self, transform: Matrix) -> Self;
}