use std::f32;


#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    length: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector {
            x: x,
            y: y,
            z: z,
            length: -1.0,
        }
    }

    pub fn crossProduct(&self, other: &Vector) -> Vector {
        Vector {
            x: (self.y * other.z) - (other.y * self.z),
            y: (self.z * other.x) - (other.z * self.x),
            z: (self.x * other.y) - (other.x * self.y), 
            
            length: -1.0,
        }
    }

    pub fn length(&mut self) -> f32 {
        if (self.length < 0.0) {
            self.length = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
        }
        self.length
    }

    pub fn normalize(&mut self) {
        let length = self.length();

        self.x /= length;
        self.y /= length;
        self.z /= length;

        self.length = 1.0;
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        let x = (self.x * other.x) + (self.y * other.y) + (self.z * other.z);

        x
    }

    pub fn angle(&mut self, other: &mut Vector) -> f32 {
        (self.dot(other) / (self.length() * other.length())).acos()
    }

    pub fn sub(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            length: -1.0,
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            
            length: -1.0,
        }
    }

    pub fn mult(&self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            
            length: -1.0,
        }
    }
}