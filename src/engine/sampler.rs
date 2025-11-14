/// Describes a sampler wrapper around the raw [`wgpu::Sampler`]
#[derive(Debug)]
pub struct Sampler {
    /// The internal [`wgpu::Sampler`]
    raw: wgpu::Sampler,
}

/// Describes a sampler
#[derive(Debug)]
pub struct SamplerDescriptor {
    /// The debugging label of this sampler
    pub label: &'static str,
    /// The texture wrapping mode to use
    pub wrapping: TextureWrapping,
    /// The texture filtering more to use
    pub filtering: TextureFiltering,
}

/// Describes a texture wrapping configuration
///
/// This will be used if any of the texture coordinates (U, V, W) exceeds the normalized range
#[derive(Debug, Clone, Copy)]
pub enum TextureWrapping {
    /// Makes a black border around the texture
    ClampToBorder,
    /// Extends the edges of the texture infinitely
    ClampToEdge,
    /// Repeats the texture
    Repeat,
    /// Repeats the texture but starts mirrored
    Mirror,
}

/// Describes a texture filtering configuration
///
/// Nearest texture filtering is useful for preserving crisp images such as pixel art
///
/// Linear texture filtering is useful for more high resolution images
/// where the blurriness isn't noticeable anymore
///
#[derive(Debug, Clone, Copy)]
pub enum TextureFiltering {
    /// Picks the texel that the texture coordinate maps closest to
    Nearest,
    /// Linearly interpolates between the neighboring texels the texture coordinate
    /// maps closest to
    Linear,
}

impl Sampler {
    /// Returns a reference to the raw [`wgpu::Sampler`]
    pub fn raw(&self) -> &wgpu::Sampler {
        &self.raw
    }
}

impl SamplerDescriptor {
    /// Builds a [`Sampler`]
    pub fn build(self, device: &wgpu::Device) -> Sampler {
        let wrapping = self.wrapping.raw();
        let filtering = self.filtering.raw();
        Sampler {
            raw: device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some(self.label),
                address_mode_u: wrapping,
                address_mode_v: wrapping,
                address_mode_w: wrapping,
                mag_filter: filtering,
                min_filter: filtering,
                mipmap_filter: filtering,
                lod_min_clamp: 0.0,
                lod_max_clamp: 100.0,
                compare: None,
                anisotropy_clamp: 1,
                border_color: if let TextureWrapping::ClampToBorder = self.wrapping {
                    Some(wgpu::SamplerBorderColor::OpaqueBlack)
                } else {
                    None
                },
            }),
        }
    }
}

impl TextureWrapping {
    /// Maps the [`TextureWrapping`] to the internal [`wgpu::AddressMode`]
    pub fn raw(self) -> wgpu::AddressMode {
        match self {
            TextureWrapping::ClampToBorder => wgpu::AddressMode::ClampToBorder,
            TextureWrapping::ClampToEdge => wgpu::AddressMode::ClampToEdge,
            TextureWrapping::Repeat => wgpu::AddressMode::Repeat,
            TextureWrapping::Mirror => wgpu::AddressMode::MirrorRepeat,
        }
    }
}

impl TextureFiltering {
    /// Maps the [`TextureFiltering`] to the internal [`wgpu::FilterMode`]
    pub fn raw(self) -> wgpu::FilterMode {
        match self {
            TextureFiltering::Nearest => wgpu::FilterMode::Nearest,
            TextureFiltering::Linear => wgpu::FilterMode::Linear,
        }
    }
}
