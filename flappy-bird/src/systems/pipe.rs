use rand::Rng;
use specs::{Entities, Join, Read, ReadExpect, System, WriteStorage};
use std::f32::consts;

use crate::*;

pub struct PipeSystem {
    last_y: f32,
    timer: f32,
}

impl PipeSystem {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            last_y: -PIPE_HEIGHT + rng.gen_range(0.0, 80.0) + 20.0,
            timer: 2.0,
        }
    }
}

impl<'s> System<'s> for PipeSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, components::Pipe>,
        WriteStorage<'s, components::Render>,
        WriteStorage<'s, components::Image>,
        WriteStorage<'s, components::Position>,
        WriteStorage<'s, components::Size>,
        WriteStorage<'s, components::Rotation>,
        ReadExpect<'s, Images>,
        Read<'s, DeltaTime>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut pipe_storage,
            mut render_storage,
            mut image_storage,
            mut pos_storage,
            mut size_storage,
            mut rotation_storage,
            images_resource,
            dt,
        ): Self::SystemData,
    ) {
        self.timer += dt.delta;

        for (e, _, pos) in (&entities, &pipe_storage, &mut pos_storage).join() {
            if pos.x > -72.0 {
                pos.x -= PIPE_SPEED * dt.delta;
            } else {
                let _ = entities.delete(e);
            }
        }

        if self.timer > 2.0 {
            let mut rng = rand::thread_rng();
            let pipe_y = (-PIPE_HEIGHT + 10.0)
                .max((PIPE_HEIGHT / 3.0).min(self.last_y + rng.gen_range(-20.0, 20.0)));

            entities
                .build_entity()
                .with(
                    components::Pipe {
                        scored: false,
                        side: PipeSide::Top,
                    },
                    &mut pipe_storage,
                )
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
                .build();

            entities
                .build_entity()
                .with(
                    components::Pipe {
                        scored: false,
                        side: PipeSide::Bottom,
                    },
                    &mut pipe_storage,
                )
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
                .build();

            self.timer = 0.0;
        }
    }
}
