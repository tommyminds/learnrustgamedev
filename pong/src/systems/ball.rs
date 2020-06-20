use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::*;

pub struct BallSystem;
impl<'s> System<'s> for BallSystem {
    type SystemData = (
        ReadStorage<'s, components::Ball>,
        WriteStorage<'s, components::Position>,
        ReadStorage<'s, components::Velocity>,
        Read<'s, DeltaTime>,
    );

    fn run(&mut self, (balls, mut positions, velocity, dt): Self::SystemData) {
        for (_, pos, vel) in (&balls, &mut positions, &velocity).join() {
            pos.x += vel.x * dt.delta;
            pos.y += vel.y * dt.delta;
        }
    }
}
