use ggez::{audio, graphics};
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
pub struct Fonts {
    pub retro: graphics::Font,
}

#[derive(Debug)]
pub struct Sounds {
    pub paddle_hit: audio::Source,
    pub score: audio::Source,
    pub wall_hit: audio::Source,
}
