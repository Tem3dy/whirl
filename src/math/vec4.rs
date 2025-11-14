use bytemuck::{Pod, Zeroable};

use crate::math::vec3::Vec3;

/// Represents an arbitrary collection of 4 components
///
/// More specifically, a `Vec4` is generally used for points and directions in 3D space,
/// where the last component `w` is used as a homogenous coordinate, which serves 2 purposes.
///
/// Namely:
/// - Multiplying a vector by a 4x4 matrix which is required if we want to encode translation information
/// - Locking translation by setting `w` to `0.0` (now it represents a direction, not a point)
/// - Perspective division (when the GPU performs the vertex shader, the vertex (as a `Vec4`) is divided by its `w` coordinate, hence the name perspective division)
#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Vec4 {
    /// The X component of the vector
    pub x: f32,
    /// The Y component of the vector
    pub y: f32,
    /// The Z component of the vector
    pub z: f32,
    /// The W component of the vector
    pub w: f32,
}

impl Vec4 {
    /// Creates a new vector
    /// - `x` -> the first component of the vector
    /// - `y` -> the second component of the vector
    /// - `z` -> the third component of the vector
    /// - `w` -> the fourth component of the vector
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a new vector with uniform components
    /// - `value` -> the uniform value of x, y, z
    pub const fn splat(scalar: f32) -> Self {
        Self {
            x: scalar,
            y: scalar,
            z: scalar,
            w: scalar,
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
        let w_cmp = (self.w - other.w).abs() < epsilon;
        x_cmp && y_cmp && z_cmp && w_cmp
    }
}

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        Self::new(value.x, value.y, value.z, 1.0)
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    // Implement unit tests
}
