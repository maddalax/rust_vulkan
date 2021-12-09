use std::ops::Mul;

use cgmath::{Quaternion, Rotation3, Vector3, Zero};
use rand::Rng;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use crate::{Instance, MAX_INSTANCES, State};
use crate::event::{EngineChange, InputObserver, UpdateObserver};
use crate::instance::InstanceType;

pub struct TestListener {}

impl InputObserver for TestListener {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut State) {}
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
