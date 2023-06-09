use math_vector::Vector;

use crate::camera::*;
use crate::scene::*;
use crate::Screen;
use colored::Color;
use colored::*;
use rayon::prelude::*;

pub fn render(
    screen: &Screen,
    camera: &Camera,
    scene: &Scene,
    max_iterations: u32,
    max_distance: f32,
) {
    print!("\x1B[2J\x1B[1;1H");

    let screen_str = (0..screen.height)
        .into_par_iter()
        .map(|y| {
            (0..screen.width)
                .into_iter()
                .map(|x| {
                    let color = render_pixel(camera, &scene, x, y, max_iterations, max_distance);
                    ' '.to_string().on_color(color).color(color).to_string()
                })
                .collect::<String>()
                + "\n"
        })
        .collect::<String>();

    print!("{}", screen_str);
}

pub fn render_pixel(
    camera: &Camera,
    scene: &Scene,
    x: u32,
    y: u32,
    max_iterations: u32,
    max_distance: f32,
) -> Color {
    let mut ray = create_ray(camera, x, y);
    let distance = ray_march(&mut ray, scene, max_iterations, max_distance);
    if distance >= max_distance {
        return Color::Black;
    }
    let point = ray.origin + ray.direction * distance;
    let normal = find_normal(scene, &point);

    let light_direction = (scene.light_position - point).normalize();

    let mut light_ray = Ray::new(point + normal, light_direction);
    let light_distance = point.distance(scene.light_position);

    let lightray_distance = ray_march(&mut light_ray, scene, max_iterations, light_distance);

    let mut brightness = Vector::dot(normal, light_direction).max(0.0); // * penumbra

    if lightray_distance < light_distance {
        let penumbra = (light_distance - lightray_distance) / light_distance;
        brightness *= 1.0 - penumbra;
        if let Color::TrueColor { r, g, b } = ray.color {
            return Color::TrueColor {
                r: ((r as f32) * brightness) as u8,
                g: ((g as f32) * brightness) as u8,
                b: ((b as f32) * brightness) as u8,
            };
        }
        panic!("Color is not true color");
    }

    if let Color::TrueColor { r, g, b } = ray.color {
        return Color::TrueColor {
            r: ((r as f32) * brightness) as u8,
            g: ((g as f32) * brightness) as u8,
            b: ((b as f32) * brightness) as u8,
        };
    }

    ray.color
}

pub fn ray_march(ray: &mut Ray, scene: &Scene, max_iterations: u32, max_distance: f32) -> f32 {
    let mut distance = 0.0;
    for _i in 0..max_iterations {
        let point = ray.origin + ray.direction * distance;
        let (sdf, color) = scene.sdf(&point);

        if sdf < ray.closest_miss {
            ray.color = color;
            ray.closest_miss = sdf;
        }

        if sdf < 1.0 {
            return distance;
        }

        distance += sdf;
        if distance > max_distance {
            return max_distance;
        }
        ray.march_count += 1;
    }

    return distance;
}

pub fn find_normal(scene: &Scene, point: &Vector<f32>) -> Vector<f32> {
    let epsilon: f32 = -0.001;

    let psdf = scene.sdf(point).0;

    let normal = Vector::new(
        psdf - scene.sdf(&(point + &Vector::new(epsilon, 0.0, 0.0))).0,
        psdf - scene.sdf(&(point + &Vector::new(0.0, epsilon, 0.0))).0,
        psdf - scene.sdf(&(point + &Vector::new(0.0, 0.0, epsilon))).0,
    );

    normal.normalize()
}

pub struct Ray {
    pub origin: Vector<f32>,
    pub direction: Vector<f32>,
    pub closest_miss: f32,
    pub march_count: u32,
    pub color: Color,
}

impl Ray {
    pub fn new(origin: Vector<f32>, direction: Vector<f32>) -> Ray {
        Ray {
            origin,
            direction,
            closest_miss: f32::MAX,
            march_count: 0,
            color: Color::White,
        }
    }
}
