use colored::Color;
use math_vector::Vector;
use std::marker::Sync;

pub struct Scene {
    pub objects: Vec<Object>,
    pub light_position: Vector<f32>,
}
#[allow(dead_code)]
impl Scene {
    pub fn new(objects: Vec<Object>, light_position: Vector<f32>) -> Scene {
        Scene {
            objects,
            light_position,
        }
    }

    pub fn sdf(&self, point: &Vector<f32>) -> (f32, Color) {
        let mut min_distance = std::f32::MAX;
        let mut color = Color::TrueColor {
            r: 255,
            g: 255,
            b: 255,
        };
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

pub struct Object {
    pub position: Vector<f32>,
    pub rotation: Vector<f32>,
    pub color: Color,
    pub shape: Shape,
}

#[allow(dead_code)]
pub enum Shape {
    Sphere { radius: f32 },
    Plane { normal: Vector<f32> },
    Donut { radius: f32, thickness: f32 },
    Cuboid { size: Vector<f32> },
    SmoothUnion {
        shape1: Box<Object>,
        shape2: Box<Object>,
        k: f32,
    },
}

unsafe impl Sync for Object {}

#[allow(dead_code)]
impl Object {
    pub fn new(position: Vector<f32>, rotation: Vector<f32>, color: Color, shape: Shape) -> Object {

        match color {
            Color::TrueColor { .. } => {}
            _ => panic!("Color is not TrueColor"),
        }

        Object {
            position,
            rotation,
            color,
            shape,
        }
    }
    
    pub fn move_by(&mut self, by: Vector<f32>) {
        self.position = self.position + by;
    }

    pub fn rotate_by(&mut self, by: Vector<f32>) {
        self.rotation = self.rotation + by;
        
        if let Shape::SmoothUnion { shape1, shape2, .. } = &mut self.shape {
            shape1.rotate_by(by);
            shape2.rotate_by(by);
        }   
    }

    pub fn sdf(&self, point: &Vector<f32>) -> (Color, f32) {
        self.shape
            .sdf(&self.position, &self.rotation, &self.color, point)
    }
}

impl Shape {
    pub fn sdf<'a>(
        &'a self,
        position: &Vector<f32>,
        rotation: &Vector<f32>,
        color: &Color,
        point: &Vector<f32>,
    ) -> (Color, f32) {
        match self {
            Shape::Sphere { radius } => {
                let distance = (point - &Vector::new(0.0, 0.0, 0.0)).length() - radius;
                (*color, distance)
            }
            Shape::Plane { normal } => {
                let distance = Vector::dot(point - position, *normal);
                (*color, distance)
            }
            Shape::Donut { radius, thickness } => {
                let mut p = point - position;
                p = p
                    .rotate_x(rotation.x)
                    .rotate_y(rotation.y)
                    .rotate_z(rotation.z);
                let mut q = Vector::new(p.x, p.y, 0.0);
                q = q.normalize() * *radius;
                let mut d = (p - q).length() - *thickness;
                d = d.abs();
                (*color, d)
            }
            Shape::Cuboid { size } => {
                let mut p = point - position;
                p = p
                    .rotate_x(rotation.x)
                    .rotate_y(rotation.y)
                    .rotate_z(rotation.z);
                let mut d = Vector::new(
                    p.x.abs() - size.x,
                    p.y.abs() - size.y,
                    p.z.abs() - size.z,
                );
                d = Vector::new(d.x.max(0.0), d.y.max(0.0), d.z.max(0.0));
                (*color, d.length())
            }
            Shape::SmoothUnion { shape1, shape2, k } => {
                let o1 = shape1.sdf(point);
                let o2 = shape2.sdf(point);
                let h = (o1.1 - o2.1 + k) / (2.0 * k);
                let h = h.max(0.0).min(1.0);
                let dist = o1.1 * (1.0 - h) + o2.1 * h - k * h * (1.0 - h);
        
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
                //unreachable
                panic!("Color is not TrueColor, but this is unreachable");
            }
        }
    }
}