use transform_data::TransformData;
use vertex_data::VertexData;
use geometry::{ Point };

pub struct Path {
    // Top left
    pub origin: Point,
    // Radius
    pub radius: f32,
    // TransformData
    pub transform_data: TransformData,
    // VertexData
    pub vertex_data: VertexData,
}