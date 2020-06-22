use ggez::{Context, GameResult};
use specs::{Join, World};

use crate::*;

pub struct PlayScene {
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl PlayScene {
    pub fn new(_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let mut dispatcher = Self::register_systems();
        dispatcher.setup(world);
        Self { dispatcher }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .with(systems::PaddleSystem, "paddle", &[])
            .with(systems::BallSystem, "ball", &[])
            .with(systems::BounceSystem, "bounce", &["paddle", "ball"])
            .with(systems::ScoreSystem, "score", &["bounce"])
            .build()
    }
}

impl super::Scene for PlayScene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::SceneSwitch {
        self.dispatcher.dispatch(world);

        let mut to_serve = false;
        let mut to_done = false;

        for (_, scored, won) in (
            &world.read_storage::<components::Player>(),
            &world.read_storage::<components::Scored>(),
            &world.read_storage::<components::Won>(),
        )
            .join()
        {
            if won.0 == true {
                to_done = true;
            }

            if scored.0 == true {
                to_serve = true;
            }
        }

        if to_done {
            scenes::SceneSwitch::replace(scenes::WonScene::new(ctx, world))
        } else if to_serve {
            scenes::SceneSwitch::replace(scenes::ServeScene::new(ctx, world))
        } else {
            scenes::SceneSwitch::None
        }
    }

    fn draw(&mut self, _world: &World, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}
