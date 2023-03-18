use glam::{Vec3, Mat4};

pub struct Camera {
    pub aspect: f32,
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
    pub znear: f32, 
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> glam::Mat4 {
        let view = glam::f32::Mat4::look_at_rh(self.eye, self.target, self.up);
        let fov_radians = self.fovy / 180.0 * std::f32::consts::PI;
        let projection = glam::Mat4::perspective_rh(fov_radians, self.aspect, self.znear, self.zfar);

        projection * view
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera { 
            aspect: 16.0/9.0, 
            eye: (0.0, 0.0, 4.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vec3::Y,
            fovy: 90.0,
            znear: 0.1,
            zfar: 100.0,
        } 
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_projection: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}

