use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{ StrokeVertex, FillVertex, StrokeOptions, FillOptions, VertexConstructor };
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math as lmath;

use geometry::{ Point };
use primitive::*;
use super::{ ElementType, ElementUpdate };

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

impl<V> ElementUpdate<V> for Circle<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone
{
    fn is_dirty(&self) -> bool { self.vertex_data.is_dirty() }

    fn make_dirty(&mut self) { self.vertex_data.make_dirty() }

    fn tesselate<Ctor>(&mut self, ctor: Ctor)
    where
        Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy
    {
        let mut mesh: VertexBuffers<V, u32> = VertexBuffers::new();

        let w = StrokeOptions::default().with_line_width(self.vertex_data.stroke_width);

        let fill = true;

        if fill {
            let _ = fill_circle(
                lmath::Point::new(self.center.x, self.center.y),
                self.radius,
                &FillOptions::tolerance(0.01),
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

    fn get_vertex_data(&self) -> &VertexData<V> {
        &self.vertex_data
    }

    fn get_vertex_data_mut(&mut self) -> &mut VertexData<V> {
        &mut self.vertex_data
    }
}

impl_basic_styles_for_struct!(Circle);