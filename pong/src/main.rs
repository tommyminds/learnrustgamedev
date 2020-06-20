mod components;
mod input;
mod scenes;
mod systems;
mod types;

use std::env;
use std::path;

use ggez::{self, *};
use ggez_extras::*;
use log::*;
use specs::*;

use types::*;

const DESIRED_FPS: u32 = 60;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 780.0;

const VIRTUAL_WIDTH: f32 = 432.0;
const VIRTUAL_HEIGHT: f32 = 243.0;

const PADDLE_SPEED: f32 = 200.0;

pub struct Game {
    world: World,
    scenes: scenes::Stack,
    input_binding: input::Binding,
}

impl Game {
    fn new(ctx: &mut Context, _resource_path: &path::Path) -> GameResult<Game> {
        let font = graphics::Font::new(ctx, "/fonts/font.ttf")?;

        let mut world = World::new();
        components::register(&mut world);
        world.insert(DeltaTime { delta: 0.0 });
        world.insert(GameFont { font });
        world.insert(input::State::new());

        let mut scenestack = scenes::Stack::new(ctx);
        let base_scene = Box::new(scenes::BaseScene::new(ctx, &mut world));
        let start_scene = Box::new(scenes::StartScene::new(ctx, &mut world));
        scenestack.push(base_scene);
        scenestack.push(start_scene);

        Ok(Self {
            world: world,
            scenes: scenestack,
            input_binding: input::create_input_binding(),
        })
    }

    fn draw_fps(&mut self, ctx: &mut Context) -> GameResult<()> {
        let font_resource = &self.world.read_resource::<GameFont>();

        let fps = timer::fps(ctx);
        let fps_display =
            graphics::Text::new((format!("FPS: {:.1}", fps), font_resource.font, 8.0));

        graphics::queue_text(
            ctx,
            &fps_display,
            mint::Point2 { x: 10.0, y: 10.0 },
            Some(graphics::WHITE),
        );

        Ok(())
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self
            .world
            .read_resource::<input::State>()
            .get_button_released(input::Button::Quit)
        {
            event::quit(ctx);
        }

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.world.write_resource::<DeltaTime>().delta = util::seconds(&timer::delta(ctx));
            self.scenes.update(&mut self.world, ctx);
            self.world.write_resource::<input::State>().update();
            self.world.maintain();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgba(40, 45, 52, 255));

        self.scenes.draw(&self.world, ctx);

        self.draw_fps(ctx)?;

        graphics::draw_queued_text(
            ctx,
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Nearest,
        )?;

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        repeat: bool,
    ) {
        if !repeat {
            if let Some(e) = self.input_binding.resolve(keycode) {
                info!("keydown: {:?}", e);
                self.world
                    .write_resource::<input::State>()
                    .update_effect(e, true);
                self.scenes.input(&mut self.world, e, true);
            }
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
    ) {
        if let Some(e) = self.input_binding.resolve(keycode) {
            info!("keyup: {:?}", e);
            self.world
                .write_resource::<input::State>()
                .update_effect(e, false);
            self.scenes.input(&mut self.world, e, false);
        }
    }
}

fn main() -> GameResult {
    logging::setup();

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    info!("Resource dir: {:?}", resource_dir);

    let cb = ContextBuilder::new("pong", "Tommy Maintz")
        .window_setup(conf::WindowSetup::default().title("Pong"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                .borderless(false),
        )
        .add_resource_path(&resource_dir);
    let (ctx, ev) = &mut cb.build().unwrap();

    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
    graphics::set_screen_coordinates(
        ctx,
        graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: VIRTUAL_WIDTH,
            h: VIRTUAL_HEIGHT,
        },
    )?;

    let state = &mut Game::new(ctx, &resource_dir)?;

    event::run(ctx, ev, state)
}
