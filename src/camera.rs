#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

pub const A_DIGIT: f32 = 1.0;
pub const B_DIGIT: f32 = 9.0;
pub const FOV: f32 = 1.0;
pub const Z_NEAR: f32 = 0.1;
pub const Z_FAR: f32 = 100.0;
pub const F: f32 = -5.0;
pub const CENTER: f32 = 10.0;
