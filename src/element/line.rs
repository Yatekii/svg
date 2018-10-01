use transform_data::TransformData;
use vertex_data::VertexData;
use geometry::{ Point };
use primitive::*;

pub struct Line<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub origin: Point,
    // Radius
    pub radius: f32,
    // TransformData
    pub transform_data: TransformData,
    // VertexData
    pub vertex_data: VertexData<V>,
}