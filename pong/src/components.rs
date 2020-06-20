use specs::{Component, VecStorage, World, WorldExt};

use crate::types::Side;

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

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Player {
    pub side: Side,
    pub name: String,
    pub score: u8,
}

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Ball {}

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Scored(pub bool);

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Won(pub bool);

#[derive(Clone, Debug, Component, Default)]
#[storage(VecStorage)]
pub struct Serving(pub bool);

pub fn register(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Size>();
    world.register::<Player>();
    world.register::<Ball>();
    world.register::<Serving>();
    world.register::<Scored>();
    world.register::<Won>();
}
