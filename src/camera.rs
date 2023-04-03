use crate::raymarcher::*;
use math_vector::Vector;

pub struct Camera {
    pub position: Vector<f32>,
    pub fov: f32,
    pub resolution: (u32, u32),
    frostum_dimensions: (f32, f32)

}

impl Camera {
    pub fn new(position: Vector<f32>, fov: f32, resolution: (u32, u32), char_ar : f32) -> Camera {
        let frostum_width = (fov / 2.0).tan() * 2.0;
        let frostum_height = frostum_width * (resolution.1 as f32 / resolution.0 as f32) * char_ar;
        Camera { 
            position, 
            fov,
            resolution,
            frostum_dimensions: (frostum_width, frostum_height)
        }
    }
}

pub fn create_ray(camera : &Camera, x: u32, y: u32) -> Ray {
    let width = camera.resolution.0 as f32;
    let height = camera.resolution.1 as f32;
    let (frostum_width, frostum_height) = camera.frostum_dimensions;
    let x = (x as f32 / width) * frostum_width - (frostum_width / 2.0);
    let y = (y as f32 / height) * frostum_height - (frostum_height / 2.0);
    let direction = Vector::new(x, y, -1.0);
    Ray::new(camera.position, direction.normalize())
}