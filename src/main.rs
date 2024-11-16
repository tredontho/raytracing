use std::io::{self, Write};

fn main() {
    // Simple PPM example

    // Image
    let image_width: usize = 256;
    let image_height: usize = 256;

    // Render

    let mut image: String = String::new();
    let header = format!("P3\n{image_width} {image_height}\n255\n");
    image.push_str(&header);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64) / ((image_width - 1) as f64);
            let g = (j as f64) / ((image_height - 1) as f64);
            let b = 0.0;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            image.push_str(&format!("{ir} {ig} {ib}\n"));
        }
    }

    io::stdout().write(image.as_bytes()).unwrap();
}
