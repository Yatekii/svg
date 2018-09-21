use super::color::Color;

pub struct VertexData {
    pub color: Color,
}

impl VertexData {
    pub fn new() -> VertexData {
        VertexData {
            color: Color::black(),
        }
    }
}