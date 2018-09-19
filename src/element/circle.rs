use geometry::{ Point, Transform };
use super::Element;

pub struct Circle {
    // Top left
    pub origin: Point,
    // Radius
    pub radius: f32,
}

impl Element for Circle {
    fn is_vbo_dirty() -> bool {
        true
    }
    fn is_transform_dirty() -> bool {
        true
    }
    fn get_vbo() -> Vec<i32> {
        vec![]
    }
    fn get_tranform() -> Transform {
        Transform::identity()
    }
}