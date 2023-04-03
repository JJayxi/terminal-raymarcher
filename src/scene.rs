use math_vector::Vector;

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
}
impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Scene {
        Scene { objects }
    }
}
impl Scene {
    pub fn sdf(&self, point: Vector<f32>) -> f32 {
        let mut min_distance = std::f32::MAX;
        for object in &self.objects {
            let distance = object.sdf(point);
            if distance < min_distance {
                min_distance = distance;
            }
        }
        min_distance
    }
}

pub trait Object {
    fn sdf(&self, point: Vector<f32>) -> f32;
    fn move_by(&mut self, by : Vector<f32>);
}

pub struct Sphere {
    pub position: Vector<f32>,
    pub radius: f32,
}
impl Sphere {
    pub fn new(position: Vector<f32>, radius: f32) -> Sphere {
        Sphere { position, radius }
    }
}

impl Object for Sphere {
    fn sdf(&self, point: Vector<f32>) -> f32 {
        (point - self.position).length() - self.radius
    }

    fn move_by(&mut self, by : Vector<f32>) {
        self.position = self.position + by;
    }
}
