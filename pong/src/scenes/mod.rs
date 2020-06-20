use ggez_extras::scene;
use specs::World;

use crate::input::Event;

pub mod base;
pub mod play;
pub mod serve;
pub mod start;

pub use base::*;
pub use play::*;
pub use serve::*;
pub use start::*;

// Shortcuts for our scene type.
pub type Switch = scene::SceneSwitch<World, Event>;
pub type Stack = scene::SceneStack<World, Event>;
