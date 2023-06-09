use crate::{Instance, PARTICLE_SIZE};
use glam::{vec3, Vec3};

pub fn create_square(grid_dimensions: (u32, u32), screen_dimensions: (u32, u32), offset: (f32, f32, f32)) -> Vec<Instance> {
    let width = grid_dimensions.0;
    let height = grid_dimensions.1;

    let grid_dimensions: (f32, f32) = (grid_dimensions.0 as f32, grid_dimensions.1 as f32);
    let screen_dimensions: (f32, f32) = (screen_dimensions.0 as f32, screen_dimensions.1 as f32);

    let instance_offset: (f32, f32) = (screen_dimensions.0 / grid_dimensions.0, screen_dimensions.1 / grid_dimensions.1);
    let offset = (
        -1.0 * (width - 1) as f32 * instance_offset.0 / 2.0,
        -1.0 * (height - 1) as f32 * instance_offset.1 / 2.0,
        offset.2
    );

    (0..height).flat_map(|y| {
        (0..width).map(move |x| {
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

pub fn create_cube(wiggle_factor: f32, grid_dimensions: (u32, u32, u32), particle_offset: Option<(f32, f32, f32)>, offset: (f32, f32, f32)) -> Vec<Instance> {
    let width = grid_dimensions.0;
    let height = grid_dimensions.1;
    let depth = grid_dimensions.2;
    let screen_space_size = 2.0;

    let grid_dimensions = (grid_dimensions.0 as f32, grid_dimensions.1 as f32, grid_dimensions.2 as f32);

    let instance_offset = (screen_space_size / grid_dimensions.0, screen_space_size / grid_dimensions.1, screen_space_size / grid_dimensions.2);
    let instance_offset = particle_offset.unwrap_or(instance_offset);

    let offset = (
        -1.0 * (width - 1) as f32 * instance_offset.0 / 2.0,
        -1.0 * (height - 1) as f32 * instance_offset.1 / 2.0,
        offset.2
    );

    (0..depth).flat_map(|z| {
        (0..height).flat_map(move |y| {
            (0..width).map(move |x| {
                let b = z as f32 / depth as f32 + 0.1;
                let w = vec3(
                    (rand::random::<f32>() - 0.5) * PARTICLE_SIZE * wiggle_factor, 
                    (rand::random::<f32>() - 0.5) * PARTICLE_SIZE * wiggle_factor, 
                    (rand::random::<f32>() - 0.5) * PARTICLE_SIZE * wiggle_factor, 
                );

                Instance {
                    position: vec3(
                        x as f32 * instance_offset.0 + offset.0 + w.x,
                        y as f32 * instance_offset.1 + offset.1 + w.y,
                        z as f32 * instance_offset.2 + offset.2 + w.z),
                    color: vec3(
                        x as f32 / width as f32 * b, 
                        y as f32 / height as f32 * b, 
                        0.8 * b,
                    ),
                }
            })
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
