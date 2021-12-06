use std::sync::{Arc, Mutex};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit::window::WindowId;

use crate::State;

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
    pub fn on_event(event_window_id: &WindowId, event: &Event<()>, event_system: &EventSystem, state: &mut State) {
        match event {
            Event::RedrawRequested(_) => {
                event_system.notify_update(state);
            }
            Event::WindowEvent {
                window_id,
                event,
            } if event_window_id == window_id => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        event_system.notify_keyboard_input(input, state);
                    }
                    _ => {}
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

    pub fn notify_update(&self, state: &mut State) {
        for observer in self.update_observers.clone() {
            let mut observer = observer.lock().unwrap();
            observer.on_update(state);
        }
    }

    pub fn notify_keyboard_input(&self, input: &KeyboardInput, state: &mut State) {
        for observer in self.input_observers.clone() {
            let mut observer = observer.lock().unwrap();
            observer.on_input_change(&input, state);
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
    fn on_update(&mut self, state: &mut State);
}

pub trait InputObserver {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut State);
}