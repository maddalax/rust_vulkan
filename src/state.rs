use std::{iter, mem};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::ops::{Index, IndexMut};
use std::sync::{Arc, Mutex};

use cgmath::{Point3, Quaternion, Vector3};
use cgmath::prelude::*;
use rand::Rng;
use wgpu::BufferAddress;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{camera, camera_controller, instance, structs};
use crate::data::{CUBE, CUBE_INDICES, TRIANGLE, TRIANGLE_INDICES};
use crate::event::{EngineChange, EngineEvent, EventSystem};
use crate::instance::{Instance, InstanceRaw, INSTANCES_PER_CHUNK, InstanceType, MAX_INSTANCES};
use crate::structs::Vertex;

pub struct KeyState {
    state: HashSet<VirtualKeyCode>,
}

impl KeyState {
    fn new() -> Self {
        KeyState {
            state: HashSet::new()
        }
    }

    pub(crate) fn is_pressed(&self, key: &VirtualKeyCode) -> bool {
        return self.state.get(key).is_some();
    }
    pub(crate) fn on_key_change(&mut self, key: VirtualKeyCode, pressed: bool) {
        if pressed {
            self.state.insert(key);
        } else {
            self.state.remove(&key);
        }
    }
}


pub struct InstanceHandler {
    pub(crate) instances: Vec<instance::Instance>,
    pub(crate) instance_changes: Vec<usize>,
}

impl InstanceHandler {
    fn new() -> InstanceHandler {
        let mut instances = Vec::with_capacity(MAX_INSTANCES);
        for i in 0..instances.capacity() {
            instances.push(Instance {
                instance_type: InstanceType::Empty,
                position: Vector3 {
                    x: (0.0),
                    y: (0.0),
                    z: (0.0),
                },
                rotation: Quaternion::from_angle_y(cgmath::Deg(2.0)),
                start_offset: 0,
                array_index: 0,
            });
        }

        InstanceHandler {
            instances,
            instance_changes: Vec::new(),
        }
    }

    pub fn add(&mut self, mut instance: instance::Instance) {
        let offsets = self.find_offset(instance.instance_type.clone());
        instance.array_index = offsets.0;
        instance.start_offset = offsets.1;

        let mut o = instance.start_offset;
        if o == 0 {
            o = 1
        }

        if instance.array_index > INSTANCES_PER_CHUNK * o {
            return;
        }

        std::mem::replace(&mut self.instances[instance.array_index], instance);
        self.instance_changes.push(offsets.0);
    }

    fn find_offset(&self, instance_type: InstanceType) -> (usize, usize) {
        if self.instances.len() == 0 {
            return (0, 0);
        }
        let mut offset = 0;
        loop {
            if self.instances.len() < offset + 1 {
                return (offset, offset);
            }
            if self.instances[offset].instance_type as i32 == instance_type as i32 || self.instances[offset].instance_type == InstanceType::Empty {
                let mut array_index = offset;
                loop {
                    let at_index = self.instances.get(array_index);
                    if at_index.is_none() || at_index.unwrap().instance_type == InstanceType::Empty {
                        return (array_index, offset);
                    }
                    array_index += 1;
                }
            }
            offset += INSTANCES_PER_CHUNK;
        }
    }
}

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,
    pub(crate) camera: camera::Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    pub(crate) camera_controller: camera_controller::CameraController,
    pub(crate) instance_handler: InstanceHandler,
    pub(crate) instance_updates: VecDeque<usize>,
    instance_buffer: wgpu::Buffer,
    pub(crate) key_state: KeyState,
    pub(crate) draw_cube: bool,
}


impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let mut camera = camera::Camera {
            eye: Point3 {
                x: (25.0),
                y: (25.0),
                z: (45.0),
            },
            target: Point3 {
                x: (0.0),
                y: (0.0),
                z: (0.0),
            },
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            ..camera::Camera::default()
        };

        camera.update();

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera.uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });


        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &camera_bind_group_layout
                ],
                push_constant_ranges: &[],
            });

        let vertex_data = vec![0; mem::size_of::<Vertex>() * MAX_INSTANCES];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_data = vec![0; mem::size_of::<u16>() * MAX_INSTANCES];

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&index_data),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    crate::structs::Vertex::desc(),
                    instance::InstanceRaw::desc()
                ], // 2.
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLAMPING
                clamp_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
        });

        let num_vertices = CUBE.len() as u32;
        let num_indices = TRIANGLE_INDICES.len() as u32;

        let camera_controller = camera_controller::CameraController::new(1.0);

        let instance_data = vec![0; mem::size_of::<InstanceRaw>() * MAX_INSTANCES];

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mut instance_updates = VecDeque::new();

        let key_state = KeyState::new();

        let instance_handler = InstanceHandler::new();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            num_vertices,
            num_indices,
            vertex_buffer,
            index_buffer,
            camera,
            camera_bind_group,
            camera_buffer,
            camera_controller,
            instance_buffer,
            instance_updates,
            key_state,
            instance_handler,
            draw_cube: true,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub(crate) fn update(&mut self) {
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera.uniform]));

        self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(CUBE));

        self.queue.write_buffer(&self.vertex_buffer, (CUBE.len() * mem::size_of::<Vertex>()) as BufferAddress, bytemuck::cast_slice(TRIANGLE));

        self.queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(CUBE_INDICES));
        self.queue.write_buffer(&self.index_buffer, (CUBE_INDICES.len() * mem::size_of::<u16>()) as BufferAddress, bytemuck::cast_slice(TRIANGLE_INDICES));

        while let Some(index) = self.instance_handler.instance_changes.pop() {
            let instance = self.instance_handler.instances.get(index).unwrap();
            let raw = instance.to_raw();
            //             self.queue.write_buffer(&self.instance_buffer, (mem::size_of::<InstanceRaw>() * offset as usize) as BufferAddress, bytemuck::cast_slice(&[raw]));
            self.queue.write_buffer(&self.instance_buffer, (index * mem::size_of::<InstanceRaw>()) as BufferAddress, bytemuck::cast_slice(&[raw]));
        }

        // while !update.is_none() {
        //     let index = update.unwrap();
        //     let raw = self.instance_handler.instances[index].to_raw();
        //     update = self.instance_updates.pop_back();
        // }
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

            // if self.draw_cube {
            //     render_pass.draw_indexed(0..cube_indices, 0, 0..self.instances.len() as _); // 3.
            // } else {
            //     render_pass.draw_indexed(cube_indices..cube_indices + triangle_indices, (CUBE.len() as i32) - 1, 0..1); // 3.
            // }

            for i in 0..MAX_INSTANCES {
                if i % INSTANCES_PER_CHUNK == 0 {
                    let instance = self.instance_handler.instances.get(i);
                    if instance.is_some() {
                        let unwrapped = instance.unwrap();

                        if unwrapped.instance_type == InstanceType::Empty {
                            break;
                        }

                        if unwrapped.instance_type == InstanceType::Cube {
                            render_pass.draw_indexed(0..cube_indices, 0, unwrapped.start_offset as u32..(unwrapped.start_offset + INSTANCES_PER_CHUNK) as u32); // 3.
                        } else {
                            render_pass.draw_indexed(cube_indices..cube_indices + triangle_indices, (CUBE.len() as i32) - 1, unwrapped.start_offset as u32..(unwrapped.start_offset + INSTANCES_PER_CHUNK) as u32); // 3.
                        }
                    }
                }
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}