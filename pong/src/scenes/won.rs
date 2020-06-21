use ggez::{graphics, Context, GameResult};
use ggez_extras::scene;

use specs::{Join, World};

use crate::*;

pub struct WonScene {}

impl WonScene {
    pub fn new(_ctx: &mut Context, _world: &mut World) -> Self {
        Self {}
    }
}

impl scene::Scene<World, input::Event> for WonScene {
    fn name(&self) -> &str {
        "WonScene"
    }

    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::Switch {
        if world
            .read_resource::<input::State>()
            .get_button_released(input::Button::Enter)
        {
            for (player, serving, won, scored) in (
                &mut world.write_storage::<components::Player>(),
                &mut world.write_storage::<components::Serving>(),
                &mut world.write_storage::<components::Won>(),
                &mut world.write_storage::<components::Scored>(),
            )
                .join()
            {
                player.score = 0;
                serving.0 = !won.0;
                won.0 = false;
                scored.0 = false;
            }

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

        for (player, won) in (
            &world.read_storage::<components::Player>(),
            &world.read_storage::<components::Won>(),
        )
            .join()
        {
            if won.0 {
                let mut t1 = graphics::Text::new((
                    format!("Player {} wins!", player.name),
                    font_resource.font,
                    32.0,
                ));
                let mut t2 =
                    graphics::Text::new(("Press Enter to restart!", font_resource.font, 16.0));

                t1.set_bounds(bounds, graphics::Align::Center);
                t2.set_bounds(bounds, graphics::Align::Center);

                graphics::queue_text(
                    ctx,
                    &t1,
                    mint::Point2 { x: 0.0, y: 10.0 },
                    Some(graphics::WHITE),
                );

                graphics::queue_text(
                    ctx,
                    &t2,
                    mint::Point2 { x: 0.0, y: 30.0 },
                    Some(graphics::WHITE),
                );
            }
        }

        Ok(())
    }

    fn input(&mut self, _world: &mut World, _e: input::Event, _started: bool) {}

    fn draw_previous(&self) -> bool {
        true
    }
}
