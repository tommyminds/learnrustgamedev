use specs::{Join, WriteExpect, ReadStorage, System, WriteStorage};

use crate::*;

pub struct ScoreSystem;

impl ScoreSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'s> System<'s> for ScoreSystem {
    type SystemData = (
        WriteExpect<'s, Sounds>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::Pipe>,
        ReadStorage<'s, components::Position>,
        ReadStorage<'s, components::Size>,
        WriteStorage<'s, components::Score>,
        WriteStorage<'s, components::Dead>,
    );

    fn run(
        &mut self,
        (mut sounds, player_storage, mut pipe_storage, pos_storage, size_storage, mut score_storage, mut dead_storage): Self::SystemData,
    ) {
        for (pipe, pipe_pos, pipe_size) in (&mut pipe_storage, &pos_storage, &size_storage).join() {
            for (_, player_pos, player_size, score, dead) in (&player_storage, &pos_storage, &size_storage, &mut score_storage, &mut dead_storage).join()
            {
                if !pipe.scored
                    && pipe.side == PipeSide::Top
                    && pipe_pos.x + PIPE_WIDTH < player_pos.x
                {
                    pipe.scored = true;
                    score.0 += 1;
                    let _ = sounds.score.play();
                }

                if (player_pos.x + 2.0) + (player_size.w - 4.0) >= pipe_pos.x && player_pos.x + 2.0 <= pipe_pos.x + pipe_size.w {
                    if (player_pos.y + 2.0) + (player_size.h - 4.0) >= pipe_pos.y && player_pos.y + 2.0 <= pipe_pos.y + pipe_size.h {
                        dead.0 = true;
                        let _ = sounds.explosion.play();
                        let _ = sounds.hurt.play();
                    }
                }
            }
        }

        for (_, player_pos, dead) in (&player_storage, &pos_storage, &mut dead_storage).join() {
            if player_pos.y > VIRTUAL_HEIGHT - 15.0 {
                dead.0 = true;
                let _ = sounds.hurt.play();
            }
        }
    }
}
