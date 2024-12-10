use std::io::{self, Write};

use color::{Color, Presets};
use ray::Ray;
use vector::{Point, Vec3};

mod color;
mod ray;
mod vector;

fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> bool {
    let oc: Vec3 = center - ray.origin();
    let a = ray.direction().dot(&ray.direction());
    let b = -2. * oc.dot(&ray.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Point::new(0., 0., -1.), 0.5, &ray) {
        return Color::colors(Presets::Red);
    } else {
        let unit_direction: Vec3 = ray.direction().normalize();
        let a: f64 = 0.5 * (unit_direction.y() + 1.);
        return ((1. - a) * Color::colors(Presets::White)) + (a * Color::new(0.5, 0.7, 1.0));
    }
}

fn main() {
    // Image Dimensions
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: f64 = 400.;
    let image_height: f64 = image_width / aspect_ratio;

    // Camera Settings
    let view_port_height: f64 = 2.;
    let view_port_width: f64 = (image_width / image_height) * view_port_height;
    let focal_length: f64 = 1.0;
    let camera_center: Point = Point::new(0.0, 0.0, 0.0);

    let viewport_u: Vec3 = Vec3::new(view_port_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0., -1.0 * view_port_height as f64, 0.0);

    let pixel_delta_u: Vec3 = viewport_u / view_port_width;
    let pixel_delta_v: Vec3 = viewport_v / view_port_height;

    let viewport_upper_left =
        camera_center - Point::new(0., 0., focal_length) - (viewport_u / 2.) - (viewport_v / 2.);

    let pixel00_loc: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render the image in P3 format. The color values are written to stdin
    println!("P3\n{} {}\n255", image_width, image_height);
    for j in 0..image_height as usize {
        io::stderr()
            .write_all(format!("\rScanlines Remaining: {}", image_height - j as f64).as_bytes())
            .unwrap();
        for i in 0..image_width as usize {
            let pixel_center: Point =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            let ray_direction: Vec3 = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(&r);
            pixel_color.write_color()
        }
    }
    println!("DONE");
}
