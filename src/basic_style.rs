use geometry::Matrix;
use color::Color;

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
        use geometry::Matrix;
        use color::Color;
        use basic_style::BasicStylableElement;
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