use std::sync::{Arc, Mutex};

use winit::dpi::{PhysicalSize, Size};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use render::instance::Instance;

use crate::event::{EventMatcher, EventSystem};
use crate::render::render_state::RenderState;
use crate::render::render_state_factory::create_render_state;

mod data;
mod event;
mod input;
mod listeners;
mod render;
mod rotation;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Vulkan Engine");
    window.set_inner_size(Size::Physical(PhysicalSize::new(1920, 1080)));

    let mut event_system = EventSystem::new();

    let test_listener = Arc::new(Mutex::new(listeners::test_listener::TestListener {}));
    let camera_key_listener = Arc::new(Mutex::new(
        listeners::camera_keyboard_listener::CameraKeyListener {},
    ));
    let key_map_listener = Arc::new(Mutex::new(listeners::key_map_listener::KeyMapListener {}));
    let camera_listener = Arc::new(Mutex::new(listeners::camera_listener::CameraListener {}));

    event_system.add_update_observer(test_listener);
    event_system.add_input_observer(camera_key_listener);
    event_system.add_input_observer(key_map_listener);
    event_system.add_update_observer(camera_listener);

    // State::new uses async code, so we're going to wait for it to finish
    let mut state: RenderState = pollster::block_on(create_render_state(&window));

    event_loop.run(move |event, _, control_flow| {
        let id = window.id();

        EventMatcher::on_event(&id, &event, &event_system, &mut state);

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == id => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        event_system.notify_keyboard_input(input, &mut state);
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
