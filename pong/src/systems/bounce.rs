use ggez::graphics::Rect;
use ggez_extras::util::collides;
use rand::Rng;
use specs::{Join, ReadStorage, System, WriteExpect, WriteStorage};

use crate::*;

pub struct BounceSystem;
impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteExpect<'s, Sounds>,
        ReadStorage<'s, components::Ball>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::Position>,
        WriteStorage<'s, components::Velocity>,
        ReadStorage<'s, components::Size>,
    );

    fn run(
        &mut self,
        (mut sounds, balls, players, mut positions, mut velocities, sizes): Self::SystemData,
    ) {
        let mut rng = rand::thread_rng();
        let mut new_ball_x: f32 = 0.0;
        let mut new_ball_y: f32 = 0.0;

        for (_, ball_pos, ball_size, ball_vel) in
            (&balls, &positions, &sizes, &mut velocities).join()
        {
            new_ball_x = ball_pos.x;
            new_ball_y = ball_pos.y;

            // Bounce from top
            if ball_pos.y <= 0.0 {
                new_ball_y = 0.0;
                ball_vel.y = -ball_vel.y;
                let _ = sounds.wall_hit.play();
            }
            // Bounce from bottom
            else if ball_pos.y >= VIRTUAL_HEIGHT - ball_size.h {
                new_ball_y = VIRTUAL_HEIGHT - ball_size.h;
                ball_vel.y = -ball_vel.y;
                let _ = sounds.wall_hit.play();
            }

            for (player, player_pos, player_size) in (&players, &positions, &sizes).join() {
                if collides(
                    Rect::new(ball_pos.x, ball_pos.y, ball_size.w, ball_size.h),
                    Rect::new(player_pos.x, player_pos.y, player_size.w, player_size.h),
                ) {
                    match player.side {
                        // Bounce from left paddle
                        Side::Left => {
                            new_ball_x = player_pos.x + player_size.w;
                            ball_vel.x = -ball_vel.x * 1.1;
                        }

                        // Bounce from right paddle
                        Side::Right => {
                            new_ball_x = player_pos.x - ball_size.w;
                            ball_vel.x = -ball_vel.x * 1.1;
                        }
                    }

                    if ball_pos.y < player_pos.y + player_size.h / 2.0 {
                        ball_vel.y = -rng.gen_range(50.0, 100.0)
                            * (player_pos.y + (player_size.h / 2.0))
                            / ball_pos.y
                    } else {
                        ball_vel.y =
                            rng.gen_range(50.0, 100.0) * (player_pos.y + player_size.h) / ball_pos.y
                    }

                    let _ = sounds.paddle_hit.play();
                }
            }
        }

        for (_, ball_pos) in (&balls, &mut positions).join() {
            ball_pos.x = new_ball_x;
            ball_pos.y = new_ball_y;
        }
    }
}
