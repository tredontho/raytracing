use raytrace::Color;
use raytrace::Vec3;

fn main() {
    // Simple PPM example

    // Image
    let image_width: usize = 256;
    let image_height: usize = 256;

    // Render

    let header = format!("P3\n{image_width} {image_height}\n255");
    println!("{header}");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_color = Color(Vec3::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0,
            ));

            println!("{pixel_color}");
        }
    }

    eprintln!("Done.");
}
