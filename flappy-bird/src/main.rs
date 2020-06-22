use std::env;
use std::path;

use ggez::{
    audio, audio::SoundSource, conf, event, graphics, mint, timer, Context, ContextBuilder,
    GameResult,
};
use ggez_extras::{logging, util};
use log::info;
use specs::{Builder, World, WorldExt};

mod components;
mod input;
mod scenes;
mod systems;
mod types;

use types::*;

const DESIRED_UPS: u32 = 60;
const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 780.0;

const VIRTUAL_WIDTH: f32 = 512.0;
const VIRTUAL_HEIGHT: f32 = 288.0;

const BACKGROUND_SCROLL_SPEED: f32 = 30.0;
const GROUND_SCROLL_SPEED: f32 = 60.0;

const BACKGROUND_LOOPING_POINT: f32 = 568.0;

const PIPE_SPEED: f32 = 60.0;
const PIPE_WIDTH: f32 = 70.0;
const PIPE_HEIGHT: f32 = 430.0;

const BIRD_WIDTH: f32 = 38.0;
const BIRD_HEIGHT: f32 = 24.0;

pub struct Game {
    world: World,
    scenes: scenes::SceneStack,
    input_binding: input::Binding,
}

impl Game {
    fn new(ctx: &mut Context, _resource_path: &path::Path) -> GameResult<Game> {
        let mut world = World::new();
        components::register(&mut world);

        let render_system = systems::RenderSystem::new(&mut world);

        let images = Images {
            background: graphics::Image::new(ctx, "/images/background.png")?,
            bird: graphics::Image::new(ctx, "/images/bird.png")?,
            ground: graphics::Image::new(ctx, "/images/ground.png")?,
            pipe: graphics::Image::new(ctx, "/images/pipe.png")?,
        };

        let mut fonts = Fonts::new();
        fonts.insert(
            FontType::Retro,
            graphics::Font::new(ctx, "/fonts/retro.ttf")?,
        );
        fonts.insert(
            FontType::Flappy,
            graphics::Font::new(ctx, "/fonts/flappy.ttf")?,
        );

        // We use a fixed DeltaTime for all our systems
        world.insert(DeltaTime {
            delta: 1.0 / DESIRED_UPS as f32,
        });
        world.insert(fonts);
        world.insert(images.clone());
        world.insert(render_system);
        world.insert(input::State::new());
        world.insert(Sounds {
            explosion: audio::Source::new(ctx, "/sounds/explosion.wav")?,
            hurt: audio::Source::new(ctx, "/sounds/hurt.wav")?,
            jump: audio::Source::new(ctx, "/sounds/jump.wav")?,
            score: audio::Source::new(ctx, "/sounds/score.wav")?,
        });

        world
            .create_entity()
            .with(components::Render { visible: true })
            .with(components::Image {
                image: images.background,
            })
            .with(components::Position {
                x: 0.0,
                y: 0.0,
                z: 0,
            })
            .with(components::Size {
                w: VIRTUAL_WIDTH,
                h: VIRTUAL_HEIGHT,
            })
            .with(components::Parallax {
                speed: BACKGROUND_SCROLL_SPEED,
                looping_point: BACKGROUND_LOOPING_POINT,
            })
            .build();

        let ground_img = graphics::Image::new(ctx, "/images/ground.png")?;
        let ground_dims = ground_img.dimensions();
        world
            .create_entity()
            .with(components::Render { visible: true })
            .with(components::Position {
                x: 0.0,
                y: VIRTUAL_HEIGHT - ground_dims.h,
                z: 0,
            })
            .with(components::Image {
                image: images.ground,
            })
            .with(components::Size {
                w: VIRTUAL_WIDTH,
                h: VIRTUAL_HEIGHT,
            })
            .with(components::Parallax {
                speed: GROUND_SCROLL_SPEED,
                looping_point: BACKGROUND_LOOPING_POINT,
            })
            .build();

        let mut scenestack = scenes::SceneStack::new(ctx);
        let title_scene = Box::new(scenes::TitleScene::new(ctx, &mut world));
        scenestack.push(title_scene, &mut world);

        Ok(Self {
            world,
            scenes: scenestack,
            input_binding: input::create_input_binding(),
        })
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

        while timer::check_update_time(ctx, DESIRED_UPS) {
            self.scenes.update(&mut self.world, ctx);
            self.world.write_resource::<input::State>().update();
            self.world.maintain();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.world
            .write_resource::<systems::RenderSystem>()
            .run(ctx, &self.world)?;

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
                self.world
                    .write_resource::<input::State>()
                    .update_effect(e, true);
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
            self.world
                .write_resource::<input::State>()
                .update_effect(e, false);
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
        .window_setup(conf::WindowSetup::default().title("Fifty Bird"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                .resizable(true)
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
