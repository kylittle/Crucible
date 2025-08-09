use std::f64::consts::PI;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::Rng;

/// Utility function to convert degrees to radians
#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Private type without an external api
/// API will be exposed through the Color
/// and Point3 structs.
#[derive(Debug, PartialEq)]
pub struct Point3 {
    values: (f64, f64, f64),
}

pub type Vec3 = Point3;

impl Point3 {
    /// Creates a new Point3 with parameterized values.
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { values: (x, y, z) }
    }

    /// Creates the point (0, 0, 0)
    pub fn origin() -> Point3 {
        Point3 {
            values: (0.0, 0.0, 0.0),
        }
    }

    /// Randomly generate a vector with x, y, and z between [0, 1)
    pub fn random_vec3() -> Vec3 {
        let mut rng = rand::rng();

        let x = rng.random();
        let y = rng.random();
        let z = rng.random();

        Vec3::new(x, y, z)
    }

    /// Randomly generate a vector with x, y, and z between [min, max)
    pub fn random_vec3_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::rng();

        let x = rng.random_range(min..max);
        let y = rng.random_range(min..max);
        let z = rng.random_range(min..max);

        Vec3::new(x, y, z)
    }

    /// Randomly generate a unit vector.
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_vec3_range(-1.0, 1.0);
            let lensq = p.length_squared();

            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere // same direction
        } else {
            -on_unit_sphere // opposite direction so invert
        }
    }

    /// Compute the reflection of a vector across the normal
    pub fn reflect_vec3(v: &Vec3, norm: &Vec3) -> Vec3 {
        v.clone() - 2.0 * v.dot(norm) * norm.clone()
    }

    pub fn x(&self) -> f64 {
        self.values.0
    }

    pub fn y(&self) -> f64 {
        self.values.1
    }

    pub fn z(&self) -> f64 {
        self.values.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        let v = self.values;
        v.0.powi(2) + v.1.powi(2) + v.2.powi(2)
    }

    /// Checks if a vector is too close to zero in all dimensions
    pub fn near_zero(&self) -> bool {
        let tolerance = 1e-8;
        self.x().abs() < tolerance && self.y().abs() < tolerance && self.z().abs() < tolerance
    }

    pub fn dot(&self, other: &Point3) -> f64 {
        let v = self.values;
        let o = other.values;

        v.0 * o.0 + v.1 * o.1 + v.2 * o.2
    }

    pub fn cross(&self, other: &Point3) -> Point3 {
        let v = self.values;
        let o = other.values;

        Point3 {
            values: (
                v.1 * o.2 - v.2 * o.1,
                v.2 * o.0 - v.0 * o.2,
                v.0 * o.1 - v.1 * o.0,
            ),
        }
    }

    pub fn unit_vector(self) -> Point3 {
        let l = self.length();
        self / l
    }
}

/// This shouldn't be too slow since there are only
/// three values to deep copy.
impl Clone for Point3 {
    fn clone(&self) -> Self {
        Point3 {
            values: (self.x(), self.y(), self.z()),
        }
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.values;
        write!(f, "{} {} {}", v.0, v.1, v.2)
    }
}

impl Neg for Point3 {
    type Output = Point3;

    fn neg(self) -> Self::Output {
        let values = self.values;
        Point3 {
            values: (-values.0, -values.1, -values.2),
        }
    }
}

impl AddAssign for Point3 {
    fn add_assign(&mut self, rhs: Self) {
        self.values.0 += rhs.x();
        self.values.1 += rhs.y();
        self.values.2 += rhs.z();
    }
}

impl MulAssign<f64> for Point3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.values.0 *= rhs;
        self.values.1 *= rhs;
        self.values.2 *= rhs;
    }
}

impl DivAssign<f64> for Point3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl Add for Point3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        let v = self.values;
        let o = rhs.values;
        Point3 {
            values: (v.0 + o.0, v.1 + o.1, v.2 + o.2),
        }
    }
}

impl Sub for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f64) -> Self::Output {
        let v = self.values;
        Point3 {
            values: (rhs * v.0, rhs * v.1, rhs * v.2),
        }
    }
}

impl Mul<Point3> for f64 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        let v = rhs.values;
        Point3 {
            values: (self * v.0, self * v.1, self * v.2),
        }
    }
}

impl Mul for Point3 {
    type Output = Point3;

    fn mul(self, rhs: Self) -> Self::Output {
        let v = rhs.values;
        let o = rhs.values;
        Point3 {
            values: (v.0 * o.0, v.1 * o.1, v.2 * o.2),
        }
    }
}

impl Div<f64> for Point3 {
    type Output = Point3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

/// Color is a struct containing an RGB value, it is
/// guaranteed to be between 0 and 1.
///
/// # Panics:
/// If r, g, or b are not between 0 and 1 constructing a
/// color panics. The type encodes the assumption.
#[derive(Debug, PartialEq)]
pub struct Color {
    rgb: Point3,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        assert!(r <= 1.0, "R must be lower than 1.0. Got {r}");
        assert!(g <= 1.0, "G must be lower than 1.0. Got {g}");
        assert!(b <= 1.0, "B must be lower than 1.0. Got {b}");
        assert!(r >= 0.0, "R must be greater or equal to 0.0. Got {r}");
        assert!(g >= 0.0, "G must be greater or equal to 0.0. Got {g}");
        assert!(b >= 0.0, "B must be greater or equal to 0.0. Got {b}");

        Color {
            rgb: Point3 { values: (r, g, b) },
        }
    }

    /// Makes a color representing black
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    /// Makes a color representing white
    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn r(&self) -> f64 {
        self.rgb.x()
    }

    pub fn g(&self) -> f64 {
        self.rgb.y()
    }

    pub fn b(&self) -> f64 {
        self.rgb.z()
    }

    // Helper function for output
    fn linear_to_gamma(linear_component: f64) -> f64 {
        linear_component.sqrt()
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color {
            rgb: self.rgb.clone(),
        }
    }
}

/// Implement display to convert a color to an RGB line for
/// a .ppm image.
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.r();
        let g = self.g();
        let b = self.b();

        let r = Color::linear_to_gamma(r);
        let g = Color::linear_to_gamma(g);
        let b = Color::linear_to_gamma(b);

        let rbyte = (255.0 * r) as u32;
        let gbyte = (255.0 * g) as u32;
        let bbyte = (255.0 * b) as u32;

        write!(f, "{rbyte} {gbyte} {bbyte}")
    }
}

/// This flips the color to its inversion, it does not
/// break the constructor properties of Color.
///
/// check here for more info:
/// https://stackoverflow.com/questions/40233986/python-is-there-a-function-or-formula-to-find-the-complementary-colour-of-a-rgb
impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Self::Output {
        let k = hilo(self.r(), self.g(), self.b());

        let inv_r = (k - self.r()).abs();
        let inv_g = (k - self.g()).abs();
        let inv_b = (k - self.b()).abs();
        Color {
            rgb: Point3 {
                values: (inv_r, inv_g, inv_b),
            },
        }
    }
}

fn min2(x: f64, y: f64) -> f64 {
    if x < y { x } else { y }
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    min2(min2(a, b), c)
}

fn max2(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

fn max3(a: f64, b: f64, c: f64) -> f64 {
    max2(max2(a, b), c)
}

fn hilo(a: f64, b: f64, c: f64) -> f64 {
    let min = min3(a, b, c);
    let max = max3(a, b, c);

    min + max
}

/// This must have a clamped add to stay in the bounds of
/// the Color properties.
impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        let sum_r = (self.r() + rhs.r()).clamp(0.0, 1.0);
        let sum_g = (self.g() + rhs.g()).clamp(0.0, 1.0);
        let sum_b = (self.b() + rhs.b()).clamp(0.0, 1.0);

        self.rgb = Point3 {
            values: (sum_r, sum_g, sum_b),
        };
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        let mul_r = (self.r() * rhs).clamp(0.0, 1.0);
        let mul_g = (self.g() * rhs).clamp(0.0, 1.0);
        let mul_b = (self.b() * rhs).clamp(0.0, 1.0);

        self.rgb = Point3 {
            values: (mul_r, mul_g, mul_b),
        };
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        let sum_r = (self.r() + rhs.r()).clamp(0.0, 1.0);
        let sum_g = (self.g() + rhs.g()).clamp(0.0, 1.0);
        let sum_b = (self.b() + rhs.b()).clamp(0.0, 1.0);

        Color {
            rgb: Point3 {
                values: (sum_r, sum_g, sum_b),
            },
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        let mul_val = if rhs < 0.0 { -self } else { self };
        let rhs = rhs.abs();

        let mul_r = (mul_val.r() * rhs).clamp(0.0, 1.0);
        let mul_g = (mul_val.g() * rhs).clamp(0.0, 1.0);
        let mul_b = (mul_val.b() * rhs).clamp(0.0, 1.0);

        Color {
            rgb: Point3 {
                values: (mul_r, mul_g, mul_b),
            },
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        let mul_val = if self < 0.0 { -rhs } else { rhs };
        let pos_s = self.abs();

        let mul_r = (pos_s * mul_val.r()).clamp(0.0, 1.0);
        let mul_g = (pos_s * mul_val.g()).clamp(0.0, 1.0);
        let mul_b = (pos_s * mul_val.b()).clamp(0.0, 1.0);

        Color {
            rgb: Point3 {
                values: (mul_r, mul_g, mul_b),
            },
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        let mul_r = (self.r() * rhs.r()).clamp(0.0, 1.0);
        let mul_g = (self.g() * rhs.g()).clamp(0.0, 1.0);
        let mul_b = (self.b() * rhs.b()).clamp(0.0, 1.0);

        Color {
            rgb: Point3 {
                values: (mul_r, mul_g, mul_b),
            },
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        let inv_self = if rhs < 0.0 { -self } else { self };
        let rhs = rhs.abs();

        (1.0 / rhs) * inv_self
    }
}

/// Randomly generate a color
pub fn random_color() -> Color {
    let v = Vec3::random_vec3();

    Color::new(v.x(), v.y(), v.z())
}

pub struct Interval {
    range: (f64, f64),
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        Interval { range: (min, max) }
    }

    pub fn min(&self) -> f64 {
        self.range.0
    }
    pub fn max(&self) -> f64 {
        self.range.1
    }

    pub fn size(&self) -> f64 {
        self.range.1 - self.range.0
    }

    pub fn contains(&self, x: f64) -> bool {
        self.range.0 <= x && x <= self.range.1
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.range.0 < x && x < self.range.1
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min(), self.max())
    }
}

pub const EMPTY: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
pub const UNIVERSE: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg_test() {
        let v = Point3 {
            values: (1.0, 2.0, 3.0),
        };

        assert_eq!(
            -v,
            Point3 {
                values: (-1.0, -2.0, -3.0)
            }
        );
    }

    #[test]
    fn plus_assign_test() {
        let mut v = Point3 {
            values: (1.0, 2.0, 3.0),
        };
        let u = Point3 {
            values: (2.0, 2.0, 1.0),
        };

        v += u;

        assert_eq!(
            v,
            Point3 {
                values: (3.0, 4.0, 4.0)
            }
        )
    }

    #[test]
    fn dot_test() {
        let v = Point3 {
            values: (1.0, 2.0, 3.0),
        };
        let u = Point3 {
            values: (2.0, 2.0, 1.0),
        };
        assert_eq!(v.dot(&u), 9.0);
    }

    #[test]
    fn cross_test() {
        let v = Point3 {
            values: (3.0, -3.0, 1.0),
        };
        let u = Point3 {
            values: (4.0, 9.0, 2.0),
        };

        assert_eq!(
            v.cross(&u),
            Point3 {
                values: (-15.0, -2.0, 39.0)
            }
        )
    }

    #[test]
    fn length_test() {
        let v = Point3 {
            values: (3.0, 4.0, 0.0),
        };

        let l = v.length();
        assert_eq!(l, 5.0);
    }

    #[test]
    #[should_panic]
    fn invalid_color_test() {
        let _ = Color::new(20.0, 30.0, 40.0);
    }

    #[test]
    fn color_display_test() {
        let c = Color::new(0.529, 0.616, 0.730);

        assert_eq!("134 157 186", c.to_string());
    }

    #[test]
    fn inv_color() {
        let r = Color::new(1.0, 0.0, 0.0);
        let comp_r = Color::new(0.0, 1.0, 1.0);

        assert_eq!(-r, comp_r);
    }

    #[test]
    fn add_color_test() {
        let mut r = Color::new(1.0, 0.0, 0.0);
        let g = Color::new(0.0, 1.0, 0.0);

        let y = Color::new(1.0, 1.0, 0.0);

        r += g;

        assert_eq!(r, y);
    }

    #[test]
    fn degrees_convert_test() {
        let r = degrees_to_radians(59.2958);
        // Accurate to about +- 2e-8
        let tolerance = 0.0000000005;

        assert!(
            (r - 1.034906943).abs() < tolerance,
            "Test is not in the accepted tolerance range"
        );
    }

    #[test]
    fn size_test() {
        let i = Interval::new(3.0, 20.0);

        assert_eq!(i.size(), 17.0);
    }

    #[test]
    fn contains_test() {
        let i = Interval::new(3.0, 20.0);

        assert_eq!(i.contains(3.0), true);
        assert_eq!(i.contains(21.0), false);
        assert_eq!(i.contains(15.0), true);
    }

    #[test]
    fn surrounds_test() {
        let i = Interval::new(3.0, 20.0);

        assert_eq!(i.surrounds(3.0), false);
        assert_eq!(i.surrounds(21.0), false);
        assert_eq!(i.surrounds(15.0), true);
    }

    #[test]
    fn universe_contains_test() {
        use rand::prelude::*;

        let mut rng = rand::rng();

        // The universe should contain everything:
        for _ in 0..10 {
            let x: f64 = rng.random_range(-500.0..500.0);

            assert!(UNIVERSE.contains(x));
        }
    }

    #[test]
    fn empty_contains_test() {
        use rand::prelude::*;

        let mut rng = rand::rng();

        // The universe should contain everything:
        for _ in 0..10 {
            let x: f64 = rng.random_range(-500.0..500.0);

            assert!(!EMPTY.contains(x));
        }
    }
}
