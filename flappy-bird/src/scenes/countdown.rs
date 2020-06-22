use ggez::Context;
use specs::{Builder, Entity, World, WorldExt};

use crate::*;

const COUNTDOWN_TIME: f32 = 0.75;

pub struct CountdownScene {
    count: u8,
    timer: f32,
    text: Entity,
}

impl CountdownScene {
    pub fn new(_ctx: &mut Context, world: &mut World) -> Self {
        let count = 3u8;
        let text = world
            .create_entity()
            .with(components::Render { visible: true })
            .with(components::Text {
                text: count.to_string(),
                font: FontType::Flappy,
                font_size: 56.0,
                color: graphics::WHITE,
                align: Alignment::Centered,
            })
            .with(components::Size {
                w: VIRTUAL_WIDTH,
                h: f32::INFINITY,
            })
            .with(components::Position {
                x: 0.0,
                y: 120.0,
                z: 1,
            })
            .build();

        Self {
            count,
            text,
            timer: 0.0,
        }
    }
}
impl scenes::Scene for CountdownScene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::SceneSwitch {
        self.timer += world.read_resource::<DeltaTime>().delta;
        if self.timer > COUNTDOWN_TIME {
            self.timer = self.timer % COUNTDOWN_TIME;
            self.count -= 1;

            if let Some(c) = world.write_storage::<components::Text>().get_mut(self.text) {
                c.text = self.count.to_string();
            }

            if self.count == 0 {
                return scenes::SceneSwitch::replace(scenes::PlayScene::new(ctx, world));
            }
        }

        scenes::SceneSwitch::None
    }

    fn on_enter(&mut self, _world: &mut World) -> GameResult<Option<Vec<Entity>>> {
        Ok(Some(vec![self.text]))
    }

    fn on_leave(&mut self, _world: &mut World) -> GameResult {
        Ok(())
    }
}
