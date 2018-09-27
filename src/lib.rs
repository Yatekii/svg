#![feature(nll)]

extern crate indextree;
extern crate nalgebra;

#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate lyon;
extern crate usvg;

pub mod element;
pub mod geometry;

pub mod common;
pub mod transform_data;
pub mod vertex_data;
pub mod color;
pub mod attribute_stack;
pub mod processor;
pub mod primitive;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
