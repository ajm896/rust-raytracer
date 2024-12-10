use crate::ray::Ray;
use crate::vector::Vec3;
mod ray;
mod vector;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.99, 0.3, 0.99)
}

fn main() {
    let nx = 200;
    let ny = 100;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let col = color(&r);
            let r = (col[0] * 255.99) as i32;
            let g = (col[1] * 255.99) as i32;
            let b = (col[2] * 255.99) as i32;
            println!("{} {} {}", r, g, b);
        }
    }
}
