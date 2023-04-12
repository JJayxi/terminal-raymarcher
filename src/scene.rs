use colored::Color;
use math_vector::Vector;
use std::marker::Sync;

pub trait Object: Sync {
    fn sdf(&self, point: &Vector<f32>) -> (Color, f32);
    fn position(&self) -> Vector<f32>;
    fn move_by(&mut self, by: Vector<f32>);
    fn rotate_by(&mut self, _by: Vector<f32>) {}
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
            let (ocolor, distance) = object.sdf(&point);
            if distance < min_distance {
                min_distance = distance;
                color = ocolor;
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
        Sphere {
            position,
            radius,
            color: Color::White,
        }
    }

    pub fn new_with_color(position: Vector<f32>, radius: f32, color: Color) -> Sphere {
        Sphere {
            position,
            radius,
            color,
        }
    }
}
impl Object for Sphere {
    fn sdf(&self, point: &Vector<f32>) -> (Color, f32) {
        (
            self.color,
            (point.clone() - self.position).length() - self.radius,
        )
    }

    fn move_by(&mut self, by: Vector<f32>) {
        self.position = self.position + by;
    }

    fn position(&self) -> Vector<f32> {
        self.position
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
        Plane {
            position,
            normal,
            color: Color::White,
        }
    }

    pub fn new_with_color(position: Vector<f32>, normal: Vector<f32>, color: Color) -> Plane {
        Plane {
            position,
            normal,
            color,
        }
    }
}
impl Object for Plane {
    fn sdf(&self, point: &Vector<f32>) -> (Color, f32) {
        (
            self.color,
            Vector::dot(point.clone() - self.position, self.normal),
        )
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
    fn sdf(&self, point: &Vector<f32>) -> (Color, f32) {
        let mut p = point.clone() - self.position;
        p = p
            .rotate_x(self.rotation.x)
            .rotate_y(self.rotation.y)
            .rotate_z(self.rotation.z);
        let mut q = Vector::new(p.x, p.y, 0.0);
        q = q.normalize() * self.radius;
        let mut d = (p - q).length() - self.thickness;
        d = d.abs();
        (self.color, d)
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
    fn sdf(&self, point: &Vector<f32>) -> (Color, f32) {
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
        (self.color, d.length())
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
    fn sdf(&self, point: &Vector<f32>) -> (Color, f32) {
        let o1 = self.object1.sdf(point);
        let o2 = self.object2.sdf(point);
        let h = (o1.1 - o2.1 + self.k) / (2.0 * self.k);
        let h = h.max(0.0).min(1.0);
        let dist = o1.1 * (1.0 - h) + o2.1 * h - self.k * h * (1.0 - h);

        if let (
            (Color::TrueColor { r: r1, g: g1, b: b1 }, _), 
            (Color::TrueColor { r: r2, g: g2, b: b2 }, _)) = 
            (o1,o2) {
                let r = ((r1 as f32) * (1.0 - h) + (r2 as f32) * h) as u8;
                let g = ((g1 as f32) * (1.0 - h) + (g2 as f32) * h) as u8;
                let b = ((b1 as f32) * (1.0 - h) + (b2 as f32) * h) as u8;
                let color = Color::TrueColor { r, g, b };
                return (color, dist);
            
        };
        panic!("OBJECTS IN SMOOTH UNION MUST HAVE TRUECOLOR");
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
}
