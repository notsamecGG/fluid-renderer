use crate::{Instance, PARTICLE_SIZE};
use glam::{vec3, Vec3};

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

pub fn create_dense_rect(grid_dimensions: (u32, u32), offset: (f32, f32, f32), particle_radius: Option<f32>, color: Option<Vec3>) -> Vec<Instance> {
    let particle_radius = particle_radius.unwrap_or(PARTICLE_SIZE);
    let color = color.unwrap_or(Vec3::ONE);

    let (width, height) = grid_dimensions;
    let width = (width as f32 / particle_radius).ceil() as u32;
    let height = (height as f32 / particle_radius).ceil() as u32;
    
    dbg!(grid_dimensions, width, height);

    (0..=height).flat_map(|y| {
        (0..=width).filter(move |x| {
            (*x == 0 || *x == width) || (y == 0 || y == height)
        }).map(move |x| {
            let pos_x = x as f32 * particle_radius + offset.0;
            let pos_y = y as f32 * particle_radius + offset.1;

            Instance {
                position: vec3(pos_x, pos_y, offset.2),
                color
            }
        })
    }).collect()
}

pub fn create_len(len: u32, color: Vec3) -> Vec<Instance> {
    (0..len).map(|_index| Instance {
        color,
        ..Default::default() 
    }).collect()
}
