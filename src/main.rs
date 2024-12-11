use camera::Camera;
use geometry::{sphere::Sphere, HittableList};
use vector::Point;

mod camera;
mod color;
mod geometry;
mod ray;
mod vector;

fn main() {
    let camera = Camera::default();
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point::new(0., -101., -1.), 100.)));
    world.add(Box::new(Sphere::new(Point::new(0., 0.0, -1.), 0.5)));

    camera.render(&world);
    // Render the image in P3 format. The color values are written to stdin

    println!("DONE");
}
