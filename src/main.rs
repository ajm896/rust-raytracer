use std::io::{self, Write};

use color::Color;

mod color;
mod ray;
mod vector;

fn main() {
    const WIDTH: f64 = 256.;
    const HEIGHT: f64 = 256.;

    // Render
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in 0..HEIGHT as usize {
        io::stderr()
            .write_all(format!("\rScanlines Remaining: {}", HEIGHT as usize - j).as_bytes())
            .unwrap();
        for i in 0..WIDTH as usize {
            let u = i as f64 / (WIDTH - 1.);
            let v = j as f64 / (HEIGHT - 1.);
            let col = Color::new(&[255.99 * u, 255.99 * v, 0.0]);
            col.write_color()
        }
    }
}
