extern crate indextree;
extern crate nalgebra;

mod element;
mod geometry;
mod common;
mod transform_data;
mod vertex_data;
mod color;
mod attribute_stack;

use common::*;

pub fn walk_tree(attribute_stack: &mut attribute_stack::AttributeStack, arena: &Arena, node_id: NodeId) {
    // We never access a node with an ID that does not exist.
    let node = arena.get(node_id).unwrap();
    use element::ElementType;
    match &node.data {
        ElementType::Circle(_circle) => println!("Circle"),
        ElementType::Line(_line) => println!("Line"),
        ElementType::Path(_path) => println!("Path"),
        ElementType::Rect(_rect) => println!("Rect"),
        ElementType::Group(group) => {
            println!("\tGroup");
            attribute_stack.transform *= &group.transform;
            for child_id in node_id.children(arena) {
                walk_tree(attribute_stack, arena, child_id);
            }
        },
    }
}

#[cfg(test)]
mod tests {

    use super::common::*;
    use super::element;
    use super::element::ElementType;
    use super::geometry::*;
    use super::attribute_stack::*;

    #[test]
    fn it_works() {
        // Create a new arena
        let arena = &mut Arena::new();

        // Add some new nodes to the arena
        let a = arena.new_node(ElementType::Group(element::Group { transform: Transform::from_matrix_unchecked(Matrix::new_scaling(3.0)) }));
        let b = arena.new_node(ElementType::Circle(element::Circle::new()));

        a.append(b, arena);

        let mut attribute_stack = AttributeStack::new();

        super::walk_tree(&mut attribute_stack, arena, a);

        assert_eq!(attribute_stack.transform, Transform::from_matrix_unchecked(Matrix::new_scaling(3.0)));
    }
}
