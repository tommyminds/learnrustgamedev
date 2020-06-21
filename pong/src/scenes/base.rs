use ggez::{graphics, Context, GameResult};
use ggez_extras::scene;
use specs::{Builder, Join, World, WorldExt};

use crate::*;

pub struct BaseScene {
    done: bool,
}

impl BaseScene {
    pub fn new(_ctx: &mut Context, world: &mut World) -> Self {
        world
            .create_entity()
            .with(components::Player {
                name: String::from("1"),
                side: Side::Left,
                score: 0,
            })
            .with(components::Position { x: 10.0, y: 30.0 })
            .with(components::Size { w: 5.0, h: 20.0 })
            .with(components::Scored(false))
            .with(components::Won(false))
            .with(components::Serving(true))
            .build();

        world
            .create_entity()
            .with(components::Player {
                name: String::from("2"),
                side: Side::Right,
                score: 0,
            })
            .with(components::Position {
                x: VIRTUAL_WIDTH - 10.0,
                y: VIRTUAL_HEIGHT - 50.0,
            })
            .with(components::Size { w: 5.0, h: 20.0 })
            .with(components::Scored(false))
            .with(components::Won(false))
            .with(components::Serving(false))
            .build();

        world
            .create_entity()
            .with(components::Ball {})
            .with(components::Position {
                x: VIRTUAL_WIDTH / 2.0 - 2.0,
                y: VIRTUAL_HEIGHT / 2.0 - 2.0,
            })
            .with(components::Size { w: 4.0, h: 4.0 })
            .with(components::Velocity::default())
            .build();

        Self { done: false }
    }

    fn draw_rects(&mut self, world: &World, ctx: &mut Context) -> GameResult<()> {
        for (pos, size) in (
            &world.read_storage::<components::Position>(),
            &world.read_storage::<components::Size>(),
        )
            .join()
        {
            let rect = graphics::Rect::new(pos.x, pos.y, size.w, size.h);
            let draw_rect = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                rect,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &draw_rect, graphics::DrawParam::default())?;
        }

        Ok(())
    }

    fn draw_scores(&mut self, world: &World, ctx: &mut Context) -> GameResult<()> {
        let font_resource = &world.read_resource::<GameFont>();

        for player in (&world.read_storage::<components::Player>()).join() {
            let score_display =
                graphics::Text::new((format!("{}", player.score), font_resource.font, 32.0));
            let pos = match player.side {
                Side::Left => mint::Point2 {
                    x: VIRTUAL_WIDTH / 2.0 - 50.0,
                    y: VIRTUAL_HEIGHT / 3.0,
                },
                Side::Right => mint::Point2 {
                    x: VIRTUAL_WIDTH / 2.0 + 30.0,
                    y: VIRTUAL_HEIGHT / 3.0,
                },
            };

            graphics::queue_text(ctx, &score_display, pos, Some(graphics::WHITE));
        }

        Ok(())
    }
}

impl scene::Scene<World, input::Event> for BaseScene {
    fn name(&self) -> &str {
        "BaseScene"
    }

    fn update(&mut self, _world: &mut World, _ctx: &mut Context) -> scenes::Switch {
        if self.done {
            scenes::Switch::Pop
        } else {
            scenes::Switch::None
        }
    }

    fn draw(&mut self, world: &World, ctx: &mut Context) -> GameResult<()> {
        self.draw_rects(&world, ctx)?;
        self.draw_scores(&world, ctx)?;

        Ok(())
    }
}
