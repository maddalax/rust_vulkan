use cgmath::prelude::*;

use std::{iter, mem};

use wgpu::BufferAddress;
use winit::event::*;

use crate::data::{CUBE, CUBE_INDICES, TRIANGLE, TRIANGLE_INDICES};
use crate::input::key_state::KeyState;
use crate::render::camera::{camera, camera_controller};

use crate::render::instance::{InstanceRaw, InstanceType};
use crate::render::instance_handler::InstanceHandler;
use crate::render::lib::{RenderStats, Vertex};

pub struct RenderState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub camera: camera::Camera,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_controller: camera_controller::CameraController,
    pub instance_handler: InstanceHandler,
    pub instance_buffer: wgpu::Buffer,
    pub(crate) key_state: KeyState,
    pub(crate) render_stats: RenderStats,
}

impl RenderState {
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub(crate) fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub(crate) fn update(&mut self) {
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera.uniform]),
        );

        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(CUBE));

        self.queue.write_buffer(
            &self.vertex_buffer,
            (CUBE.len() * mem::size_of::<Vertex>()) as BufferAddress,
            bytemuck::cast_slice(TRIANGLE),
        );

        self.queue
            .write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(CUBE_INDICES));
        self.queue.write_buffer(
            &self.index_buffer,
            (CUBE_INDICES.len() * mem::size_of::<u16>()) as BufferAddress,
            bytemuck::cast_slice(TRIANGLE_INDICES),
        );

        while let Some(index) = self.instance_handler.instance_changes.pop() {
            let instance = self.instance_handler.instances.get(index).unwrap();
            let raw = instance.to_raw();
            self.queue.write_buffer(
                &self.instance_buffer,
                (index * mem::size_of::<InstanceRaw>()) as BufferAddress,
                bytemuck::cast_slice(&[raw]),
            );
        }
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
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

            render_pass.set_pipeline(&self.render_pipeline); // 2.

            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            let triangle_indices = TRIANGLE_INDICES.len() as u32;
            let cube_indices = CUBE_INDICES.len() as u32;
            let mut offset = 0;
            self.render_stats.draw_calls = 0;

            if self.instance_handler.instances.is_empty() {
                return Ok(());
            }

            for _ in 0..self.instance_handler.max_index {
                if offset > self.instance_handler.instances.len() - 1 {
                    break;
                }

                let instance = self.instance_handler.instances.get(offset).unwrap();

                if instance.instance_type == InstanceType::Empty {
                    offset += 1;
                    continue;
                }

                let max_instances = instance.max_allowed;

                if instance.instance_type == InstanceType::Cube {
                    self.render_stats.draw_calls += 1;
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
                    self.render_stats.draw_calls += 1;
                }

                offset += instance.max_allowed;
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        println!(
            "Draw Calls: {}. Total Entities: {}",
            self.render_stats.draw_calls, self.instance_handler.total_added
        );

        Ok(())
    }
}
