use crate::engine::{buffer::Buffer, sampler::Sampler, texture::Texture};

/// Describes a wrapper around the raw [`wgpu::BindGroup`]
#[derive(Debug)]
pub struct Group {
    /// The internal [`wgpu::BindGroup`]
    raw: wgpu::BindGroup,
}

/// Describes a wrapper around the raw [`wgpu::BindGroupLayout`]
#[derive(Debug)]
pub struct GroupLayout {
    /// The internal [`wgpu::BindGroupLayout`]
    raw: wgpu::BindGroupLayout,
}

/// Describes a group
///
/// A group is a collection of GPU resources that get used and consumed by shaders
#[derive(Debug)]
pub struct GroupDescriptor<'a> {
    /// The debugging label of the group
    pub label: &'static str,
    /// The group layout specifying the layout of the group's entries
    pub layout: &'a GroupLayout,
    /// The group entries, can hold:
    /// - [`Buffer`]
    /// - [`Texture`]
    /// - [`Sampler`]
    pub entries: &'a [GroupEntry<'a>],
}

/// Describes a group layout
///
/// A group layout is the layout of a collection of GPU resources
/// that are intended to be used and consumed by shaders
///
#[derive(Debug)]
pub struct GroupLayoutDescriptor<'a> {
    /// The debugging label of the group layout
    pub label: &'static str,
    /// The group layout entries, the entries can be of type:
    /// - [`GroupLayoutResource::Buffer`]
    /// - [`GroupLayoutResource::Texture`]
    /// - [`GroupLayoutResource::Sampler`]
    pub entries: &'a [GroupLayoutEntry],
}

/// Describes a group entry
///
/// A group entry is a wrapper around the actual GPU resource
/// with a binding which represents the index of the resource in the group
///
#[derive(Debug)]
pub struct GroupEntry<'a> {
    /// The index of the resource in the group
    pub binding: u32,
    /// The actual resource (buffer, sampler, texture)
    pub resource: GroupResource<'a>,
}

/// Describes a group layout entry
///
/// A group layout entry specifies the resource so the GPU
/// knows which resource to expect at which index
///
#[derive(Debug)]
pub struct GroupLayoutEntry {
    /// The index of the resource in the group
    pub binding: u32,
    /// The expected resource, specified by a configuration
    pub resource: GroupLayoutResource,
    /// The shader stage the resource should be visible in
    pub stage: Stage,
}

/// Describes a wrapper around the actual GPU resource (buffer, sampler, texture)
#[derive(Debug)]
pub enum GroupResource<'a> {
    /// A buffer resource, holding a reference to a [`Buffer`]
    Buffer(&'a Buffer),
    /// A sampler resource, holding a reference to a [`Sampler`]
    Sampler(&'a Sampler),
    /// A texture resource, holding a reference to a [`Texture`]
    Texture(&'a Texture),
}

/// Describes the expected resource type in a group
#[derive(Debug, Clone, Copy)]
pub enum GroupLayoutResource {
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

/// Describes the shader visibility of an entry
#[derive(Debug, Clone, Copy)]
pub enum Stage {
    /// Specifies that the resource is accessible only in the vertex shader
    Vertex,
    /// Specifies that the resource is accessible only in the fragment shader
    Fragment,
    /// Specifies that the resource is accessible by both shaders
    Both,
}

impl Group {
    /// Returns a reference to the raw [`wgpu::BindGroup`]
    pub fn raw(&self) -> &wgpu::BindGroup {
        &self.raw
    }
}

impl GroupLayout {
    /// Returns a reference to the raw [`wgpu::BindGroupLayout`]
    pub fn raw(&self) -> &wgpu::BindGroupLayout {
        &self.raw
    }
}

impl<'a> GroupDescriptor<'a> {
    /// Builds a [`Group`]
    /// - `layout` -> the group layout that must match the group
    /// - `device` -> the [`wgpu::Device`] required to create a raw [`wgpu::BindGroup`]
    pub fn build(&self, device: &wgpu::Device) -> Group {
        let entries: Vec<_> = self
            .entries
            .iter()
            .map(|entry| wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: entry.resource.raw(),
            })
            .collect();
        Group {
            raw: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(self.label),
                layout: self.layout.raw(),
                entries: Box::leak(entries.into_boxed_slice()),
            }),
        }
    }
}

impl<'a> GroupLayoutDescriptor<'a> {
    /// Builds a [`GroupLayout`]
    /// - `device` -> the [`wgpu::Device`] required to create a raw [`wgpu::BindGroupLayout`]
    pub fn build(&self, device: &wgpu::Device) -> GroupLayout {
        let entries: Vec<_> = self
            .entries
            .iter()
            .map(|entry| wgpu::BindGroupLayoutEntry {
                binding: entry.binding,
                visibility: entry.stage.raw(),
                ty: entry.resource.raw(),
                count: None,
            })
            .collect();
        GroupLayout {
            raw: device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(self.label),
                entries: Box::leak(entries.into_boxed_slice()),
            }),
        }
    }
}

impl<'a> GroupResource<'a> {
    /// Maps the [`GroupResource`] to the internal [`wgpu::BindingResource`]
    pub fn raw(&self) -> wgpu::BindingResource<'a> {
        match self {
            GroupResource::Buffer(buffer) => buffer.raw().as_entire_binding(),
            GroupResource::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler.raw()),
            GroupResource::Texture(texture) => wgpu::BindingResource::TextureView(texture.view()),
        }
    }
}

impl GroupLayoutResource {
    /// Maps the [`GroupLayoutResource`] to the internal [`wgpu::BindingType`]
    pub fn raw(&self) -> wgpu::BindingType {
        match self {
            GroupLayoutResource::Buffer(config) => config.raw(),
            GroupLayoutResource::Sampler(config) => config.raw(),
            GroupLayoutResource::Texture(config) => config.raw(),
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

impl Stage {
    /// Maps the [`Stage`] to the internal [`wgpu::ShaderStages`]
    pub fn raw(&self) -> wgpu::ShaderStages {
        match self {
            Stage::Vertex => wgpu::ShaderStages::VERTEX,
            Stage::Fragment => wgpu::ShaderStages::FRAGMENT,
            Stage::Both => wgpu::ShaderStages::VERTEX_FRAGMENT,
        }
    }
}

/// A builder utility for creating a [`Group`] in a more ergonomic way.
#[derive(Debug)]
pub struct GroupBuilder<'a> {
    /// The current location of a resource
    cursor: u32,
    /// The list of resources
    entries: Vec<GroupEntry<'a>>,
}

impl<'a> GroupBuilder<'a> {
    /// Creates a new [`GroupBuilder`].
    pub fn new() -> Self {
        Self {
            cursor: 0,
            entries: Vec::with_capacity(4),
        }
    }

    /// Adds a [`Buffer`] resource.
    pub fn add_buffer(mut self, buffer: &'a Buffer) -> Self {
        self.entries.push(GroupEntry {
            binding: self.cursor,
            resource: GroupResource::Buffer(buffer),
        });
        self.cursor += 1;
        self
    }

    /// Adds a [`Sampler`] resource.
    pub fn add_sampler(mut self, sampler: &'a Sampler) -> Self {
        self.entries.push(GroupEntry {
            binding: self.cursor,
            resource: GroupResource::Sampler(sampler),
        });
        self.cursor += 1;
        self
    }

    /// Adds a [`Texture`] resource.
    pub fn add_texture(mut self, texture: &'a Texture) -> Self {
        self.entries.push(GroupEntry {
            binding: self.cursor,
            resource: GroupResource::Texture(texture),
        });
        self.cursor += 1;
        self
    }

    /// Builds a [`Group`] and consumes this [`GroupBuilder`].
    /// - `label` -> the label of the group
    /// - `layout` -> the matching [`GroupLayout`]
    /// - `device` -> the device needed to create the [`Group`]
    ///
    /// If the entry list is empty, the application panics.
    pub fn build(self, label: &'static str, layout: &GroupLayout, device: &wgpu::Device) -> Group {
        if self.entries.is_empty() {
            panic!("Couldn't create a Group, missing entries!");
        }

        GroupDescriptor {
            label,
            layout,
            entries: &self.entries,
        }
        .build(device)
    }
}

/// A builder utility for creating a [`GroupLayout`] in a more ergonomic way.
#[derive(Debug)]
pub struct GroupLayoutBuilder {
    /// The current location of a layout resource
    cursor: u32,
    /// The list of layout resources
    entries: Vec<GroupLayoutEntry>,
}

impl GroupLayoutBuilder {
    /// Creates a new [`GroupLayoutBuilder`].
    pub fn new() -> Self {
        Self {
            cursor: 0,
            entries: Vec::with_capacity(4),
        }
    }

    /// Adds a uniform buffer layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_uniform_buffer(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Buffer(BufferConfig::Uniform),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a storage buffer layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_storage_buffer(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Buffer(BufferConfig::Storage),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a nearest sampler layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_nearest_sampler(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Sampler(SamplerConfig::Nearest),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a linear sampler layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_linear_sampler(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Sampler(SamplerConfig::Linear),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a compare sampler layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_compare_sampler(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Sampler(SamplerConfig::Compare),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a depth texture layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_depth_texture(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Texture(TextureConfig::D2(TextureKind::Depth)),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a 1D texture layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_texture_1d(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Texture(TextureConfig::D1(TextureKind::Image)),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a 2D texture layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_texture_2d(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Texture(TextureConfig::D2(TextureKind::Image)),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a 3D texture layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_texture_3d(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Texture(TextureConfig::D3(TextureKind::Image)),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Adds a cubemap texture layout resource.
    /// - `stage` -> the [`Stage`] specifying the visibility of the resource
    pub fn add_texture_cubemap(mut self, stage: Stage) -> Self {
        self.entries.push(GroupLayoutEntry {
            binding: self.cursor,
            resource: GroupLayoutResource::Texture(TextureConfig::Cubemap(TextureKind::Image)),
            stage,
        });
        self.cursor += 1;
        self
    }

    /// Builds a [`GroupLayout`] and consumes this [`GroupLayoutBuilder`].
    /// - `label` -> the label of the group layout
    /// - `device` -> the device needed to create the [`GroupLayout`]
    ///
    /// If the entry list is empty, the application panics.
    pub fn build(self, label: &'static str, device: &wgpu::Device) -> GroupLayout {
        if self.entries.is_empty() {
            panic!("Couldn't create a GroupLayout, missing entries!");
        }

        GroupLayoutDescriptor {
            label,
            entries: &self.entries,
        }
        .build(device)
    }
}
