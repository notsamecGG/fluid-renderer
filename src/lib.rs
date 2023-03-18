use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod modules;
pub use modules::*;


const PARTICLE_SIZE: f32 = 0.1;
const GRID_DIMENSIONS: (u32, u32) = (20, 20);

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
    let instances = create_grid(GRID_DIMENSIONS, (2, 2), (-1.0, -1.0, 0.0));
    let camera = Camera {
        aspect: aspect_ratio,
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

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
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
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &mut so w have to dereference it twice
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
                    // We're ignoring timeouts
                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
