use vector::Vector;
use bmp::Pixel;

pub struct Globe {
    pub position: Vector,
    pub radius: f32,
    pub color: Pixel,
    pub light_defraction: f32,
    pub light_intensity: f32,
}