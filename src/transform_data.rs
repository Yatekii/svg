use geometry::*;

#[derive(Clone)]
pub struct TransformData {
    pub local_transform: Matrix,
    pub group_transform: Matrix,
}

impl TransformData {
    pub fn new() -> TransformData {
        TransformData {
            local_transform: Matrix::identity(),
            group_transform: Matrix::identity(),
        }
    }
}