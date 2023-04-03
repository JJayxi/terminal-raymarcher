use rayscii::*;
use math_vector::Vector;

mod camera;
mod raymarcher;
mod scene;
use camera::Camera;
use scene::Scene;
use scene::Object;
use scene::Sphere;
use raymarcher::render;

use std::{thread, time::Duration};


fn main() {    

    let screen = Screen::new(150, 50);

    let camera = Camera::new(Vector::new(0.0, 0.0, 0.0), 90.0, (screen.width, screen.height), 1.7);
    let mut scene_objs: Vec<Box<dyn Object>>= Vec::new();
    scene_objs.push(Box::new(Sphere::new(Vector::new(0.0, 0.0, -80.0), 40.0)));
    let mut scene = Scene::new(scene_objs);

    

    for _i in 0..300 {
        render(&screen, &camera, &scene, 30, 100.0);
        thread::sleep(Duration::from_millis(20));
        
        let sinv = ((_i as f32) / 10.0).cos() * 3.0;

        scene.objects[0].move_by(Vector::new(sinv, 0.0, 0.0));
    }
}
