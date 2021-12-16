use std::collections::HashSet;
use winit::event::VirtualKeyCode;

pub struct KeyState {
    state: HashSet<VirtualKeyCode>,
}

impl KeyState {
    pub(crate) fn new() -> Self {
        KeyState {
            state: HashSet::new(),
        }
    }

    pub(crate) fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
        return self.state.get(key).is_some();
    }
    pub(crate) fn on_key_change(&mut self, key: VirtualKeyCode, pressed: bool) {
        if pressed {
            self.state.insert(key);
        } else {
            self.state.remove(&key);
        }
    }
}
