use crate::data::{CUBE, CUBE_INDICES, TRIANGLE, TRIANGLE_INDICES};
use crate::render::instance::InstanceRaw;
use crate::render::lib::Vertex;
use crate::RenderState;
use std::mem;
use wgpu::BufferAddress;

pub fn on_update(state: &mut RenderState) {
    state.queue.write_buffer(
        &state.camera_buffer,
        0,
        bytemuck::cast_slice(&[state.camera.uniform]),
    );

    state
        .queue
        .write_buffer(&state.vertex_buffer, 0, bytemuck::cast_slice(CUBE));

    state.queue.write_buffer(
        &state.vertex_buffer,
        (CUBE.len() * mem::size_of::<Vertex>()) as BufferAddress,
        bytemuck::cast_slice(TRIANGLE),
    );

    state
        .queue
        .write_buffer(&state.index_buffer, 0, bytemuck::cast_slice(CUBE_INDICES));
    state.queue.write_buffer(
        &state.index_buffer,
        (CUBE_INDICES.len() * mem::size_of::<u16>()) as BufferAddress,
        bytemuck::cast_slice(TRIANGLE_INDICES),
    );

    while let Some(index) = state.instance_handler.instance_changes.pop() {
        let instance = state.instance_handler.instances.get(index).unwrap();
        let raw = instance.to_raw();
        state.queue.write_buffer(
            &state.instance_buffer,
            (index * mem::size_of::<InstanceRaw>()) as BufferAddress,
            bytemuck::cast_slice(&[raw]),
        );
    }
}
