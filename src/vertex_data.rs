use geometry::Matrix;
use super::color::Color;

use gfx;
use lyon::tessellation;
use transform_data::TransformData;

pub type ColorFormat = gfx::format::Rgba8;

gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "a_position",
        primitive_id: u32 = "a_prim_id",
    }

    // a 2x3 matrix (last two members of data1 unused).
    constant Transform {
        data: [[f32; 3]; 3] = "data",
    }

    constant Primitive {
        local_transform: u32 = "local_transform",
        group_transform: u32 = "group_transform",
        color: u32 = "color",
    }

    constant Globals {
        zoom: [f32; 2] = "u_zoom",
        pan: [f32; 2] = "u_pan",
        aspect_ratio: f32 = "u_aspect_ratio",
    }

    pipeline fill_pipeline {
        vbo: gfx::VertexBuffer<Vertex> = (),
        out_color: gfx::RenderTarget<ColorFormat> = "out_color",
        globals: gfx::ConstantBuffer<Globals> = "Globals",
        primitives: gfx::ConstantBuffer<Primitive> = "u_primitives",
        transforms: gfx::ConstantBuffer<Transform> = "u_transforms",
    }
}

// This struct carries the data for each vertex
pub struct VertexCtor {
    pub primitive_id: u32,
}

// Handle conversions to the gfx vertex format
impl tessellation::VertexConstructor<tessellation::FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertex {
        assert!(!vertex.position.x.is_nan());
        assert!(!vertex.position.y.is_nan());

        Vertex {
            position: vertex.position.to_array(),
            primitive_id: self.primitive_id,
        }
    }
}

pub struct Buffers {
    pub vbo: Vec<Vertex>,
    pub ibo: Vec<u32>,
    pub pbo: Vec<Primitive>,
    pub tbo: Vec<Matrix>,
    pub cbo: Vec<Color>,
}

pub struct VertexData {
    pub vbo: Vec<Vertex>,
    pub ibo: Vec<u32>,
    pub transform_data: TransformData,
    pub color: Color,
}

impl VertexData {
    pub fn new() -> VertexData {
        VertexData {
            vbo: vec![],
            ibo: vec![],
            transform_data: TransformData::new(),
            color: Color::black(),
        }
    }
    pub fn apply_to(&self, buffers: &mut Buffers) {
        let len = buffers.vbo.len() as u32;
        let len_transform = buffers.tbo.len() as u32;
        buffers.vbo.extend(&self.vbo);
        buffers.ibo.extend(&self.ibo.iter().map(|x| x + len).collect::<Vec<_>>());
        buffers.pbo.push(Primitive {
            local_transform: len_transform,
            group_transform: len_transform + 1,
            color: buffers.cbo.len() as u32,
        });
        buffers.tbo.push(self.transform_data.local_transform);
        buffers.tbo.push(self.transform_data.group_transform);
        buffers.cbo.push(self.color);
    }
}