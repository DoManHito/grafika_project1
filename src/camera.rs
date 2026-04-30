#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

// Stale z numeru albumu 2175|1|9|
pub const A_DIGIT: f32 = 1.0;
pub const B_DIGIT: f32 = 9.0;
// Rozmiar okna
pub const WIDTH: u32 = 640;
pub const HEIGHT: u32 = 480;
// Stale sluzace do definiowania odleglasci renderowanych objektow
pub const Z_NEAR: f32 = 0.1;
pub const Z_FAR: f32 = 100.0;
// Stala ogniskowa
pub const F: f32 = 5.0;
// Stala do zazadzania predkoscia
pub const SPEED_MULTIPLIER: f32 = 1.0;
// Ostatnia klatka
pub const FRAME_LIMIT: u32 = 180;
// Pierwsza klatka
pub const FRAME_START: u32 = 0;
// Celowe klatki na sekunde
pub const TARGET_FPS: f64 = 60.0;
pub const FRAME_DURATION: f64 = 1.0 / TARGET_FPS;

