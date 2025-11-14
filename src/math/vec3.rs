use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bytemuck::{Pod, Zeroable};

use crate::math::vec2::Vec2;

/// Represents an arbitrary collection of 3 components
///
/// More specifically, a [`Vec3`] is generally used for points and directions in 3D space
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Zeroable, Pod)]
pub struct Vec3 {
    /// The X component of the vector
    pub x: f32,
    /// The Y component of the vector
    pub y: f32,
    /// The Z component of the vector
    pub z: f32,
}

impl Vec3 {
    /// A vector with all components of value 0.0, also known as a zero vector
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    /// A unit vector pointing to the right
    pub const RIGHT: Self = Self::new(1.0, 0.0, 0.0);
    /// A unit vector pointing to the left
    pub const LEFT: Self = Self::new(-1.0, 0.0, 0.0);
    /// A unit vector pointing upwards
    pub const UP: Self = Self::new(0.0, 1.0, 0.0);
    /// A unit vector pointing downwards
    pub const DOWN: Self = Self::new(0.0, -1.0, 0.0);
    /// A unit vector pointing forwards
    pub const FORWARD: Self = Self::new(0.0, 0.0, 1.0);
    /// A unit vector pointing backwards
    pub const BACKWARD: Self = Self::new(0.0, 0.0, -1.0);

    /// Creates a new vector
    /// - `x` -> the first component of the vector
    /// - `y` -> the second component of the vector
    /// - `z` -> the third component of the vector
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates a new vector with uniform components
    /// - `value` -> the uniform value of x, y, z
    pub const fn splat(scalar: f32) -> Self {
        Self {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }

    /// Returns a new flipped vector from the original
    ///
    /// Flipping inverses the vectors direction while preserving its length
    pub fn flip(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Flips the vector in place
    ///
    /// Flipping inverses the vectors direction while preserving its length
    pub fn flip_self(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    /// Returns a new normalized vector from the original
    ///
    /// Normalizing a vector makes its length equal to 1, making it a unit vector
    /// while preserving its direction
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len < f32::EPSILON {
            return Self::ZERO;
        }

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    /// Normalizes the vector in place
    ///
    /// Normalizing a vector makes its length equal to 1, making it a unit vector
    /// while preserving its direction
    pub fn normalize_self(&mut self) {
        let len = self.length();
        if len < f32::EPSILON {
            return;
        }

        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    /// Returns the length of the vector
    pub fn length(self) -> f32 {
        self.length_sq().sqrt()
    }

    /// Returns the squared length of the vector
    pub fn length_sq(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Returns the distance between 2 vectors as points
    /// - `self` -> the first vector (as point)
    /// - `other` -> the second vector (as point)
    pub fn dist(self, other: Self) -> f32 {
        self.dist_sq(other).sqrt()
    }

    /// Returns the squared distance between 2 vectors as points
    /// - `self` -> the first vector (as point)
    /// - `other` -> the second vector (as point)
    pub fn dist_sq(self, other: Self) -> f32 {
        let x_diff = (self.x - other.x) * (self.x - other.x);
        let y_diff = (self.y - other.y) * (self.y - other.y);
        let z_diff = (self.z - other.z) * (self.z - other.z);
        x_diff + y_diff + z_diff
    }

    /// Returns the dot product between 2 vectors
    /// - `self` -> the first vector
    /// - `other` -> the second vector
    ///
    /// For a meaningful result, both vectors should be normalized before the dot product is computed
    ///
    /// If the vectors are of huge magnitude, the value yielded will be unnecessarily large
    ///
    /// The dot product represents how much the vectors point in the same direction
    /// - `dot > 0` -> the vectors point roughly in the same direction
    /// - `dot = 0` -> the vectors are perpendicular
    /// - `dot < 0` -> the vectors point roughly in the opposite direction
    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Returns the cross product between 2 vectors
    /// - `self` -> the first vector
    /// - `other` -> the second vector
    ///
    /// The cross product returns a new vector that is perpendicular to the 2 source vectors
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Linearly interpolates between 2 vectors
    /// - `self` -> the source vector
    /// - `other` -> the target vector
    /// - `factor` -> the interpolation factor
    pub fn lerp(self, other: Self, factor: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * factor,
            y: self.y + (other.y - self.y) * factor,
            z: self.z + (other.z - self.z) * factor,
        }
    }

    /// Compares 2 vectors and returns if they're equal or not
    /// - `self` -> the first vector
    /// - `other` -> the second vector
    /// - `epsilon` -> a very small value to account for floating-point errors
    pub fn cmp(self, other: Self, epsilon: f32) -> bool {
        let x_cmp = (self.x - other.x).abs() < epsilon;
        let y_cmp = (self.y - other.y).abs() < epsilon;
        let z_cmp = (self.z - other.z).abs() < epsilon;
        x_cmp && y_cmp && z_cmp
    }
}

impl From<Vec2> for Vec3 {
    fn from(value: Vec2) -> Self {
        Self::new(value.x, value.y, 1.0)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        if scalar.abs() < f32::EPSILON {
            panic!("Division by near-zero value");
        }

        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        if scalar.abs() < f32::EPSILON {
            panic!("Division by near-zero value");
        }

        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{EPSILON, cmp_f32};

    #[test]
    fn add() {
        {
            let v1 = Vec3::new(1.0, 0.0, 0.0);
            let v2 = Vec3::new(0.0, 1.0, 0.0);
            let v3 = Vec3::new(0.0, 0.0, 1.0);
            let expected = Vec3::new(1.0, 1.0, 1.0);
            assert!((v1 + v2 + v3).cmp(expected, EPSILON));
        }

        {
            let v1 = Vec3::new(5.0, 1.0, -7.0);
            let v2 = Vec3::new(-3.0, 0.0, 2.0);
            let v3 = Vec3::new(4.0, 8.0, 9.0);
            let expected = Vec3::new(6.0, 9.0, 4.0);
            assert!((v1 + v2 + v3).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn sub() {
        {
            let v1 = Vec3::new(1.0, 0.0, 0.0);
            let v2 = Vec3::new(0.0, 1.0, 0.0);
            let v3 = Vec3::new(0.0, 0.0, 1.0);
            let expected = Vec3::new(1.0, -1.0, -1.0);
            assert!((v1 - v2 - v3).cmp(expected, EPSILON));
        }

        {
            let v1 = Vec3::new(5.0, 1.0, -7.0);
            let v2 = Vec3::new(-3.0, 0.0, 2.0);
            let v3 = Vec3::new(4.0, 8.0, 9.0);
            let expected = Vec3::new(4.0, -7.0, -18.0);
            assert!((v1 - v2 - v3).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn mul() {
        {
            let v = Vec3::new(1.0, 1.0, 1.0);
            let f = 5.0;
            let expected = Vec3::new(5.0, 5.0, 5.0);
            assert!((v * f).cmp(expected, EPSILON));
        }

        {
            let v = Vec3::new(3.0, 5.0, 7.0);
            let f = 5.0;
            let expected = Vec3::new(15.0, 25.0, 35.0);
            assert!((v * f).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn div() {
        {
            let v = Vec3::new(1.0, 1.0, 1.0);
            let f = 5.0;
            let expected = Vec3::new(0.2, 0.2, 0.2);
            assert!((v / f).cmp(expected, EPSILON));
        }

        {
            let v = Vec3::new(3.0, 5.0, 7.0);
            let f = 5.0;
            let expected = Vec3::new(0.6, 1.0, 1.4);
            assert!((v / f).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn flip() {
        {
            let v = Vec3::new(1.0, 1.0, 1.0);
            let expected = Vec3::new(-1.0, -1.0, -1.0);
            assert!(v.flip().cmp(expected, EPSILON));
        }

        {
            let v = Vec3::new(2.5, 4.0, 6.0);
            let expected = Vec3::new(-2.5, -4.0, -6.0);
            assert!(v.flip().cmp(expected, EPSILON));
        }
    }

    #[test]
    fn normalize() {
        {
            let v = Vec3::new(0.0, 3.0, 4.0);
            let expected = Vec3::new(0.0, 0.6, 0.8);
            assert!(v.normalize().cmp(expected, EPSILON));
        }

        {
            let v = Vec3::new(4.0, 3.0, 0.0);
            let expected = Vec3::new(0.8, 0.6, 0.0);
            assert!(v.normalize().cmp(expected, EPSILON));
        }
    }

    #[test]
    fn length() {
        {
            let v = Vec3::new(3.0, 4.0, 0.0);
            let expected = 5.0;
            assert!(cmp_f32(v.length(), expected, EPSILON));
        }

        {
            let v = Vec3::new(8.0, 6.0, 0.0);
            let expected = 10.0;
            assert!(cmp_f32(v.length(), expected, EPSILON));
        }
    }

    #[test]
    fn length_sq() {
        {
            let v = Vec3::new(3.0, 4.0, 0.0);
            let expected = 25.0;
            assert!(cmp_f32(v.length_sq(), expected, EPSILON));
        }

        {
            let v = Vec3::new(8.0, 6.0, 0.0);
            let expected = 100.0;
            assert!(cmp_f32(v.length_sq(), expected, EPSILON));
        }
    }

    #[test]
    fn dist() {
        {
            let v1 = Vec3::new(4.0, 6.0, 0.0);
            let v2 = Vec3::new(1.0, 2.0, 0.0);
            let expected = 5.0;
            assert!(cmp_f32(v1.dist(v2), expected, EPSILON));
        }

        {
            let v1 = Vec3::new(10.0, 7.0, 0.0);
            let v2 = Vec3::new(2.0, 1.0, 0.0);
            let expected = 10.0;
            assert!(cmp_f32(v1.dist(v2), expected, EPSILON));
        }
    }

    #[test]
    fn dist_sq() {
        {
            let v1 = Vec3::new(4.0, 6.0, 1.0);
            let v2 = Vec3::new(1.0, 2.0, 1.0);
            let expected = 25.0;
            assert!(cmp_f32(v1.dist_sq(v2), expected, EPSILON));
        }

        {
            let v1 = Vec3::new(10.0, 7.0, 1.0);
            let v2 = Vec3::new(2.0, 1.0, 1.0);
            let expected = 100.0;
            assert!(cmp_f32(v1.dist_sq(v2), expected, EPSILON));
        }
    }

    #[test]
    fn dot() {
        {
            let v1 = Vec3::new(3.0, 4.0, 6.0);
            let v2 = Vec3::new(4.0, 5.0, 7.0);
            let expected = 74.0;
            assert!(cmp_f32(v1.dot(v2), expected, EPSILON));
        }

        {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(4.0, 5.0, 6.0);
            let expected = 32.0;
            assert!(cmp_f32(v1.dot(v2), expected, EPSILON));
        }
    }

    #[test]
    fn lerp() {
        {
            let v1 = Vec3::new(5.0, 10.0, 15.0);
            let v2 = Vec3::new(10.0, 20.0, 30.0);
            let expected = Vec3::new(7.5, 15.0, 22.5);
            assert!(v1.lerp(v2, 0.5).cmp(expected, EPSILON));
        }

        {
            let v1 = Vec3::new(50.0, 60.0, 70.0);
            let v2 = Vec3::new(80.0, 70.0, 60.0);
            let expected = Vec3::new(65.0, 65.0, 65.0);
            assert!(v1.lerp(v2, 0.5).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn cross() {
        {
            let v1 = Vec3::new(1.0, 0.0, 0.0);
            let v2 = Vec3::new(0.0, 1.0, 0.0);
            let expected = Vec3::new(0.0, 0.0, 1.0);
            assert!(v1.cross(v2).cmp(expected, EPSILON));
        }

        {
            let v1 = Vec3::new(-1.0, 0.0, 0.0);
            let v2 = Vec3::new(0.0, -1.0, 0.0);
            let expected = Vec3::new(0.0, 0.0, 1.0);
            assert!(v1.cross(v2).cmp(expected, EPSILON));
        }
    }
}
