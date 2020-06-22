use rand::Rng;
use specs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use std::f32::consts;

use crate::*;

pub struct PipeSystem {
    pipe_pairs: Vec<(Entity, Entity)>,
    last_y: f32,
    timer: f32,
}

impl PipeSystem {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            pipe_pairs: Vec::new(),
            last_y: -PIPE_HEIGHT + rng.gen_range(0.0, 80.0) + 20.0,
            timer: 2.0,
        }
    }
}

impl<'s> System<'s> for PipeSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, components::Player>,
        WriteStorage<'s, components::Pipe>,
        WriteStorage<'s, components::Render>,
        WriteStorage<'s, components::Image>,
        WriteStorage<'s, components::Position>,
        WriteStorage<'s, components::Size>,
        WriteStorage<'s, components::Rotation>,
        WriteStorage<'s, components::Score>,
        ReadExpect<'s, Images>,
        Read<'s, DeltaTime>,
    );

    fn run(
        &mut self,
        (
            entities,
            player_storage,
            mut pipe_storage,
            mut render_storage,
            mut image_storage,
            mut pos_storage,
            mut size_storage,
            mut rotation_storage,
            mut score_storage,
            images_resource,
            dt,
        ): Self::SystemData,
    ) {
        self.timer += dt.delta;

        for (_, pos) in (&pipe_storage, &mut pos_storage).join() {
            pos.x -= PIPE_SPEED * dt.delta;
        }

        if self.timer > 2.0 {
            let mut rng = rand::thread_rng();
            let pipe_y = (-PIPE_HEIGHT + 10.0)
                .max((PIPE_HEIGHT / 3.0).min(self.last_y + rng.gen_range(-20.0, 20.0)));

            self.pipe_pairs.push((
                entities
                    .build_entity()
                    .with(components::Pipe::default(), &mut pipe_storage)
                    .with(components::Render { visible: true }, &mut render_storage)
                    .with(
                        components::Image {
                            image: images_resource.pipe.clone(),
                        },
                        &mut image_storage,
                    )
                    .with(
                        components::Size {
                            w: PIPE_WIDTH,
                            h: PIPE_HEIGHT,
                        },
                        &mut size_storage,
                    )
                    .with(
                        components::Rotation {
                            deg: consts::PI,
                            x: 1.0,
                            y: 1.0,
                        },
                        &mut rotation_storage,
                    )
                    .with(
                        components::Position {
                            x: VIRTUAL_WIDTH + 64.0,
                            y: pipe_y,
                            z: 1,
                        },
                        &mut pos_storage,
                    )
                    .build(),
                entities
                    .build_entity()
                    .with(components::Pipe::default(), &mut pipe_storage)
                    .with(components::Render { visible: true }, &mut render_storage)
                    .with(
                        components::Image {
                            image: images_resource.pipe.clone(),
                        },
                        &mut image_storage,
                    )
                    .with(
                        components::Size {
                            w: PIPE_WIDTH,
                            h: PIPE_HEIGHT,
                        },
                        &mut size_storage,
                    )
                    .with(
                        components::Position {
                            x: VIRTUAL_WIDTH + 64.0,
                            y: pipe_y + PIPE_HEIGHT + 90.0,
                            z: 1,
                        },
                        &mut pos_storage,
                    )
                    .build(),
            ));

            self.timer = 0.0;
        }
    }
}
