use lyon::lyon_tessellation::StrokeVertex;
use lyon::lyon_tessellation::VertexBuffers;
use lyon::lyon_tessellation::FillVertex;
use lyon::lyon_tessellation::VertexConstructor;
use crate::element::ElementUpdate;
use crate::transform_data::TransformData;
use crate::vertex_data::VertexData;
use crate::geometry::{ Point };
use crate::primitive::*;

#[derive(Debug)]
pub struct Path<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub origin: Point,
    // Radius
    pub radius: f32,
    // TransformData
    pub transform_data: TransformData,
    // VertexData
    pub vertex_data: VertexData<V>,
}

impl<V> ElementUpdate<V> for Path<V>
where
    V: TransformPrimitive + ColorPrimitive + Clone,
{
    fn is_dirty(&self) -> bool { self.vertex_data.is_dirty() }

    fn make_dirty(&mut self) { self.vertex_data.make_dirty() }

    fn tesselate<Ctor>(&mut self, _ctor: Ctor)
    where
        Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy
    {
        // let mut mesh: VertexBuffers<V, u32> = VertexBuffers::new();

        // let w = StrokeOptions::default().with_line_width(self.vertex_data.stroke_width);

        // let fill = true;

        // let r = BorderRadii::new_all_same(5.0);
        // // Euclid rectangles have the origin at the top left which means
        // //      X = leftmost point in normal notation
        // //      Y = bottommost point in normal notation as Y is inverted
        // //          (Y positive points downwards on the screen)
        // let euclid_rectangle = lmath::Rect::new(
        //     lmath::Point::new(self.origin.x, self.origin.y),
        //     lmath::Size::new(self.dimensions.x, self.dimensions.y)
        // );

        // if fill {
        //     let _ = fill_rounded_rectangle(
        //         &euclid_rectangle,
        //         &r,
        //         &FillOptions::tolerance(0.000001),
        //         &mut BuffersBuilder::new(&mut mesh, ctor)
        //     );
        // } else {
        //     let _ = stroke_rounded_rectangle(
        //         &euclid_rectangle,
        //         &r,
        //         &w,
        //         &mut BuffersBuilder::new(&mut mesh, ctor)
        //     );
        // }

        // self.vertex_data.set_vertex_data(mesh.vertices, mesh.indices);
    }

    fn get_vertex_data(&self) -> &VertexData<V> {
        &self.vertex_data
    }

    fn get_vertex_data_mut(&mut self) -> &mut VertexData<V> {
        &mut self.vertex_data
    }
}

impl_basic_styles_for_struct!(Path);