use raytrace::Color;
use raytrace::Ray;
use raytrace::Vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;

    // Calculate height from aspect ratio and image width, and ensure it's positive
    let image_height = max(1, ((image_width as f64) / aspect_ratio) as usize);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width / image_height) as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / (image_width as f64);
    let pixel_delta_v = &viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel
    let viewport_upper_left =
        &camera_center - &Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);
    eprintln!("image_height = {}", image_height);

    // Render

    let header = format!("P3\n{image_width} {image_height}\n255");
    println!("{header}");

    for j in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                &pixel00_loc + &(i as f64 * &pixel_delta_u + (j as f64 * &pixel_delta_v));
            let ray_direction = &pixel_center - &camera_center;
            let r = Ray::new(camera_center.clone(), ray_direction);
            let pixel_color = ray_color(&r);

            println!("{pixel_color}");
        }
    }

    eprintln!("Done.");
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction: Vec3 = r.direction().normalize();
    let a: f64 = 0.5 * (unit_direction.y() + 1.0);
    Color::from((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + (a * Vec3::new(0.5, 0.7, 1.0)))
}

fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a >= b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn max() {
        let (a, b) = (0, 1);
        assert_eq!(1, super::max(a, b));
        let (a, b) = (0, 0);
        assert_eq!(0, super::max(a, b));
        let (a, b) = (1, 0);
        assert_eq!(1, super::max(a, b));
    }
}
