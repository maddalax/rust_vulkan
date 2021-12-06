use winit::event::KeyboardInput;

use crate::event::{EngineChange, InputObserver, UpdateObserver};
use crate::State;

pub struct CameraKeyListener {}

impl InputObserver for CameraKeyListener {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut State) {
        state.camera_controller.process_events(input);
    }
}

impl UpdateObserver for CameraKeyListener {
    fn on_update(&mut self, state: &mut State) {}
}
