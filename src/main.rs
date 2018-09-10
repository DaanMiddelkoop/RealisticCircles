extern crate bmp;

mod vector;
mod world;
mod shapes;

use bmp::Pixel;
use vector::Vector;
use world::World;
use shapes::Globe;

use std::f32;

fn main() {
    println!("Hello, world!");

    let mut bitmap = bmp::open("test_image.bmp").unwrap();

    let position = Vector::new(0.0, 0.0, 0.0);

    let world = World::new();

    for j in 0..bitmap.get_width() {
        for i in 0..bitmap.get_height() {
            bitmap.set_pixel(i, j, trace_pixel(i as f32, j as f32, position, &world));
        }
    }

    bitmap.save("test_image.bmp").unwrap();

}

fn trace_pixel(mut x: f32, mut y: f32, position: Vector, world: &World) -> Pixel {

    // let dir_x = cos(vertical_angle) * cos(horizontal_angle)
    // let dir_y = sin(vertical_angle)
    // let dir_z = cos(vertical_angle) * sin(horizontal_angle)

    let direction = Vector::new(1.0, 0.0, 0.0);

    let mut normal_x = Vector::new(0.0, 0.0, 1.0);
    let mut normal_y = direction.crossProduct(&normal_x);

    //normal_x = world.Vector(-math.sin(horizontal_angle), 0, math.cos(horizontal_angle))
    //normal_y = self.direction.crossProduct(normal_x)

    normal_x.normalize();
    normal_y.normalize();

    x /= 1000.0;
    y /= 1000.0;
    x -= 0.5;
    y -= 0.5;


    let trace_direction = Vector::new(
        direction.x + (normal_x.x * x) + (normal_y.x * y),
        direction.y + (normal_x.y * x) + (normal_y.y * y),
        direction.z + (normal_x.z * x) + (normal_y.z * y),
    );

    let mut result = trace_ray(position, trace_direction, world, 1);
    if (result.hit) {
        

        

        return result.color;
    }
    Pixel::new(0, 0, 0)
} 

fn trace_ray(position: Vector, direction: Vector, world: &World, bounces: i32) -> TraceResult {
    // Handle other objects here to when added.
    

    trace_spheres(position, direction, world, bounces)
}



fn trace_spheres(position: Vector, direction: Vector, world: &World, bounces: i32) -> TraceResult {
    let mut best_result = TraceResult::new();

    let mut closest_dist = f32::INFINITY;

    for x in &world.spheres {
        let result = trace_sphere(position, direction, x, world, bounces);

        if (result.hit && result.distance < closest_dist) {
            closest_dist = result.distance;

            best_result = result;
        }
    }

    best_result
}

fn trace_sphere(position: Vector, mut direction: Vector, sphere: &Globe, world: &World, bounces: i32) -> TraceResult {
    let direction_to_center = sphere.position.sub(&position);

    let angle = direction_to_center.angle(&direction);

    let b = angle.sin() * direction_to_center.length();


    if (b >= sphere.radius || angle > f32::consts::PI / 2.0) {
        return TraceResult::new();
    }

    let radius = sphere.radius;
    let g = ((radius * radius) - (b * b)).sqrt();
    let h = ((direction_to_center.length() * direction_to_center.length()) - (b * b)).sqrt();

    // println!("direction: {:?}", direction);
    // println!("position: {:?}", position);
    // println!("dir_center: {:?}", direction_to_center);
    // println!("angle: {:?}", angle);
    // println!("direction_to_center: {:?}", direction_to_center.length());
    // println!("b: {:?}", b);
    // println!("g: {:?}", g);
    // println!("h: {:?}", h);

    if (angle > f32::consts::PI / 2.0) {
        println!("what the fuck?");
    }
    let hit_distance = h - g - 0.000001;

    direction.normalize(); 
    
    let hitpoint = position.add(&direction.mult(hit_distance));

    let result = TraceResult {
        hit: true,
        color: sphere.color,
        distance: hit_distance,
        hitpoint: hitpoint,
        normal: hitpoint.sub(&sphere.position),
        light_intensity: 0.0,
    };
    if (bounces > 0) {
        return trace_lights(result, hitpoint, world, bounces - 1);
    }

    result
}

pub fn trace_lights(mut result: TraceResult, position: Vector, world: &World, bounces: i32) -> TraceResult {
    let mut total_color = Vector::new(1.0, 1.0, 1.0);

    let mut hit_something = false;

    for light in &world.lights {
        let direction_to_light = light.position.sub(&position);

        let new_result = trace_ray(position, direction_to_light, world, 0);
        if (new_result.hit && new_result.distance < direction_to_light.length()) {
            hit_something = true;
        } else {
            // calculate color influenced by this light.

            let mut intensity = light.intensity / direction_to_light.length();

            let light_angle = result.normal.angle(&direction_to_light);
            
            intensity *= light_angle.cos().abs();

            result.light_intensity += intensity;
        }

        // trace soft shadows?
    }

    

    let original_color = result.color;

    result.color.r = (result.color.r as f32 * total_color.x) as u8;
    result.color.g = (result.color.g as f32 * total_color.y) as u8;
    result.color.b = (result.color.b as f32 * total_color.z) as u8;

    result.color.r = (result.color.r as f32 * result.light_intensity).min(original_color.r as f32) as u8;
    result.color.g = (result.color.g as f32 * result.light_intensity).min(original_color.g as f32) as u8;
    result.color.b = (result.color.b as f32 * result.light_intensity).min(original_color.b as f32) as u8;

    result
}

pub fn trace_light(position: Vector, direction: Vector, world: &World) -> TraceResult {
    

    TraceResult::new()
}

pub struct TraceResult {
    hit: bool,
    distance: f32,
    color: Pixel,
    hitpoint: Vector,
    normal: Vector,
    light_intensity: f32,
}

impl TraceResult {
    pub fn new() -> Self {
        TraceResult {
            hit: false,
            distance: 0.0,
            color: Pixel::new(0, 0, 0),
            hitpoint: Vector::new(0.0, 0.0, 0.0),
            normal: Vector::new(0.0, 0.0, 0.0),
            light_intensity: 0.0,
        }
    }
}