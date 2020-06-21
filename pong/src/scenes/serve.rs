use ggez::{graphics, Context, GameResult};
use ggez_extras::scene;
use rand::Rng;

use specs::{Join, World};

use crate::*;

pub struct ServeScene {}

impl ServeScene {
    pub fn new(_ctx: &mut Context, world: &mut World) -> Self {
        let mut rng = rand::thread_rng();

        for (_, vel, pos) in (
            &world.read_storage::<components::Ball>(),
            &mut world.write_storage::<components::Velocity>(),
            &mut world.write_storage::<components::Position>(),
        )
            .join()
        {
            pos.x = VIRTUAL_WIDTH / 2.0 - 2.0;
            pos.y = VIRTUAL_HEIGHT / 2.0 - 2.0;

            for (player, serving) in (
                &world.read_storage::<components::Player>(),
                &world.read_storage::<components::Serving>(),
            )
                .join()
            {
                if serving.0 {
                    vel.x = match player.side {
                        types::Side::Left => rng.gen_range(180.0, 220.0),
                        types::Side::Right => -rng.gen_range(180.0, 220.0),
                    }
                }
            }
            vel.y = rng.gen_range(-50.0, 50.0);
        }

        Self {}
    }
}

impl scene::Scene<World, input::Event> for ServeScene {
    fn name(&self) -> &str {
        "ServeScene"
    }

    fn update(&mut self, world: &mut World, ctx: &mut Context) -> scenes::Switch {
        if world
            .read_resource::<input::State>()
            .get_button_released(input::Button::Enter)
        {
            for (_, scored, won, serving) in (
                &world.read_storage::<components::Player>(),
                &mut world.write_storage::<components::Scored>(),
                &mut world.write_storage::<components::Won>(),
                &mut world.write_storage::<components::Serving>(),
            )
                .join()
            {
                scored.0 = false;
                won.0 = false;
                serving.0 = false;
            }

            scenes::Switch::Replace(Box::new(scenes::PlayScene::new(ctx, world)))
        } else {
            scenes::Switch::None
        }
    }

    fn draw(&mut self, world: &World, ctx: &mut Context) -> GameResult<()> {
        let font_resource = &world.read_resource::<Fonts>();
        let bounds = mint::Point2 {
            x: VIRTUAL_WIDTH,
            y: f32::INFINITY,
        };

        for (player, serving) in (
            &world.read_storage::<components::Player>(),
            &world.read_storage::<components::Serving>(),
        )
            .join()
        {
            if serving.0 {
                let mut text = graphics::Text::new((
                    format!("Player {} to serve!\nPress Enter to serve!", player.name),
                    font_resource.retro,
                    10.0,
                ));

                text.set_bounds(bounds, graphics::Align::Center);

                graphics::queue_text(
                    ctx,
                    &text,
                    mint::Point2 { x: 0.0, y: 10.0 },
                    Some(graphics::WHITE),
                );
            }
        }

        Ok(())
    }
}
