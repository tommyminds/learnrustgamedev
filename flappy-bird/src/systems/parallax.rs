use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::*;

pub struct ParallaxSystem;
impl<'s> System<'s> for ParallaxSystem {
    type SystemData = (
        ReadStorage<'s, components::Parallax>,
        WriteStorage<'s, components::Position>,
        Read<'s, DeltaTime>,
    );

    fn run(&mut self, (parallax, mut positions, dt): Self::SystemData) {
        for (parallax, pos) in (&parallax, &mut positions).join() {
            pos.x = -((-pos.x + parallax.speed * dt.delta) % parallax.looping_point);
        }
    }
}
