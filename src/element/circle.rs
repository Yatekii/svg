use geometry::{ Point, Transform };
use super::Element;

use transform_data::TransformData;
use vertex_data::VertexData;

pub struct Circle {
    // Top left
    pub origin: Point,
    // Radius
    pub radius: f32,
    // TransformData
    pub transform_data: TransformData,
    // VertexData
    pub vertex_data: VertexData,
    
}

impl Circle {
    pub fn new() -> Circle {
        // TODO: tesselate circle!

        Circle {
            origin: Point::origin(),
            radius: 1.0,
            transform_data: TransformData::new(),
            vertex_data: VertexData::new()
        }
    }
}