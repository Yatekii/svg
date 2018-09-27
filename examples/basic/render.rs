use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

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

pub static MAX_PRIMITIVES: usize = 512;
pub static MAX_TRANSFORMS: usize = 512;

pub static VERTEX_SHADER: &'static str = "
    #version 150
    #line 118

    uniform Globals {
        vec2 u_zoom;
        vec2 u_pan;
        float u_aspect_ratio;
    };

    uniform VehicleAttributes {
        vec4 transform_data0;
        vec4 transform_data1;
        vec2 u_position;
    };

    struct Primitive {
        uint transform;
        uint color;
    };

    struct Transform {
        vec4 data0;
        vec4 data1;
    };

    uniform u_primitives { Primitive primitives[512]; };
    uniform u_transforms { Transform transforms[512]; };

    in vec2 a_position;
    in uint a_prim_id;

    out vec4 v_color;

    void main() {
        Primitive prim = primitives[a_prim_id];

        Transform t = transforms[prim.transform];
        mat3 transform = mat3(
            t.data0.x, t.data0.y, 0.0,
            t.data0.z, t.data0.w, 0.0,
            t.data1.x, t.data1.y, 1.0
        );

        mat3 object_transform = mat3(
            transform_data0.x, transform_data0.y, 0.0,
            transform_data0.z, transform_data0.w, 0.0,
            transform_data1.x, transform_data1.y, 1.0
        );

        vec2 pos = (transform * object_transform * vec3(a_position, 1.0)).xy + u_position;
        gl_Position = vec4((pos.xy + u_pan) * u_zoom, 0.0, 1.0);
        gl_Position.y *= -1.0;
        gl_Position.x /= u_aspect_ratio;

        uint mask = 0x000000FFu;
        uint color = prim.color;
        v_color = vec4(
            float((color >> 24) & mask),
            float((color >> 16) & mask),
            float((color >>  8) & mask),
            float(color & mask)
        ) / 255.0;
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