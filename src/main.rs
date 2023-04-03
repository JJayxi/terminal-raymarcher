use rayscii::*;
use math_vector::Vector;

use std::time::Instant;
use std::f32::consts;

mod camera;
mod raymarcher;
mod scene;
use camera::Camera;
use scene::*;
use raymarcher::render;

use std::{thread, time::Duration};


fn main() {    

    let screen = Screen::new(150, 50);

    let camera = Camera::new(Vector::new(0.0, 0.0, 0.0), 90.0, (screen.width, screen.height), 1.7);
    let mut scene_objs: Vec<Box<dyn Object>>= Vec::new();
    scene_objs.push(Box::new(Sphere::new(Vector::new(20.0, 10.0, -80.0), 40.0)));
    scene_objs.push(Box::new(Plane::new(Vector::new(0.0, 30.0, 0.0), Vector::new(0.1, -1.0, 0.0))));
    scene_objs.push(Box::new(Donut::new(Vector::new(-40.0, 0.0, -60.0), Vector::new(consts::PI / 4.0, 0.0, 0.0), 30.0, 7.0)));
    let mut scene = Scene::new(scene_objs);

    

    for _i in 0..300 {
        let now = Instant::now();
        render(&screen, &camera, &scene, 15, 100.0);
        let elapsed = now.elapsed();
        println!("Frame time: {:?}", elapsed);
        
        if elapsed < Duration::from_millis(120) {
            let wait_time = Duration::from_millis(120) - elapsed;
            thread::sleep(wait_time);
        }
        
        
        let sinv = ((_i as f32) / 10.0).cos() * 3.0;

        scene.objects[0].move_by(Vector::new(sinv, 0.0, 0.0));
        scene.objects[2].rotate_by(Vector::new(0.1, 0.0, 0.0));
    }
}
