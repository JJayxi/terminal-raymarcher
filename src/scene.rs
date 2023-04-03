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
    fn rotate_by(&mut self, _by : Vector<f32>) {
        
    }
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

pub struct Plane {
    pub position: Vector<f32>,
    pub normal: Vector<f32>,
}
impl Plane {
    pub fn new(position: Vector<f32>, normal: Vector<f32>) -> Plane {
        Plane { position, normal }
    }
}
impl Object for Plane {
    fn sdf(&self, point: Vector<f32>) -> f32 {
        Vector::dot(point - self.position, self.normal)
    }

    fn move_by(&mut self, _by : Vector<f32>) {
        self.position = self.position + _by;
    }
}

pub struct Donut {
    pub position: Vector<f32>,
    pub rotation: Vector<f32>,
    pub radius: f32,
    pub thickness: f32,
}
impl Donut {
    pub fn new(position: Vector<f32>, rotation: Vector<f32>, radius: f32, thickness: f32) -> Donut {
        Donut { position, rotation, radius, thickness}
    }
}

impl Object for Donut {
    fn sdf(&self, point: Vector<f32>) -> f32 {
        let mut p = point - self.position;
        p = p.rotate_x(self.rotation.x);
        p = p.rotate_y(self.rotation.y);
        p = p.rotate_z(self.rotation.z);
        let mut q = Vector::new(p.x, p.y, 0.0);
        q = q.normalize() * self.radius;
        let mut d = (p - q).length() - self.thickness;
        d = d.abs();
        d
    }

    fn move_by(&mut self, by : Vector<f32>) {
        self.position = self.position + by;
    }

    fn rotate_by(&mut self, by : Vector<f32>) {
        self.rotation = self.rotation + by;
    }
}


