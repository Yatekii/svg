#![feature(nll)]

extern crate indextree;
extern crate nalgebra;

pub mod element;
pub mod geometry;

pub mod common;
pub mod transform_data;
pub mod vertex_data;
pub mod color;
pub mod attribute_stack;
pub mod processor;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
