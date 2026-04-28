use crate::{camera, vertices};
use std::{sync::Arc};
use wgpu::util::DeviceExt;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

pub struct State {
    config_zad1: bool,

    // Fps
    last_frame_inst: std::time::Instant,
    frame_count: u32,
    accum_time: f32,

    // Device
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    window: Arc<Window>,

    // Buffer
    render_pipeline: wgpu::RenderPipeline,

    vertex_a_buffer: wgpu::Buffer,
    vertex_b_buffer: wgpu::Buffer,

    index_a_buffer: wgpu::Buffer,
    index_b_buffer: wgpu::Buffer,

    num_indices_a: u32,
    num_indices_b: u32,

    frame_index: u32,
    depth_view: wgpu::TextureView,

    // Models
    model_a_buffer: wgpu::Buffer,
    model_b_buffer: wgpu::Buffer,

    model_a_bind_group: wgpu::BindGroup,
    model_b_bind_group: wgpu::BindGroup,

    // Camera
    camera_uniform: camera::CameraUniform,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: wgpu::Buffer,
}

impl State {
    pub async fn new(window: Arc<Window>, config_zad1: bool) -> anyhow::Result<State> {
        let size = window.inner_size();

        // Device
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::defaults(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        // Camera
        let proj = glam::Mat4::perspective_rh(
            camera::FOV, 
            6.4 / 4.8, 
            camera::Z_NEAR, 
            camera::Z_FAR
        );
        let view = glam::Mat4::look_at_rh(
            glam::Vec3::new(0.0, 0.0, camera::F),
            glam::Vec3::new(0.0, 0.0, camera::CENTER),
            glam::Vec3::Y,
        );
        let correction = glam::Mat4::from_scale(glam::vec3(1.0, -1.0, 1.0));

        let camera_uniform = camera::CameraUniform {
            view_proj: (correction * proj * view).to_cols_array_2d(),
        };

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // Models
        let model_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("model_bind_group_layout"),
            });

        let initial_matrix = glam::Mat4::IDENTITY.to_cols_array_2d();

        let model_a_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model A Buffer"),
            contents: bytemuck::cast_slice(&[initial_matrix]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let model_b_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model B Buffer"),
            contents: bytemuck::cast_slice(&[initial_matrix]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let model_a_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &model_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_a_buffer.as_entire_binding(),
            }],
            label: Some("model_a_bind_group"),
        });

        let model_b_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &model_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_b_buffer.as_entire_binding(),
            }],
            label: Some("model_b_bind_group"),
        });

        // Buffers
        let (
            vertex_a_buffer,
            vertex_b_buffer,
            index_a_buffer,
            index_b_buffer,
            num_indices_a,
            num_indices_b,
        ) = if config_zad1 {
            let na = vertices::INDICES_A_ZAD1.len() as u32;
            let nb = vertices::INDICES_B_ZAD1.len() as u32;
            let va = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(vertices::VERTICES_A_ZAD1),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

            let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(vertices::VERTICES_B_ZAD1),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

            let ia = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(vertices::INDICES_A_ZAD1),
                usage: wgpu::BufferUsages::INDEX,
            });

            let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(vertices::INDICES_B_ZAD1),
                usage: wgpu::BufferUsages::INDEX,
            });

            (va, vb, ia, ib, na, nb)
        } else {
            let na = vertices::INDICES_A_ZAD2.len() as u32;
            let nb = vertices::INDICES_B_ZAD2.len() as u32;
            let va = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(vertices::VERTICES_A_ZAD2),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

            let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(vertices::VERTICES_B_ZAD2),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

            let ia = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(vertices::INDICES_A_ZAD2),
                usage: wgpu::BufferUsages::INDEX,
            });

            let ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(vertices::INDICES_B_ZAD2),
                usage: wgpu::BufferUsages::INDEX,
            });

            (va, vb, ia, ib, na, nb)
        };

        let shader_source = include_str!("shader.wgsl");
        let render_pipeline = create_pipeline(
            &device,
            &config,
            &camera_bind_group_layout,
            &model_bind_group_layout,
            shader_source,
            "fs_main",
            "vs_main",
        );

        Ok(Self {
            config_zad1,
            last_frame_inst: std::time::Instant::now(),
            frame_count: 0,
            accum_time: 0.0,
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,
            render_pipeline,
            vertex_a_buffer,
            index_a_buffer,
            num_indices_a,
            vertex_b_buffer,
            index_b_buffer,
            num_indices_b,
            camera_bind_group,
            frame_index: 0,
            depth_view,
            camera_uniform,
            camera_buffer,
            model_a_buffer,
            model_b_buffer,
            model_a_bind_group,
            model_b_bind_group,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth Texture"),
                size: wgpu::Extent3d {
                    width: width,
                    height: height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            self.depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

            // Camera
            let aspect = width as f32 / height as f32;
            let fov_y = camera::FOV;
            let proj = glam::Mat4::perspective_rh(fov_y, aspect, camera::Z_NEAR, camera::Z_FAR);
            let view = glam::Mat4::look_at_rh(
                glam::Vec3::new(0.0, 0.0, camera::F),
                glam::Vec3::new(0.0, 0.0, camera::CENTER),
                glam::Vec3::Y,
            );
            let correction = glam::Mat4::from_scale(glam::vec3(1.0, -1.0, 1.0));
            self.camera_uniform.view_proj = (correction * proj * view).to_cols_array_2d();
            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            (KeyCode::Space, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn handle_mouse_moved(&mut self, _x: f64, _y: f64) {}

    pub fn update(&mut self) {

        // Fps
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_frame_inst).as_secs_f32();
        self.last_frame_inst = now;

        self.accum_time += dt;
        self.frame_count += 1;

        if self.accum_time >= 1.0 {
            let fps = self.frame_count as f32 / self.accum_time;
            self.window
                .set_title(&format!("Projekt 1 | FPS: {:.1}", fps));
            self.accum_time = 0.0;
            self.frame_count = 0;
        }

        if self.config_zad1 {
            return;
        }

        // Rotation
        let speed_multiplier = 1.0;
        let n = speed_multiplier * (self.frame_index) as f32;
        self.frame_index += 1;

        let roll_a = (n * (2.0 + camera::A_DIGIT / 10.0)).to_radians();
        let pitch_a = (n * (1.0 + camera::B_DIGIT / 10.0)).to_radians();

        let rotation_a = glam::Mat4::from_rotation_y(pitch_a) * glam::Mat4::from_rotation_z(roll_a);
        let translation_a = glam::Mat4::from_translation(glam::vec3(0.0, 0.5, 10.0));
        let model_matrix_a = translation_a * rotation_a;

        let roll_b = (-n * (2.0 + camera::B_DIGIT / 10.0)).to_radians();
        let pitch_b = (-n * (1.0 + camera::A_DIGIT / 10.0)).to_radians();

        let rotation_b = glam::Mat4::from_rotation_y(pitch_b) * glam::Mat4::from_rotation_z(roll_b);
        let translation_b = glam::Mat4::from_translation(glam::vec3(0.5, 0.0, 10.0));
        let model_matrix_b = translation_b * rotation_b;

        // Write into buffer
        self.queue.write_buffer(
            &self.model_a_buffer,
            0,
            bytemuck::cast_slice(&model_matrix_a.to_cols_array_2d()),
        );
        self.queue.write_buffer(
            &self.model_b_buffer,
            0,
            bytemuck::cast_slice(&model_matrix_b.to_cols_array_2d()),
        );
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.surface.configure(&self.device, &self.config);
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

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
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

            render_pass.set_bind_group(1, &self.model_a_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_a_buffer.slice(..));
            render_pass.set_index_buffer(self.index_a_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices_a, 0, 0..1);

            render_pass.set_bind_group(1, &self.model_b_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_b_buffer.slice(..));
            render_pass.set_index_buffer(self.index_b_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices_b, 0, 0..1);
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    camera_bind_group_layout: &wgpu::BindGroupLayout,
    model_bind_group_layout: &wgpu::BindGroupLayout,
    shader_path: &str,
    fragment_shader: &str,
    vertex_shader: &str,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_path.into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
            Some(&camera_bind_group_layout),
            Some(&model_bind_group_layout),
        ],
        immediate_size: 0,
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(vertex_shader),
            buffers: &[vertices::Vertex::desc()],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(fragment_shader),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Cw,
            cull_mode: None,
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare: Some(wgpu::CompareFunction::Less),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    })
}
