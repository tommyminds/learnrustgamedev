use specs::{Join, Read, WriteExpect, ReadExpect, ReadStorage, System, WriteStorage};

use crate::*;

const GRAVITY: f32 = 18.0;

pub struct PlayerSystem;
impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteExpect<'s, Sounds>,
        ReadExpect<'s, input::State>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::Position>,
        WriteStorage<'s, components::Velocity>,
        Read<'s, DeltaTime>,
    );

    fn run(&mut self, (mut sounds, input_state, players, mut positions, mut velocity, dt): Self::SystemData) {
        for (_, pos, vel) in (&players, &mut positions, &mut velocity).join() {
            vel.y += GRAVITY * dt.delta;
            if input_state.get_button_pressed(input::Button::Space) {
                vel.y = -4.0;
                let _ = sounds.jump.play();
            }
            pos.y += vel.y;
        }
    }
}
