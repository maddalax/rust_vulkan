use winit::event::KeyboardInput;

use crate::event::{InputObserver, UpdateObserver};
use crate::RenderState;

pub struct CameraKeyListener {}

impl InputObserver for CameraKeyListener {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut RenderState) {
        state.camera_controller.process_events(input);
    }
}

impl UpdateObserver for CameraKeyListener {
    fn on_update(&mut self, _state: &mut RenderState) {}
}
