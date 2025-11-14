use std::ops::Mul;

use bytemuck::{Pod, Zeroable};

use crate::math::{vec2::Vec2, vec3::Vec3};

/// A matrix represents a linear transformation that is performed on a vector
///
/// A 3x3 matrix [`Mat3`] is useful for 2D transformations and it can do the following:
/// - Translation -> translating (shifting) a vector in 2D space
/// - Scaling -> scaling a vector in 2D space
/// - Rotating -> rotating a vector in 2D space
#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Mat3 {
    /// The vector that determines where the X basis unit vector lands
    pub x_axis: Vec3,
    /// The vector that determines where the Y basis unit vector lands
    pub y_axis: Vec3,
    /// The vector that determines where the Z basis unit vector lands
    pub w_axis: Vec3,
}

impl Mat3 {
    /// Creates a new identity matrix
    pub fn new() -> Self {
        Self {
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            w_axis: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    /// Creates a new scale matrix
    /// - `scale` -> the [`Vec2`] containing the scalar values for `x` and `y`
    pub fn scale(scale: Vec2) -> Self {
        let sx = scale.x;
        let sy = scale.y;
        Self {
            x_axis: Vec3::new(sx, 0.0, 0.0),
            y_axis: Vec3::new(0.0, sy, 0.0),
            w_axis: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    /// Creates a new translation matrix
    /// - `translation` -> the [`Vec2`] containing the translation values for `x` and `y`
    pub fn translate(translation: Vec2) -> Self {
        let tx = translation.x;
        let ty = translation.y;
        Self {
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            w_axis: Vec3::new(tx, ty, 1.0),
        }
    }

    /// Creates a new rotation matrix
    /// - `angle` -> the angle in radians that specifies the amount of rotation
    pub fn rotate(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            x_axis: Vec3::new(cos, sin, 0.0),
            y_axis: Vec3::new(-sin, cos, 0.0),
            w_axis: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    /// Multiplies this matrix by a vector
    ///
    /// This effectively applies the linear transformation described by the matrix
    /// - `self` -> this matrix
    /// - `vec` -> the vector
    pub fn multiply_vec(&self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: (self.x_axis.x * vec.x) + (self.y_axis.x * vec.y) + (self.w_axis.x * vec.z),
            y: (self.x_axis.y * vec.x) + (self.y_axis.y * vec.y) + (self.w_axis.y * vec.z),
            z: (self.x_axis.z * vec.x) + (self.y_axis.z * vec.y) + (self.w_axis.z * vec.z),
        }
    }

    /// Multiplies this matrix by a matrix
    ///
    /// This effectively combines the linear transformations of both those matrices
    /// into a single matrix
    ///
    /// - `self` -> the first matrix
    /// - `mat` -> the second matrix
    pub fn multiply_mat(&self, mat: &Self) -> Self {
        Self {
            x_axis: self.multiply_vec(mat.x_axis),
            y_axis: self.multiply_vec(mat.y_axis),
            w_axis: self.multiply_vec(mat.w_axis),
        }
    }

    /// Transposes this matrix
    ///
    /// Transposing refers to re-arranging the matrix so that rows become columns
    /// and columns become rows
    ///
    /// This is particularly useful if we want to switch between row-major
    /// and column-major matrix representations
    pub fn transpose(self) -> Self {
        Self {
            x_axis: Vec3::new(self.x_axis.x, self.y_axis.x, self.w_axis.x),
            y_axis: Vec3::new(self.x_axis.y, self.y_axis.y, self.w_axis.y),
            w_axis: Vec3::new(self.x_axis.z, self.y_axis.z, self.w_axis.z),
        }
    }

    /// Returns the matrix data as an array
    fn raw(&self) -> [[f32; 3]; 3] {
        [
            [self.x_axis.x, self.x_axis.y, self.x_axis.z],
            [self.y_axis.x, self.y_axis.y, self.y_axis.z],
            [self.w_axis.x, self.w_axis.y, self.w_axis.z],
        ]
    }

    /// Composes a new matrix from an array
    fn of(data: [[f32; 3]; 3]) -> Self {
        Self {
            x_axis: Vec3::new(data[0][0], data[0][1], data[0][2]),
            y_axis: Vec3::new(data[1][0], data[1][1], data[1][2]),
            w_axis: Vec3::new(data[2][0], data[2][1], data[2][2]),
        }
    }

    /// Compares this matrix against another with an epsilon value to account
    /// for floating point inaccuracies
    ///
    /// - `self` -> the first matrix
    /// - `other` -> the second matrix
    /// - `epsilon` -> a very small value
    fn cmp(&self, other: &Self, epsilon: f32) -> bool {
        let m1 = self.raw();
        let m2 = other.raw();
        for i in 0..3 {
            for j in 0..3 {
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

impl Default for Mat3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Mul for Mat3 {
    type Output = Self;

    fn mul(self, mat: Self) -> Self::Output {
        self.multiply_mat(&mat)
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
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
            let m = Mat3::new();
            let v = Vec3::new(1.0, 1.0, 0.0);
            let expected = Vec3::new(1.0, 1.0, 0.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat3::new();
            let v = Vec3::new(-1.0, -2.0, 0.0);
            let expected = Vec3::new(-1.0, -2.0, 0.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn scale() {
        {
            let m = Mat3::scale(Vec2::new(0.1, 0.3));
            let v = Vec3::new(2.0, 1.0, 0.0);
            let expected = Vec3::new(0.2, 0.3, 0.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat3::scale(Vec2::new(0.5, 2.0));
            let v = Vec3::new(2.0, 1.0, 0.0);
            let expected = Vec3::new(1.0, 2.0, 0.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn rotate() {
        {
            let m = Mat3::rotate(PI / 2.0);
            let v = Vec3::new(1.0, 0.0, 0.0);
            let expected = Vec3::new(0.0, 1.0, 0.0);
            assert!((m * v).cmp(expected, EPSILON))
        }

        {
            let m = Mat3::rotate(-PI / 2.0);
            let v = Vec3::new(1.0, 0.0, 0.0);
            let expected = Vec3::new(0.0, -1.0, 0.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn translate() {
        {
            let m = Mat3::translate(Vec2::new(5.0, 10.0));
            let v = Vec3::new(1.0, 2.0, 1.0);
            let expected = Vec3::new(6.0, 12.0, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat3::translate(Vec2::new(-3.4, -12.1));
            let v = Vec3::new(5.9, 2.3, 1.0);
            let expected = Vec3::new(2.5, -9.8, 1.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn multiply_mat() {
        {
            let m1 = Mat3::of([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
            let m2 = Mat3::of([[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]]);
            let expected = Mat3::of([[30.0, 24.0, 18.0], [84.0, 69.0, 54.0], [138.0, 114.0, 90.0]]);
            assert!((m2 * m1).cmp(&expected, EPSILON));
        }

        {
            let m1 = Mat3::of([[-1.0, -2.0, -3.0], [-4.0, -5.0, -6.0], [-7.0, -8.0, -9.0]]);
            let m2 = Mat3::of([[-9.0, -8.0, -7.0], [-6.0, -5.0, -4.0], [-3.0, -2.0, -1.0]]);
            let expected = Mat3::of([[30.0, 24.0, 18.0], [84.0, 69.0, 54.0], [138.0, 114.0, 90.0]]);
            assert!((m2 * m1).cmp(&expected, EPSILON));
        }
    }

    #[test]
    fn multiply_vec() {
        {
            let m = Mat3::of([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
            let v = Vec3::new(5.0, 10.0, 15.0);
            let expected = Vec3::new(150.0, 180.0, 210.0);
            assert!((m * v).cmp(expected, EPSILON));
        }

        {
            let m = Mat3::of([[1.0, 5.0, 10.0], [15.0, 20.0, 25.0], [30.0, 35.0, 40.0]]);
            let v = Vec3::new(1.0, 2.0, 3.0);
            let expected = Vec3::new(121.0, 150.0, 180.0);
            assert!((m * v).cmp(expected, EPSILON));
        }
    }

    #[test]
    fn transform() {
        {
            let scale = Mat3::scale(Vec2::new(10.0, 5.0));
            let rotate = Mat3::rotate(PI / 2.0);
            let translate = Mat3::translate(Vec2::new(100.0, -100.0));
            let vertex = Vec3::new(1.0, 1.0, 1.0);
            let expected = Vec3::new(95.0, -90.0, 1.0);

            assert!((translate * rotate * scale * vertex).cmp(expected, EPSILON));
        }

        {
            let scale = Mat3::scale(Vec2::new(2.0, 2.0));
            let rotate = Mat3::rotate(PI);
            let translate = Mat3::translate(Vec2::new(-17.0, 25.0));
            let vertex = Vec3::new(-5.0, 2.0, 1.0);
            let expected = Vec3::new(44.0, -54.0, 1.0);
            assert!((scale * rotate * translate * vertex).cmp(expected, EPSILON));
        }
    }
}
