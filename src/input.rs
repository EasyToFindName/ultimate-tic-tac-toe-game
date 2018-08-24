use piston_window::*;

pub enum InputMapperEvent {
    MousePressed(MouseButton, f64, f64),
    None
}

pub struct InputMapper {
    mouse_pos: [f64; 2],
}

impl InputMapper {
    pub fn new() -> Self {
        InputMapper{ mouse_pos: [0.0; 2] }
    }

    pub fn process_event(&mut self, e: &Event) -> InputMapperEvent {
        if let Some(args) = e.mouse_cursor_args() {
            self.mouse_pos = args;
            return InputMapperEvent::None; // TODO: return mouse moved event
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            return InputMapperEvent::MousePressed(button, self.mouse_pos[0], self.mouse_pos[1]);
        }

        InputMapperEvent::None
    }
}