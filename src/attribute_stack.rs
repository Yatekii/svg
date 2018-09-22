use geometry::*;
use color::*;

#[derive(Clone)]
pub struct AttributeStack {
    pub transform: Transform,
    pub color: Color,
}

impl AttributeStack {
    pub fn new() -> AttributeStack {
        AttributeStack {
            transform: Transform::identity(),
            color: Color::black(),
        }
    }
}