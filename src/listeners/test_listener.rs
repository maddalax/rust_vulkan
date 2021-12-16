use winit::event::{KeyboardInput, VirtualKeyCode};

use crate::event::{InputObserver, UpdateObserver};
use crate::State;

pub struct TestListener {}

impl InputObserver for TestListener {
    fn on_input_change(&mut self, _input: &KeyboardInput, _state: &mut State) {}
}

impl UpdateObserver for TestListener {
    fn on_update(&mut self, state: &mut State) {
        if state.key_state.is_pressed(&VirtualKeyCode::C) {
            let mut instance = state.instance_handler.get(0);
            instance.position.x += 0.5;
            state.instance_handler.update(0);
        }

        if state.key_state.is_pressed(&VirtualKeyCode::X) {
            let mut instance = state.instance_handler.get(0);
            instance.position.x -= 0.5;
            state.instance_handler.update(0);
        }
    }
}
