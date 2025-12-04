//! # Whirl
//! A modern graphics toolkit.
//!
//! It's meant to provide a simpler abstraction layer between the user
//! and the graphics card, essentially it functions as a graphics toolkit.
//!
//! ## Features
//! The toolkit currently provides:
//! - Math API (matrices, vectors)
//! - Graphics API that gives safe handles to things like buffers, textures, pipelines
//!
//! ## Uses
//! The toolkit can be used in any context where graphics are involved, such as:
//! - Desktop applications
//! - Games
//! - Simulations
//! - General-purpose rendering

pub mod math;
pub mod graphics;
