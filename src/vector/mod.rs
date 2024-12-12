use core::f64;

pub mod ops;
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    vec3: [f64; 3],
}

impl Vec3 {
    pub fn from_array(arr: &[f64; 3]) -> Self {
        Self { vec3: *arr }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { vec3: [e0, e1, e2] }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.vec3[0] * other.vec3[0] + self.vec3[1] * other.vec3[1] + self.vec3[2] * other.vec3[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::from_array(&[
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.length();
        if mag > 0.0 {
            return Vec3::from_array(&[
                (self.vec3[0] / mag),
                (self.vec3[1] / mag),
                (self.vec3[2] / mag),
            ]);
        } else {
            return Vec3::zero();
        }
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }

    pub fn zero() -> Vec3 {
        Vec3::from_array(&[0.0, 0.0, 0.0])
    }
}

pub type Point = Vec3;
impl Point {
    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }
}
