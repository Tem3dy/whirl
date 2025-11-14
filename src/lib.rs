//! # Whirl
//! A modern graphics engine.
//!
//! It's meant to provide a simpler abstraction layer between the user
//! and the graphics card, essentially it functions as a graphics toolkit.
//!
//! ## Features
//! The engine currently supports these things:
//! - Math tools (matrices, vectors)
//! - Abstractions over graphics concepts
//!
//! ## Uses
//! The engine can be used in any context where graphics are involved, such as:
//! - Desktop applications
//! - Games
//! - Simulations
//! - General-purpose rendering

/// The math module, containing all the essential types and functionality.
pub mod math {
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
}

/// The engine module, containing all the essential GPU abstractions and functionality.
pub mod engine {
    /// Contains functionality related to GPU buffers.
    pub mod buffer;
    /// Contains functionality related to GPU colors.
    pub mod color;
    /// Contains functionality related to GPU bind groups and bind group layouts.
    pub mod group;
    /// Contains functionality related to GPU buffer layouts.
    pub mod layout;
    /// Contains functionality related to GPU render passes.
    pub mod pass;
    /// Contains functionality related to GPU pipelines.
    pub mod pipeline;
    /// Contains functionality related to GPU samplers.
    pub mod sampler;
    /// Contains functionality related to GPU shaders.
    pub mod shader;
    /// Contains functionality related to GPU textures.
    pub mod texture;
}
