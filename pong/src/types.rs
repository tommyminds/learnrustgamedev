use ggez::graphics::Font;
use specs::Entity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ServingPlayer(pub Option<Entity>);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct WinningPlayer(pub Option<Entity>);

#[derive(Clone, Debug, Default)]
pub struct DeltaTime {
    pub delta: f32,
}

#[derive(Clone, Debug, Default)]
pub struct GameFont {
    pub font: Font,
}
