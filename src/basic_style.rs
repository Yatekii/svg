use crate::geometry::Matrix;
use crate::color::Color;

pub trait BasicStylableElement {
    fn fill(self, fill: Color) -> Self where Self: Sized;
    fn fill_ref(&mut self, fill: Color) -> &mut Self where Self: Sized;
    fn stroke(self, stroke: Color) -> Self where Self: Sized;
    fn stroke_ref(&mut self, stroke: Color) -> &mut Self where Self: Sized;
    fn stroke_width(self, width: f32) -> Self where Self: Sized;
    fn stroke_width_ref(&mut self, width: f32) -> &mut Self where Self: Sized;
    fn transform(self, transform: Matrix) -> Self where Self: Sized;
    fn transform_ref(&mut self, transform: Matrix) -> &mut Self where Self: Sized;
}

#[macro_export]
macro_rules! impl_element_style {
    ( $c:ident, [$func_name:ident, $func_name_ref:ident]( $( $name:ident:$args:ty ), * ) $body_geometry:block $body_group:block) => (
        pub fn $func_name(self, $( $name:$args ), *) -> Self {
            match self {
                ElementType::Circle(mut $c) => { $body_geometry ElementType::Circle($c) },
                ElementType::Line(mut $c) => { $body_geometry ElementType::Line($c) },
                ElementType::Path(mut $c) => { $body_geometry ElementType::Path($c) },
                ElementType::Rect(mut $c) => { $body_geometry ElementType::Rect($c) },
                ElementType::Group(mut $c) => { $body_group ElementType::Group($c) },
            }
        }

        pub fn $func_name_ref( &mut self,  $( $name:$args ), * ) -> &mut Self {
            match self {
                ElementType::Circle(ref mut $c) => { $body_geometry self },
                ElementType::Line(ref mut $c) => { $body_geometry self },
                ElementType::Path(ref mut $c) => { $body_geometry self },
                ElementType::Rect(ref mut $c) => { $body_geometry self },
                ElementType::Group(ref mut $c) => { $body_group self },
            }
        }
    )
}

#[macro_export]
macro_rules! impl_basic_style {
    ( $sel:ident, [$func_name:ident, $func_name_ref:ident]( $( $name:ident:$args:ty ), * ) $body:block) => (
        fn $func_name(mut $sel, $( $name:$args ), *) -> Self {
            $body
            $sel
        }

        fn $func_name_ref( &mut $sel,  $( $name:$args ), * ) -> &mut Self {
            $body
            $sel
        }
    )
}

#[macro_export]
macro_rules! impl_basic_styles_for_struct {
    ($struct:ident) => {
        use crate::geometry::Matrix;
        use crate::color::Color;
        use crate::basic_style::BasicStylableElement;
        impl<V> BasicStylableElement for $struct<V>
        where
            V: TransformPrimitive + ColorPrimitive + Clone
        {
            impl_basic_style!(self, [fill, fill_ref](fill: Color) {
                self.vertex_data.fill = fill;
            });

            impl_basic_style!(self, [stroke, stroke_ref](stroke: Color) {
                self.make_dirty();
                self.vertex_data.stroke = stroke;
            });

            impl_basic_style!(self, [stroke_width, stroke_width_ref](stroke_width: f32) {
                self.make_dirty();
                self.vertex_data.stroke_width = stroke_width;
            });

            impl_basic_style!(self, [transform, transform_ref](matrix: Matrix) {
                self.vertex_data.set_local_transform(matrix);
            });
        }
    };
}