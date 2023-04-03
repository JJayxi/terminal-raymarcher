use math_vector::Vector;
use rayscii::*;

use std::f32::consts;
use std::time::Instant;

mod camera;
mod raymarcher;
mod scene;
use camera::Camera;
use raymarcher::render;
use scene::*;

use std::{thread, time::Duration};

fn main() {
    let screen = Screen::new(150, 50);

    let camera = Camera::new(
        Vector::new(0.0, 0.0, 0.0),
        90.0,
        (screen.width, screen.height),
        1.7,
    );
    let mut scene_objs: Vec<Box<dyn Object>> = Vec::new();
    //scene_objs.push(Box::new(
    //    Sphere::new(Vector::new(20.0, 10.0, -80.0), 40.0)
    //));

    scene_objs.push(Box::new(Plane::new(
        Vector::new(0.0, 30.0, 0.0),
        Vector::new(0.1, -1.0, 0.0),
    )));

    scene_objs.push(Box::new(SmoothUnion::new(
        Box::new(Donut::new(
            Vector::new(30.0, 0.0, -60.0),
            Vector::new(consts::PI / 4.0, 0.0, 0.0),
            30.0,
            7.0,
        )),
        Box::new(Cuboid::new(
            Vector::new(-30.0, 0.0, -60.0),
            Vector::new(0.0, 0.0, 0.0),
            Vector::new(20.0, 20.0, 20.0),
        )),
        Vector::new(0.0, 0.0, -60.0),
        false,
        40.0,
    )));

    let mut scene = Scene::new(scene_objs);

    loop {
        let now = Instant::now();
        render(&screen, &camera, &scene, 30, 100.0);
        let elapsed = now.elapsed();
        println!("Frame time: {:?}", elapsed);

        let frame_time: u64 = 100;
        if elapsed < Duration::from_millis(frame_time) {
            let wait_time = Duration::from_millis(frame_time) - elapsed;
            thread::sleep(wait_time);
        }

        //let sinv = ((_i as f32) / 10.0).cos() * 3.0;

        //scene.objects[0].rotate_by(Vector::new(0.05, 0.1, 0.13));
        //scene.objects[0].move_by(Vector::new(sinv, 0.0, 0.0));
        scene.objects[1].rotate_by(Vector::new(0.1, 0.2, 0.15));
        //scene.objects[1].move_by(Vector::new(0.0, 0.0, sinv / 2.0));
    }
}
