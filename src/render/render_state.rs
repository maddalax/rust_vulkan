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
use crate::render::renderer::on_render;
use crate::render::updater::on_update;

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
        on_update(self);
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        on_render(self)
    }
}
