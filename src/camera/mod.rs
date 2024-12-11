use std::io::{self, Write};

use crate::{
    color::{Color, Presets},
    geometry::{interval::Interval, HitRecord, Hittable, HittableList},
    ray::Ray,
    vector::{Point, Vec3},
};

pub struct Camera {
    pub image_width: f64,
    pub aspect_ratio: f64,
    camera_origin: Point,
    image_height: f64,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

// Image Dimensions

impl Camera {
    fn new(
        image_width: f64,
        aspect_ratio: f64,
        camera_origin: Point,
        image_height: f64,
        pixel00_loc: Point,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
    ) -> Camera {
        Camera {
            image_width,
            aspect_ratio,
            camera_origin,
            image_height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
    pub fn render(&self, world: &HittableList) {
        println!(
            "P3\n{} {}\n255",
            self.image_width as usize, self.image_height as usize
        );
        for j in 0..self.image_height as usize {
            io::stderr()
                .write_all(
                    format!("\rScanlines Remaining: {}", self.image_height - j as f64).as_bytes(),
                )
                .unwrap();
            for i in 0..self.image_width as usize {
                let pixel_center: Point = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);

                let ray_direction: Vec3 = pixel_center - self.camera_origin;
                let r: Ray = Ray::new(self.camera_origin, ray_direction);

                let pixel_color: Color = self.ray_color(&r, &world);
                pixel_color.write_color()
            }
        }
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        let mut tmp_hit_rec = HitRecord::default();

        if world.hit(ray, &Interval { range: 0.0..100. }, &mut tmp_hit_rec) {
            return 0.5 * (tmp_hit_rec.get_normal() + Color::new(1., 1., 1.));
        }

        let unit_direction: Vec3 = ray.direction().normalize();
        let a: f64 = 0.5 * (unit_direction.y() + 1.);
        return ((1. - a) * Color::colors(Presets::White)) + (a * Color::new(0.5, 0.7, 1.0));
    }
}

impl Default for Camera {
    fn default() -> Self {
        // Image Dimensions
        let aspect_ratio: f64 = 16. / 9.;
        let image_width: f64 = 1920. * 0.25;
        let image_height: f64 = image_width / aspect_ratio;

        // Camera Settings
        let view_port_height: f64 = 2.;
        let view_port_width: f64 = (image_width / image_height) * view_port_height;
        let focal_length: f64 = 1.0;
        let camera_origin: Point = Point::new(0.0, 0.0, 0.0);

        let viewport_u: Vec3 = Vec3::new(view_port_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0., -view_port_height as f64, 0.0);

        let pixel_delta_u: Vec3 = viewport_u / image_width;
        let pixel_delta_v: Vec3 = viewport_v / image_height;

        let viewport_upper_left = camera_origin
            - Point::new(0., 0., focal_length)
            - (viewport_u / 2.)
            - (viewport_v / 2.);

        let pixel00_loc: Point = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera::new(
            image_width,
            aspect_ratio,
            camera_origin,
            image_height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        )
    }
}
