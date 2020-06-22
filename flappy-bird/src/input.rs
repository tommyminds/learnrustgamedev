//! Typedefs for input shortcuts.
use ggez::event::*;
use ggez_extras::input;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Enter,
    Quit,
    Space,
}

pub type Binding = input::InputBinding<Button>;
pub type State = input::InputState<Button>;

/// Create the default keybindings for our input state.
pub fn create_input_binding() -> input::InputBinding<Button> {
    input::InputBinding::new()
        .bind_key_to_button(KeyCode::Space, Button::Space)
        .bind_key_to_button(KeyCode::Return, Button::Enter)
        .bind_key_to_button(KeyCode::Escape, Button::Quit)
}
