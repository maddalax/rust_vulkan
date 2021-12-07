use cgmath::{Quaternion, Rotation3, Vector3};
use rand::Rng;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use crate::event::InputObserver;
use crate::Instance;
use crate::instance::InstanceType;
use crate::state::State;

pub struct KeyMapListener {}

impl InputObserver for KeyMapListener {
    fn on_input_change(&mut self, input: &KeyboardInput, state: &mut State) {
        if input.virtual_keycode.is_none() {
            return;
        }
        let code = input.virtual_keycode.unwrap();
        let pressed = input.state == ElementState::Pressed;
        state.key_state.on_key_change(code, pressed);

        let mut rng = rand::thread_rng();

        if code == VirtualKeyCode::Space {
            for i in 0..100 {
                state.instance_handler.add(Instance {
                    instance_type: InstanceType::Cube,
                    position: Vector3 {
                        x: (rng.gen_range(0.0..100.0)),
                        y: (rng.gen_range(0.0..100.0)),
                        z: (rng.gen_range(0.0..100.0)),
                    },
                    rotation: Quaternion::from_angle_y(cgmath::Deg(2.0)),
                    start_offset: 0,
                    array_index: 0,
                });
                state.instance_handler.add(Instance {
                    instance_type: InstanceType::Triangle,
                    position: Vector3 {
                        x: (rng.gen_range(0.0..100.0)),
                        y: (rng.gen_range(0.0..100.0)),
                        z: (rng.gen_range(0.0..100.0)),
                    },
                    rotation: Quaternion::from_angle_y(cgmath::Deg(2.0)),
                    start_offset: 0,
                    array_index: 0,
                });
            }
        }
    }
}