use crate::vector::Vec3;
mod ray;
mod vector;

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let v = Vec3::new(x as f64 / nx as f64, y as f64 / ny as f64, 0.2);
            let r = (v.x * 255.99) as i32;
            let g = (v.y * 255.99) as i32;
            let b = (v.z * 255.99) as i32;
            println!("{} {} {}", r, g, b);
        }
    }
}
