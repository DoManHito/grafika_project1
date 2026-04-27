#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 4],
    color_front: [f32; 4],
    color_back: [f32; 4],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 2 * std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub const VERTICES_A_ZAD1: &[Vertex] = &[
    Vertex {
        position: [2.0, 0.5, 10.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 0
    Vertex {
        position: [-1.0, -1.5, 12.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 1
    Vertex {
        position: [-1.0, 1.5, 8.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 2
    Vertex {
        position: [0.8, 0.3, 10.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 3
    Vertex {
        position: [-0.25, -0.25, 10.5, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 4
    Vertex {
        position: [-0.25, 0.5, 9.5, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 5
];

pub const INDICES_A_ZAD1: &[u16] = &[0, 1, 3, 1, 4, 3, 1, 2, 4, 2, 5, 4, 0, 5, 2, 0, 3, 5];

pub const VERTICES_B_ZAD1: &[Vertex] = &[
    Vertex {
        position: [0.5, -2.0, 10.0, 1.0],
        color_front: BLUE,
        color_back: YELLOW,
    }, // 0
    Vertex {
        position: [-0.5, 2.0, 11.0, 1.0],
        color_front: BLUE,
        color_back: YELLOW,
    }, // 1
    Vertex {
        position: [1.5, 2.0, 9.0, 1.0],
        color_front: BLUE,
        color_back: YELLOW,
    }, // 2
];

pub const INDICES_B_ZAD1: &[u16] = &[0, 1, 2];

pub const VERTICES_A_ZAD2: &[Vertex] = &[
    Vertex {
        position: [2.0, 0.0, 0.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 0
    Vertex {
        position: [-1.0, -2.0, 2.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 1
    Vertex {
        position: [-1.0, 1.0, -2.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 2
    Vertex {
        position: [0.8, -0.2, 0.0, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 3
    Vertex {
        position: [-0.25, -0.75, 0.5, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 4
    Vertex {
        position: [-0.25, 0.0, -1.5, 1.0],
        color_front: RED,
        color_back: GREEN,
    }, // 5
];

pub const INDICES_A_ZAD2: &[u16] = &[0, 1, 3, 1, 4, 3, 1, 2, 4, 2, 5, 4, 0, 5, 2, 0, 3, 5];

pub const VERTICES_B_ZAD2: &[Vertex] = &[
    Vertex {
        position: [0.0, -2.0, 0.0, 1.0],
        color_front: BLUE,
        color_back: YELLOW,
    }, // 0
    Vertex {
        position: [-1.0, 2.0, 1.0, 1.0],
        color_front: BLUE,
        color_back: YELLOW,
    }, // 1
    Vertex {
        position: [1.0, 2.0, -1.0, 1.0],
        color_front: BLUE,
        color_back: YELLOW,
    }, // 2
];

pub const INDICES_B_ZAD2: &[u16] = &[0, 1, 2];

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
