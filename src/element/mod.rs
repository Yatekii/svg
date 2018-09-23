pub mod circle;
pub mod line;
pub mod path;
pub mod rect;
pub mod group;

pub use self::circle::Circle;
pub use self::line::Line;
pub use self::path::Path;
pub use self::rect::Rect;
pub use self::group::Group;

use geometry::Transform;
use color::Color;

pub enum ElementType {
    Circle(circle::Circle),
    Line(line::Line),
    Path(path::Path),
    Rect(rect::Rect),
    Group(group::Group),
}

pub trait Element {
    fn get_vbo(&self) -> &Vec<i32>;
    fn get_local_tranform(&self) -> &Transform;
    fn get_group_tranform(&self) -> &Transform;
}

pub trait ElementUpdate {
    fn set_group_transform(&mut self, transform: &Transform);
    fn set_local_transform(&mut self, transform: &Transform);
    fn set_color(&mut self, color: &Color);
}