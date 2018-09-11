extern crate bmp;
extern crate rand;
extern crate threads_pool;

mod vector;
mod world;
mod shapes;

use bmp::Pixel;
use vector::Vector;
use world::World;
use shapes::Globe;

use std::sync::Arc;
use std::sync::Mutex;

use threads_pool::*;

use std::f32;

fn main() {
    println!("Hello, world!");

    let mut bitmap = bmp::open("2000x2000.bmp").unwrap();

    let position = Vector::new(0.0, 0.0, 0.0);

    let world = World::new();
    
    let pool = ThreadPool::new(100);

    let a = Arc::new(world);

    let width = bitmap.get_width();
    let height = bitmap.get_height();

    let b = Arc::new(Mutex::new(bitmap));

    let mut worker_id = pool.get_first_worker_id().unwrap() as u32;

    {
        let c = Arc::new(pool);


        for x in 0..100 {
            let lock = a.clone();
            let bmplock = b.clone();
            let poollock = c.clone();
            println!("test");
            c.clone().execute(move || {
                for l in 0..20 {
                    let j = l + (x * 20);
                    //if (x == 0) {
                    println!("{:?}: progress: {:?}", x, l);
                    //}
                    for i in 0..height {
                        let pixel = trace_pixel(i as f32, j as f32, position, &*lock);
                        
                        (*bmplock.lock().unwrap()).set_pixel(i, j, pixel);
                    }

                }
                
                //(*bmplock.lock().unwrap()).save("partialResult.bmp");
            })
        }
    }

    // let vector1 = Vector::new(1.0, 2.0, 0.0);
    // let vector2 = Vector::new(1.0, 0.0, 0.0);
    // println!("result: {:?}", randomize_light_defraction_direction(vector1, vector2, 0.0));
    let bmplock = b.clone();
    (*bmplock.lock().unwrap()).save("something_beautifull6.bmp").unwrap();

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

    x /= 2000.0;
    y /= 2000.0;
    x -= 0.5;
    y -= 0.5;


    let trace_direction = Vector::new(
        direction.x + (normal_x.x * x) + (normal_y.x * y),
        direction.y + (normal_x.y * x) + (normal_y.y * y),
        direction.z + (normal_x.z * x) + (normal_y.z * y),
    );

    let mut result = trace_ray(position, trace_direction, world, 2);
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
    let mut direction_to_center = sphere.position.sub(&position);

    let angle = direction_to_center.angle(&mut direction);

    let b = angle.sin() * direction_to_center.length();


    if (b >= sphere.radius || angle > f32::consts::PI / 2.0) {
        return TraceResult::new();
    }

    let radius = sphere.radius;
    let g = ((radius * radius) - (b * b)).sqrt();
    let h = ((direction_to_center.length() * direction_to_center.length()) - (b * b)).sqrt();
    let hit_distance = h - g - 0.000001;

    direction.normalize(); 
    
    let hitpoint = position.add(&direction.mult(hit_distance));

    let result = TraceResult {
        hit: true,
        color: sphere.color,
        distance: hit_distance,
        hitpoint: hitpoint,
        normal: sphere.position.sub(&hitpoint),
        light_intensity: sphere.light_intensity,
        light_defraction: sphere.light_defraction,
    };
    if (bounces > 0) {
        return trace_lights(result, direction, hitpoint, world, bounces - 1);
    }

    result
}

pub fn trace_lights(mut result: TraceResult, direction: Vector, position: Vector, world: &World, bounces: i32) -> TraceResult {
    let mut total_color = Vector::new(1.0, 1.0, 1.0);

    let mut hit_something = false;
    
    // Hard shadows
    for sphere in &world.spheres {
        let mut new_direction = sphere.position.sub(&result.hitpoint);

        let new_result = trace_ray(result.hitpoint, new_direction, world, bounces);

        if (new_result.hit && new_result.distance < new_direction.length()&& new_result.distance > 0.01) {
            // Hit some object meaning shadow casting :O
            
            let mut intensity = new_result.light_intensity / new_result.distance;
            
            //println!("light distance: {:?}", new_result.distance);
            //println!("light intensity: {:?}", intensity);
            let light_angle = result.normal.angle(&mut new_direction);

            
            //println!("angle: {:?}", light_angle);
            intensity *= light_angle.cos().abs();
            
            //println!("light intensity: {:?}", intensity);
            result.light_intensity += intensity;
        }
    }
    
    //Soft shadows
    let n = (50.0 * result.light_defraction) as usize + 1;
    for _ in 0..n {
        let new_direction = randomize_light_defraction_direction(result.normal, direction, result.light_defraction);
        
        //println!("directions {:?}", new_direction);

        let new_result = trace_ray(position, new_direction, world, bounces);
        
        if (new_result.hit) {

            let mut intensity = new_result.light_intensity;
            intensity /= n as f32;

            result.light_intensity += intensity;
        }

        // trace soft shadows?
        
    }

    

    let original_color = result.color;

    result.color.r = (result.color.r as f32 * result.light_intensity).min(original_color.r as f32) as u8;
    result.color.g = (result.color.g as f32 * result.light_intensity).min(original_color.g as f32) as u8;
    result.color.b = (result.color.b as f32 * result.light_intensity).min(original_color.b as f32) as u8;

    result
}

pub fn randomize_light_defraction_direction(mut normal: Vector, direction: Vector, shattering: f32) -> Vector {
    normal.normalize();

    let mut random = Vector::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0);

    while ((random.x * random.x) + (random.y * random.y) + (random.z * random.z) > 1.0) {
        random.x = rand::random::<f32>() * 2.0 - 1.0;
        random.y = rand::random::<f32>() * 2.0 - 1.0;
        random.z = rand::random::<f32>() * 2.0 - 1.0;
    }

    //random.normalize();

    // random.x = random.x * random.x * random.x;
    // random.y = random.y * random.y * random.y;
    // random.z = random.z * random.z * random.z;

    // random.x = random.x * random.x.abs();
    // random.y = random.y * random.y.abs();
    // random.z = random.z * random.z.abs();

    //random.normalize();
    random = random.mult(shattering);
    normal = normal.add(&random);
    normal.normalize();
     
    let mut dot_product = direction.dot(&normal);
    dot_product *= -2.0;
    let term = normal.mult(dot_product);

    let mut result = direction.add(&term);

    


    // result.x *= 1.0 + (random.x * shattering);
    // result.y *= 1.0 + (random.y * shattering);
    // result.z *= 1.0 + (random.z * shattering);
    
    //println!("random: {:?}, result vector: {:?}", result, random);
    //result = result.add(&random);
    //println!("resulting vector: {:?}", result);
    result
}

pub struct TraceResult {
    hit: bool,
    distance: f32,
    color: Pixel,
    hitpoint: Vector,
    normal: Vector,
    light_intensity: f32,
    light_defraction: f32,
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
            light_defraction: 0.0,
        }
    }
}