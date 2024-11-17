use std::fmt::Display;

use crate::Vec3;

pub struct Color(pub Vec3);

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Color(v) = self;
        let rbyte = (255.999 * v.x()) as i64;
        let gbyte = (255.999 * v.y()) as i64;
        let bbyte = (255.999 * v.z()) as i64;
        write!(f, "{} {} {}", rbyte, gbyte, bbyte)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let c = Color(Vec3::new(1.0, 0.0, 0.5));

        assert_eq!("255 0 127", format!("{c}"));
    }
}
