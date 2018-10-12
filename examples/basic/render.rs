use svg::geometry::Matrix;
use gfx;

use lyon::tessellation;

use svg::primitive::*;
use svg::color;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        position: [f32; 2] = "a_position",
        local_transform_index: u32 = "a_local_transform_index",
        group_transform_index: u32 = "a_group_transform_index",
        color_index: u32 = "a_color_index",
    }

    // a 2x3 matrix (last two members of data1 unused).
    constant Transform {
        data: [[f32; 4]; 4] = "data",
    }

    constant Color {
        data: [f32; 4] = "data",
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
        colors: gfx::ConstantBuffer<Color> = "u_colors",
        transforms: gfx::ConstantBuffer<Transform> = "u_transforms",
    }
}

// This struct carries the data for each vertex
#[derive(Clone, Copy)]
pub struct VertexCtor;

// Handle conversions to the gfx vertex format
impl tessellation::VertexConstructor<tessellation::FillVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> Vertex {
        assert!(!vertex.position.x.is_nan());
        assert!(!vertex.position.y.is_nan());

        Vertex {
            position: vertex.position.to_array(),
            local_transform_index: 0,
            group_transform_index: 0,
            color_index: 0,
        }
    }
}

impl tessellation::VertexConstructor<tessellation::StrokeVertex, Vertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> Vertex {
        assert!(!vertex.position.x.is_nan());
        assert!(!vertex.position.y.is_nan());

        Vertex {
            position: vertex.position.to_array(),
            local_transform_index: 0,
            group_transform_index: 0,
            color_index: 0,
        }
    }
}

impl TransformPrimitive for Vertex {
    fn set_local_transform_index(&mut self, index: u32) {
        self.local_transform_index = index;
    }

    fn set_group_transform_index(&mut self, index: u32) {
        self.group_transform_index = index;
    }
}

impl ColorPrimitive for Vertex{
    fn set_color_index(&mut self, index: u32) {
        self.color_index = index;
    }
}

// Default scene has all values set to zero
#[derive(Copy, Clone, Debug, Default)]
pub struct Scene {
    pub zoom: f32,
    pub pan: [f32; 2],
    pub aspect_ratio: f32,
    pub wireframe: bool,
}

impl Scene {
    pub fn new(zoom: f32, pan: [f32; 2], aspect_ratio: f32) -> Self {
        Self {
            zoom,
            pan,
            aspect_ratio,
            wireframe: false,
        }
    }
}

impl From<color::Color> for Color {
    fn from(color: color::Color) -> Self {
        Color {
            data: color.color
        }
    }
}

impl From<Matrix> for Transform {
    fn from(matrix: Matrix) -> Self {
        Transform {
            data: matrix.into()
        }
    }
}

// Extract the relevant globals from the scene struct
impl From<Scene> for Globals {
    fn from(scene: Scene) -> Self {
        Globals {
            zoom: [scene.zoom, scene.zoom],
            pan: scene.pan,
            aspect_ratio: scene.aspect_ratio
        }
    }
}

pub static MAX_COLORS: usize = 512;
pub static MAX_TRANSFORMS: usize = 512;

pub static VERTEX_SHADER: &'static str = "
    #version 150
    #line 118

    uniform Globals {
        vec2 u_zoom;
        vec2 u_pan;
        float u_aspect_ratio;
    };

    struct Color {
        vec4 data;
    };

    struct Transform {
        mat4 data;
    };

    uniform u_colors { Color colors[512]; };
    uniform u_transforms { Transform transforms[512]; };

    in vec2 a_position;
    in uint a_local_transform_index;
    in uint a_color_index;

    out vec4 v_color;

    void main() {
        mat4 transform = transforms[a_local_transform_index].data;
        vec4 color = colors[a_color_index].data;

        vec2 pos = (transform * vec4(a_position, 1.0, 1.0)).xy;
        gl_Position = vec4((pos.xy + u_pan) * u_zoom, 0.0, 1.0);
        gl_Position.y *= -1.0;
        gl_Position.x /= u_aspect_ratio;

        v_color = color;
    }
";

// The fragment shader is dead simple. It just applies the color computed in the vertex shader.
// A more advanced renderer would probably compute texture coordinates in the vertex shader and
// sample the color from a texture here.
pub static FRAGMENT_SHADER: &'static str = "
    #version 150
    in vec4 v_color;
    out vec4 out_color;

    void main() {
        out_color = v_color;
    }
";