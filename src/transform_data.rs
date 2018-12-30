use crate::geometry::*;

#[derive(Clone, Debug)]
pub struct TransformData {
    pub local_transform: Matrix,
    pub group_transform: Matrix,
    pub real_transform: Matrix,
}

impl TransformData {
    pub fn new() -> TransformData {
        TransformData {
            local_transform: Matrix::identity(),
            group_transform: Matrix::identity(),
            real_transform: Matrix::identity(),
        }
    }
}