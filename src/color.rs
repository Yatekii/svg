#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub color: [f32; 4]
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color { Color { color: [r, g, b, a] } }
    pub fn black() -> Color {  Color { color: [0.0, 0.0, 0.0, 1.0] } }
    pub fn none() -> Color {  Color { color: [0.0, 0.0, 0.0, 0.0] } }
}