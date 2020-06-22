use ggez::Context;
use specs::{Builder, Entity, World, WorldExt};

use crate::*;

pub struct PlayScene;

impl PlayScene {
    pub fn new(_ctx: &mut Context, _world: &mut World) -> Self {
        Self {}
    }
}

impl scenes::Scene for PlayScene {
    fn update(&mut self, _world: &mut World, _ctx: &mut Context) -> scenes::SceneSwitch {
        scenes::SceneSwitch::None
    }

    fn on_enter(&mut self, world: &mut World) -> GameResult<Option<Vec<Entity>>> {
        Ok(None)
    }

    fn on_leave(&mut self, _world: &mut World) -> GameResult {
        Ok(())
    }
}
