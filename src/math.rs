//! The math module, containing all the essential types and functionality.
//!
/// Contains functionality related to 2D vectors.
pub mod vec2;
/// Contains functionality related to 3D vectors.
pub mod vec3;
/// Contains functionality related to 4D vectors.
pub mod vec4;

/// Contains functionality related to 3x3 matrices.
pub mod mat3;
/// Contains functionality related to 4x4 matrices.
pub mod mat4;
/// Contains functionality related to quaternions.
pub mod quat;

/// `EPSILON` is a small number `(0.001)` that is used for equality comparisons
pub const EPSILON: f32 = 1e-3;

/// Compares 2 f32 values against an epsilon value, returns true if they're equal
pub fn cmp_f32(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

/// Compares 2 f64 values against an epsilon value, returns true if they're equal
pub fn cmp_f64(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Linearly interpolates between 2 f32 values
pub fn lerp(a: f32, b: f32, factor: f32) -> f32 {
    a + (b - a) * factor
}
