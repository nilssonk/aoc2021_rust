#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Line<T> = (Vec2<T>, Vec2<T>);
