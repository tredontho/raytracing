use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            e: [
                self.y() * rhs.z() - self.z() * rhs.y(),
                self.z() * rhs.x() - self.x() * rhs.z(),
                self.x() * rhs.y() - self.y() * rhs.x(),
            ],
        }
    }

    pub fn normalize(&self) -> Self {
        self / self.length()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// Feels like there should be a better way of implementing traits for both references and values
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Self::Output {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        self + &-rhs
    }
}

// Hadamard product
impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()],
        }
    }
}

// Scalar multiplication on the right
impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.x() * rhs, self.y() * rhs, self.z() * rhs],
        }
    }
}

// Scalar multiplcation on the left
impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        *self = &*self + rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut a = Vec3::zero();
        let b = Vec3::new(1.0, 2.0, 3.0);

        a += &b;
        assert_eq!(a, b);
    }

    #[test]
    fn mul_assign() {
        let mut a = Vec3::zero();
        a *= 3.0;
        assert_eq!(Vec3::zero(), a);

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        a *= 3.0;
        assert_eq!(Vec3::new(3.0, 6.0, 9.0), a);
    }

    #[test]
    fn div_assign() {
        let mut a = Vec3::zero();
        a /= 3.0;
        assert_eq!(Vec3::zero(), a);

        let mut a = Vec3::new(3.0, 6.0, 9.0);
        a /= 3.0;
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), a);
    }

    #[test]
    fn neg() {
        let a = Vec3::new(1.0, 0.0, -1.0);
        let b = -&a;
        assert_eq!(Vec3::new(-1.0, 0.0, 1.0), b);
        let c = -a;
        assert_eq!(Vec3::new(-1.0, 0.0, 1.0), c);
        assert_eq!(b, c);
    }

    #[test]
    fn sub() {
        let a = Vec3::new(1.0, 0.0, -1.0);
        assert_eq!(a, &a - &Vec3::zero());
        assert_eq!(Vec3::zero(), &a - &a);
    }

    #[test]
    fn add() {
        let a = Vec3::new(1.0, 0.0, -1.0);
        assert_eq!(a, &a + &Vec3::zero());
        assert_eq!(Vec3::new(2.0, 0.0, -2.0), &a + &a);
    }

    #[test]
    fn mul() {
        let a = Vec3::new(2.0, 0.0, -2.0);
        assert_eq!(Vec3::zero(), &a * &Vec3::zero());
        assert_eq!(Vec3::new(4.0, 0.0, 4.0), &a * &a);
    }

    #[test]
    fn scalar_mul() {
        let a = Vec3::new(2.0, 0.0, -2.0);
        let scalar: f64 = 5.0;
        assert_eq!(Vec3::zero(), &a * 0.0);
        assert_eq!(Vec3::zero(), 0.0 * &a);
        assert_eq!(Vec3::new(10.0, 0.0, -10.0), &a * scalar);
        assert_eq!(Vec3::new(10.0, 0.0, -10.0), scalar * &a);
    }

    #[test]
    fn scalar_div() {
        let a = Vec3::new(2.0, 0.0, -2.0);
        let scalar = 2.0;

        assert_eq!(Vec3::new(1.0, 0.0, -1.0), &a / scalar)
    }

    #[test]
    fn dot() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        // Orthogonal vectors dot products are zero
        assert_eq!(0.0, i.dot(&j));
        assert_eq!(0.0, i.dot(&k));
        assert_eq!(0.0, j.dot(&k));

        assert_eq!(1.0, i.dot(&i));
        assert_eq!(1.0, j.dot(&j));
        assert_eq!(1.0, k.dot(&k));
    }

    #[test]
    fn cross() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(Vec3::zero(), a.cross(&a));

        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(c, a.cross(&b));
    }

    #[test]
    fn normalize() {
        let a = Vec3::new(3.0, 4.0, 0.0);
        assert!((&Vec3::new(0.6, 0.8, 0.0) - &a.normalize()).length() < 0.0001);
    }

    #[test]
    fn display() {
        let expected = format!("1.1 2.1 3.1");
        let a = Vec3::new(1.1, 2.1, 3.1);
        assert_eq!(expected, format!("{a}"));
    }
}
