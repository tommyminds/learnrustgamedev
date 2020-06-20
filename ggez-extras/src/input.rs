use ggez::event::KeyCode;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum InputType {
    KeyEvent(KeyCode),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputEffect<Button>
where
    Button: Eq + Hash + Clone,
{
    Button(Button),
}

#[derive(Debug, Copy, Clone, Default)]
struct ButtonState {
    pressed: bool,
    pressed_last_frame: bool,
}

pub struct InputBinding<Button>
where
    Button: Hash + Eq + Clone,
{
    bindings: HashMap<InputType, InputEffect<Button>>,
}

impl<Button> InputBinding<Button>
where
    Button: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        InputBinding {
            bindings: HashMap::new(),
        }
    }

    /// Adds a key binding connecting the given keycode to the given
    /// logical button.
    pub fn bind_key_to_button(mut self, keycode: KeyCode, button: Button) -> Self {
        self.bindings.insert(
            InputType::KeyEvent(keycode),
            InputEffect::Button(button.clone()),
        );
        self
    }

    /// Takes an physical input type and turns it into a logical input type (keycode -> button/button).
    pub fn resolve(&self, keycode: KeyCode) -> Option<InputEffect<Button>> {
        self.bindings.get(&InputType::KeyEvent(keycode)).cloned()
    }
}

#[derive(Debug)]
pub struct InputState<Button>
where
    Button: Hash + Eq + Clone,
{
    buttons: HashMap<Button, ButtonState>,
}

impl<Button> InputState<Button>
where
    Button: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        InputState {
            buttons: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        for (_button, button_status) in self.buttons.iter_mut() {
            button_status.pressed_last_frame = button_status.pressed;
        }
    }

    /// This method should get called by your key_down_event handler.
    pub fn update_button_down(&mut self, button: Button) {
        self.update_effect(InputEffect::Button(button), true);
    }

    /// This method should get called by your key_up_event handler.
    pub fn update_button_up(&mut self, button: Button) {
        self.update_effect(InputEffect::Button(button), false);
    }

    /// Takes an InputEffect and actually applies it.
    pub fn update_effect(&mut self, effect: InputEffect<Button>, started: bool) {
        match effect {
            InputEffect::Button(button) => {
                let f = || ButtonState::default();
                let button_status = self.buttons.entry(button).or_insert_with(f);
                button_status.pressed = started;
            }
        }
    }

    fn get_button(&self, button: Button) -> ButtonState {
        let d = ButtonState::default();
        let button_status = self.buttons.get(&button).unwrap_or(&d);
        *button_status
    }

    pub fn get_button_down(&self, button: Button) -> bool {
        self.get_button(button).pressed
    }

    pub fn get_button_up(&self, button: Button) -> bool {
        !self.get_button(button).pressed
    }

    /// Returns whether or not the button was pressed this frame,
    /// only returning true if the press happened this frame.
    ///
    /// Basically, `get_button_down()` and `get_button_up()` are level
    /// triggers, this and `get_button_released()` are edge triggered.
    pub fn get_button_pressed(&self, button: Button) -> bool {
        let b = self.get_button(button);
        b.pressed && !b.pressed_last_frame
    }

    pub fn get_button_released(&self, button: Button) -> bool {
        let b = self.get_button(button);
        !b.pressed && b.pressed_last_frame
    }

    pub fn reset_input_state(&mut self) {
        for (_button, button_status) in self.buttons.iter_mut() {
            button_status.pressed = false;
            button_status.pressed_last_frame = false;
        }
    }
}
