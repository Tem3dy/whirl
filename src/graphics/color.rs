use crate::math::vec4::Vec4;

/// Specifies a color in the RGBA color space.
///
/// A color is determined by 4 channels:
/// - Red
/// - Green
/// - Blue
/// - Alpha
///
/// The `Red`, `Green`, `Blue` channels define the actual color.
///
/// The `Alpha` channel defines the opacity of the color,
/// and comes into play whenever blending is enabled.
///
/// An alpha channel with a value of `1.0` means that the color is fully opaque,
/// meanwhile an alpha channel with a value of `0.0` means that the color is fully transparent.
///
/// Internally, all channels are stored as normalized floats.
/// Normalized meaning they're in the range of `0.0 - 1.0`.
///
/// # Examples:
/// ```rust
/// const RED: Color = Color::opaque(
///     1.0, // Red channel (full)
///     0.0, // Green channel (no value)
///     0.0, // Blue channel (no value)
/// );
///
/// const GREEN: Color = Color::opaque(
///     0.0, // Red channel (no value)
///     1.0, // Green channel (full)
///     0.0, // Blue channel (no value)
/// );
///
/// const BLUE: Color = Color::opaque(
///     0.0, // Red channel (no value)
///     0.0, // Green channel (no value)
///     1.0, // Blue channel (full)
/// )
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    /// An opaque white color.
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    /// An opaque black color.
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// An opaque red color.
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// An opaque green color.
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    /// An opaque blue color.
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    /// Creates a new [`Color`].
    ///
    /// If the value of any channel is beyond the normalized range (`0.0 - 1.0`),
    /// the value of the channel is clamped to the normalized range.
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            r: red.clamp(0.0, 1.0),
            g: green.clamp(0.0, 1.0),
            b: blue.clamp(0.0, 1.0),
            a: alpha.clamp(0.0, 1.0),
        }
    }

    /// Creates a new opaque [`Color`].
    ///
    /// An opaque color is simply a color with the alpha value set to `1.0`,
    /// meaning the color is fully opaque.
    pub fn opaque(red: f32, green: f32, blue: f32) -> Self {
        Self::new(red, green, blue, 1.0)
    }

    /// Darkens the color by a factor ranging from `0.0` to `1.0`
    ///
    /// If the factor would exceed the channel's min/max value,
    /// the channel's value is clamped to the normalized range
    pub fn darken(&self, factor: f32) -> Self {
        Self {
            r: (self.r * (1.0 - factor.clamp(0.0, 1.0))).clamp(0.0, 1.0),
            g: (self.g * (1.0 - factor.clamp(0.0, 1.0))).clamp(0.0, 1.0),
            b: (self.b * (1.0 - factor.clamp(0.0, 1.0))).clamp(0.0, 1.0),
            a: self.a,
        }
    }

    /// Lightens the color by a factor ranging from `0.0` to `1.0`
    ///
    /// If the factor would exceed the channel's min/max value,
    /// the channel's value is clamped to the normalized range
    pub fn lighten(&self, factor: f32) -> Self {
        Self {
            r: (self.r * (1.0 + factor.clamp(0.0, 1.0))).clamp(0.0, 1.0),
            g: (self.g * (1.0 + factor.clamp(0.0, 1.0))).clamp(0.0, 1.0),
            b: (self.b * (1.0 + factor.clamp(0.0, 1.0))).clamp(0.0, 1.0),
            a: self.a,
        }
    }

    /// Returns the value of the red channel.
    pub fn red(self) -> f32 {
        self.r
    }

    /// Returns the value of the green channel.
    pub fn green(self) -> f32 {
        self.g
    }

    /// Returns the value of the blue channel.
    pub fn blue(self) -> f32 {
        self.b
    }

    /// Returns the value of the alpha channel.
    pub fn alpha(self) -> f32 {
        self.a
    }

    /// Maps the color to a [`wgpu::Color`].
    pub fn raw(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }

    /// Maps the color to a [`Vec4`].
    pub fn as_vec(self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}
