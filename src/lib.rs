use std::time::{Instant, Duration};

use glam::vec3a;
use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
};

pub use wgpu;
pub use winit;

pub use imgui;
pub use imgui_wgpu;
pub use imgui_winit_support;

mod modules;
pub use modules::*;


pub const PARTICLE_SIZE: f32 = 0.08;
pub const GRID_DIMENSIONS: (u32, u32) = (20, 20);
pub const CUBE_DIMENSIONS: (u32, u32, u32) = (20, 20, 20);


pub struct InitOutput {
    pub event_loop: EventLoop<()>,
    pub window: Window,
    pub aspect_ratio: f32,
}

pub fn init() -> InitOutput {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let winit::dpi::PhysicalSize{width, height} = window.inner_size();
    let aspect_ratio = width as f32 / height as f32;
    
    InitOutput {
        event_loop,
        window,
        aspect_ratio
    }
}

pub fn init_ui(state: &State, font_size: f64) -> (imgui::Context, imgui_winit_support::WinitPlatform, imgui_wgpu::Renderer) {
    let mut ctxt = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut ctxt);

    platform.attach_window(
        ctxt.io_mut(), 
        &state.window, 
        imgui_winit_support::HiDpiMode::Default,
    );

    ctxt.set_ini_filename(None);
    set_ui_size(&mut ctxt, &state.window);

    let font_size = (font_size * state.window.scale_factor()) as f32;
    ctxt.fonts().add_font(&[imgui::FontSource::DefaultFontData { 
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()

        }),
    }]);

    let render_config = imgui_wgpu::RendererConfig {
        texture_format: state.surface_format,
        depth_format: Some(wgpu::TextureFormat::Depth32Float),
        ..Default::default()
    };

    let renderer = imgui_wgpu::Renderer::new(&mut ctxt, &state.device, &state.queue, render_config);

    (ctxt, platform, renderer)
}

pub fn set_ui_size(ctxt: &mut imgui::Context, window: &winit::window::Window){
    ctxt.io_mut().font_global_scale = (1.0 / window.scale_factor()) as f32;
}

pub fn handle_windowing(state: &mut State, imgui_ctxt: &mut imgui::Context, event: &WindowEvent, control_flow: &mut ControlFlow) {
    if !state.input(event) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.set_exit(),
            WindowEvent::Resized(physical_size) => {
                set_ui_size(imgui_ctxt, &state.window);
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &mut so w have to dereference it twice
                set_ui_size(imgui_ctxt, &state.window);
                state.resize(**new_inner_size);
            }
            _ => {}
        }
    }
}

pub fn handle_rendering(state: &mut State, imgui_renderer: &mut imgui_wgpu::Renderer, draw_data: &imgui::DrawData, control_flow: &mut ControlFlow) {
    match state.render_with_ui(imgui_renderer, draw_data) {
        Ok(_) => {}
        // Reconfigure the surface if it's lost or outdated
        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
        // The system is out of memory, we should probably quit
        Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
        // We're ignoring timeouts
        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let winit::dpi::PhysicalSize{width, height} = window.inner_size();
    let aspect_ratio = width as f32 / height as f32;

    let shader_source = wgpu::ShaderSource::Wgsl(std::fs::read_to_string("src/shader.wgsl").unwrap().into());
    let vertices = Quad.scale(PARTICLE_SIZE);
    let indices = Quad::INDICES;
    let instances = create_cube(0.1, CUBE_DIMENSIONS, None, (-1.0, -1.0, -2.0));
    let camera = Camera {
        aspect: aspect_ratio,
        eye: vec3a(-4.0, 2.0, 2.0),
        fovy: 45.0,
        ..Default::default()
    };

    let mut state = State::new(
        window, 
        shader_source, 
        vertices.as_slice(), 
        indices, 
        instances, 
        camera
    ).await;
    
    let (mut imgui_ctxt, mut imgui_platform, mut imgui_renderer) = init_ui(&state, 10.0);
    let mut frame_delta = Duration::new(0, 0);

    event_loop.run(move |event, _, control_flow| {
        let frame_start = Instant::now();
        imgui_platform.handle_event(imgui_ctxt.io_mut(), &state.window, &event);

        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                crate::handle_windowing(&mut state, &mut imgui_ctxt, event, control_flow)
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();

                imgui_platform.prepare_frame(imgui_ctxt.io_mut(), &state.window).expect("Failed to prepare ui frame");
                imgui_ctxt.io_mut().update_delta_time(frame_delta);
                let ui = imgui_ctxt.frame();

                {
                    ui.window("Info")
                        .size([200.0, 100.0], imgui::Condition::FirstUseEver)
                        .build(|| {
                            ui.text("hello, world");
                        });
                }

                crate::handle_rendering(&mut state, &mut imgui_renderer, imgui_ctxt.render(), control_flow)
            },
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
        frame_delta = frame_start.elapsed();
    });
}
