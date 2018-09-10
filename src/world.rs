use std::vec::Vec;
use shapes::Globe;
use vector::Vector;

pub struct World {
    pub spheres: Vec<Globe>,
}

impl World {
    pub fn new() -> Self {
        let mut globes = Vec::new();
        globes.push(Globe {
            position: Vector::new(10.0, 0.0, 0.0),
            radius: 2.0,
        });


        World {
            spheres: globes,
        }
    }
}