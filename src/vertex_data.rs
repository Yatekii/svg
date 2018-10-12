use geometry::*;
use super::color::Color;
use primitive::*;

use gfx;
use lyon::tessellation::geometry_builder::{ VertexBuffers };
use transform_data::TransformData;

pub type ColorFormat = gfx::format::Rgba8;

pub struct Buffers<V: TransformPrimitive + ColorPrimitive + Clone, M, C>
where M: From<Matrix>, C: From<Color> {
    pub vbo: Vec<V>,
    pub ibo: Vec<u32>,
    pub tbo: Vec<M>,
    pub cbo: Vec<C>,
}

impl<V: TransformPrimitive + ColorPrimitive + Clone, M, C> Buffers<V, M, C>
where M: From<Matrix>, C: From<Color> {
    pub fn new() -> Buffers<V, M, C> {
        Buffers {
            vbo: vec![],
            ibo: vec![],
            tbo: vec![],
            cbo: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct VertexData<V: TransformPrimitive + ColorPrimitive + Clone> {
    vbo: Vec<V>,
    ibo: Vec<u32>,
    dirty: bool,
    pub transform_data: TransformData,
    pub fill: Color,
    pub stroke: Color,
}

impl<V: TransformPrimitive + ColorPrimitive + Clone> VertexData<V> {
    pub fn new() -> VertexData<V> {
        VertexData {
            vbo: vec![],
            ibo: vec![],
            dirty: false,
            transform_data: TransformData::new(),
            fill: Color::black(),
            stroke: Color::none(),
        }
    }

    pub fn from_vertex_buffers(vertex_buffers: VertexBuffers<V, u32>) -> VertexData<V> {
        VertexData {
            vbo: vertex_buffers.vertices,
            ibo: vertex_buffers.indices,
            dirty: false,
            transform_data: TransformData::new(),
            fill: Color::black(),
            stroke: Color::none(),
        }
    }

    pub fn set_vertex_data(&mut self, vbo: Vec<V>, ibo: Vec<u32>) {
        self.vbo = vbo;
        self.ibo = ibo;
        self.dirty = true;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn make_dirty(&mut self) { self.dirty = true; }

    pub fn apply_to<M, C>(&self, buffers: &mut Buffers<V, M, C>)
    where M: From<Matrix>, C: From<Color> {
        let len = buffers.vbo.len() as u32;
        let len_transform = buffers.tbo.len() as u32;
        buffers.vbo.extend(self.vbo.clone().drain(..).map(|mut v| {
            v.set_local_transform_index(len_transform);
            v.set_group_transform_index(len_transform + 1);
            v.set_color_index(buffers.cbo.len() as u32);
            v
        }).collect::<Vec<_>>());
        buffers.ibo.extend(&self.ibo.iter().map(|x| x + len).collect::<Vec<_>>());
        buffers.tbo.push(self.transform_data.local_transform.into());
        buffers.tbo.push(self.transform_data.group_transform.into());
        buffers.cbo.push(self.fill.into());
        // TODO: stroke
    }
}