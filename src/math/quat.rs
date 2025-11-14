use std::ops::Mul;

use crate::math::vec3::Vec3;

/// A quaternion describes rotation in 3D with an axis and an angle
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    /// The X component of the rotation axis
    pub x: f32,
    /// The Y component of the rotation axis
    pub y: f32,
    /// The Z component of the rotation axis
    pub z: f32,
    /// The rotation angle about the rotation axis
    pub w: f32,
}

impl Quat {
    /// Creates a new identity quaternion
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Creates a new quaternion from the values `(x, y, z, w)`
    pub fn of(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a new quaternion from a rotation axis and an angle
    /// - `axis` -> is usually a basis vector in a specified direction (right, forward, up, ...)
    /// - `angle` -> the amount of rotation in radians
    pub fn from_axis(axis: Vec3, angle: f32) -> Self {
        let half_angle = angle / 2.0;
        let sin_half = half_angle.sin();
        let cos_half = half_angle.cos();
        let axis = axis.normalize();

        Self {
            x: axis.x * sin_half,
            y: axis.y * sin_half,
            z: axis.z * sin_half,
            w: cos_half,
        }
        .normalize()
    }

    /// Creates a new quaternion from the euler angles `(pitch, yaw, roll)`
    /// - `pitch` -> the amount of rotation about the X axis in radians
    /// - `yaw` -> the amount of rotation about the Y axis in radians
    /// - `roll` -> the amount of rotation about the Z axis in radians
    pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Self {
        let pitch = pitch / 2.0;
        let yaw = yaw / 2.0;
        let roll = roll / 2.0;

        let pitch_cos = pitch.cos();
        let yaw_cos = yaw.cos();
        let roll_cos = roll.cos();

        let pitch_sin = pitch.sin();
        let yaw_sin = yaw.sin();
        let roll_sin = roll.sin();

        let q_yaw = Self {
            x: 0.0,
            y: yaw_sin,
            z: 0.0,
            w: yaw_cos,
        };
        let q_pitch = Self {
            x: pitch_sin,
            y: 0.0,
            z: 0.0,
            w: pitch_cos,
        };
        let q_roll = Self {
            x: 0.0,
            y: 0.0,
            z: roll_sin,
            w: roll_cos,
        };
        q_yaw * q_pitch * q_roll
    }

    /// Multiplies 2 quaternions together, resulting in a combined rotation
    /// - `self` -> the first quaternion
    /// - `other` -> the second quaternion
    pub fn multiply(&self, other: &Self) -> Self {
        Self {
            x: (self.w * other.x) + (self.x * other.w) + (self.y * other.z) - (self.z * other.y),
            y: (self.w * other.y) - (self.x * other.z) + (self.y * other.w) + (self.z * other.x),
            z: (self.w * other.z) + (self.x * other.y) - (self.y * other.x) + (self.z * other.w),
            w: (self.w * other.w) - (self.x * other.x) - (self.y * other.y) - (self.z * other.z),
        }
        .normalize()
    }

    /// Inverses the quaternion, which results in an inversed rotation effect
    pub fn inverse(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    /// Computes the dot product between 2 quaternions
    /// - `self` -> the first quaternion
    /// - `other` -> the second quaternion
    pub fn dot(&self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    /// Normalizes the quaternion
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len < f32::EPSILON {
            panic!("Division by near-zero ({}) length in quaternion!", len);
        }

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }

    /// Computes the length of the quaternion
    pub fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    /// Compares 2 quaternions and returns if they're equal or not
    /// - `self` -> the first quaternion
    /// - `other` -> the second quaternion
    /// - `epsilon` -> a very small value to account for floating-point errors
    pub fn cmp(&self, other: &Self, epsilon: f32) -> bool {
        let x_cmp = (self.x - other.x).abs() < epsilon;
        let y_cmp = (self.y - other.y).abs() < epsilon;
        let z_cmp = (self.z - other.z).abs() < epsilon;
        let w_cmp = (self.w - other.w).abs() < epsilon;
        x_cmp && y_cmp && z_cmp && w_cmp
    }
}

impl Mul for Quat {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.multiply(&other)
    }
}

#[cfg(test)]
mod tests {
    // Implement unit tests
}
