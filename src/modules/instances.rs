use crate::Instance;
use cgmath::vec3;

pub fn create_grid(grid_dimensions: (u32, u32), screen_dimensions: (u32, u32), offset: (f32, f32, f32)) -> Vec<Instance> {
    let width = grid_dimensions.0;
    let height = grid_dimensions.1;

    let grid_dimensions: (f32, f32) = (grid_dimensions.0 as f32, grid_dimensions.1 as f32);
    let screen_dimensions: (f32, f32) = (screen_dimensions.0 as f32, screen_dimensions.1 as f32);

    let instance_offset: (f32, f32) = (screen_dimensions.0 / grid_dimensions.0, screen_dimensions.1 / grid_dimensions.1);

    (1..height).flat_map(|y| {
        (1..width).map(move |x| {
            Instance {
                position: vec3(
                    x as f32 * instance_offset.0 + offset.0,
                    y as f32 * instance_offset.1 + offset.1,
                    offset.2 ),
                color: vec3(
                    x as f32 / width as f32, 
                    y as f32 / height as f32, 
                    0.8
                ),
            }
        })
    })
    .collect::<Vec<_>>()
}
