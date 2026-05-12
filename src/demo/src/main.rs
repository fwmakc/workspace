#![allow(deprecated)]

//! Phase 0 — Playable Demo (refactored)
//!
//! Modular interactive prototype: window, wgpu rendering, cursor, clicks, text.
//! Run with: `cargo run --bin demo`

use tracing::{info, warn};
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::dpi::PhysicalPosition;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod graphics;
mod input;

use app::{AppState, CMD_FONT_SIZE, CMD_PANEL_COLOR, CMD_PANEL_HEIGHT, CURSOR_COLOR, FONT_SIZE, TEXT_COLOR};

const BRAND_COLOR: wgpu::Color = wgpu::Color {
    r: 0x0a as f64 / 255.0,
    g: 0x0e as f64 / 255.0,
    b: 0x1a as f64 / 255.0,
    a: 1.0,
};
use graphics::shape::{Shape, ShapeRenderer};
use graphics::text::{TextEntry, TextRenderer};
use graphics::GraphicsContext;
use input::{Command, InputHandler};

/// Convert screen pixels to Normalized Device Coordinates.
fn screen_to_ndc(x: f32, y: f32, w: f32, h: f32) -> [f32; 2] {
    [(x / w) * 2.0 - 1.0, 1.0 - (y / h) * 2.0]
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
        .expect("failed to create window");

    let mut ctx = pollster::block_on(GraphicsContext::new(window));
    let mut shapes = ShapeRenderer::new(&ctx);
    let text_renderer = TextRenderer::new(&ctx);
    let mut app = AppState::default();
    let mut input = InputHandler::default();

    let mut last_frame_time = std::time::Instant::now();
    let mut frame_count = 0u32;

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, window_id } => {
                    if window_id != ctx.window().id() {
                        return;
                    }

                    let commands = input.handle(&event, app.command_bar_visible);
                    for cmd in commands {
                        match cmd {
                            Command::Exit => {
                                info!("exiting gracefully");
                                elwt.exit();
                            }
                            Command::ToggleCommandBar => {
                                app.toggle_command_bar();
                                info!("command bar toggled: {}", app.command_bar_visible);
                            }
                            Command::Type(ch, target) => {
                                app.type_char(ch, target);
                            }
                            Command::Backspace(target) => {
                                app.backspace(target);
                            }
                            Command::Execute => {
                                if let Some(cmd) = app.execute_command() {
                                    info!("command executed: {}", cmd);
                                }
                            }
                            Command::AddCircle => {
                                app.add_circle();
                                info!(
                                    "circle added at ({:.0}, {:.0}), total {}",
                                    app.cursor_pos.x, app.cursor_pos.y, app.circles.len()
                                );
                            }
                            Command::ClearCircles => {
                                app.clear_circles();
                                info!("circles cleared");
                            }
                            Command::ResizeCursor(delta) => {
                                app.resize_cursor(delta);
                            }
                            Command::CursorMoved(x, y) => {
                                app.cursor_pos = PhysicalPosition::new(x, y);
                            }
                            Command::WindowResized(w, h) => {
                                ctx.resize(PhysicalSize::new(w, h));
                            }
                        }
                    }
                }
                Event::AboutToWait => {
                    frame_count += 1;
                    let now = std::time::Instant::now();
                    let elapsed = now.duration_since(last_frame_time).as_secs_f64();
                    if elapsed >= 1.0 {
                        let fps = frame_count as f64 / elapsed;
                        ctx.window().set_title(&format!("CORE OS Demo — {:.0} FPS", fps));
                        frame_count = 0;
                        last_frame_time = now;
                    }

                    // --- Build shape list ---
                    shapes.clear();
                    let (w, h) = ctx.window_size_f32();
                    let to_ndc = |x, y| screen_to_ndc(x, y, w, h);

                    // Cursor
                    let half = app.cursor_size / 2.0;
                    let cx = app.cursor_pos.x as f32;
                    let cy = app.cursor_pos.y as f32;
                    shapes.push(
                        &Shape::Rect {
                            x: cx - half,
                            y: cy - half,
                            w: app.cursor_size,
                            h: app.cursor_size,
                            color: CURSOR_COLOR,
                        },
                        to_ndc,
                    );

                    // Circles
                    for circle in &app.circles {
                        shapes.push(
                            &Shape::Circle {
                                x: circle.x,
                                y: circle.y,
                                radius: circle.radius,
                                color: circle.color,
                            },
                            to_ndc,
                        );
                    }

                    // Command bar panel
                    if app.command_bar_visible {
                        shapes.push(
                            &Shape::Rect {
                                x: 0.0,
                                y: h - CMD_PANEL_HEIGHT,
                                w,
                                h: CMD_PANEL_HEIGHT,
                                color: CMD_PANEL_COLOR,
                            },
                            to_ndc,
                        );
                    }

                    shapes.upload(&ctx);

                    // --- Build text entries ---
                    let mut text_entries: Vec<TextEntry<'_>> = Vec::new();
                    if !app.typed_text.is_empty() {
                        text_entries.push(TextEntry {
                            text: &app.typed_text,
                            font_size: FONT_SIZE,
                            screen_x: 20.0,
                            screen_y_baseline: 60.0,
                            color: TEXT_COLOR,
                        });
                    }
                    if app.command_bar_visible && !app.command_text.is_empty() {
                        text_entries.push(TextEntry {
                            text: &app.command_text,
                            font_size: CMD_FONT_SIZE,
                            screen_x: 10.0,
                            screen_y_baseline: h - (CMD_PANEL_HEIGHT / 2.0) + (CMD_FONT_SIZE / 2.0),
                            color: TEXT_COLOR,
                        });
                    }

                    let (text_draw_calls, text_vertices) =
                        text_renderer.prepare(&ctx, &text_entries, to_ndc);
                    text_renderer.upload(&ctx, &text_vertices);

                    // --- Render ---
                    let output = match ctx.acquire_frame() {
                        Ok(f) => f,
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            ctx.resize(ctx.size());
                            return;
                        }
                        Err(e) => {
                            warn!("surface error: {}", e);
                            return;
                        }
                    };

                    let view = output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let mut encoder = ctx.create_encoder("Render Encoder");
                    {
                        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Demo Render Pass"),
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(BRAND_COLOR),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            occlusion_query_set: None,
                            timestamp_writes: None,
                        });

                        shapes.render(&mut pass);

                        text_renderer.set_vertex_buffer(&mut pass);
                        for call in &text_draw_calls {
                            call.render(&mut pass, text_renderer.pipeline());
                        }
                    }

                    ctx.submit(std::iter::once(encoder.finish()));
                    output.present();

                    ctx.request_redraw();
                }
                _ => {}
            }
        })
        .expect("event loop failed");

    info!("CORE OS Demo exited gracefully");
}
