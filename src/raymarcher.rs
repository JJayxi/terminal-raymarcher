use math_vector::Vector;

use crate::scene::*;
use crate::Screen;
use crate::camera::*;



pub fn render(screen: &Screen, camera: &Camera, scene: &Scene, max_iterations: u32, max_distance: f32) {
    let brightnesschars = [' ', '.', ',', '-', '=', '+', '*', '#', '%', '@'];

    print!("\x1B[2J\x1B[1;1H");
    let mut screen_str = String::with_capacity(((screen.width + 1) * screen.height) as usize);

    for y in 0..screen.height {
        for x in 0..screen.width {
            let brightness = render_pixel(camera, scene, x, y, max_iterations, max_distance);
            let character = brightnesschars[(brightness * 10.0) as usize];
            screen_str.push(character);
        }
        screen_str.push('\n');
    }

    print!("{}", screen_str);
}

pub fn render_pixel(camera: &Camera, scene: &Scene, x: u32, y: u32, max_iterations: u32, max_distance: f32) -> f32 {
    let distance = ray_march(camera, scene, x, y, max_iterations, max_distance);
    //println!("{} {} {}", x, y, distance);
    let brightness = 1.0 - (distance / max_distance).min(1.0);
    brightness_adjust(brightness)
}

fn brightness_adjust(brightness: f32) -> f32 {
    brightness.powf(1.0 / 2.2)
}

pub fn ray_march(camera: &Camera, scene: &Scene, x: u32, y: u32, max_iterations: u32, max_distance: f32) -> f32 {
    let ray = create_ray(camera, x, y);
    let mut distance = 0.0;
    for _i in 0..max_iterations {
        let point = ray.origin + ray.direction * distance;
        let sdf = scene.sdf(point);
        //println!("Distance: {} SDF: {}", distance, sdf);
        if sdf < 0.01 {
            return distance;
        }
        distance += sdf;
        if distance > max_distance {
            return max_distance;
        }
    }

    return distance;
}

pub struct Ray {
    pub origin: Vector<f32>,
    pub direction: Vector<f32>
}

impl Ray {
    pub fn new(origin: Vector<f32>, direction: Vector<f32>) -> Ray {
        Ray { origin, direction }
    }
}