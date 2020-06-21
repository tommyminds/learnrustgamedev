use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::*;

pub struct ScoreSystem;
impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        WriteExpect<'s, Sounds>,
        ReadStorage<'s, components::Ball>,
        WriteStorage<'s, components::Player>,
        ReadStorage<'s, components::Position>,
        ReadStorage<'s, components::Size>,
        WriteStorage<'s, components::Serving>,
        WriteStorage<'s, components::Scored>,
        WriteStorage<'s, components::Won>,
    );

    fn run(
        &mut self,
        (mut sounds, balls, mut players, positions, sizes, mut serving, mut scored, mut won): Self::SystemData,
    ) {
        for (_, ball_pos, ball_size) in (&balls, &positions, &sizes).join() {
            if ball_pos.x < 0.0 {
                for (player, serving, scored, won) in
                    (&mut players, &mut serving, &mut scored, &mut won).join()
                {
                    match player.side {
                        Side::Left => serving.0 = true,
                        Side::Right => {
                            scored.0 = true;
                            player.score += 1;
                            if player.score >= 3 {
                                won.0 = true;
                            }
                        }
                    }
                }

                let _ = sounds.score.play();
            }

            if ball_pos.x + ball_size.w > VIRTUAL_WIDTH {
                for (player, serving, scored, won) in
                    (&mut players, &mut serving, &mut scored, &mut won).join()
                {
                    match player.side {
                        Side::Left => {
                            scored.0 = true;
                            player.score += 1;
                            if player.score >= 3 {
                                won.0 = true;
                            }
                        }
                        Side::Right => serving.0 = true,
                    }
                }

                let _ = sounds.score.play();
            }
        }
    }
}
