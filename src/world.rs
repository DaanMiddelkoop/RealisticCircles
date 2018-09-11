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
        // globes.push(Globe {
        //     position: Vector::new(5.0, 0.0, -2.0),
        //     radius: 1.0,
        //     color: Pixel::new(0, 255, 0),
        //     light_defraction: 0.0,
        //     light_intensity: 0.05,
        // });

        // globes.push(Globe {
        //     position: Vector::new(5.0, 0.0, 2.0),
        //     radius: 1.0,
        //     color: Pixel::new(0, 255, 0),
        //     light_defraction: 0.1,
        //     light_intensity: 0.05,
        // });

        globes.push(Globe {
            position: Vector::new(7.0, 0.0, 0.0),
            radius: 1.0,
            color: Pixel::new(0, 255, 0),
            light_defraction: 1.0,
            light_intensity: 0.0,
        });
        globes.push(Globe {
            position: Vector::new(-7.0, 0.0, 0.0),
            radius: 1.0,
            color: Pixel::new(0, 255, 0),
            light_defraction: 0.0,
            light_intensity: 30.5,
        });

        // globes.push(Globe {
        //     position: Vector::new(-3.1, 0.0, 0.0),
        //     radius: 0.0,
        //     color: Pixel::new(0, 255, 0),
        //     light_defraction: 0.0,
        //     light_intensity: 0.0,
        // });


        World {
            spheres: globes,
            lights: Vec::new(),
        }
    }
}