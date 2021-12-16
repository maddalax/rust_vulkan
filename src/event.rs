use std::sync::{Arc, Mutex};

use crate::RenderState;
use winit::event::*;
use winit::window::WindowId;

pub enum EngineEvent {
    KeyPress,
}

pub struct EngineChange {
    pub(crate) event: EngineEvent,
}

pub struct EventSystem {
    update_observers: Vec<Arc<Mutex<dyn UpdateObserver>>>,
    input_observers: Vec<Arc<Mutex<dyn InputObserver>>>,
}

pub struct EventMatcher {}

impl EventMatcher {
    pub fn on_event(
        event_window_id: &WindowId,
        event: &Event<()>,
        event_system: &EventSystem,
        state: &mut RenderState,
    ) {
        match event {
            Event::RedrawRequested(_) => {
                event_system.notify_update(state);
            }
            Event::WindowEvent { window_id, event } if event_window_id == window_id => {
                if let WindowEvent::KeyboardInput { input, .. } = event {
                    event_system.notify_keyboard_input(input, state);
                }
            }
            _ => {}
        }
    }
}

impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem {
            update_observers: vec![],
            input_observers: vec![],
        }
    }

    pub fn notify_update(&self, state: &mut RenderState) {
        for observer in self.update_observers.clone() {
            let mut observer = observer.lock().unwrap();
            observer.on_update(state);
        }
    }

    pub fn notify_keyboard_input(&self, input: &KeyboardInput, state: &mut RenderState) {
        for observer in self.input_observers.clone() {
            let mut observer = observer.lock().unwrap();
            observer.on_input_change(input, state);
        }
    }

    pub fn add_update_observer(&mut self, observer: Arc<Mutex<dyn UpdateObserver>>) {
        self.update_observers.push(observer);
    }

    pub fn add_input_observer(&mut self, observer: Arc<Mutex<dyn InputObserver>>) {
        self.input_observers.push(observer);
    }
}

pub trait UpdateObserver {
    fn on_update(&mut self, state: &mut RenderState);
}

pub trait InputObserver {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut RenderState);
}
