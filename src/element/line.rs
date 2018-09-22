use transform_data::TransformData;
use vertex_data::VertexData;
use geometry::{ Point };

pub struct Line {
    // Top left
    origin: Point,
    // Radius
    radius: f32,
    // TransformData
    pub transform_data: TransformData,
    // VertexData
    pub vertex_data: VertexData,
}