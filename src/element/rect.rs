use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::{ StrokeVertex, FillVertex, StrokeOptions, FillOptions, VertexConstructor };
use lyon::tessellation::geometry_builder::{VertexBuffers, BuffersBuilder};
use lyon::math as lmath;

use crate::geometry::{ Point, Vector };
use crate::primitive::*;
use super::{ ElementType, ElementUpdate };

use crate::vertex_data::VertexData;

#[derive(Clone, Debug)]
pub struct Rect<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub origin: Point,
    // Width, Height
    pub dimensions: Vector,
    // VertexData
    pub vertex_data: VertexData<V>,
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Rect<V> {
    pub fn new() -> Rect<V> {
        Rect {
            origin: Point::origin(),
            dimensions: Vector::identity(),
            vertex_data: VertexData::<V>::new(),
        }
    }

    pub fn origin(mut self, origin: Point) -> Self {
        self.origin = origin;
        self
    }

    pub fn x(mut self, x: f32) -> Self {
        self.origin.x = x;
        self
    }

    pub fn y(mut self, y: f32) -> Self {
        self.origin.y = y;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.dimensions.x = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.dimensions.y = height;
        self
    }

    pub fn dimensions(mut self, dimensions: Vector) -> Self {
        self.dimensions = dimensions;
        self
    }

    pub fn wrap(self) -> ElementType<V> {
        ElementType::Rect(self)
    }

    fn make_dirty(&mut self) { self.vertex_data.make_dirty() }
}

impl<V> ElementUpdate<V> for Rect<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone,
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

        let r = BorderRadii::new_all_same(5.0);
        // Euclid rectangles have the origin at the top left which means
        //      X = leftmost point in normal notation
        //      Y = bottommost point in normal notation as Y is inverted
        //          (Y positive points downwards on the screen)
        let euclid_rectangle = lmath::Rect::new(
            lmath::Point::new(self.origin.x, self.origin.y),
            lmath::Size::new(self.dimensions.x, self.dimensions.y)
        );

        if fill {
            let _ = fill_rounded_rectangle(
                &euclid_rectangle,
                &r,
                &FillOptions::tolerance(0.000001),
                &mut BuffersBuilder::new(&mut mesh, ctor)
            );
        } else {
            let _ = stroke_rounded_rectangle(
                &euclid_rectangle,
                &r,
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

impl_basic_styles_for_struct!(Rect);