use ggez::{graphics, Context, GameResult};
use specs::World;

use crate::*;

pub struct StartScene {}

impl StartScene {
    pub fn new(_ctx: &mut Context, _world: &mut World) -> Self {
        Self {}
    }
}

impl super::Scene for StartScene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::SceneSwitch {
        if world
            .read_resource::<input::State>()
            .get_button_released(input::Button::Enter)
        {
            scenes::SceneSwitch::replace(scenes::ServeScene::new(ctx, world))
        } else {
            scenes::SceneSwitch::None
        }
    }

    fn draw(&mut self, world: &World, ctx: &mut Context) -> GameResult<()> {
        let font_resource = &world.read_resource::<Fonts>();

        let t1 = graphics::Text::new(("Welcome to Pong!", font_resource.retro, 10.0));
        let t2 = graphics::Text::new(("Press Enter to begin!", font_resource.retro, 10.0));

        let t1_x = (VIRTUAL_WIDTH / 2.0) - (t1.dimensions(ctx).0 / 2) as f32;
        let t2_x = (VIRTUAL_WIDTH / 2.0) - (t2.dimensions(ctx).0 / 2) as f32;

        graphics::queue_text(
            ctx,
            &t1,
            mint::Point2 { x: t1_x, y: 10.0 },
            Some(graphics::WHITE),
        );

        graphics::queue_text(
            ctx,
            &t2,
            mint::Point2 { x: t2_x, y: 20.0 },
            Some(graphics::WHITE),
        );

        Ok(())
    }
}
