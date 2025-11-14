/// Describes a wrapper around the internal [`wgpu::VertexBufferLayout`]
#[derive(Debug, Clone)]
pub struct BufferLayout {
    /// The internal [`wgpu::VertexBufferLayout`]
    raw: wgpu::VertexBufferLayout<'static>,
}

/// Describes a buffer layout
#[derive(Debug)]
pub struct BufferLayoutDescriptor {
    /// Specifies the advance rate of the items in the buffer
    pub layout_kind: BufferLayoutKind,
    /// Specifies the buffer attributes that form the layout
    pub attributes: &'static [BufferAttribute],
}

/// Describes a buffer attribute
#[derive(Debug)]
pub struct BufferAttribute {
    /// Specifies the location of the buffer attribute for shader access
    pub location: u32,
    /// Specifies the component size of the attribute
    pub size: u32,
    /// Specifies the data type of the attribute
    pub format: BufferAttributeFormat,
}

/// Describes the buffer layout kind (which specifies the advance rate)
#[derive(Debug, Clone, Copy)]
pub enum BufferLayoutKind {
    /// Specifies that the pipeline will advance to the next item (a vertex)
    /// per each vertex drawn as specified by a vertex count when using a draw call
    Geometry,
    /// Specifies that the pipeline will advance to the next item (an instance)
    /// per each instance drawn as specified by an instance count when using a draw call
    Instance,
}

/// Describes the buffer attribute format
#[derive(Debug, Clone, Copy)]
pub enum BufferAttributeFormat {
    /// The buffer attribute is composed of 32-bit floats
    F32,
    /// The buffer attribute is composed of 32-bit signed integers
    I32,
    /// The buffer attribute is composed of 32-bit unsigned integers
    U32,
}

impl BufferLayout {
    /// Consumes self and returns the raw [`wgpu::VertexBufferLayout`]
    pub fn raw(self) -> wgpu::VertexBufferLayout<'static> {
        self.raw
    }
}

impl BufferLayoutDescriptor {
    /// Consumes self and builds a [`BufferLayout`]
    pub fn build(self) -> BufferLayout {
        let mut offset = 0;
        let mut stride = 0;
        let mut attributes = Vec::new();
        for attribute in self.attributes {
            attributes.push(wgpu::VertexAttribute {
                format: attribute.raw_format(),
                shader_location: attribute.location,
                offset,
            });
            let advance = (attribute.size * attribute.format.raw_bytes()) as u64;
            offset += advance;
            stride += advance;
        }
        BufferLayout {
            raw: wgpu::VertexBufferLayout {
                array_stride: stride,
                step_mode: self.layout_kind.raw(),
                attributes: Box::leak(attributes.into_boxed_slice()),
            },
        }
    }
}

impl BufferAttribute {
    /// Returns the raw [`wgpu::VertexFormat`] which specifies both the size
    /// and the type of the attribute based on 2 parameters:
    ///
    /// - format: [`BufferAttributeFormat`], which represents the data type used in the attribute
    /// - size: [`u32`], which represents the number of components in the attribute
    fn raw_format(&self) -> wgpu::VertexFormat {
        match (&self.format, self.size) {
            // F32 pairs
            (BufferAttributeFormat::F32, 1) => wgpu::VertexFormat::Float32,
            (BufferAttributeFormat::F32, 2) => wgpu::VertexFormat::Float32x2,
            (BufferAttributeFormat::F32, 3) => wgpu::VertexFormat::Float32x3,
            (BufferAttributeFormat::F32, 4) => wgpu::VertexFormat::Float32x4,

            // I32 pairs
            (BufferAttributeFormat::I32, 1) => wgpu::VertexFormat::Sint32,
            (BufferAttributeFormat::I32, 2) => wgpu::VertexFormat::Sint32x2,
            (BufferAttributeFormat::I32, 3) => wgpu::VertexFormat::Sint32x3,
            (BufferAttributeFormat::I32, 4) => wgpu::VertexFormat::Sint32x4,

            // U32 pairs
            (BufferAttributeFormat::U32, 1) => wgpu::VertexFormat::Uint32,
            (BufferAttributeFormat::U32, 2) => wgpu::VertexFormat::Uint32x2,
            (BufferAttributeFormat::U32, 3) => wgpu::VertexFormat::Uint32x3,
            (BufferAttributeFormat::U32, 4) => wgpu::VertexFormat::Uint32x4,
            _ => panic!(
                "Expected valid attribute, got: ({:?}, {})",
                self.format, self.size
            ),
        }
    }
}

impl BufferLayoutKind {
    /// Maps [`BufferLayoutKind`] to the internal [`wgpu::VertexStepMode`]
    fn raw(self) -> wgpu::VertexStepMode {
        match self {
            BufferLayoutKind::Geometry => wgpu::VertexStepMode::Vertex,
            BufferLayoutKind::Instance => wgpu::VertexStepMode::Instance,
        }
    }
}

impl BufferAttributeFormat {
    /// Returns the raw amount of bytes for the underlying attribute format
    ///
    /// - [`BufferAttributeFormat::F32`] => 4 bytes,
    /// - [`BufferAttributeFormat::I32`] => 4 bytes,
    /// - [`BufferAttributeFormat::U32`] => 4 bytes,
    ///
    fn raw_bytes(self) -> u32 {
        match self {
            BufferAttributeFormat::F32 => 4,
            BufferAttributeFormat::I32 => 4,
            BufferAttributeFormat::U32 => 4,
        }
    }
}

/// Constructs a new geometry [`BufferLayout`] from an array of buffer attributes.
pub fn create_geometry_layout(attributes: &'static [BufferAttribute]) -> BufferLayout {
    BufferLayoutDescriptor {
        layout_kind: BufferLayoutKind::Geometry,
        attributes,
    }
    .build()
}

/// Constructs a new instance [`BufferLayout`] from an array of buffer attributes.
pub fn create_instance_layout(attributes: &'static [BufferAttribute]) -> BufferLayout {
    BufferLayoutDescriptor {
        layout_kind: BufferLayoutKind::Instance,
        attributes,
    }
    .build()
}
