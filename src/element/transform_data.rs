use geometry::*;

pub struct TransformData {
    pub local_transform: Transform,
    pub group_transform: Transform,
}

impl TransformData {
    pub fn new() -> TransformData {
        TransformData {
            local_transform: Transform::identity(),
            group_transform: Transform::identity(),
        }
    }
}