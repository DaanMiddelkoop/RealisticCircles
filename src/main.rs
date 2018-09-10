extern crate bmp;

mod vector;
mod world;
mod shapes;

use bmp::Pixel;
use vector::Vector;
use world::World;
use shapes::Globe;

fn main() {
    println!("Hello, world!");

    let mut bitmap = bmp::open("test_image.bmp").unwrap();

    let position = Vector::new(0.0, 0.0, 0.0);

    let world = World::new();

    for i in 0..bitmap.get_width() {
        for j in 0..bitmap.get_height() {
            bitmap.set_pixel(i, j, trace_pixel(i as f32, j as f32, position, &world));
        }
    }

    bitmap.save("test_image.bmp").unwrap();

}

fn trace_pixel(x: f32, y: f32, position: Vector, world: &World) -> Pixel {

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

    let trace_direction = Vector::new(
        direction.x + (normal_x.x * x) + (normal_y.x * y),
        direction.y + (normal_x.y * x) + (normal_y.y * y),
        direction.z + (normal_x.z * x) + (normal_y.z * y),
    );

    trace_ray(position, trace_direction, world)
} 

fn trace_ray(position: Vector, direction: Vector, world: &World) -> Pixel {



    trace_spheres(position, direction, &world.spheres)
}

fn trace_spheres(position: Vector, direction: Vector, spheres: &Vec<Globe>) -> Pixel {

    Pixel::new(0, 0, 0)
}

fn trace_sphere(position: Vector, direction: Vector, sphere: &Globe) -> bool {
    let direction_to_center = sphere.position.sub(&position);

    let angle = direction_to_center.angle(&direction);

    false
}

