use std::iter;
use crate::data::{CUBE, CUBE_INDICES, TRIANGLE_INDICES};
use crate::render::instance::InstanceType;
use crate::RenderState;

pub fn on_render(state: &mut RenderState) -> Result<(), wgpu::SurfaceError> {
    let output = state.surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = state
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&state.render_pipeline); // 2.

        render_pass.set_bind_group(0, &state.camera_bind_group, &[]);

        render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, state.instance_buffer.slice(..));

        render_pass.set_index_buffer(state.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        let triangle_indices = TRIANGLE_INDICES.len() as u32;
        let cube_indices = CUBE_INDICES.len() as u32;
        let mut offset = 0;
        state.render_stats.draw_calls = 0;

        if state.instance_handler.instances.is_empty() {
            return Ok(());
        }

        for _ in 0..state.instance_handler.max_index {
            if offset > state.instance_handler.instances.len() - 1 {
                break;
            }

            let instance = state.instance_handler.instances.get(offset).unwrap();

            if instance.instance_type == InstanceType::Empty {
                offset += 1;
                continue;
            }

            let max_instances = instance.max_allowed;

            if instance.instance_type == InstanceType::Cube {
                state.render_stats.draw_calls += 1;
                render_pass.draw_indexed(
                    0..cube_indices,
                    0,
                    instance.start_offset as u32
                        ..(instance.start_offset + max_instances) as u32,
                ); // 3.
            } else {
                render_pass.draw_indexed(
                    cube_indices..cube_indices + triangle_indices,
                    (CUBE.len() as i32) - 1,
                    instance.start_offset as u32
                        ..(instance.start_offset + max_instances) as u32,
                ); // 3.
                state.render_stats.draw_calls += 1;
            }

            offset += instance.max_allowed;
        }
    }

    state.queue.submit(iter::once(encoder.finish()));
    output.present();

    println!(
        "Draw Calls: {}. Total Entities: {}",
        state.render_stats.draw_calls, state.instance_handler.total_added
    );

    Ok(())
}
