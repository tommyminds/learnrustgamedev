#![allow(dead_code)]

use ggez::{Context, GameResult};
use specs::{Entity, World, WorldExt};

pub mod countdown;
pub mod play;
pub mod score;
pub mod title;

pub use countdown::*;
pub use play::*;
pub use score::*;
pub use title::*;

/// A command to change to a new scene, either by pushign a new one,
/// popping one or replacing the current scene (pop and then push).
pub enum SceneSwitch {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
}

/// A trait for you to implement on a scene.
pub trait Scene {
    fn update(&mut self, world: &mut World, ctx: &mut Context) -> SceneSwitch;
    fn on_enter(&mut self, world: &mut World) -> GameResult<Option<Vec<Entity>>>;
    fn on_leave(&mut self, world: &mut World) -> GameResult;
}

impl SceneSwitch {
    pub fn replace<S>(scene: S) -> Self
    where
        S: Scene + 'static,
    {
        SceneSwitch::Replace(Box::new(scene))
    }

    pub fn push<S>(scene: S) -> Self
    where
        S: Scene + 'static,
    {
        SceneSwitch::Push(Box::new(scene))
    }
}

/// A stack of `Scene`'s, together with a context object.
pub struct SceneStack {
    scenes: Vec<Box<dyn Scene>>,
    entities: Vec<Option<Vec<Entity>>>,
}

impl SceneStack {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            scenes: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.scenes.len() == 0
    }

    /// Add a new scene to the top of the stack.
    pub fn push(&mut self, mut scene: Box<dyn Scene>, world: &mut World) {
        self.entities
            .push(scene.on_enter(world).expect("error pushing scene"));
        self.scenes.push(scene);
    }

    /// Remove the top scene from the stack and returns it;
    /// panics if there is none.
    pub fn pop(&mut self, world: &mut World) -> Box<dyn Scene> {
        let mut scene = self
            .scenes
            .pop()
            .expect("ERROR: Popped an empty scene stack.");
        scene.on_leave(world).expect("error popping scene");

        if let Some(Some(scene_entities)) = self.entities.pop() {
            let _ = world.delete_entities(scene_entities.as_slice());
        }

        scene
    }

    /// Returns the current scene; panics if there is none.
    pub fn current(&self) -> &dyn Scene {
        &**self
            .scenes
            .last()
            .expect("ERROR: Tried to get current scene of an empty scene stack.")
    }

    /// Executes the given SceneSwitch command; if it is a pop or replace
    /// it returns `Some(old_scene)`, otherwise `None`
    pub fn switch(&mut self, next_scene: SceneSwitch, world: &mut World) -> Option<Box<dyn Scene>> {
        match next_scene {
            SceneSwitch::None => None,
            SceneSwitch::Pop => {
                let s = self.pop(world);
                Some(s)
            }
            SceneSwitch::Push(s) => {
                self.push(s, world);
                None
            }
            SceneSwitch::Replace(s) => {
                let old_scene = self.pop(world);
                self.push(s, world);
                Some(old_scene)
            }
        }
    }

    // These functions must be on the SceneStack because otherwise
    // if you try to get the current scene and the world to call
    // update() on the current scene it causes a double-borrow.  :/
    pub fn update(&mut self, world: &mut World, ctx: &mut Context) {
        let next_scene = {
            let current_scene = &mut **self
                .scenes
                .last_mut()
                .expect("Tried to update empty scene stack");
            current_scene.update(world, ctx)
        };

        self.switch(next_scene, world);
    }
}
