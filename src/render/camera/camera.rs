use crate::render::lib::OPENGL_TO_WGPU_MATRIX;
use cgmath::{Deg, Point3, SquareMatrix, Vector3};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct Camera {
    pub(crate) eye: cgmath::Point3<f32>,
    pub(crate) target: cgmath::Point3<f32>,
    pub(crate) up: cgmath::Vector3<f32>,
    pub(crate) aspect: f32,
    pub(crate) fovy: f32,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
    pub(crate) model_rotation: cgmath::Deg<f32>,
    pub(crate) uniform: CameraUniform,
}

impl Camera {
    fn build_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        OPENGL_TO_WGPU_MATRIX * proj * view * cgmath::Matrix4::from_angle_z(self.model_rotation)
    }

    pub fn update(&mut self) {
        self.uniform.view_proj = (self.build_projection_matrix()
            * cgmath::Matrix4::from_angle_z(self.model_rotation))
        .into()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            eye: Point3 {
                x: (0.0),
                y: (0.0),
                z: (0.0),
            },
            target: Point3 {
                x: (0.0),
                y: (0.0),
                z: (0.0),
            },
            up: Vector3 {
                x: (0.0),
                y: (0.0),
                z: (0.0),
            },
            aspect: 0.0,
            fovy: 0.0,
            znear: 0.0,
            zfar: 0.0,
            model_rotation: Deg(0.0),
            uniform: CameraUniform {
                view_proj: cgmath::Matrix4::identity().into(),
            },
        }
    }
}
