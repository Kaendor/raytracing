use std::{
    fs::File,
    io::{Cursor, Write},
};

use glam::Vec3;
use itertools::Itertools;

use crate::ray::Ray;

mod ray;

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.;
    let height = image_width as f32 / aspect_ratio;

    let image_height = if height < 1. { 1. } else { height };

    // Camera
    let focal_lenght = 1.0;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * image_width as f32 / image_height;
    let camera_center = Vec3::ZERO;

    // Viewport vectors
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Step vectors
    let delta_u = viewport_u / image_width;
    let delta_v = viewport_v / image_height;

    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_lenght) - viewport_u / 2. - viewport_v / 2.;

    let first_pixel_pos = viewport_upper_left + 0.5 * (delta_u + delta_v);

    // Render

    let mut cursor = Cursor::new(Vec::new());

    writeln!(cursor, "P3\n").expect("p3");
    writeln!(cursor, "{image_width} {image_height}\n").expect("size");
    writeln!(cursor, "255\n").expect("length");

    for (j, i) in (0..image_height as i32).cartesian_product(0..image_width as i32) {
        let pixel_center = first_pixel_pos + (i as f32 * delta_u) + (j as f32 * delta_v);
        let ray_direction = pixel_center - camera_center;
        let ray = Ray::new(camera_center, ray_direction);

        let u8_color = ray.color();

        writeln!(cursor, "{} {} {}", u8_color.x, u8_color.y, u8_color.z).expect("color");
    }

    println!("Done");

    let mut file = File::create("render.ppm").expect("create file");
    file.write(&cursor.into_inner()).expect("Write to file");
}
