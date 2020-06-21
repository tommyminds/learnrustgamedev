use specs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};

use crate::*;

pub struct PaddleSystem;
impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, components::Position>,
        ReadStorage<'s, components::Size>,
        ReadStorage<'s, components::Player>,
        ReadExpect<'s, input::State>,
        Read<'s, DeltaTime>,
    );

    fn run(&mut self, (mut positions, sizes, players, input, dt): Self::SystemData) {
        for (pos, size, player) in (&mut positions, &sizes, &players).join() {
            let amount = PADDLE_SPEED * dt.delta;
            match player.side {
                Side::Left => {
                    if input.get_button_down(input::Button::LeftPlayerUp) {
                        pos.y = (pos.y - amount).max(0.0);
                    }
                    if input.get_button_down(input::Button::LeftPlayerDown) {
                        pos.y = (pos.y + amount).min(VIRTUAL_HEIGHT - size.h);
                    }
                }
                Side::Right => {
                    if input.get_button_down(input::Button::RightPlayerUp) {
                        pos.y = (pos.y - amount).max(0.0);
                    }
                    if input.get_button_down(input::Button::RightPlayerDown) {
                        pos.y = (pos.y + amount).min(VIRTUAL_HEIGHT - size.h);
                    }
                }
            }
        }
    }
}
