use ggez::{graphics, Context, GameResult};
use ggez_extras::scene;
use specs::World;

use crate::*;

pub struct StartScene {}

impl StartScene {
    pub fn new(_ctx: &mut Context, _world: &mut World) -> Self {
        Self {}
    }
}

impl scene::Scene<World, input::Event> for StartScene {
    fn name(&self) -> &str {
        "StartScene"
    }

    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::Switch {
        if world
            .read_resource::<input::State>()
            .get_button_released(input::Button::Enter)
        {
            scenes::Switch::Replace(Box::new(scenes::ServeScene::new(ctx, world)))
        } else {
            scenes::Switch::None
        }
    }

    fn draw(&mut self, world: &World, ctx: &mut Context) -> GameResult<()> {
        let font_resource = &world.read_resource::<GameFont>();
        let bounds = mint::Point2 {
            x: VIRTUAL_WIDTH,
            y: f32::INFINITY,
        };

        let mut text = graphics::Text::new((
            "Welcome to Pong!\nPress Enter to begin!",
            font_resource.font,
            10.0,
        ));

        text.set_bounds(bounds, graphics::Align::Center);

        graphics::queue_text(
            ctx,
            &text,
            mint::Point2 { x: 0.0, y: 10.0 },
            Some(graphics::WHITE),
        );

        Ok(())
    }
}
