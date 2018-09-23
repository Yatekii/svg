use element::circle::Circle;
use element::Element;
use element::ElementUpdate;
use attribute_stack::*;
use common::*;
use element::ElementType;


pub fn process_tree(mut attribute_stack: AttributeStack, arena: &mut Arena, node_id: NodeId) {
    // We never access a node with an ID that does not exist.
    let node = arena.get_mut(node_id).unwrap();
    match &mut node.data {
        ElementType::Circle(circle) => update_node(circle, &attribute_stack),
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

pub fn update_node<T>(element: &mut T, attribute_stack: &AttributeStack)
    where T: ElementUpdate  {
    element.set_group_transform(&attribute_stack.transform);
}