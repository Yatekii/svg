use std;


use lyon::tessellation::{ StrokeVertex, FillVertex, VertexConstructor };


use crate::geometry::Matrix;
use crate::color::Color;
use crate::element::ElementUpdate;
use crate::attribute_stack::*;
use crate::common::*;
use crate::element::ElementType;
use crate::vertex_data::Buffers;
use crate::primitive::*;


pub fn process_tree<V, Ctor>(ctor: Ctor, mut attribute_stack: AttributeStack, arena: &mut Arena<V>, node_id: NodeId)
where
    V: TransformPrimitive + ColorPrimitive + Clone + std::fmt::Debug,
    Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy 
{
    // We never access a node with an ID that does not exist.
    let node = arena.get_mut(node_id).unwrap();
    match &mut node.data {
        ElementType::Circle(circle) => update_node(ctor, circle, &attribute_stack),
        ElementType::Line(line) =>  update_node(ctor, line, &attribute_stack),
        ElementType::Path(_path) => println!("Path"),
        ElementType::Rect(rect) => update_node(ctor, rect, &attribute_stack),
        ElementType::Group(group) => {
            attribute_stack.transform *= &group.transform;
            for child_id in node_id.children(arena).collect::<Vec<NodeId>>() {
                process_tree(ctor, attribute_stack.clone(), arena, child_id);
            }
        },
        _ => ()
    }
}

pub fn update_node<T, V, Ctor>(ctor: Ctor, element: &mut T, attribute_stack: &AttributeStack)
where
    T: ElementUpdate<V>,
    V: TransformPrimitive + ColorPrimitive + Clone + std::fmt::Debug,
    Ctor: VertexConstructor<FillVertex, V> + VertexConstructor<StrokeVertex, V> + Copy
{
    element.get_vertex_data_mut().set_group_transform(attribute_stack.transform);
    if element.is_dirty() {
        element.tesselate(ctor);
    }
}

pub fn generate_buffer<V: TransformPrimitive + ColorPrimitive + Clone, M, C>(arena: &mut Arena<V>, node_id: NodeId, buffers: &mut Buffers<V, M, C>)
where
    M: From<Matrix>,
    C: From<Color> + std::fmt::Debug,
{
    // We never access a node with an ID that does not exist.
    let node = arena.get(node_id).unwrap();
    match &node.data {
        ElementType::Circle(circle) => add_to_buffer(circle, buffers),
        ElementType::Line(line) => add_to_buffer(line, buffers),
        ElementType::Path(_path) => println!("Path"),
        ElementType::Rect(rect) => add_to_buffer(rect, buffers),
        ElementType::Group(_group) => {
            for child_id in node_id.children(arena).collect::<Vec<NodeId>>() {
                generate_buffer(arena, child_id, buffers);
            }
        },
        _ => ()
    }
}

pub fn add_to_buffer<T, V, M, C>(element: &T, buffers: &mut Buffers<V, M, C>)
where
    T: ElementUpdate<V>, V: TransformPrimitive + ColorPrimitive + Clone, M: From<Matrix>, C: From<Color>,
    C: std::fmt::Debug,
{
    element.get_vertex_data().apply_to(buffers);
}