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

pub enum ElementType {
    Circle(circle::Circle),
    Line(line::Line),
    Path(path::Path),
    Rect(rect::Rect),
    Group(group::Group),
}

pub trait Element {
    fn is_vbo_dirty() -> bool;
    fn is_transform_dirty() -> bool;
    fn get_vbo() -> Vec<i32>;
    fn get_tranform() -> Transform;
}