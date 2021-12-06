use cgmath::{Quaternion, Rotation3, Vector3, Zero};
use rand::Rng;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use crate::{Instance, MAX_INSTANCES, State};
use crate::event::{EngineChange, InputObserver, UpdateObserver};

pub struct TestListener {}

impl InputObserver for TestListener {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut State) {}
}

impl UpdateObserver for TestListener {
    fn on_update(&mut self, state: &mut State) {
        if state.key_state.is_pressed(&VirtualKeyCode::Z) {
            if state.instances.len() == MAX_INSTANCES as usize {
                println!("No more instances allowed.");
                return;
            }

            let mut rng = rand::thread_rng();

            let len = state.instances.len();

            for i in 0..1000 {
                state.instances.push(Instance {
                    position: Vector3 {
                        x: (rng.gen_range(0.0..250.0)),
                        y: (rng.gen_range(0.0..250.0)),
                        z: (rng.gen_range(0.0..250.0)),
                    },
                    rotation: Quaternion::from_angle_y(cgmath::Deg(2.0)),
                });
                state.instance_updates.push_back(state.instances.len() - 1);
            }

            println!("Instances {}", state.instances.len());
        }

        if state.key_state.is_pressed(&VirtualKeyCode::X) {
            state.instances[0].position.x -= 0.10;
            state.instance_updates.push_back(0);
        }

        if state.key_state.is_pressed(&VirtualKeyCode::C) {
            state.instances[0].position.x += 0.10;
            state.instance_updates.push_back(0);
        }
    }
}
