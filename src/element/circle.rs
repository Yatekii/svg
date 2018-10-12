use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{ StrokeVertex, FillVertex, StrokeOptions, FillOptions, VertexConstructor };
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math as lmath;


use color::Color;
use geometry::{ Point, Matrix };
use primitive::*;
use super::{ ElementType, Element, ElementUpdate, BasicStylableElement };

use vertex_data::VertexData;

#[derive(Debug)]
pub struct Circle<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub center: Point,
    // Radius
    pub radius: f32,
    // VertexData
    pub vertex_data: VertexData<V>,
    
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Circle<V> {
    pub fn new() -> Circle<V> {
        Circle {
            center: Point::origin(),
            radius: 1.0,
            vertex_data: VertexData::<V>::new(),
        }
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn center(mut self, center: Point) -> Self {
        self.center = center;
        self
    }

    pub fn r(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn cx(mut self, center_x: f32) -> Self {
        self.center.x = center_x;
        self
    }

    pub fn cy(mut self, center_y: f32) -> Self {
        self.center.y = center_y;
        self
    }

    pub fn wrap(self) -> ElementType<V> {
        ElementType::Circle(self)
    }

    fn make_dirty(&mut self) { self.vertex_data.make_dirty() }
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Element<V> for Circle<V> {
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

impl<V, Ctor> ElementUpdate<V, Ctor> for Circle<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone,
    Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy
{
    fn is_dirty(&self) -> bool { self.vertex_data.is_dirty() }

    fn make_dirty(&mut self) { self.vertex_data.make_dirty() }

    fn tesselate(&mut self, ctor: Ctor) {
        let mut mesh: VertexBuffers<V, u32> = VertexBuffers::new();

        let w = StrokeOptions::default().with_line_width(6.5);

        let fill = true;

        if fill {
            let _ = fill_circle(
                lmath::Point::new(self.center.x, self.center.y),
                self.radius,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut mesh, ctor)
            );
        } else {
            let _ = stroke_circle(
                lmath::Point::new(self.center.x, self.center.y),
                self.radius,
                &w,
                &mut BuffersBuilder::new(&mut mesh, ctor)
            );
        }

        self.vertex_data.set_vertex_data(mesh.vertices, mesh.indices);
    }

    fn set_group_transform(&mut self, transform: &Matrix) {
        self.vertex_data.transform_data.group_transform = transform.clone();
    }

    fn set_local_transform(&mut self, transform: &Matrix) {
        self.vertex_data.transform_data.local_transform = transform.clone();
    }
}

impl<V> BasicStylableElement for Circle<V>
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
}