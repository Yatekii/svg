use color::Color;
use element::ElementUpdate;
use geometry::{ Point, Matrix };
use primitive::*;
use super::Element;

use transform_data::TransformData;
use vertex_data::VertexData;

pub struct Circle<V: TransformPrimitive + ColorPrimitive + Clone> {
    // Top left
    pub origin: Point,
    // Radius
    pub radius: f32,
    // TransformData
    pub transform_data: TransformData,
    // VertexData
    pub vertex_data: VertexData<V>,
    
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> Circle<V> {
    pub fn new() -> Circle<V> {
        // TODO: tesselate circle!

        Circle {
            origin: Point::origin(),
            radius: 1.0,
            transform_data: TransformData::new(),
            vertex_data: VertexData::<V>::new()
        }
    }
}

// impl Element for Circle {
//     fn get_vbo(&self) -> &Vec<i32> {
//         &vec![]
//     }

//     fn get_local_tranform(&self) -> &Transform {
//         &self.transform_data.local_transform
//     }

//     fn get_group_tranform(&self) -> &Transform {
//         &self.transform_data.group_transform
//     }
// }

impl<V: TransformPrimitive + ColorPrimitive + Clone> ElementUpdate for Circle<V> {
    fn set_group_transform(&mut self, transform: &Matrix) {
        self.transform_data.group_transform = transform.clone();
    }

    fn set_local_transform(&mut self, transform: &Matrix) {
        self.transform_data.local_transform = transform.clone();
    }

    fn set_color(&mut self, color: &Color) {
        self.vertex_data.color = color.clone();
    }
}