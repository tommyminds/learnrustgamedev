use ggez::Context;
use specs::{Dispatcher, DispatcherBuilder, Entity, World};

use crate::*;

pub struct PlayScene {
    dispatcher: Dispatcher<'static, 'static>,
}

impl PlayScene {
    pub fn new(_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let mut dispatcher = Self::register_systems();
        dispatcher.setup(world);
        Self { dispatcher }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        DispatcherBuilder::new()
            .with(systems::ParallaxSystem, "parallax", &[])
            .build()
    }
}

impl scenes::Scene for PlayScene {
    fn update(&mut self, world: &mut World, _ctx: &mut Context) -> scenes::SceneSwitch {
        self.dispatcher.dispatch(world);
        scenes::SceneSwitch::None
    }

    fn on_enter(&mut self, _world: &mut World) -> GameResult<Option<Vec<Entity>>> {
        Ok(None)
    }

    fn on_leave(&mut self, _world: &mut World) -> GameResult {
        Ok(())
    }
}
