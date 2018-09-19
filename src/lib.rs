extern crate indextree;
extern crate nalgebra;

mod element;
mod geometry;
mod common;

use common::*;

pub fn walk_tree(arena: &Arena, node_id: NodeId) {
    // We never access a node with an ID that does not exist.
    let node = arena.get(node_id).unwrap();
    use element::ElementType;
    match &node.data {
        ElementType::Circle(_circle) => println!("Circle"),
        ElementType::Line(_line) => println!("Line"),
        ElementType::Path(_path) => println!("Path"),
        ElementType::Rect(_rect) => println!("Rect"),
        ElementType::Group(_group) => {
            println!("\tGroup");
            for child_id in node_id.children(arena) {
                walk_tree(arena, child_id);
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

    #[test]
    fn it_works() {
        // Create a new arena
        let arena = &mut Arena::new();

        // Add some new nodes to the arena
        let a = arena.new_node(ElementType::Group(element::Group { transform: Transform::identity() }));
        let b = arena.new_node(ElementType::Circle(element::Circle { origin: Point::origin(), radius: 1.0 }));

        a.append(b, arena);

        super::walk_tree(arena, a);

        assert_eq!(b.ancestors(arena).into_iter().count(), 2);
    }
}
