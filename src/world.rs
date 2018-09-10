use std::vec::Vec;
use shapes::Globe;
use vector::Vector;
use bmp::Pixel;

pub struct World {
    pub spheres: Vec<Globe>,
    pub lights: Vec<Light>,
}

pub struct Light {
    pub position: Vector,
    pub intensity: f32,
}

impl World {
    pub fn new() -> Self {
        let mut globes = Vec::new();
        globes.push(Globe {
            position: Vector::new(10.0, 0.0, 0.0),
            radius: 2.0,
            color: Pixel::new(0, 255, 0),
        });
        globes.push(Globe {
            position: Vector::new(7.0, 2.0, 2.0),
            radius: 0.5,
            color: Pixel::new(0, 255, 0),
        });

        let mut lights = Vec::new();
        lights.push(Light {
            position: Vector::new(0.0, 10.0, 10.0),
            intensity: 5.0,
        });
        lights.push(Light {
            position: Vector::new(0.0, 10.0, 10.0),
            intensity: 5.0,
        });


        World {
            spheres: globes,
            lights: lights,
        }
    }
}