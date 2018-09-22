use attribute_stack::*;
use common::*;
use element::ElementType;


pub fn process_tree(mut attribute_stack: AttributeStack, arena: &mut Arena, node_id: NodeId) {
    // We never access a node with an ID that does not exist.
    let node = arena.get(node_id).unwrap();
    match &node.data {
        ElementType::Circle(_circle) => update_node(arena, node_id, &attribute_stack),
        ElementType::Line(_line) => println!("Line"),
        ElementType::Path(_path) => println!("Path"),
        ElementType::Rect(_rect) => println!("Rect"),
        ElementType::Group(group) => {
            println!("\tGroup");
            attribute_stack.transform *= &group.transform;
            for child_id in node_id.children(arena).collect::<Vec<NodeId>>() {
                process_tree(attribute_stack.clone(), arena, child_id);
            }
        },
    }
}

pub fn update_node(arena: &mut Arena, node_id: NodeId, attribute_stack: &AttributeStack) {
    arena.get_mut(node_id).map(|node| {
        match &mut node.data {
            ElementType::Circle(circle) => {
                circle.transform_data.group_transform = attribute_stack.transform.clone();
            },
            ElementType::Line(line) => {
                line.transform_data.group_transform = attribute_stack.transform.clone();
            },
            ElementType::Path(path) => {
                path.transform_data.group_transform = attribute_stack.transform.clone();
            },
            ElementType::Rect(rect) => {
                rect.transform_data.group_transform = attribute_stack.transform.clone();
            },
            ElementType::Group(_group) => (),
        }
    });
}