use std::ops::Mul;

use bytemuck::{Pod, Zeroable};

use crate::math::{quat::Quat, vec3::Vec3, vec4::Vec4};

/// A matrix represents a linear transformation that is performed on a vector
///
/// A 4x4 matrix [`Mat4`] is useful for 3D transformations and it can do the following:
/// - Translation -> translating (shifting) a vector in 3D space
/// - Scaling -> scaling a vector in 3D space
/// - Rotating -> rotating a vector in 3D space (with the help of quaternions)
#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Mat4 {
    /// The vector that determines where the X basis unit vector lands
    pub x_axis: Vec4,
    /// The vector that determines where the Y basis unit vector lands
    pub y_axis: Vec4,
    /// The vector that determines where the Z basis unit vector lands
    pub z_axis: Vec4,
    /// The vector that's used for homogenous coordinates, encoding the 3D translation
    pub w_axis: Vec4,
}

impl Mat4 {
    /// Creates a new identity matrix
    pub fn new() -> Self {
        Self {
            x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, 1.0, 0.0),
            w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Creates a new scale matrix
    /// - `scale` -> the [`Vec3`] containing the scalar values for `x`, `y` and `z`
    pub fn scale(scale: Vec3) -> Self {
        let sx = scale.x;
        let sy = scale.y;
        let sz = scale.z;
        Self {
            x_axis: Vec4::new(sx, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, sy, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, sz, 0.0),
            w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Creates a new translation matrix
    /// - `translation` -> the [`Vec3`] containing the translation values for `x`, `y` and `z`
    pub fn translate(translation: Vec3) -> Self {
        let tx = translation.x;
        let ty = translation.y;
        let tz = translation.z;
        Self {
            x_axis: Vec4::new(1.0, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, 1.0, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, 1.0, 0.0),
            w_axis: Vec4::new(tx, ty, tz, 1.0),
        }
    }

    /// Creates a new rotation matrix from a quaternion
    /// - `quat` -> the [`Quat`] that specifies the rotation axis and the rotation amount
    pub fn rotate(quat: Quat) -> Self {
        let q = quat.normalize();
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);

        let i_rot_x = 1.0 - 2.0 * (y * y + z * z);
        let j_rot_x = 2.0 * (x * y - z * w);
        let k_rot_x = 2.0 * (x * z + y * w);

        let i_rot_y = 2.0 * (x * y + z * w);
        let j_rot_y = 1.0 - 2.0 * (x * x + z * z);
        let k_rot_y = 2.0 * (y * z - x * w);

        let i_rot_z = 2.0 * (x * z - y * w);
        let j_rot_z = 2.0 * (y * z + x * w);
        let k_rot_z = 1.0 - 2.0 * (x * x + y * y);

        Self {
            x_axis: Vec4::new(i_rot_x, i_rot_y, i_rot_z, 0.0),
            y_axis: Vec4::new(j_rot_x, j_rot_y, j_rot_z, 0.0),
            z_axis: Vec4::new(k_rot_x, k_rot_y, k_rot_z, 0.0),
            w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    /// Multiplies this matrix by a vector
    ///
    /// This effectively applies the linear transformation described by the matrix
    /// - `self` -> this matrix
    /// - `vec` -> the vector
    #[rustfmt::skip]
    pub fn multiply_vec(&self, vec: Vec4) -> Vec4 {
        Vec4 {
            x: (self.x_axis.x * vec.x) + (self.y_axis.x * vec.y) + (self.z_axis.x * vec.z) + (self.w_axis.x * vec.w),
            y: (self.x_axis.y * vec.x) + (self.y_axis.y * vec.y) + (self.z_axis.y * vec.z) + (self.w_axis.y * vec.w),
            z: (self.x_axis.z * vec.x) + (self.y_axis.z * vec.y) + (self.z_axis.z * vec.z) + (self.w_axis.z * vec.w),
            w: (self.x_axis.w * vec.x) + (self.y_axis.w * vec.y) + (self.z_axis.w * vec.z) + (self.w_axis.w * vec.w),
        }
    }

    /// Multiplies this matrix by a matrix
    ///
    /// This effectively combines the linear transformations of both those matrices
    /// into a single matrix
    ///
    /// `self` -> the first matrix
    /// `mat` -> the second matrix
    #[rustfmt::skip]
    pub fn multiply_mat(&self, mat: &Self) -> Self {
        Self {
            x_axis: self.multiply_vec(mat.x_axis),
            y_axis: self.multiply_vec(mat.y_axis),
            z_axis: self.multiply_vec(mat.z_axis),
            w_axis: self.multiply_vec(mat.w_axis),
        }
    }

    /// Transposes this matrix
    ///
    /// Transposing refers to re-arranging the matrix so that rows become columns
    /// and columns become rows
    ///
    /// This is particularly useful if we want to switch between row-major and
    /// column-major matrix representations
    pub fn transpose(self) -> Self {
        Self {
            x_axis: Vec4::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            y_axis: Vec4::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            z_axis: Vec4::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            w_axis: Vec4::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        }
    }

    /// Creates a new view matrix
    ///
    /// A view matrix describes a camera in 3D space by the following 3 components:
    /// - `eye` -> the 3D position of the camera, also known as the "eye" of the camera
    /// - `target` -> the 3D position that specifies where the camera is looking at
    /// - `up` -> a unit vector specifying which axis is the direction `UP`, usually `(0.0, 1.0, 0.0)`
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        // Forward vector
        let f = (eye - target).normalize();
        // Right vector
        let r = f.cross(up).normalize();
        // Up vector
        let u = r.cross(f).normalize();
        let view = Self {
            x_axis: Vec4::new(r.x, r.y, r.z, 0.0),
            y_axis: Vec4::new(u.x, u.y, u.z, 0.0),
            z_axis: Vec4::new(f.x, f.y, f.z, 0.0),
            w_axis: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
        .transpose();

        view * Self::translate(Vec3::new(-eye.x, -eye.y, -eye.z))
    }

    /// Creates a new orthographic projection matrix
    ///
    /// A ortographic projection matrix describes the transformation from view to clip space
    ///
    /// It's essentially responsible for projecting a scene into a 2D plane (a screen)
    /// without perspective
    ///
    /// It's composed of the following:
    /// - `left` -> the left plane
    /// - `right` -> the right plane
    /// - `top` -> the top plane
    /// - `bottom` -> the bottom plane
    /// - `near` -> the near plane
    /// - `far` -> the far plane
    ///
    /// These 6 planes together form a cube frustum and then transform positions
    /// in this frustum into normalized device coordinates
    ///
    /// The orthographic projection matrix is useful for 2D, but for 3D,
    /// we usually want a perspective projection matrix
    pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Self {
        let w = right - left;
        let h = top - bottom;
        let d = far - near;

        Self {
            x_axis: Vec4::new(2.0 / w, 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, 2.0 / h, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, 1.0 / d, 0.0),
            w_axis: Vec4::new(
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                -near / (far - near),
                1.0,
            ),
        }
    }

    /// Creates a new perspective projection matrix
    ///
    /// A perspective projection matrix describes the transformation from view to clip space
    ///
    /// It's essentially responsible for projecting a 3D scene into a 2D plane (a screen)
    /// and applying perspective
    ///
    /// It's composed of the following:
    /// - `fov` -> the field of view in radians, representing the vertical angle of sight
    /// - `aspect_ratio` -> the aspect ratio of the window
    /// - `near` -> the near plane (how close we are able to see before clipping)
    /// - `far` -> the far plane (how far we are able to see before clipping)
    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let half_height = (fov / 2.0).tan();
        Self {
            x_axis: Vec4::new(1.0 / (aspect_ratio * half_height), 0.0, 0.0, 0.0),
            y_axis: Vec4::new(0.0, 1.0 / half_height, 0.0, 0.0),
            z_axis: Vec4::new(0.0, 0.0, -((far + near) / (far - near)), -1.0),
            w_axis: Vec4::new(0.0, 0.0, -((2.0 * far * near) / (far - near)), 0.0),
        }
    }

    /// Returns the matrix data as an array
    pub fn raw(&self) -> [[f32; 4]; 4] {
        [
            [self.x_axis.x, self.x_axis.y, self.x_axis.z, self.x_axis.w],
            [self.y_axis.x, self.y_axis.y, self.y_axis.z, self.y_axis.w],
            [self.z_axis.x, self.z_axis.y, self.z_axis.z, self.z_axis.w],
            [self.w_axis.x, self.w_axis.y, self.w_axis.z, self.w_axis.w],
        ]
    }

    /// Composes a new matrix from an array
    pub fn of(data: [[f32; 4]; 4]) -> Self {
        Self {
            x_axis: Vec4::new(data[0][0], data[0][1], data[0][2], data[0][3]),
            y_axis: Vec4::new(data[1][0], data[1][1], data[1][2], data[1][3]),
            z_axis: Vec4::new(data[2][0], data[2][1], data[2][2], data[2][3]),
            w_axis: Vec4::new(data[3][0], data[3][1], data[3][2], data[3][3]),
        }
    }

    /// Compares this matrix against another with an epsilon value to account
    /// for floating point inaccuracies
    ///
    /// - `self` -> the first matrix
    /// - `other` -> the second matrix
    /// - `epsilon` -> a very small value
    pub fn cmp(&self, other: &Self, epsilon: f32) -> bool {
        let m1 = self.raw();
        let m2 = other.raw();
        for i in 0..4 {
            for j in 0..4 {
                let x = m1[i][j];
                let y = m2[i][j];

                let d = (x - y).abs();
                if d > epsilon {
                    return false;
                }
            }
        }

        true
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, mat: Self) -> Self::Output {
        self.multiply_mat(&mat)
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        self.multiply_vec(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::EPSILON;
    use std::f32::consts::PI;

    #[test]
    fn new() {
        {
            let m = Mat4::new();
            let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
            let expected = Vec4::new(1.0, 1.0, 1.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat4::new();
            let v = Vec4::new(-1.0, -2.0, -3.0, 1.0);
            let expected = Vec4::new(-1.0, -2.0, -3.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn scale() {
        {
            let m = Mat4::scale(Vec3::new(0.1, 0.3, 0.5));
            let v = Vec4::new(3.0, 2.0, 1.0, 1.0);
            let expected = Vec4::new(0.3, 0.6, 0.5, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat4::scale(Vec3::new(0.5, 2.0, 8.0));
            let v = Vec4::new(4.0, 3.0, 2.0, 1.0);
            let expected = Vec4::new(2.0, 6.0, 16.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn rotate() {
        {
            let m = Mat4::rotate(Quat::from_axis(Vec3::new(1.0, 0.0, 0.0), PI / 2.0));
            let v = Vec4::new(0.0, 1.0, 0.0, 1.0);
            let expected = Vec4::new(0.0, 0.0, 1.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat4::rotate(Quat::from_axis(Vec3::new(0.0, 1.0, 0.0), PI / 2.0));
            let v = Vec4::new(1.0, 0.0, 0.0, 1.0);
            let expected = Vec4::new(0.0, 0.0, -1.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat4::rotate(Quat::from_axis(Vec3::new(0.0, 0.0, 1.0), PI / 2.0));
            let v = Vec4::new(0.0, 1.0, 0.0, 1.0);
            let expected = Vec4::new(-1.0, 0.0, 0.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat4::rotate(Quat::from_euler(PI / 2.0, PI / 2.0, -PI / 2.0));
            let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
            let expected = Vec4::new(-1.0, -1.0, -1.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn translate() {
        {
            let m = Mat4::translate(Vec3::new(5.0, 10.0, 15.0));
            let v = Vec4::new(1.0, 2.0, 1.0, 1.0);
            let expected = Vec4::new(6.0, 12.0, 16.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat4::translate(Vec3::new(-3.4, -12.1, 17.4));
            let v = Vec4::new(5.9, 2.3, 1.0, 1.0);
            let expected = Vec4::new(2.5, -9.8, 18.4, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn multiply_mat() {
        {
            let m1 = Mat4::of([
                [1.0, 1.0, 1.0, 1.0],
                [2.0, 2.0, 2.0, 2.0],
                [3.0, 3.0, 3.0, 3.0],
                [4.0, 4.0, 4.0, 4.0],
            ]);
            let m2 = Mat4::of([
                [1.0, 1.0, 1.0, 1.0],
                [2.0, 2.0, 2.0, 2.0],
                [3.0, 3.0, 3.0, 3.0],
                [4.0, 4.0, 4.0, 4.0],
            ]);

            let expected = Mat4::of([
                [10.0, 10.0, 10.0, 10.0],
                [20.0, 20.0, 20.0, 20.0],
                [30.0, 30.0, 30.0, 30.0],
                [40.0, 40.0, 40.0, 40.0],
            ]);
            assert!((m2 * m1).cmp(&expected, EPSILON));
        }

        {
            let m1 = Mat4::of([
                [-1.0, -1.0, -1.0, -1.0],
                [-2.0, -2.0, -2.0, -2.0],
                [-3.0, -3.0, -3.0, -3.0],
                [-4.0, -4.0, -4.0, -4.0],
            ]);
            let m2 = Mat4::of([
                [-1.0, -1.0, -1.0, -1.0],
                [-2.0, -2.0, -2.0, -2.0],
                [-3.0, -3.0, -3.0, -3.0],
                [-4.0, -4.0, -4.0, -4.0],
            ]);

            let expected = Mat4::of([
                [10.0, 10.0, 10.0, 10.0],
                [20.0, 20.0, 20.0, 20.0],
                [30.0, 30.0, 30.0, 30.0],
                [40.0, 40.0, 40.0, 40.0],
            ]);
            assert!((m2 * m1).cmp(&expected, EPSILON));
        }
    }

    #[rustfmt::skip]
    #[test]
    fn multiply_vec() {
        {
            let m = Mat4::of([
                [ 1.0,  2.0,  3.0,  4.0],
                [ 5.0,  6.0,  7.0,  8.0],
                [ 9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);
            let v = Vec4::new(5.0, 10.0, 15.0, 20.0);
            let expected = Vec4::new(450.0, 500.0, 550.0, 600.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat4::of([
                [ 0.0,  5.0, 10.0, 15.0],
                [20.0, 25.0, 30.0, 35.0],
                [40.0, 45.0, 50.0, 55.0],
                [60.0, 65.0, 70.0, 75.0],
            ]);
            let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
            let expected = Vec4::new(400.0, 450.0, 500.0, 550.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn transform() {
        {
            let scale = Mat4::scale(Vec3::new(2.0, 2.0, 2.0));
            let rotation = Mat4::rotate(Quat::from_euler(PI / 2.0, 0.0, 0.0));
            let translation = Mat4::translate(Vec3::new(10.0, 5.0, 0.0));
            let vertex = Vec4::new(1.0, 1.0, 1.0, 1.0);
            let expected = Vec4::new(12.0, 3.0, 2.0, 1.0);
            assert!((translation * rotation * scale * vertex).cmp(expected, EPSILON));
        }

        {
            let scale = Mat4::scale(Vec3::new(1.5, 1.5, 1.5));
            let rotation = Mat4::rotate(Quat::from_euler(0.0, 0.0, PI / 2.0));
            let translation = Mat4::translate(Vec3::new(5.0, 4.0, 3.0));
            let vertex = Vec4::new(1.0, 1.0, 1.0, 1.0);
            let expected = Vec4::new(3.5, 5.5, 4.5, 1.0);
            assert!((translation * rotation * scale * vertex).cmp(expected, EPSILON));
        }
    }
}
