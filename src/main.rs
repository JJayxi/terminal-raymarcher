use math_vector::Vector;
use rayscii::*;

use std::{f32::consts, time::Instant};

mod camera;
mod raymarcher;
mod scene;
use camera::Camera;
use colored::Color;
use raymarcher::render;
use scene::*;

use std::{thread, time::Duration};

fn main() {
    let screen = Screen::new(150, 50);

    let camera = Camera::new(
        Vector::new(0.0, 0.0, 10.0),
        Vector::new(0.0, 0.0, 0.0),
        90.0,
        (screen.width, screen.height),
        1.7,
    );
    let mut scene_objs: Vec<Object> = Vec::new();

    //scene_objs.push(Box::new(Plane::new_with_color(
    //    Vector::new(0.0, -60.0, 0.0),
    //    Vector::new(0., 1.0, 0.0),
    //    Color::TrueColor {
    //        r: (230),
    //        g: (255),
    //        b: (230),
    //    },
    //)));

    scene_objs.push(Object::new(
        Vector::new(0.0, -60.0, 0.0),
        Vector::new(0.0, 0.0, 0.0),
        Color::TrueColor {
            r: (230),
            g: (255),
            b: (230),
        },
        Shape::Plane {
            normal: Vector::new(0.0, 1.0, 0.0),
        },
    ));
    {
        let donut = Object {
            position: Vector::new(-30.0, 0.0, -60.0),
            rotation: Vector::new(consts::PI / 4.0, 0.0, 0.0),
            color: Color::TrueColor {
                r: 255,
                g: 100,
                b: 100,
            },
            shape: Shape::Donut {
                radius: 32.0,
                thickness: 8.0,
            },
        };

        let cuboid = Object {
            position: Vector::new(30.0, 0.0, -60.0),
            rotation: Vector::new(0.0, 0.0, 0.0),
            color: Color::TrueColor {
                r: 150,
                g: 150,
                b: 255,
            },
            shape: Shape::Cuboid {
                size: Vector::new(20.0, 20.0, 20.0),
            },
        };

        let union = Object {
            position: Vector::new(0.0, 0.0, -60.0),
            rotation: Vector::new(0.0, 0.0, 0.0),
            color: Color::TrueColor { r: 0, g: 0, b: 0 },
            shape: Shape::SmoothUnion {
                shape1: Box::new(donut),
                shape2: Box::new(cuboid),
                k: 30.0,
            },
        };

        scene_objs.push(union);
    }

    let light_position = Vector::new(0.0, 50.0, 0.0);
    let mut scene = Scene::new(scene_objs, light_position);

    let mut _frame_count = 0;
    loop {
        let now = Instant::now();
        render(&screen, &camera, &scene, 60, 400.0);
        let elapsed = now.elapsed();
        println!("Frame time: {:?}", elapsed);
        thread::sleep(Duration::from_millis(10));
        /*let frame_time: u64 = 100;
        if elapsed < Duration::from_millis(frame_time) {
            let wait_time = Duration::from_millis(frame_time) - elapsed;
            thread::sleep(wait_time);
        }*/

        //scene.objects[1].rotate_by(Vector::new(0.07, 0.15, 0.11));
        scene.objects[1].rotate_by(Vector::new(0.07, 0.15, 0.11));
        _frame_count += 1;

        //camera.rotate_by(Vector::new(0.0, 0.0, 0.0));
    }
}
