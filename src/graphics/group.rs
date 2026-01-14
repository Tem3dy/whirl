use crate::graphics::{buffer::AnyBufferHandle, sampler::Sampler, texture::Texture};

/// Describes a wrapper around the raw [`wgpu::BindGroup`]
#[derive(Debug)]
pub struct BindGroup {
    /// The internal [`wgpu::BindGroup`]
    raw: wgpu::BindGroup,
}

/// Describes a wrapper around the raw [`wgpu::BindGroupLayout`]
#[derive(Debug)]
pub struct BindGroupLayout {
    /// The internal [`wgpu::BindGroupLayout`]
    raw: wgpu::BindGroupLayout,
}

/// Describes a [`BindGroup`].
///
/// A bind group is a collection of GPU resources that get used and consumed by shaders
#[derive(Debug)]
pub struct BindGroupDescriptor<'a> {
    /// The optional debugging label of the bind group
    pub label: Option<&'a str>,
    /// The [`BindGroupLayout`] specifying the layout of the bind group's entries.
    pub layout: &'a BindGroupLayout,
    /// The bind group entries can be of type:
    /// - [`AnyBufferHandle`]
    /// - [`Texture`]
    /// - [`Sampler`]
    pub entries: &'a [BindGroupEntry<'a>],
}

/// Describes a [`BindGroupLayoutDescriptor`].
///
/// A bind group layout is the layout of a collection of GPU resources
/// that are intended to be used and consumed by shaders
#[derive(Debug)]
pub struct BindGroupLayoutDescriptor<'a> {
    /// The optional debugging label of the bind group layout
    pub label: Option<&'a str>,
    /// The bind group layout's entries can be of type:
    /// - [`LayoutResource::Buffer`]
    /// - [`LayoutResource::Texture`]
    /// - [`LayoutResource::Sampler`]
    pub entries: &'a [BindGroupLayoutEntry],
}

/// Describes a [`BindGroupEntry`].
///
/// A bind group entry is a wrapper around the actual GPU resource
/// with a binding which represents the index of the resource in the bind group
#[derive(Debug)]
pub struct BindGroupEntry<'a> {
    /// The index of the resource in the bind group
    pub binding: u32,
    /// The actual resource (buffer, sampler, texture)
    pub resource: Resource<'a>,
}

/// Describes a [`BindGroupLayoutEntry`].
///
/// A bind group layout entry specifies the resource so the GPU
/// knows which resource to expect at which index
#[derive(Debug)]
pub struct BindGroupLayoutEntry {
    /// The index of the resource in the bind group layout
    pub binding: u32,
    /// The expected resource, specified by a configuration
    pub resource: LayoutResource,
    /// The shader stage the resource should be visible in
    pub access: ResourceAccess,
}

/// Describes a wrapper around the actual GPU resource (buffer, sampler, texture)
#[derive(Debug)]
pub enum Resource<'a> {
    /// A buffer resource, holding a reference to an [`AnyBufferHandle`] trait object.
    Buffer(&'a dyn AnyBufferHandle),
    /// A sampler resource, holding a reference to a [`Sampler`]
    Sampler(&'a Sampler),
    /// A texture resource, holding a reference to a [`Texture`]
    Texture(&'a Texture),
}

/// Describes the expected resource type in a [`BindGroupLayout`]
#[derive(Debug, Clone, Copy)]
pub enum LayoutResource {
    /// The expected resource is a buffer specified by a [`BufferConfig`]
    Buffer(BufferConfig),
    /// The expected resource is a sampler specified by a [`SamplerConfig`]
    Sampler(SamplerConfig),
    /// The expected resource is a texture specified by a [`TextureConfig`]
    Texture(TextureConfig),
}

/// Describes the configuration of an expected buffer resource
#[derive(Debug, Clone, Copy)]
pub enum BufferConfig {
    /// The expected resource is a uniform buffer,
    /// which is meant for small amounts of data
    Uniform,
    /// The expected resource is a storage buffer,
    /// which is meant for large amounts of data
    Storage,
}

/// Describes the configuration of an expected sampler resource
#[derive(Debug, Clone, Copy)]
pub enum SamplerConfig {
    /// The expected resource is a sampler that cannot interpolate between pixels (nearest)
    Nearest,
    /// The expected resource is a sampler that can interpolate between pixels (linear)
    Linear,
    /// The expected resource is a sampler that compares between pixels (shadow maps)
    Compare,
}

/// Describes the configuration of an expected texture resource
///
/// # Examples
///
/// - 1D textures are useful for gradients
/// - 2D textures are the most common type of textures, flat 2D images
/// - 3D textures are less common but they're useful for volumetric effects like fog or smoke
/// - Cubemap textures are useful for skyboxes and environments
#[derive(Debug, Clone, Copy)]
pub enum TextureConfig {
    /// The expected resource is a 1D texture of any kind
    D1(TextureKind),
    /// The expected resource is a 2D texture of any kind
    D2(TextureKind),
    /// The expected resource is a 3D texture of any kind
    D3(TextureKind),
    /// The expected resource is a cubemap, which is formed by 6 textures
    Cubemap(TextureKind),
}

/// Describes the kind of the texture
#[derive(Debug, Clone, Copy)]
pub enum TextureKind {
    /// Texture is sourced from image data
    Image,
    /// Texture is sourced from a depth buffer
    Depth,
}

/// Describes the the resource accessibility of a [`Resource`] in a [`BindGroup`]
#[derive(Debug, Clone, Copy)]
pub enum ResourceAccess {
    /// Specifies that the resource is accessible only in the vertex shader
    Vertex,
    /// Specifies that the resource is accessible only in the fragment shader
    Fragment,
    /// Specifies that the resource is accessible by either shader of the two
    Either,
}

impl BindGroup {
    /// Returns a reference to the raw [`wgpu::BindGroup`]
    pub fn raw(&self) -> &wgpu::BindGroup {
        &self.raw
    }
}

impl BindGroupLayout {
    /// Returns a reference to the raw [`wgpu::BindGroupLayout`]
    pub fn raw(&self) -> &wgpu::BindGroupLayout {
        &self.raw
    }
}

impl<'a> BindGroupDescriptor<'a> {
    /// Builds a [`BindGroup`]
    /// - `device` -> the [`wgpu::Device`] required to create a raw [`wgpu::BindGroup`]
    pub fn build(&self, device: &wgpu::Device) -> BindGroup {
        let entries: Vec<_> = self
            .entries
            .iter()
            .map(|entry| wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: entry.resource.raw(),
            })
            .collect();
        BindGroup {
            raw: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: self.label,
                layout: self.layout.raw(),
                entries: Box::leak(entries.into_boxed_slice()),
            }),
        }
    }
}

impl<'a> BindGroupLayoutDescriptor<'a> {
    /// Builds a [`BindGroupLayout`]
    /// - `device` -> the [`wgpu::Device`] required to create a raw [`wgpu::BindGroupLayout`]
    pub fn build(&self, device: &wgpu::Device) -> BindGroupLayout {
        let entries: Vec<_> = self
            .entries
            .iter()
            .map(|entry| wgpu::BindGroupLayoutEntry {
                binding: entry.binding,
                visibility: entry.access.raw(),
                ty: entry.resource.raw(),
                count: None,
            })
            .collect();
        BindGroupLayout {
            raw: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: self.label,
                entries: Box::leak(entries.into_boxed_slice()),
            }),
        }
    }
}

impl<'a> Resource<'a> {
    /// Maps the [`Resource`] to the internal [`wgpu::BindingResource`]
    pub fn raw(&self) -> wgpu::BindingResource<'a> {
        match self {
            Resource::Buffer(buffer) => buffer.raw().as_entire_binding(),
            Resource::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler.raw()),
            Resource::Texture(texture) => wgpu::BindingResource::TextureView(texture.view()),
        }
    }
}

impl LayoutResource {
    /// Maps the [`LayoutResource`] to the internal [`wgpu::BindingType`]
    pub fn raw(&self) -> wgpu::BindingType {
        match self {
            LayoutResource::Buffer(config) => config.raw(),
            LayoutResource::Sampler(config) => config.raw(),
            LayoutResource::Texture(config) => config.raw(),
        }
    }
}

impl BufferConfig {
    /// Maps the [`BufferConfig`] to the internal [`wgpu::BindingType::Buffer`]
    pub fn raw(&self) -> wgpu::BindingType {
        match self {
            BufferConfig::Uniform => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            BufferConfig::Storage => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        }
    }
}

impl SamplerConfig {
    /// Maps the [`SamplerConfig`] to the internal [`wgpu::BindingType::Sampler`]
    pub fn raw(&self) -> wgpu::BindingType {
        match self {
            SamplerConfig::Nearest => {
                wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering)
            }
            SamplerConfig::Linear => {
                wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
            }
            SamplerConfig::Compare => {
                wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison)
            }
        }
    }
}

impl TextureConfig {
    /// Maps the [`TextureConfig`] to the internal [`wgpu::BindingType::Texture`]
    #[rustfmt::skip]
    pub fn raw(&self) -> wgpu::BindingType {
        let sample_type = match self {
            TextureConfig::D1(kind) |
            TextureConfig::D2(kind) |
            TextureConfig::D3(kind) |
            TextureConfig::Cubemap(kind) => match kind {
                TextureKind::Image => wgpu::TextureSampleType::Float { filterable: true },
                TextureKind::Depth => wgpu::TextureSampleType::Depth,
            },
        };
        let view_dimension = match self {
            TextureConfig::D1(_) => wgpu::TextureViewDimension::D1,
            TextureConfig::D2(_) => wgpu::TextureViewDimension::D2,
            TextureConfig::D3(_) => wgpu::TextureViewDimension::D3,
            TextureConfig::Cubemap(_) => wgpu::TextureViewDimension::Cube,
        };
        wgpu::BindingType::Texture {
            sample_type,
            view_dimension,
            multisampled: false,
        }
    }
}

impl ResourceAccess {
    /// Maps the [`ResourceAccess`] to the internal [`wgpu::ShaderStages`]
    pub fn raw(&self) -> wgpu::ShaderStages {
        match self {
            ResourceAccess::Vertex => wgpu::ShaderStages::VERTEX,
            ResourceAccess::Fragment => wgpu::ShaderStages::FRAGMENT,
            ResourceAccess::Either => wgpu::ShaderStages::VERTEX_FRAGMENT,
        }
    }
}

/// A builder utility for creating a [`BindGroup`] in a more ergonomic way.
#[derive(Debug)]
pub struct BindGroupBuilder<'a> {
    /// The optional debugging label of this bind group
    label: Option<&'a str>,
    /// The current location of a resource
    cursor: u32,
    /// The list of resources
    entries: Vec<BindGroupEntry<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    /// Creates a new [`BindGroupBuilder`].
    pub fn new() -> Self {
        Self {
            label: None,
            cursor: 0,
            entries: Vec::with_capacity(4),
        }
    }

    /// Sets a label to this [`BindGroup`].
    /// - `label` -> the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Adds a buffer resource.
    pub fn add_buffer(mut self, buffer: &'a dyn AnyBufferHandle) -> Self {
        self.entries.push(BindGroupEntry {
            binding: self.cursor,
            resource: Resource::Buffer(buffer),
        });
        self.cursor += 1;
        self
    }

    /// Adds a sampler resource.
    pub fn add_sampler(mut self, sampler: &'a Sampler) -> Self {
        self.entries.push(BindGroupEntry {
            binding: self.cursor,
            resource: Resource::Sampler(sampler),
        });
        self.cursor += 1;
        self
    }

    /// Adds a texture resource.
    pub fn add_texture(mut self, texture: &'a Texture) -> Self {
        self.entries.push(BindGroupEntry {
            binding: self.cursor,
            resource: Resource::Texture(texture),
        });
        self.cursor += 1;
        self
    }

    /// Builds a [`BindGroup`] and consumes this [`BindGroupBuilder`].
    /// - `layout` -> the matching [`BindGroupLayout`]
    /// - `device` -> the device needed to create the [`BindGroup`]
    ///
    /// If the entry list is empty, the caller thread panics.
    pub fn build(self, layout: &BindGroupLayout, device: &wgpu::Device) -> BindGroup {
        if self.entries.is_empty() {
            panic!("Couldn't create a `BindGroup`, missing entries!");
        }

        BindGroupDescriptor {
            label: self.label,
            layout,
            entries: &self.entries,
        }
        .build(device)
    }
}

/// A builder utility for creating a [`BindGroupLayout`] in a more ergonomic way.
#[derive(Debug)]
pub struct BindGroupLayoutBuilder<'a> {
    /// The optional debugging label of this bind group layout
    label: Option<&'a str>,
    /// The current location of a layout resource
    cursor: u32,
    /// The list of layout resources
    entries: Vec<BindGroupLayoutEntry>,
}

impl<'a> BindGroupLayoutBuilder<'a> {
    /// Creates a new [`BindGroupLayoutBuilder`].
    pub fn new() -> Self {
        Self {
            label: None,
            cursor: 0,
            entries: Vec::with_capacity(4),
        }
    }

    /// Sets a label to this [`BindGroupLayout`].
    /// - `label` -> the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Adds a uniform buffer layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_uniform_buffer(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Buffer(BufferConfig::Uniform),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a storage buffer layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_storage_buffer(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Buffer(BufferConfig::Storage),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a nearest sampler layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_nearest_sampler(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Sampler(SamplerConfig::Nearest),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a linear sampler layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_linear_sampler(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Sampler(SamplerConfig::Linear),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a compare sampler layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_compare_sampler(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Sampler(SamplerConfig::Compare),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a depth texture layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_depth_texture(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Texture(TextureConfig::D2(TextureKind::Depth)),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a 1D texture layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_texture_1d(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Texture(TextureConfig::D1(TextureKind::Image)),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a 2D texture layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_texture_2d(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Texture(TextureConfig::D2(TextureKind::Image)),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a 3D texture layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_texture_3d(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Texture(TextureConfig::D3(TextureKind::Image)),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Adds a cubemap texture layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_texture_cubemap(mut self, access: ResourceAccess) -> Self {
        self.entries.push(BindGroupLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Texture(TextureConfig::Cubemap(TextureKind::Image)),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Builds a [`BindGroupLayout`] and consumes this [`BindGroupLayoutBuilder`].
    /// - `device` -> the device needed to create the [`BindGroupLayout`]
    ///
    /// If the entry list is empty, the caller thread panics.
    pub fn build(self, device: &wgpu::Device) -> BindGroupLayout {
        if self.entries.is_empty() {
            panic!("Couldn't create a `BindGroupLayout`, missing entries!");
        }

        BindGroupLayoutDescriptor {
            label: self.label,
            entries: &self.entries,
        }
        .build(device)
    }
}
