use winit::event::{ElementState, KeyboardInput};

use crate::event::InputObserver;
use crate::state::State;

pub struct KeyMapListener {}

impl InputObserver for KeyMapListener {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut State) {
        let code = input.virtual_keycode.unwrap();
        let pressed = input.state == ElementState::Pressed;
        state.key_state.on_key_change(code, pressed);
    }
}