use std::io::{Cursor, Write};

fn main() {
    let width = 256;
    let height = 256;

    let mut cursor = Cursor::new(Vec::new());

    write!(cursor, "P3").expect("p3");
    write!(cursor, "{width} {height}").expect("size");
    write!(cursor, "255").expect("length");

    for j in 0..height {
        for i in 0..width {
            let r = i as f32 / (width - 1) as f32;
            let g = j as f32 / (height - 1) as f32;
            let b = 0.;

            let r: u8 = (r * 255.) as u8;
            let g: u8 = (g * 255.) as u8;
            let b: u8 = (b * 255.) as u8;

            write!(cursor, "{r} {g} {b}").expect("color");
        }
    }

    println!("{:?}", cursor.into_inner());
}
