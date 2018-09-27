pub trait TransformPrimitive {
    fn set_local_transform_index(&mut self, index: u32);
    fn set_group_transform_index(&mut self, index: u32);
}

pub trait ColorPrimitive {
    fn set_color_index(&mut self, index: u32);
}