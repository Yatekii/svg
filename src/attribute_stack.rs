use crate::geometry::*;
use crate::color::*;

#[derive(Clone)]
pub struct AttributeStack {
    pub transform: Matrix,
    pub color: Color,
}

impl AttributeStack {
    pub fn new() -> AttributeStack {
        AttributeStack {
            transform: Matrix::identity(),
            color: Color::black(),
        }
    }
}