use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::*;

pub struct PhysicsSystem;
impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, components::Position>,
        ReadStorage<'s, components::Velocity>,
        Read<'s, DeltaTime>,
    );

    fn run(&mut self, (mut positions, velocity, dt): Self::SystemData) {
        for (pos, vel) in ( &mut positions, &velocity).join() {
            pos.x += vel.x * dt.delta;
            pos.y += vel.y * dt.delta;
        }
    }
}
