use ggez::{Context, GameResult};

/// A command to change to a new scene, either by pushign a new one,
/// popping one or replacing the current scene (pop and then push).
pub enum SceneSwitch<C, E> {
    None,
    Push(Box<dyn Scene<C, E>>),
    Replace(Box<dyn Scene<C, E>>),
    Pop,
}

/// A trait for you to implement on a scene.
/// Defines the callbacks the scene uses:
/// a common context type `C`
pub trait Scene<C, E> {
    fn update(&mut self, world: &mut C, ctx: &mut Context) -> SceneSwitch<C, E>;
    fn draw(&mut self, world: &C, ctx: &mut Context) -> GameResult<()>;

    /// Only used for human-readable convenience (or not at all, tbh)
    fn name(&self) -> &str;

    /// This returns whether or not to draw the next scene down on the
    /// stack as well; this is useful for layers or GUI stuff that
    /// only partially covers the screen.
    fn draw_previous(&self) -> bool {
        false
    }
}

impl<C, E> SceneSwitch<C, E> {
    pub fn replace<S>(scene: S) -> Self
    where
        S: Scene<C, E> + 'static,
    {
        SceneSwitch::Replace(Box::new(scene))
    }

    pub fn push<S>(scene: S) -> Self
    where
        S: Scene<C, E> + 'static,
    {
        SceneSwitch::Push(Box::new(scene))
    }
}

/// A stack of `Scene`'s, together with a context object.
pub struct SceneStack<C, E> {
    scenes: Vec<Box<dyn Scene<C, E>>>,
}

impl<C, E> SceneStack<C, E> {
    pub fn new(_ctx: &mut Context) -> Self {
        Self { scenes: Vec::new() }
    }

    pub fn is_empty(&mut self) -> bool {
        self.scenes.len() == 0
    }

    /// Add a new scene to the top of the stack.
    pub fn push(&mut self, scene: Box<dyn Scene<C, E>>) {
        self.scenes.push(scene)
    }

    /// Remove the top scene from the stack and returns it;
    /// panics if there is none.
    pub fn pop(&mut self) -> Box<dyn Scene<C, E>> {
        self.scenes
            .pop()
            .expect("ERROR: Popped an empty scene stack.")
    }

    /// Returns the current scene; panics if there is none.
    pub fn current(&self) -> &dyn Scene<C, E> {
        &**self
            .scenes
            .last()
            .expect("ERROR: Tried to get current scene of an empty scene stack.")
    }

    /// Executes the given SceneSwitch command; if it is a pop or replace
    /// it returns `Some(old_scene)`, otherwise `None`
    pub fn switch(&mut self, next_scene: SceneSwitch<C, E>) -> Option<Box<dyn Scene<C, E>>> {
        match next_scene {
            SceneSwitch::None => None,
            SceneSwitch::Pop => {
                let s = self.pop();
                Some(s)
            }
            SceneSwitch::Push(s) => {
                self.push(s);
                None
            }
            SceneSwitch::Replace(s) => {
                let old_scene = self.pop();
                self.push(s);
                Some(old_scene)
            }
        }
    }

    // These functions must be on the SceneStack because otherwise
    // if you try to get the current scene and the world to call
    // update() on the current scene it causes a double-borrow.  :/
    pub fn update(&mut self, world: &mut C, ctx: &mut Context) {
        let next_scene = {
            let current_scene = &mut **self
                .scenes
                .last_mut()
                .expect("Tried to update empty scene stack");
            current_scene.update(world, ctx)
        };
        self.switch(next_scene);
    }

    /// We walk down the scene stack until we find a scene where we aren't
    /// supposed to draw the previous one, then draw them from the bottom up.
    ///
    /// This allows for layering GUI's and such.
    fn draw_scenes(scenes: &mut [Box<dyn Scene<C, E>>], world: &C, ctx: &mut Context) {
        assert!(scenes.len() > 0);
        if let Some((current, rest)) = scenes.split_last_mut() {
            if current.draw_previous() {
                SceneStack::draw_scenes(rest, world, ctx);
            }
            current
                .draw(world, ctx)
                .expect("I would hope drawing a scene never fails!");
        }
    }

    /// Draw the current scene.
    pub fn draw(&mut self, world: &C, ctx: &mut Context) {
        SceneStack::draw_scenes(&mut self.scenes, world, ctx)
    }
}
