use geometry::*;
use super::color::Color;
use primitive::*;

use gfx;
use lyon::tessellation;
use transform_data::TransformData;

pub type ColorFormat = gfx::format::Rgba8;

pub struct Buffers<V: TransformPrimitive + ColorPrimitive + Clone> {
    pub vbo: Vec<V>,
    pub ibo: Vec<u32>,
    pub tbo: Vec<Matrix>,
    pub cbo: Vec<Color>,
}

pub struct VertexData<V: TransformPrimitive + ColorPrimitive + Clone> {
    pub vbo: Vec<V>,
    pub ibo: Vec<u32>,
    pub transform_data: TransformData,
    pub color: Color,
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> VertexData<V> {
    pub fn new() -> VertexData<V> {
        VertexData {
            vbo: vec![],
            ibo: vec![],
            transform_data: TransformData::new(),
            color: Color::black(),
        }
    }
    pub fn apply_to(&self, buffers: &mut Buffers<V>) {
        let len = buffers.vbo.len() as u32;
        let len_transform = buffers.tbo.len() as u32;
        buffers.vbo.extend(self.vbo.clone().drain(..).map(|mut v| {
            v.set_local_transform_index(len_transform);
            v.set_group_transform_index(len_transform + 1);
            v.set_color_index(buffers.cbo.len() as u32);
            v
        }).collect::<Vec<_>>());
        buffers.ibo.extend(&self.ibo.iter().map(|x| x + len).collect::<Vec<_>>());
        buffers.tbo.push(self.transform_data.local_transform);
        buffers.tbo.push(self.transform_data.group_transform);
        buffers.cbo.push(self.color);
    }
}