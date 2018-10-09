use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{ StrokeVertex, FillVertex, StrokeOptions, FillOptions, VertexConstructor };
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math as lmath;


use color::Color;
use geometry::{ Point, Matrix };
use primitive::*;
use super::{ ElementType, Element, ElementUpdate };

use vertex_data::VertexData;

pub struct Circle<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub center: Point,
    // Radius
    pub radius: f32,
    // Is the circle filled or just a stroke
    pub fill: bool,
    // Color (fill or stroke)
    pub color: Color,
    // VertexData
    pub vertex_data: VertexData<V>,
    
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Circle<V> {
    pub fn new() -> Circle<V> {
        Circle {
            center: Point::origin(),
            radius: 1.0,
            fill: true,
            color: Color::black(),
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

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn wrap(self) -> ElementType<V> {
        ElementType::Circle(self)
    }

    fn tesselate<Ctor>(&mut self, ctor: Ctor)
    where Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy {
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

        self.vertex_data = VertexData::from_vertex_buffers(mesh);
    }
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

impl<V: TransformPrimitive + ColorPrimitive + Clone> ElementUpdate for Circle<V> {
    fn set_group_transform(&mut self, transform: &Matrix) {
        self.vertex_data.transform_data.group_transform = transform.clone();
    }

    fn set_local_transform(&mut self, transform: &Matrix) {
        self.vertex_data.transform_data.local_transform = transform.clone();
    }

    fn set_color(&mut self, color: &Color) {
        self.vertex_data.color = color.clone();
    }
}