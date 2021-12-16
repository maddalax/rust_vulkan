use crate::event::UpdateObserver;
use crate::state::State;

pub struct CameraListener {}

impl UpdateObserver for CameraListener {
    fn on_update(&mut self, state: &mut State) {
        state.camera_controller.update_camera(&mut state.camera);
        state.camera.update();
    }
}
