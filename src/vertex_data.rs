use geometry::*;
use super::color::Color;

use gfx;
use lyon::tessellation;
use transform_data::TransformData;

pub type ColorFormat = gfx::format::Rgba8;

pub struct Buffers<V> {
    pub vbo: Vec<V>,
    pub ibo: Vec<u32>,
    pub pbo: Vec<PrimitiveIndex>,
    pub tbo: Vec<Matrix>,
    pub cbo: Vec<Color>,
}

pub struct VertexData<V> {
    pub vbo: Vec<V>,
    pub ibo: Vec<u32>,
    pub transform_data: TransformData,
    pub color: Color,
}

pub struct PrimitiveIndex {
    pub local_transform_id: u32,
    pub group_transform_id: u32,
    pub color: u32
}

impl<V> VertexData<V> {
    pub fn new() -> VertexData<V> {
        VertexData {
            vbo: vec![],
            ibo: vec![],
            transform_data: TransformData::new(),
            color: Color::black(),
        }
    }
    pub fn apply_to<V>(&self, buffers: &mut Buffers<V>) {
        let len = buffers.vbo.len() as u32;
        let len_transform = buffers.tbo.len() as u32;
        buffers.vbo.extend(self.vbo.iter().cloned());
        buffers.ibo.extend(&self.ibo.iter().map(|x| x + len).collect::<Vec<_>>());
        buffers.pbo.push(PrimitiveIndex {
            local_transform_id: len_transform,
            group_transform_id: len_transform + 1,
            color: buffers.cbo.len() as u32,
        });
        buffers.tbo.push(self.transform_data.local_transform);
        buffers.tbo.push(self.transform_data.group_transform);
        buffers.cbo.push(self.color);
    }
}