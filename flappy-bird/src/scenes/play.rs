use ggez::Context;
use rand::Rng;
use specs::{Dispatcher, DispatcherBuilder, Entity, World, WorldExt, Join};

use crate::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PipeAlignment {
    Top,
    Bottom,
}

pub struct PlayScene {
    dispatcher: Dispatcher<'static, 'static>,
    pipe_pairs: Vec<(Entity, Entity)>,
    last_y: f32,
    score_entity: Option<Entity>,
}

impl scenes::Scene for PlayScene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::SceneSwitch {
        self.dispatcher.dispatch(world);

        if world
            .read_resource::<input::State>()
            .get_button_released(input::Button::Enter)
        {
            scenes::SceneSwitch::replace(scenes::TitleScene::new(ctx, world))
        } else {
            for score in (world.read_storage::<components::Score>()).join() {
                world.write_storage::<components::Text>().get_mut(self.score_entity.unwrap()).unwrap().text = format!("Score: {}", score.0)
            }

            scenes::SceneSwitch::None
        }
    }

    fn on_enter(&mut self, world: &mut World) -> GameResult<Option<Vec<Entity>>> {
        let entities: Vec<Entity> = vec![self.create_score_text(world)];
        Ok(Some(entities))
    }

    fn on_leave(&mut self, world: &mut World) -> GameResult {
        let mut pipe_entities: Vec<Entity> = Vec::new();
        for (e, _) in (&world.entities(), &world.read_storage::<components::Pipe>()).join() {
            pipe_entities.push(e);
        }
        let _ = world.delete_entities(pipe_entities.as_slice());
        Ok(())
    }
}

impl PlayScene {
    pub fn new(_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let mut dispatcher = Self::register_systems();
        dispatcher.setup(world);

        let mut rng = rand::thread_rng();
        Self {
            dispatcher,
            pipe_pairs: Vec::new(),
            last_y: -PIPE_HEIGHT + rng.gen_range(0.0, 80.0) + 20.0,
            score_entity: None,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        DispatcherBuilder::new()
            .with(systems::ParallaxSystem, "parallax", &[])
            .with(systems::PhysicsSystem, "physics", &[])
            .with(systems::PipeSystem::new(), "pipe", &[])
            .build()
    }

    fn create_score_text(&mut self, world: &mut World) -> Entity {
        let entity = world
            .create_entity()
            .with(components::Render { visible: true })
            .with(components::Text {
                text: String::from("Score: 0"),
                font: FontType::Flappy,
                font_size: 28.0,
                color: graphics::WHITE,
                align: Alignment::Centered,
            })
            .with(components::Position {
                x: 8.0,
                y: 8.0,
                z: 1,
            })
            .build();
        self.score_entity = Some(entity.clone());
        entity
    }
}
