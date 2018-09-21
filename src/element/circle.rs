use geometry::{ Point, Transform };
use super::Element;

use super::transform_data::TransformData;
use super::vertex_data::VertexData;

pub struct Circle {
    // Top left
    pub origin: Point,
    // Radius
    pub radius: f32,
    // TransformData
    pub transform_data: TransformData,
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

impl Element for Circle {
    fn get_vbo() -> Vec<i32> {
        vec![]
    }

    fn get_tranform() -> Transform {
        Transform::identity()
    }
}