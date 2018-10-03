use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{ StrokeVertex, FillVertex, StrokeOptions, FillOptions, VertexConstructor };
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math as lmath;

use color::Color;
use geometry::{ Point, Vector, Matrix };
use primitive::*;
use super::{ ElementType, Element, ElementUpdate };

use vertex_data::VertexData;

#[derive(Clone)]
pub struct Rect<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub origin: Point,
    // Width, Height
    pub dimensions: Vector,
    // Is the circle filled or just a stroke
    pub fill: bool,
    // Color (fill or stroke)
    pub color: Color,
    // VertexData
    pub vertex_data: VertexData<V>,
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Rect<V> {
    pub fn new() -> Rect<V> {
        Rect {
            origin: Point::origin(),
            dimensions: Vector::identity(),
            fill: true,
            color: Color::black(),
            vertex_data: VertexData::<V>::new(),
        }
    }
}

pub struct RectBuilder<V, Ctor>
where
V: TransformPrimitive + ColorPrimitive + Clone,
Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> {
    ctor: Ctor,
    rect: Rect<V>
}

impl<V, Ctor> RectBuilder<V, Ctor>
where
V: TransformPrimitive + ColorPrimitive + Clone,
Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy {
    pub fn new(ctor: Ctor) -> Self {
        RectBuilder {
            ctor: ctor,
            rect: Rect::new()
        }
    }

    pub fn origin(mut self, origin: Point) -> Self {
        self.rect.origin = origin;
        self
    }

    pub fn dimensions(mut self, dimensions: Vector) -> Self {
        self.rect.dimensions = dimensions;
        self
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.rect.fill = fill;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.rect.color = color;
        self
    }

    pub fn finalize(mut self) -> ElementType<V> {
        let mut mesh: VertexBuffers<V, u32> = VertexBuffers::new();

        let w = StrokeOptions::default().with_line_width(6.5);

        let fill = true;

        let r = BorderRadii::new_all_same(5.0);
        // Euclid rectangles have the origin at the top left which means
        //      X = leftmost point in normal notation
        //      Y = bottommost point in normal notation as Y is inverted
        //          (Y positive points downwards on the screen)
        let euclid_rectangle = lmath::Rect::new(
            lmath::Point::new(self.rect.origin.x, self.rect.origin.y),
            lmath::Size::new(self.rect.dimensions.x, self.rect.dimensions.y)
        );

        if fill {
            let _ = fill_rounded_rectangle(
                &euclid_rectangle,
                &r,
                &FillOptions::default(),
                &mut BuffersBuilder::new(&mut mesh, self.ctor)
            );
        } else {
            let _ = stroke_rounded_rectangle(
                &euclid_rectangle,
                &r,
                &w,
                &mut BuffersBuilder::new(&mut mesh, self.ctor)
            );
        }

        self.rect.vertex_data = VertexData::from_vertex_buffers(mesh);
        ElementType::Rect(self.rect)
    }
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Element<V> for Rect<V> {
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

impl<V: TransformPrimitive + ColorPrimitive + Clone> ElementUpdate for Rect<V> {
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