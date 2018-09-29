use lyon::lyon_tessellation::GeometryBuilder;
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{ StrokeVertex, FillVertex, StrokeOptions, FillOptions, VertexConstructor };
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math as lmath;


use color::Color;
use element::ElementUpdate;
use geometry::{ Point, Matrix };
use primitive::*;
use super::Element;

use transform_data::TransformData;
use vertex_data::VertexData;

pub struct Circle<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub center: Point,
    // Radius
    pub radius: f32,
    // VertexData
    pub vertex_data: VertexData<V>,
    
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Circle<V> {
    pub fn new<Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V>>(center: Point, radius: f32, ctor: Ctor) -> Circle<V> {
        // TODO: tesselate circle!
        let mut mesh: VertexBuffers<V, u32> = VertexBuffers::new();

        let w = StrokeOptions::default().with_line_width(6.5);

        let fill = true;

        if fill {
            let _ = fill_circle(
                lmath::Point::new(center.x, center.y),
                radius,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut mesh, ctor)
            );
        } else {
            let _ = stroke_circle(
                lmath::Point::new(center.x, center.y),
                radius,
                &w,
                &mut BuffersBuilder::new(&mut mesh, ctor)
            );
        }

        Circle {
            center: Point::origin(),
            radius: 1.0,
            vertex_data: VertexData::<V>::from_vertex_buffers(mesh)
        }
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