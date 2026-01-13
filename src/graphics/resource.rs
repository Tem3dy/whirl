use crate::graphics::{buffer::AnyBufferHandle, sampler::Sampler, texture::Texture};

/// Describes a wrapper around the raw [`wgpu::BindGroup`]
#[derive(Debug)]
pub struct ResourceSet {
    /// The internal [`wgpu::BindGroup`]
    raw: wgpu::BindGroup,
}

/// Describes a wrapper around the raw [`wgpu::BindGroupLayout`]
#[derive(Debug)]
pub struct ResourceSetLayout {
    /// The internal [`wgpu::BindGroupLayout`]
    raw: wgpu::BindGroupLayout,
}

/// Describes a [`ResourceSet`].
///
/// A resource set is a collection of GPU resources that get used and consumed by shaders
#[derive(Debug)]
pub struct ResourceSetDescriptor<'a> {
    /// The optional debugging label of the resource set
    pub label: Option<&'a str>,
    /// The [`ResourceSetLayout`] specifying the layout of the resource set's entries
    pub layout: &'a ResourceSetLayout,
    /// The resource set entries can be of type:
    /// - [`Buffer`]
    /// - [`Texture`]
    /// - [`Sampler`]
    pub entries: &'a [ResourceSetEntry<'a>],
}

/// Describes a [`ResourceSetLayout`].
///
/// A resource set layout is the layout of a collection of GPU resources
/// that are intended to be used and consumed by shaders
#[derive(Debug)]
pub struct ResourceSetLayoutDescriptor<'a> {
    /// The optional debugging label of the resource set layout
    pub label: Option<&'a str>,
    /// The resource set layout entries can be of type:
    /// - [`LayoutResource::Buffer`]
    /// - [`LayoutResource::Texture`]
    /// - [`LayoutResource::Sampler`]
    pub entries: &'a [ResourceSetLayoutEntry],
}

/// Describes a [`ResourceSetEntry`].
///
/// A resource set entry is a wrapper around the actual GPU resource
/// with a binding which represents the index of the resource in the resource set
#[derive(Debug)]
pub struct ResourceSetEntry<'a> {
    /// The index of the resource in the resource set
    pub binding: u32,
    /// The actual resource (buffer, sampler, texture)
    pub resource: Resource<'a>,
}

/// Describes a [`ResourceSetLayoutEntry`].
///
/// A resource set layout entry specifies the resource so the GPU
/// knows which resource to expect at which index
#[derive(Debug)]
pub struct ResourceSetLayoutEntry {
    /// The index of the resource in the resource set layout
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

/// Describes the expected resource type in a [`ResourceSet`]
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

/// Describes the the resource accessibility of a [`Resource`] in a [`ResourceSet`]
#[derive(Debug, Clone, Copy)]
pub enum ResourceAccess {
    /// Specifies that the resource is accessible only in the vertex shader
    Vertex,
    /// Specifies that the resource is accessible only in the fragment shader
    Fragment,
    /// Specifies that the resource is accessible by either shader of the two
    Either,
}

impl ResourceSet {
    /// Returns a reference to the raw [`wgpu::BindGroup`]
    pub fn raw(&self) -> &wgpu::BindGroup {
        &self.raw
    }
}

impl ResourceSetLayout {
    /// Returns a reference to the raw [`wgpu::BindGroupLayout`]
    pub fn raw(&self) -> &wgpu::BindGroupLayout {
        &self.raw
    }
}

impl<'a> ResourceSetDescriptor<'a> {
    /// Builds a [`ResourceSet`]
    /// - `device` -> the [`wgpu::Device`] required to create a raw [`wgpu::BindGroup`]
    pub fn build(&self, device: &wgpu::Device) -> ResourceSet {
        let entries: Vec<_> = self
            .entries
            .iter()
            .map(|entry| wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: entry.resource.raw(),
            })
            .collect();
        ResourceSet {
            raw: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: self.label,
                layout: self.layout.raw(),
                entries: Box::leak(entries.into_boxed_slice()),
            }),
        }
    }
}

impl<'a> ResourceSetLayoutDescriptor<'a> {
    /// Builds a [`ResourceSetLayout`]
    /// - `device` -> the [`wgpu::Device`] required to create a raw [`wgpu::BindGroupLayout`]
    pub fn build(&self, device: &wgpu::Device) -> ResourceSetLayout {
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
        ResourceSetLayout {
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

/// A builder utility for creating a [`ResourceSet`] in a more ergonomic way.
#[derive(Debug)]
pub struct ResourceSetBuilder<'a> {
    /// The optional debugging label of this resource set
    label: Option<&'a str>,
    /// The current location of a resource
    cursor: u32,
    /// The list of resources
    entries: Vec<ResourceSetEntry<'a>>,
}

impl<'a> ResourceSetBuilder<'a> {
    /// Creates a new [`ResourceSetBuilder`].
    pub fn new() -> Self {
        Self {
            label: None,
            cursor: 0,
            entries: Vec::with_capacity(4),
        }
    }

    /// Sets a label to this [`ResourceSet`].
    /// - `label` -> the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Adds a [`Buffer`] resource.
    pub fn add_buffer(mut self, buffer: &'a dyn AnyBufferHandle) -> Self {
        self.entries.push(ResourceSetEntry {
            binding: self.cursor,
            resource: Resource::Buffer(buffer),
        });
        self.cursor += 1;
        self
    }

    /// Adds a [`Sampler`] resource.
    pub fn add_sampler(mut self, sampler: &'a Sampler) -> Self {
        self.entries.push(ResourceSetEntry {
            binding: self.cursor,
            resource: Resource::Sampler(sampler),
        });
        self.cursor += 1;
        self
    }

    /// Adds a [`Texture`] resource.
    pub fn add_texture(mut self, texture: &'a Texture) -> Self {
        self.entries.push(ResourceSetEntry {
            binding: self.cursor,
            resource: Resource::Texture(texture),
        });
        self.cursor += 1;
        self
    }

    /// Builds a [`ResourceSet`] and consumes this [`ResourceSetBuilder`].
    /// - `layout` -> the matching [`ResourceSetLayout`]
    /// - `device` -> the device needed to create the [`ResourceSet`]
    ///
    /// If the entry list is empty, the caller thread panics.
    pub fn build(self, layout: &ResourceSetLayout, device: &wgpu::Device) -> ResourceSet {
        if self.entries.is_empty() {
            panic!("Couldn't create a `ResourceSet`, missing entries!");
        }

        ResourceSetDescriptor {
            label: self.label,
            layout,
            entries: &self.entries,
        }
        .build(device)
    }
}

/// A builder utility for creating a [`ResourceSetLayout`] in a more ergonomic way.
#[derive(Debug)]
pub struct ResourceSetLayoutBuilder<'a> {
    /// The optional debugging label of this resource set layout
    label: Option<&'a str>,
    /// The current location of a layout resource
    cursor: u32,
    /// The list of layout resources
    entries: Vec<ResourceSetLayoutEntry>,
}

impl<'a> ResourceSetLayoutBuilder<'a> {
    /// Creates a new [`ResourceSetLayoutBuilder`].
    pub fn new() -> Self {
        Self {
            label: None,
            cursor: 0,
            entries: Vec::with_capacity(4),
        }
    }

    /// Sets a label to this [`ResourceSetLayout`].
    /// - `label` -> the label
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Adds a uniform buffer layout resource.
    /// - `access` -> the [`ResourceAccess`] specifying the shader accessibility of the resource
    pub fn add_uniform_buffer(mut self, access: ResourceAccess) -> Self {
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
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
        self.entries.push(ResourceSetLayoutEntry {
            binding: self.cursor,
            resource: LayoutResource::Texture(TextureConfig::Cubemap(TextureKind::Image)),
            access,
        });
        self.cursor += 1;
        self
    }

    /// Builds a [`ResourceSetLayout`] and consumes this [`ResourceSetLayoutBuilder`].
    /// - `device` -> the device needed to create the [`ResourceSetLayout`]
    ///
    /// If the entry list is empty, the caller thread panics.
    pub fn build(self, device: &wgpu::Device) -> ResourceSetLayout {
        if self.entries.is_empty() {
            panic!("Couldn't create a `ResourceSetLayout`, missing entries!");
        }

        ResourceSetLayoutDescriptor {
            label: self.label,
            entries: &self.entries,
        }
        .build(device)
    }
}
