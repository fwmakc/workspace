//! Phase 0 — Playable Demo
//!
//! Minimal interactive prototype: window, wgpu rendering, cursor, clicks, text.
//! Run with: `cargo run --bin demo`

use tracing::{info, warn};
use wgpu::{
    Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, SurfaceConfiguration, TextureViewDescriptor,
};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

/// CoreOS brand color: dark navy.
const BRAND_COLOR: Color = Color {
    r: 0x0a as f64 / 255.0,
    g: 0x0e as f64 / 255.0,
    b: 0x1a as f64 / 255.0,
    a: 1.0,
};

const CURSOR_COLOR: [f32; 4] = [0.0, 0.9, 1.0, 1.0]; // cyan
const CURSOR_SIZE_MIN: f32 = 4.0;
const CURSOR_SIZE_MAX: f32 = 64.0;
const CURSOR_SIZE_DEFAULT: f32 = 16.0;

const CIRCLE_COLORS: [[f32; 4]; 5] = [
    [1.0, 0.2, 0.2, 1.0],
    [0.2, 1.0, 0.2, 1.0],
    [0.2, 0.2, 1.0, 1.0],
    [1.0, 1.0, 0.2, 1.0],
    [1.0, 0.2, 1.0, 1.0],
];

const MAX_SHAPES: usize = 10_000;
const VERTICES_PER_SHAPE: usize = 6;

const FONT_SIZE: f32 = 32.0;
const CMD_FONT_SIZE: f32 = 24.0;
const TEXT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const CMD_PANEL_COLOR: [f32; 4] = [0.1, 0.12, 0.18, 0.95];
const CMD_PANEL_HEIGHT: f32 = 48.0;

// ------------------------------------------------------------------
// Shape shader (cursor + circles + panel)
// ------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ShapeVertex {
    position: [f32; 2],
    center: [f32; 2],
    radius: f32,
    _pad: f32,
    color: [f32; 4],
}

const SHAPE_SHADER: &str = r#"
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec2<f32>,
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) pos: vec2<f32>,
    @location(1) center: vec2<f32>,
    @location(2) radius: f32,
    @location(3) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.world_pos = pos;
    out.center = center;
    out.radius = radius;
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.radius > 0.0 && length(in.world_pos - in.center) > in.radius) {
        discard;
    }
    return in.color;
}
"#;

// ------------------------------------------------------------------
// Text shader (font atlas)
// ------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 4],
}

const TEXT_SHADER: &str = r#"
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) pos: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(pos, 0.0, 1.0);
    out.tex_coords = tex_coords;
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(t_diffuse, s_diffuse, in.tex_coords).r;
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
"#;

// ------------------------------------------------------------------

#[derive(Clone, Copy)]
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: [f32; 4],
}

struct DemoState {
    window: Window,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
    cursor_pos: PhysicalPosition<f64>,
    cursor_size: f32,
    circles: Vec<Circle>,
    next_color_idx: usize,

    // Shape rendering
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,

    // Text rendering
    font: fontdue::Font,
    typed_text: String,
    text_dirty: bool,
    text_pipeline: wgpu::RenderPipeline,
    text_bind_group_layout: wgpu::BindGroupLayout,
    text_sampler: wgpu::Sampler,
    text_vertex_buffer: wgpu::Buffer,
    text_texture: Option<wgpu::Texture>,
    text_bind_group: Option<wgpu::BindGroup>,
    text_vertices: Vec<TextVertex>,

    // Command Bar
    command_bar_visible: bool,
    command_text: String,
    cmd_text_dirty: bool,
    cmd_text_texture: Option<wgpu::Texture>,
    cmd_text_bind_group: Option<wgpu::BindGroup>,
    cmd_text_vertices: Vec<TextVertex>,
}

impl DemoState {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = unsafe {
            instance.create_surface_unsafe(
                wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap(),
            )
        }
        .expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("No GPU adapter found. Ensure you have a GPU or WARP/llvmpipe available.");

        info!("Selected adapter: {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Demo Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create wgpu device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let max_texture_size = device.limits().max_texture_dimension_2d;
        let width = size.width.min(max_texture_size).max(1);
        let height = size.height.min(max_texture_size).max(1);

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // ---- Shape pipeline ----
        let shape_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(SHAPE_SHADER.into()),
        });

        let shape_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Shape Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Pipeline"),
            layout: Some(&shape_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shape_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<ShapeVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                            shader_location: 3,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shape_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shape Vertex Buffer"),
            size: (std::mem::size_of::<ShapeVertex>() * VERTICES_PER_SHAPE * MAX_SHAPES)
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ---- Text pipeline ----
        let text_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(TEXT_SHADER.into()),
        });

        let text_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Text Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let text_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Text Pipeline Layout"),
                bind_group_layouts: &[&text_bind_group_layout],
                push_constant_ranges: &[],
            });

        let text_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Pipeline"),
            layout: Some(&text_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &text_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &text_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let text_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Text Sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let text_vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Vertex Buffer"),
            size: (std::mem::size_of::<TextVertex>() * VERTICES_PER_SHAPE * 1000)
                as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // ---- Load font ----
        let font_path = if std::path::Path::new(r"C:\Windows\Fonts\segoeui.ttf").exists() {
            r"C:\Windows\Fonts\segoeui.ttf"
        } else if std::path::Path::new(r"C:\Windows\Fonts\arial.ttf").exists() {
            r"C:\Windows\Fonts\arial.ttf"
        } else {
            panic!("No suitable system font found.");
        };
        let font_data = std::fs::read(font_path).expect("Failed to read font file");
        let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
            .expect("Invalid font data");

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size: PhysicalSize::new(width, height),
            cursor_pos: PhysicalPosition::new(0.0, 0.0),
            cursor_size: CURSOR_SIZE_DEFAULT,
            circles: Vec::new(),
            next_color_idx: 0,
            render_pipeline,
            vertex_buffer,
            font,
            typed_text: String::new(),
            text_dirty: true,
            text_pipeline,
            text_bind_group_layout,
            text_sampler,
            text_vertex_buffer,
            text_texture: None,
            text_bind_group: None,
            text_vertices: Vec::new(),
            command_bar_visible: false,
            command_text: String::new(),
            cmd_text_dirty: true,
            cmd_text_texture: None,
            cmd_text_bind_group: None,
            cmd_text_vertices: Vec::new(),
        }
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            warn!("Resize to zero ignored");
            return;
        }
        let max_texture_size = self.device.limits().max_texture_dimension_2d;
        let width = new_size.width.min(max_texture_size).max(1);
        let height = new_size.height.min(max_texture_size).max(1);
        self.size = PhysicalSize::new(width, height);
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.text_dirty = true;
        self.cmd_text_dirty = true;
        info!("Resized to {}x{}", width, height);
    }

    fn window_size_f32(&self) -> (f32, f32) {
        let s = self.window.inner_size();
        (s.width as f32, s.height as f32)
    }

    fn screen_to_ndc(&self, x: f32, y: f32) -> [f32; 2] {
        let (w, h) = self.window_size_f32();
        [(x / w) * 2.0 - 1.0, 1.0 - (y / h) * 2.0]
    }

    fn px_to_ndc(&self, px: f32) -> f32 {
        let (w, h) = self.window_size_f32();
        let scale = ((2.0 / w) + (2.0 / h)) / 2.0;
        px * scale
    }

    fn build_shape_vertices(&self) -> Vec<ShapeVertex> {
        let mut verts = Vec::with_capacity(VERTICES_PER_SHAPE * (1 + self.circles.len() + 1));

        // Cursor quad
        let half = self.cursor_size / 2.0;
        let cx = self.cursor_pos.x as f32;
        let cy = self.cursor_pos.y as f32;
        let left = self.screen_to_ndc(cx - half, cy)[0];
        let right = self.screen_to_ndc(cx + half, cy)[0];
        let top = self.screen_to_ndc(cx, cy - half)[1];
        let bottom = self.screen_to_ndc(cx, cy + half)[1];

        verts.extend_from_slice(&[
            ShapeVertex { position: [left, bottom], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CURSOR_COLOR },
            ShapeVertex { position: [right, bottom], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CURSOR_COLOR },
            ShapeVertex { position: [left, top], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CURSOR_COLOR },
            ShapeVertex { position: [right, bottom], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CURSOR_COLOR },
            ShapeVertex { position: [right, top], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CURSOR_COLOR },
            ShapeVertex { position: [left, top], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CURSOR_COLOR },
        ]);

        // Circles
        for circle in &self.circles {
            let [c_ndc_x, c_ndc_y] = self.screen_to_ndc(circle.x, circle.y);
            let r_ndc = self.px_to_ndc(circle.radius);
            let left = c_ndc_x - r_ndc;
            let right = c_ndc_x + r_ndc;
            let top = c_ndc_y + r_ndc;
            let bottom = c_ndc_y - r_ndc;
            verts.extend_from_slice(&[
                ShapeVertex { position: [left, bottom], center: [c_ndc_x, c_ndc_y], radius: r_ndc, _pad: 0.0, color: circle.color },
                ShapeVertex { position: [right, bottom], center: [c_ndc_x, c_ndc_y], radius: r_ndc, _pad: 0.0, color: circle.color },
                ShapeVertex { position: [left, top], center: [c_ndc_x, c_ndc_y], radius: r_ndc, _pad: 0.0, color: circle.color },
                ShapeVertex { position: [right, bottom], center: [c_ndc_x, c_ndc_y], radius: r_ndc, _pad: 0.0, color: circle.color },
                ShapeVertex { position: [right, top], center: [c_ndc_x, c_ndc_y], radius: r_ndc, _pad: 0.0, color: circle.color },
                ShapeVertex { position: [left, top], center: [c_ndc_x, c_ndc_y], radius: r_ndc, _pad: 0.0, color: circle.color },
            ]);
        }

        // Command Bar panel
        if self.command_bar_visible {
            let (w, h) = self.window_size_f32();
            let left = self.screen_to_ndc(0.0, h - CMD_PANEL_HEIGHT)[0];
            let right = self.screen_to_ndc(w, h - CMD_PANEL_HEIGHT)[0];
            let top = self.screen_to_ndc(0.0, 0.0)[1];
            let bottom = self.screen_to_ndc(0.0, h)[1];
            verts.extend_from_slice(&[
                ShapeVertex { position: [left, bottom], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CMD_PANEL_COLOR },
                ShapeVertex { position: [right, bottom], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CMD_PANEL_COLOR },
                ShapeVertex { position: [left, top], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CMD_PANEL_COLOR },
                ShapeVertex { position: [right, bottom], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CMD_PANEL_COLOR },
                ShapeVertex { position: [right, top], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CMD_PANEL_COLOR },
                ShapeVertex { position: [left, top], center: [0.0, 0.0], radius: 0.0, _pad: 0.0, color: CMD_PANEL_COLOR },
            ]);
        }

        verts
    }

    fn layout_text(
        &self,
        text: &str,
        font_size: f32,
        screen_x_start: f32,
        screen_y_baseline: f32,
        color: [f32; 4],
    ) -> Option<(Vec<TextVertex>, Vec<u8>, u32, u32)> {
        if text.is_empty() {
            return None;
        }

        let padding = 2u32;
        let mut pen_x = 0u32;
        let mut max_height = 0u32;

        for c in text.chars() {
            let (metrics, _) = self.font.rasterize(c, font_size);
            pen_x += metrics.width as u32 + padding;
            max_height = max_height.max(metrics.height as u32);
        }

        let atlas_width = pen_x.next_power_of_two().max(64).min(2048);
        let atlas_height = max_height.next_power_of_two().max(64).min(2048);

        let mut atlas_data = vec![0u8; (atlas_width * atlas_height) as usize];
        let mut char_info = Vec::new();
        pen_x = 0;

        for c in text.chars() {
            let (metrics, bitmap) = self.font.rasterize(c, font_size);
            let w = metrics.width as u32;
            let h = metrics.height as u32;

            for y in 0..h {
                for x in 0..w {
                    let atlas_idx = ((y * atlas_width) + pen_x + x) as usize;
                    let bitmap_idx = (y * w + x) as usize;
                    if atlas_idx < atlas_data.len() && bitmap_idx < bitmap.len() {
                        atlas_data[atlas_idx] = bitmap[bitmap_idx];
                    }
                }
            }

            char_info.push((pen_x, w, h, metrics));
            pen_x += w + padding;
        }

        let mut vertices = Vec::new();
        let mut cursor_x = screen_x_start;

        for (i, _c) in text.chars().enumerate() {
            let (atlas_x, w, h, metrics) = char_info[i];
            let w_f = w as f32;
            let h_f = h as f32;

            let left = cursor_x + metrics.xmin as f32;
            let right = left + w_f;
            let top = screen_y_baseline - (metrics.ymin as f32 + h_f);
            let bottom = screen_y_baseline - metrics.ymin as f32;

            let [ndc_left, ndc_top] = self.screen_to_ndc(left, top);
            let [ndc_right, ndc_bottom] = self.screen_to_ndc(right, bottom);

            let uv_left = atlas_x as f32 / atlas_width as f32;
            let uv_right = (atlas_x + w) as f32 / atlas_width as f32;
            let uv_top = 0.0;
            let uv_bottom = h_f / atlas_height as f32;

            vertices.extend_from_slice(&[
                TextVertex { position: [ndc_left, ndc_bottom], tex_coords: [uv_left, uv_bottom], color },
                TextVertex { position: [ndc_right, ndc_bottom], tex_coords: [uv_right, uv_bottom], color },
                TextVertex { position: [ndc_left, ndc_top], tex_coords: [uv_left, uv_top], color },
                TextVertex { position: [ndc_right, ndc_bottom], tex_coords: [uv_right, uv_bottom], color },
                TextVertex { position: [ndc_right, ndc_top], tex_coords: [uv_right, uv_top], color },
                TextVertex { position: [ndc_left, ndc_top], tex_coords: [uv_left, uv_top], color },
            ]);

            cursor_x += metrics.advance_width;
        }

        Some((vertices, atlas_data, atlas_width, atlas_height))
    }

    fn upload_text_block(
        &self,
        atlas_data: &[u8],
        atlas_width: u32,
        atlas_height: u32,
    ) -> (wgpu::Texture, wgpu::BindGroup) {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Text Atlas"),
            size: wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            texture.as_image_copy(),
            atlas_data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(atlas_width),
                rows_per_image: Some(atlas_height),
            },
            wgpu::Extent3d {
                width: atlas_width,
                height: atlas_height,
                depth_or_array_layers: 1,
            },
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Text Bind Group"),
            layout: &self.text_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.text_sampler),
                },
            ],
        });

        (texture, bind_group)
    }

    fn update_text(&mut self) {
        if !self.text_dirty {
            return;
        }
        self.text_dirty = false;

        if let Some((verts, atlas_data, w, h)) = self.layout_text(
            &self.typed_text,
            FONT_SIZE,
            20.0,
            60.0,
            TEXT_COLOR,
        ) {
            let (tex, bg) = self.upload_text_block(&atlas_data, w, h);
            self.text_vertices = verts;
            self.text_texture = Some(tex);
            self.text_bind_group = Some(bg);
        } else {
            self.text_texture = None;
            self.text_bind_group = None;
            self.text_vertices.clear();
        }
    }

    fn update_cmd_text(&mut self) {
        if !self.cmd_text_dirty {
            return;
        }
        self.cmd_text_dirty = false;

        if let Some((verts, atlas_data, w, h)) = self.layout_text(
            &self.command_text,
            CMD_FONT_SIZE,
            10.0,
            self.window_size_f32().1 - (CMD_PANEL_HEIGHT / 2.0) + (CMD_FONT_SIZE / 2.0),
            TEXT_COLOR,
        ) {
            let (tex, bg) = self.upload_text_block(&atlas_data, w, h);
            self.cmd_text_vertices = verts;
            self.cmd_text_texture = Some(tex);
            self.cmd_text_bind_group = Some(bg);
        } else {
            self.cmd_text_texture = None;
            self.cmd_text_bind_group = None;
            self.cmd_text_vertices.clear();
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.update_text();
        self.update_cmd_text();

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        let shape_vertices = self.build_shape_vertices();
        let shape_vertex_count = shape_vertices.len() as u32;
        self.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&shape_vertices),
        );

        let text_vertex_count = self.text_vertices.len() as u32;
        if text_vertex_count > 0 {
            self.queue.write_buffer(
                &self.text_vertex_buffer,
                0,
                bytemuck::cast_slice(&self.text_vertices),
            );
        }

        let cmd_vertex_count = self.cmd_text_vertices.len() as u32;
        if cmd_vertex_count > 0 {
            self.queue.write_buffer(
                &self.text_vertex_buffer,
                (std::mem::size_of::<TextVertex>() * self.text_vertices.len()) as wgpu::BufferAddress,
                bytemuck::cast_slice(&self.cmd_text_vertices),
            );
        }

        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Demo Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(BRAND_COLOR),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // Shapes
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..shape_vertex_count, 0..1);

            // Typed text
            if text_vertex_count > 0 {
                render_pass.set_pipeline(&self.text_pipeline);
                render_pass.set_bind_group(
                    0,
                    self.text_bind_group.as_ref().unwrap(),
                    &[],
                );
                render_pass.set_vertex_buffer(0, self.text_vertex_buffer.slice(..));
                render_pass.draw(0..text_vertex_count, 0..1);
            }

            // Command bar text
            if self.command_bar_visible && cmd_vertex_count > 0 {
                render_pass.set_bind_group(
                    0,
                    self.cmd_text_bind_group.as_ref().unwrap(),
                    &[],
                );
                render_pass.set_vertex_buffer(
                    0,
                    self.text_vertex_buffer.slice(
                        (std::mem::size_of::<TextVertex>() * self.text_vertices.len()) as wgpu::BufferAddress
                            ..,
                    ),
                );
                render_pass.draw(0..cmd_vertex_count, 0..1);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn update_title_with_fps(&self, fps: f64) {
        self.window
            .set_title(&format!("CORE OS Demo — {:.0} FPS", fps));
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("CORE OS Phase 0 — Playable Demo starting...");

    let event_loop = EventLoop::new().unwrap();
    let window_attrs = winit::window::WindowAttributes::default()
        .with_title("CORE OS Demo")
        .with_inner_size(PhysicalSize::new(1280, 720));
    let window = event_loop
        .create_window(window_attrs)
        .expect("Failed to create window");

    let mut state = pollster::block_on(DemoState::new(window));

    let mut last_frame_time = std::time::Instant::now();
    let mut frame_count = 0u32;
    let mut modifiers = winit::keyboard::ModifiersState::empty();

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, window_id } => {
                    if window_id == state.window.id() {
                        match event {
                            WindowEvent::CloseRequested => {
                                info!("Exit requested");
                                elwt.exit();
                            }
                            WindowEvent::ModifiersChanged(m) => {
                                modifiers = m.state();
                            }
                            WindowEvent::KeyboardInput { event, .. } => {
                                if event.state == winit::event::ElementState::Pressed {
                                    match &event.logical_key {
                                        winit::keyboard::Key::Named(
                                            winit::keyboard::NamedKey::Escape,
                                        ) => {
                                            if modifiers.control_key() && modifiers.shift_key() {
                                                info!("Panic gesture triggered — graceful exit");
                                                elwt.exit();
                                            } else if state.command_bar_visible {
                                                state.command_bar_visible = false;
                                                state.cmd_text_dirty = true;
                                                info!("Command bar closed");
                                            } else {
                                                info!("Exit requested");
                                                elwt.exit();
                                            }
                                        }
                                        winit::keyboard::Key::Named(
                                            winit::keyboard::NamedKey::Backspace,
                                        ) => {
                                            if state.command_bar_visible {
                                                state.command_text.pop();
                                                state.cmd_text_dirty = true;
                                            } else if !state.typed_text.is_empty() {
                                                state.typed_text.pop();
                                                state.text_dirty = true;
                                            }
                                        }
                                        winit::keyboard::Key::Named(
                                            winit::keyboard::NamedKey::Enter,
                                        ) => {
                                            if state.command_bar_visible
                                                && !state.command_text.is_empty()
                                            {
                                                info!(
                                                    "Command executed: {}",
                                                    state.command_text
                                                );
                                                state.command_text.clear();
                                                state.cmd_text_dirty = true;
                                            }
                                        }
                                        winit::keyboard::Key::Named(
                                            winit::keyboard::NamedKey::Space,
                                        ) if modifiers.shift_key() => {
                                            state.command_bar_visible =
                                                !state.command_bar_visible;
                                            state.cmd_text_dirty = true;
                                            info!(
                                                "Command bar toggled: {}",
                                                state.command_bar_visible
                                            );
                                        }
                                        winit::keyboard::Key::Character(ch) => {
                                            if !event.repeat {
                                                if state.command_bar_visible {
                                                    state.command_text.push_str(ch.as_str());
                                                    state.cmd_text_dirty = true;
                                                } else {
                                                    state.typed_text.push_str(ch.as_str());
                                                    state.text_dirty = true;
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            WindowEvent::Resized(physical_size) => {
                                state.resize(physical_size);
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                state.cursor_pos = position;
                            }
                            WindowEvent::MouseInput {
                                button,
                                state: btn_state,
                                ..
                            } => {
                                if btn_state == winit::event::ElementState::Pressed {
                                    match button {
                                        MouseButton::Left => {
                                            let color = CIRCLE_COLORS[state.next_color_idx];
                                            state.next_color_idx =
                                                (state.next_color_idx + 1) % CIRCLE_COLORS.len();
                                            state.circles.push(Circle {
                                                x: state.cursor_pos.x as f32,
                                                y: state.cursor_pos.y as f32,
                                                radius: state.cursor_size,
                                                color,
                                            });
                                            info!(
                                                "Circle added at ({}, {}), total {}",
                                                state.cursor_pos.x as u32,
                                                state.cursor_pos.y as u32,
                                                state.circles.len()
                                            );
                                        }
                                        MouseButton::Right => {
                                            state.circles.clear();
                                            state.next_color_idx = 0;
                                            info!("Circles cleared");
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            WindowEvent::MouseWheel { delta, .. } => {
                                let dy = match delta {
                                    MouseScrollDelta::LineDelta(_, y) => y,
                                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 20.0,
                                };
                                state.cursor_size = (state.cursor_size + dy * 2.0)
                                    .clamp(CURSOR_SIZE_MIN, CURSOR_SIZE_MAX);
                            }
                            WindowEvent::RedrawRequested => {
                                if let Err(e) = state.render() {
                                    if e == wgpu::SurfaceError::Lost
                                        || e == wgpu::SurfaceError::Outdated
                                    {
                                        state.resize(state.size);
                                    } else {
                                        panic!("Render error: {e}");
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::AboutToWait => {
                    frame_count += 1;
                    let now = std::time::Instant::now();
                    let elapsed = now.duration_since(last_frame_time).as_secs_f64();
                    if elapsed >= 1.0 {
                        let fps = frame_count as f64 / elapsed;
                        state.update_title_with_fps(fps);
                        frame_count = 0;
                        last_frame_time = now;
                    }

                    state.window.request_redraw();
                }
                _ => {}
            }
        })
        .expect("Event loop failed");

    info!("CORE OS Demo exited gracefully");
}
