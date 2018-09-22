extern crate svg;

use svg::common::*;
use svg::element;
use svg::element::ElementType;
use svg::geometry::*;
use svg::attribute_stack::*;
use svg::processor::process_tree;

fn main() {
    // Create a new arena
    let arena = &mut Arena::new();

    // Add some new nodes to the arena
    let a = arena.new_node(ElementType::Group(element::Group { transform: Transform::from_matrix_unchecked(Matrix::new_scaling(3.0)) }));
    let b = arena.new_node(ElementType::Circle(element::Circle::new()));

    a.append(b, arena);

    let attribute_stack = AttributeStack::new();

    process_tree(attribute_stack, arena, a);
}