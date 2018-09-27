#![feature(duration_as_u128)]
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate svg;
extern crate lyon;

mod render;

use svg::common::*;
use svg::element;
use svg::element::ElementType;
use svg::geometry::*;
use svg::attribute_stack::*;
use svg::processor::process_tree;

use gfx::traits::{Device, FactoryExt};
use glutin::GlContext;

use std::time::Instant;

use render::{
    fill_pipeline, ColorFormat, DepthFormat, Scene
};

const WINDOW_SIZE: f32 = 800.0;
const CANVAS_WIDTH: f32 = 1200.0;
const CANVAS_HEIGHT: f32 = 500.0;

fn main() {
    // Create a new arena
    let arena = &mut Arena::<render::Vertex>::new();

    // Add some new nodes to the arena
    let a = arena.new_node(ElementType::Group(element::Group { transform: Matrix::new_scaling(3.0) }));
    let b = arena.new_node(ElementType::Circle(element::Circle::new()));

    a.append(b, arena);

    let attribute_stack = AttributeStack::new();

    process_tree(attribute_stack, arena, a);

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
    let pan = [CANVAS_WIDTH / -2.0, CANVAS_HEIGHT / -2.0];
    let zoom = 2.0 / f32::max(CANVAS_WIDTH, CANVAS_HEIGHT);
    let mut scene = Scene::new(zoom, pan, width / height);

    // Set up event processing and rendering
    let mut event_loop = glutin::EventsLoop::new();
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
    let wireframe_pso = factory.create_pipeline_from_program(
        &shader,
        gfx::Primitive::TriangleList,
        rasterizer_state,
        fill_pipeline::new(),
    ).unwrap();

    let mut cmd_queue: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let globals = factory.create_constant_buffer::<render::Globals>(1);
    let transforms = factory.create_constant_buffer::<render::Transform>(1);
    let colors = factory.create_constant_buffer::<render::Color>(1);

    loop {
        //let t = Instant::now();

        gfx_window_glutin::update_views(&window, &mut main_fbo, &mut main_depth);

        cmd_queue.clear(&main_fbo.clone(), [0.15, 0.15, 0.16, 1.0]);

        cmd_queue.update_constant_buffer(&globals, &scene.into());

        // TODO: Draw
        // vehicle_instances = vehicle_instances
        //     .into_iter()
        //     .filter(|vi| !vi.left_station)
        //     .enumerate()
        //     .map(|(i, mut instance)| {
        //         instance.update(i);
        //         cmd_queue.update_constant_buffer(&vehicle_attributes, &instance.clone().into());
        //         instance.draw(&mut cmd_queue, if scene.wireframe { &wireframe_pso } else { &pso }, &globals, &vehicle_attributes, &main_fbo);
        //         instance
        //     })
        //     .collect();
        // mondaine.draw(&mut cmd_queue, if scene.wireframe { &wireframe_pso } else { &pso }, &globals, &vehicle_attributes, &main_fbo);
        
        cmd_queue.flush(&mut device);

        window.swap_buffers().unwrap();

        device.cleanup();
        //println!("{}", t.elapsed().as_micros());
    }
}