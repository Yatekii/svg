#![feature(duration_as_u128)]
#![feature(nll)]
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate svg;
extern crate lyon;

mod render;

use std::time::Instant;

use svg::common::*;
use svg::element::{ Circle, Rect, Line };
use svg::element::group::GroupBuilder;
use svg::geometry::*;
use svg::attribute_stack::*;
use svg::processor::{ process_tree, generate_buffer };
use svg::vertex_data::*;
use svg::color::*;

use gfx::traits::{Device, FactoryExt};
use glutin::GlContext;

// use std::time::Instant;

use render::{
    fill_pipeline, ColorFormat, DepthFormat, Scene
};

const WINDOW_SIZE: f32 = 800.0;
const CANVAS_WIDTH: f32 = 1200.0;
const CANVAS_HEIGHT: f32 = 500.0;

fn main() {
    // let app = App::new("Lyon svg_render example")
    //     .version("0.1")
    //     .arg(Arg::with_name("MSAA")
    //         .long("msaa")
    //         .short("m")
    //         .help("Sets MSAA sample count (integer)")
    //         .value_name("SAMPLES")
    //         .takes_value(true)
    //         .required(false))
    //     .get_matches();

    println!("Use arrow keys to pan, pageup and pagedown to zoom.");

    let scale = CANVAS_WIDTH / CANVAS_HEIGHT;

    // set window scale
    let (width, height) = if scale < 1.0 {
        (WINDOW_SIZE, WINDOW_SIZE * scale)
    } else {
        (WINDOW_SIZE, WINDOW_SIZE / scale)
    };

    // init the scene object
    // use the viewBox, if available, to set the initial zoom and pan
    // let pan = [CANVAS_WIDTH / -2.0, CANVAS_HEIGHT / -2.0];
    // let zoom = 2.0 / f32::max(CANVAS_WIDTH, CANVAS_HEIGHT);
    let pan = [0.0, 0.0];
    let zoom = 0.5;
    let scene = Scene::new(zoom, pan, width / height);

    // Set up event processing and rendering
    let event_loop = glutin::EventsLoop::new();
    let glutin_builder = glutin::WindowBuilder::new()
        .with_dimensions(width as u32, height as u32)
        .with_decorations(true)
        .with_title("SVG Renderer");

    // Create a new GL context.
    let context = glutin::ContextBuilder::new()
        .with_multisampling(8);
        //.with_vsync(true);

    // Create all the necessary context with the window.
    let (window, mut device, mut factory, mut main_fbo, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(glutin_builder, context, &event_loop);

    // Load the shaders.
    let shader = factory.link_program(
        render::VERTEX_SHADER.as_bytes(),
        render::FRAGMENT_SHADER.as_bytes(),
    ).unwrap();

    // Create the normal shader.
    let mut rasterizer_state = gfx::state::Rasterizer::new_fill();
    rasterizer_state.samples = Some(gfx::state::MultiSample);
    let pso = factory.create_pipeline_from_program(
        &shader,
        gfx::Primitive::TriangleList,
        rasterizer_state,
        fill_pipeline::new(),
    ).unwrap();

    // Create the wireframe program.
    let mut rasterizer_state = gfx::state::Rasterizer::new_fill();
    rasterizer_state.method = gfx::state::RasterMethod::Line(1);
    // let wireframe_pso = factory.create_pipeline_from_program(
    //     &shader,
    //     gfx::Primitive::TriangleList,
    //     rasterizer_state,
    //     fill_pipeline::new(),
    // ).unwrap();

    let mut cmd_queue: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    // -------------------------- Generate VBO ------------------------------- //

    let t = Instant::now();

    // Create a new arena
    let arena = &mut Arena::<render::Vertex>::new();

    // Add some new nodes to the arena
    use svg::element::BasicStylableElement;
    let b = GroupBuilder::new(arena);
    let root =
        b.append(|_b| Circle::new()
            .radius(0.2)
            .cx(-0.5)
            .cy(-0.5)
            .fill(Color::blue())
            .wrap())
        .append(|_b| Rect::new()
            .x(1.0)
            .y(1.0)
            .width(0.5)
            .height(0.7)
            .wrap())
        .append(|_b| Line::new()
            .x1(0.5)
            .y1(0.5)
            .x2(-0.5)
            .y2(-0.5)
            .fill(Color::red())
            .stroke_width(0.01)
            .wrap())
        .append(|b|
            b.append(|_b| Circle::new()
                .radius(0.3)
                .fill(Color::blue())
                .wrap())
            .append(|_b| Rect::new()
                .dimensions(Vector::new(1.0, 1.0))
                .fill(Color::black())
                .wrap())
            .finalize()
            .wrap())
        .to_root();

    // let a = arena.new_node(ElementType::Group(element::Group { transform: Matrix::new_scaling(3.0) }));
    // let b = arena.new_node(ElementType::Circle(builder.circle().center(Point::new(0.0, 0.0)).radius(1.0).finalize()));
    // // let b = arena.new_node(ElementType::Rect(builder.rect().origin(Point::new(-0.5, -0.5)).dimensions(Vector::new(1.0, 1.0)).finalize()));

    // a.append(b, arena);

    let attribute_stack = AttributeStack::new();

    process_tree(render::VertexCtor, attribute_stack, arena, root);

    let buffers = &mut Buffers::new();
    generate_buffer(arena, root, buffers);

    println!("{:?}", buffers.vbo);
    println!("{:?}", buffers.tbo);
    println!("{:?}", scene);

    println!("It took {}us to tesselate.", t.elapsed().as_micros());

    // ----------------------------------------------------------------------- //

    loop {
        //let t = Instant::now();

        // Update the window view.
        gfx_window_glutin::update_views(&window, &mut main_fbo, &mut main_depth);

        // Clear the draw canvas with a default color.
        cmd_queue.clear(&main_fbo.clone(), [0.15, 0.15, 0.16, 1.0]);

        let globals = factory.create_constant_buffer::<render::Globals>(1);
        let transforms = factory.create_constant_buffer::<render::Transform>(render::MAX_TRANSFORMS);
        let colors = factory.create_constant_buffer::<render::Color>(render::MAX_COLORS);

        // Update the global state.
        cmd_queue.update_constant_buffer(&globals, &scene.into());

        // Update the transform and the color buffers.
        cmd_queue.update_buffer(&transforms, &buffers.tbo[..], 0).unwrap();
        cmd_queue.update_buffer(&colors, &buffers.cbo[..], 0).unwrap();

        let (vbo, ibo) = factory.create_vertex_buffer_with_slice(&buffers.vbo[..], &buffers.ibo[..]);

        //println!("{:?}", vbo.len());

        cmd_queue.draw(
            &ibo,
            &pso,
            &render::fill_pipeline::Data {
                vbo: vbo,
                out_color: main_fbo.clone(),
                globals: globals.clone(),
                transforms: transforms,
                colors: colors,
            },
        );
        
        cmd_queue.flush(&mut device);

        window.swap_buffers().unwrap();

        device.cleanup();
        //println!("{}", t.elapsed().as_micros());
    }
}