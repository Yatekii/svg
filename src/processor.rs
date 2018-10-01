use geometry::Matrix;
use color::Color;
use element::Element;
use element::ElementUpdate;
use attribute_stack::*;
use common::*;
use element::ElementType;
use vertex_data::Buffers;
use primitive::*;


pub fn process_tree<V: TransformPrimitive + ColorPrimitive + Clone>(mut attribute_stack: AttributeStack, arena: &mut Arena<V>, node_id: NodeId) {
    // We never access a node with an ID that does not exist.
    let node = arena.get_mut(node_id).unwrap();
    match &mut node.data {
        ElementType::Circle(circle) => update_node(circle, &attribute_stack),
        ElementType::Line(_line) => println!("Line"),
        ElementType::Path(_path) => println!("Path"),
        ElementType::Rect(rect) => update_node(rect, &attribute_stack),
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

pub fn generate_buffer<V: TransformPrimitive + ColorPrimitive + Clone, M, C>(arena: &mut Arena<V>, node_id: NodeId, buffers: &mut Buffers<V, M, C>)
where M: From<Matrix>, C: From<Color> {
    // We never access a node with an ID that does not exist.
    let node = arena.get(node_id).unwrap();
    match &node.data {
        ElementType::Circle(circle) => add_to_buffer(circle, buffers),
        ElementType::Line(_line) => println!("Line"),
        ElementType::Path(_path) => println!("Path"),
        ElementType::Rect(rect) => add_to_buffer(rect, buffers),
        ElementType::Group(_group) => {
            for child_id in node_id.children(arena).collect::<Vec<NodeId>>() {
                generate_buffer(arena, child_id, buffers);
            }
        },
    }
}

pub fn add_to_buffer<T, V, M, C>(element: &T, buffers: &mut Buffers<V, M, C>)
    where T: Element<V>, V: TransformPrimitive + ColorPrimitive + Clone, M: From<Matrix>, C: From<Color> {
    element.get_vertex_data().apply_to(buffers);
}