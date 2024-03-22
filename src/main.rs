use std::{
    fs::File,
    io::{Cursor, Write},
};

use glam::Vec3;
use itertools::Itertools;

fn main() {
    let width = 256;
    let height = 256;

    let mut cursor = Cursor::new(Vec::new());

    writeln!(cursor, "P3\n").expect("p3");
    writeln!(cursor, "{width} {height}\n").expect("size");
    writeln!(cursor, "255\n").expect("length");

    for (i, j) in (0..height).cartesian_product(0..width) {
        let r = i as f32 / (width - 1) as f32;
        let g = j as f32 / (height - 1) as f32;
        let b = 0.5;

        let color = Vec3::new(r, g, b) * 255.;
        let u8_color = color.as_u16vec3();

        writeln!(cursor, "{} {} {}", u8_color.x, u8_color.y, u8_color.z).expect("color");
    }

    println!("Done");

    let mut file = File::create("render.ppm").expect("create file");
    file.write(&cursor.into_inner()).expect("Write to file");
}
