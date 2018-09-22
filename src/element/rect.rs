use transform_data::TransformData;
use vertex_data::VertexData;
use geometry::{ Point, Vector };

pub struct Rect {
    // Top left
    origin: Point,
    // Width, Height
    dimensions: Vector,
    // TransformData
    pub transform_data: TransformData,
    // VertexData
    pub vertex_data: VertexData,
}