use std::f32;


#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn crossProduct(&self, other: &Vector) -> Vector {
        Vector {
            x: (self.y * other.z) - (other.y * self.z),
            y: (self.z * other.x) - (other.z * self.x),
            z: (self.x * other.y) - (other.x * self.y), 
        }
    }

    pub fn normalize(&mut self) {
        let length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();

        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) 
    }

    pub fn angle(&self, other: &Vector) -> f32 {
        self.dot(other).acos()
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}