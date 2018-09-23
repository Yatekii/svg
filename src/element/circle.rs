use color::Color;
use element::ElementUpdate;
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

impl ElementUpdate for Circle {
    fn set_group_transform(&mut self, transform: &Transform) {
        self.transform_data.group_transform = transform.clone();
    }

    fn set_local_transform(&mut self, transform: &Transform) {
        self.transform_data.local_transform = transform.clone();
    }

    fn set_color(&mut self, color: &Color) {
        self.vertex_data.color = color.clone();
    }
}