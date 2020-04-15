use coffee::input::{self, keyboard, Input};
pub struct InputListener {
    pub pressed_keys: std::collections::HashSet<keyboard::KeyCode>,
}

impl Input for InputListener {
    fn new() -> InputListener {
        return InputListener {
            pressed_keys: std::collections::HashSet::new(),
        };
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::Input { state, key_code } => {
                    if state == input::ButtonState::Pressed {
                        self.pressed_keys.insert(key_code);
                    } else {
                        self.pressed_keys.remove(&key_code);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn clear(&mut self) {}
}
