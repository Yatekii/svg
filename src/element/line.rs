use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{ StrokeVertex, FillVertex, StrokeOptions, FillOptions, VertexConstructor };
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math as lmath;

use color::Color;
use geometry::{ Point, Vector, Matrix };
use primitive::*;
use super::{ ElementType, Element, ElementUpdate, BasicStylableElement };

use vertex_data::VertexData;

#[derive(Debug)]
pub struct Line<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Start of the line
    pub start: Point,
    // End of the line
    pub end: Point,
    // VertexData
    pub vertex_data: VertexData<V>,
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Line<V> {
    pub fn new() -> Line<V> {
        Line {
            start: Point::origin(),
            end: Point::origin(),
            vertex_data: VertexData::<V>::new(),
        }
    }

    pub fn start(mut self, start: Point) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: Point) -> Self {
        self.end = end;
        self
    }

    pub fn x1(mut self, x2: f32) -> Self {
        self.start.x = x2;
        self
    }

    pub fn x2(mut self, x2: f32) -> Self {
        self.end.x = x2;
        self
    }

    pub fn y1(mut self, y2: f32) -> Self {
        self.start.y = y2;
        self
    }

    pub fn y2(mut self, y2: f32) -> Self {
        self.end.y = y2;
        self
    }

    pub fn wrap(self) -> ElementType<V> {
        ElementType::Line(self)
    }

    fn make_dirty(&mut self) { self.vertex_data.make_dirty() }
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Element<V> for Line<V> {
    fn get_vertex_data(&self) -> &VertexData<V> {
        &self.vertex_data
    }

    fn get_local_tranform(&self) -> &Matrix {
        &self.vertex_data.transform_data.local_transform
    }

    fn get_group_tranform(&self) -> &Matrix {
        &self.vertex_data.transform_data.group_transform
    }
}

impl<V, Ctor> ElementUpdate<V, Ctor> for Line<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone,
    Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy
{
    fn is_dirty(&self) -> bool { self.vertex_data.is_dirty() }

    fn make_dirty(&mut self) { self.vertex_data.make_dirty() }

    fn tesselate(&mut self, ctor: Ctor) {
        let mut mesh: VertexBuffers<V, u32> = VertexBuffers::new();

        let w = StrokeOptions::default().with_line_width(self.vertex_data.stroke_width);

        let is_closed = false;

        let mut points = Vec::new();

        points.push(lmath::Point::new(self.start.x, self.start.y));
        points.push(lmath::Point::new(self.end.x, self.end.y));

        let _ = stroke_polyline(
            points.into_iter(),
            is_closed,
            &w,
            &mut BuffersBuilder::new(&mut mesh, ctor)
        );

        self.vertex_data.set_vertex_data(mesh.vertices, mesh.indices);
    }

    fn set_group_transform(&mut self, transform: &Matrix) {
        self.vertex_data.transform_data.group_transform = transform.clone();
    }

    fn set_local_transform(&mut self, transform: &Matrix) {
        self.vertex_data.transform_data.local_transform = transform.clone();
    }
}

impl<V> BasicStylableElement for Line<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone
{
    fn fill(mut self, fill: Color) -> Self {
        self.vertex_data.fill = fill;
        self
    }

    fn stroke(mut self, stroke: Color) -> Self {
        self.make_dirty();
        self.vertex_data.stroke = stroke;
        self
    }

    fn stroke_width(mut self, width: f32) -> Self {
        self.make_dirty();
        self.vertex_data.stroke_width = width;
        self
    }
}