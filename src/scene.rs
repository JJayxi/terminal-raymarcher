use math_vector::Vector;
use std::marker::Sync;
use colored::Color;

pub trait Object: Sync {
    fn sdf(&self, point: &Vector<f32>) -> f32;
    fn position(&self) -> Vector<f32>;
    fn move_by(&mut self, by: Vector<f32>);
    fn rotate_by(&mut self, _by: Vector<f32>) {}
    fn get_color(&self) -> Color;
    
}
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub light_position: Vector<f32>,
}
#[allow(dead_code)]
impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>, light_position: Vector<f32>) -> Scene {
        Scene {
            objects,
            light_position,
        }
    }
}
impl Scene {
    pub fn sdf(&self, point: &Vector<f32>) -> (f32, Color) {
        let mut min_distance = std::f32::MAX;
        let mut color = Color::White;
        for object in &self.objects {
            let distance = object.sdf(&point);
            if distance < min_distance {
                min_distance = distance;
                color = object.get_color();
            }
        }
        (min_distance, color)
    }
}

pub struct Sphere {
    pub position: Vector<f32>,
    pub radius: f32,
    pub color: Color,
}
#[allow(dead_code)]
impl Sphere {
    pub fn new(position: Vector<f32>, radius: f32) -> Sphere {
        Sphere { position, radius, color: Color::White }
    }

    pub fn new_with_color(position: Vector<f32>, radius: f32, color: Color) -> Sphere {
        Sphere { position, radius, color }
    }
}
impl Object for Sphere {
    fn sdf(&self, point: &Vector<f32>) -> f32 {
        (point.clone() - self.position).length() - self.radius
    }

    fn move_by(&mut self, by: Vector<f32>) {
        self.position = self.position + by;
    }

    fn position(&self) -> Vector<f32> {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

pub struct Plane {
    pub position: Vector<f32>,
    pub normal: Vector<f32>,
    pub color: Color,
}
#[allow(dead_code)]
impl Plane {
    pub fn new(position: Vector<f32>, normal: Vector<f32>) -> Plane {
        Plane { position, normal, color: Color::White }
    }

    pub fn new_with_color(position: Vector<f32>, normal: Vector<f32>, color: Color) -> Plane {
        Plane { position, normal, color }
    }
}
impl Object for Plane {
    fn sdf(&self, point: &Vector<f32>) -> f32 {
        Vector::dot(point.clone() - self.position, self.normal)
    }

    fn move_by(&mut self, _by: Vector<f32>) {
        self.position = self.position + _by;
    }

    fn rotate_by(&mut self, by: Vector<f32>) {
        self.normal = self.normal.rotate_x(by.x).rotate_y(by.y).rotate_z(by.z);
    }

    fn position(&self) -> Vector<f32> {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

pub struct Donut {
    pub position: Vector<f32>,
    pub rotation: Vector<f32>,
    pub radius: f32,
    pub thickness: f32,
    pub color: Color,
}
#[allow(dead_code)]
impl Donut {
    pub fn new(position: Vector<f32>, rotation: Vector<f32>, radius: f32, thickness: f32) -> Donut {
        Donut {
            position,
            rotation,
            radius,
            thickness,
            color: Color::White,
        }
    }

    pub fn new_with_color(
        position: Vector<f32>,
        rotation: Vector<f32>,
        radius: f32,
        thickness: f32,
        color: Color,
    ) -> Donut {
        Donut {
            position,
            rotation,
            radius,
            thickness,
            color,
        }
    }
}
impl Object for Donut {
    fn sdf(&self, point: &Vector<f32>) -> f32 {
        let mut p = point.clone() - self.position;
        p = p
            .rotate_x(self.rotation.x)
            .rotate_y(self.rotation.y)
            .rotate_z(self.rotation.z);
        let mut q = Vector::new(p.x, p.y, 0.0);
        q = q.normalize() * self.radius;
        let mut d = (p - q).length() - self.thickness;
        d = d.abs();
        d
    }

    fn move_by(&mut self, by: Vector<f32>) {
        self.position = self.position + by;
    }

    fn rotate_by(&mut self, by: Vector<f32>) {
        self.rotation = self.rotation + by;
    }

    fn position(&self) -> Vector<f32> {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

pub struct Cuboid {
    pub position: Vector<f32>,
    pub rotation: Vector<f32>,
    pub size: Vector<f32>,
    pub color: Color,
}
#[allow(dead_code)]
impl Cuboid {
    pub fn new(position: Vector<f32>, rotation: Vector<f32>, size: Vector<f32>) -> Cuboid {
        Cuboid {
            position,
            rotation,
            size,
            color: Color::White,
        }
    }

    pub fn new_with_color(
        position: Vector<f32>,
        rotation: Vector<f32>,
        size: Vector<f32>,
        color: Color,
    ) -> Cuboid {
        Cuboid {
            position,
            rotation,
            size,
            color,
        }
    }
}
impl Object for Cuboid {
    fn sdf(&self, point: &Vector<f32>) -> f32 {
        let mut p = point - &self.position;
        p = p
            .rotate_x(self.rotation.x)
            .rotate_y(self.rotation.y)
            .rotate_z(self.rotation.z);
        let mut d = Vector::new(
            p.x.abs() - self.size.x,
            p.y.abs() - self.size.y,
            p.z.abs() - self.size.z,
        );
        d = Vector::new(d.x.max(0.0), d.y.max(0.0), d.z.max(0.0));
        d.length()
    }

    fn move_by(&mut self, by: Vector<f32>) {
        self.position = self.position + by;
    }

    fn rotate_by(&mut self, by: Vector<f32>) {
        self.rotation = self.rotation + by;
    }

    fn position(&self) -> Vector<f32> {
        self.position
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

pub struct SmoothUnion {
    pub object1: Box<dyn Object>,
    pub object2: Box<dyn Object>,
    pub center: Vector<f32>,
    pub centered: bool,
    pub k: f32,
}
#[allow(dead_code)]
impl SmoothUnion {
    pub fn new(
        object1: Box<dyn Object>,
        object2: Box<dyn Object>,
        center: Vector<f32>,
        centered: bool,
        k: f32,
    ) -> SmoothUnion {
        SmoothUnion {
            object1,
            object2,
            center,
            centered,
            k,
        }
    }
}
impl Object for SmoothUnion {
    fn sdf(&self, point: &Vector<f32>) -> f32 {
        let h = (self.object1.sdf(point) - self.object2.sdf(point) + self.k) / (2.0 * self.k);
        let h = h.max(0.0).min(1.0);
        self.object1.sdf(point) * (1.0 - h) + self.object2.sdf(point) * h - self.k * h * (1.0 - h)
    }

    fn move_by(&mut self, by: Vector<f32>) {
        self.object1.move_by(by);
        self.object2.move_by(by);
        self.center = self.center + by;
    }

    fn rotate_by(&mut self, by: Vector<f32>) {
        self.object1.rotate_by(by);
        self.object2.rotate_by(by);

        if self.centered {
            let mut p = self.object1.position() - self.center;
            p = p.rotate_x(by.x).rotate_y(by.y).rotate_z(by.z);
            self.object1.move_by(p);

            let mut p = self.object2.position() - self.center;
            p = p.rotate_x(by.x).rotate_y(by.y).rotate_z(by.z);
            self.object2.move_by(p);
            panic!("Not implemented correctly yet!");
        }
    }

    fn position(&self) -> Vector<f32> {
        self.center
    }

    fn get_color(&self) -> Color {
        panic!("TODO: Implement get_color for SmoothUnion, will need another parameter for position");
    }
}
