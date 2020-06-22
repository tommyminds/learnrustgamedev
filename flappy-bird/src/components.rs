use crate::types::{Alignment, FontType};
use ggez::graphics;
use specs::{Component, FlaggedStorage, VecStorage, World, WorldExt};

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Debug)]
pub struct Render {
    pub visible: bool,
    pub z_index: u32,
}

impl Component for Render {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Image {
    pub image: graphics::Image,
}

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Text {
    pub text: String,
    pub font_size: f32,
    pub font: FontType,
    pub align: Alignment,
    pub color: graphics::Color,
}

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Parallax {
    pub speed: f32,
    pub looping_point: f32,
}

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Player;

pub fn register(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Size>();
    world.register::<Render>();
    world.register::<Player>();
    world.register::<Image>();
    world.register::<Text>();
    world.register::<Parallax>();
}