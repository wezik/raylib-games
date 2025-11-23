use raylib::color::Color;

pub mod building;
pub mod monster;
pub mod player;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct EntityId(pub usize);

#[derive(Debug)]
pub struct Draw {
    pub color: Color,
}
