use ggez::Context;
use specs::{Builder, Entity, World, WorldExt};

use crate::*;

pub struct ScoreScene {
    final_score: u8,
}

impl ScoreScene {
    pub fn new(final_score: u8) -> Self {
        Self {final_score}
    }
}

impl scenes::Scene for ScoreScene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::SceneSwitch {
        if world
            .read_resource::<input::State>()
            .get_button_released(input::Button::Enter)
        {
            scenes::SceneSwitch::replace(scenes::CountdownScene::new(ctx, world))
        } else {
            scenes::SceneSwitch::None
        }
    }

    fn on_enter(&mut self, world: &mut World) -> GameResult<Option<Vec<Entity>>> {
        Ok(Some(vec![
            world
                .create_entity()
                .with(components::Render { visible: true })
                .with(components::Text {
                    text: String::from("Oof! You lost!"),
                    font: FontType::Flappy,
                    font_size: 28.0,
                    color: graphics::WHITE,
                    align: Alignment::Centered,
                })
                .with(components::Size {
                    w: VIRTUAL_WIDTH,
                    h: f32::INFINITY,
                })
                .with(components::Position {
                    x: 0.0,
                    y: 64.0,
                    z: 1,
                })
                .build(),
            world
                .create_entity()
                .with(components::Render { visible: true })
                .with(components::Text {
                    text: format!("Score: {}", self.final_score),
                    font: FontType::Flappy,
                    font_size: 14.0,
                    color: graphics::WHITE,
                    align: Alignment::Centered,
                })
                .with(components::Size {
                    w: VIRTUAL_WIDTH,
                    h: f32::INFINITY,
                })
                .with(components::Position {
                    x: 0.0,
                    y: 100.0,
                    z: 1,
                })
                .build(),
            world
                .create_entity()
                .with(components::Render { visible: true })
                .with(components::Text {
                    text: String::from("Press Enter to Play Again!"),
                    font: FontType::Flappy,
                    font_size: 14.0,
                    color: graphics::WHITE,
                    align: Alignment::Centered,
                })
                .with(components::Size {
                    w: VIRTUAL_WIDTH,
                    h: f32::INFINITY,
                })
                .with(components::Position {
                    x: 0.0,
                    y: 160.0,
                    z: 1,
                })
                .build(),
        ]))
    }

    fn on_leave(&mut self, _world: &mut World) -> GameResult {
        Ok(())
    }
}
