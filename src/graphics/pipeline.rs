use crate::graphics::{
    group::BindGroupLayout, layout::BufferLayout, shader::Shader, texture::TextureFormat,
};

/// Specifies which operation the GPU should perform when assembling geometry
#[derive(Debug, Clone, Copy)]
pub enum Draw {
    /// Fills out the geometry
    Fill,
    /// Draws out the individual vertex positions of the geometry
    Points,
    /// Connects the individual vertex positions with lines and leaves the geometry unfilled
    Wireframe,
}

/// Specifies which face of rendered geometry should the GPU cull
///
/// It's important to note that what is considered a front or a back face
/// depends on the [`Winding`] setting
#[derive(Debug, Clone, Copy)]
pub enum Cull {
    /// Culls front faces
    Front,
    /// Culls back faces
    Back,
    /// Doesn't cull faces
    None,
}

/// Specifies the winding order when drawing geometry which then determines
/// if a face is in the front or in the back
#[derive(Debug, Clone, Copy)]
pub enum Winding {
    /// The face is considered front-facing if its indices are clockwise
    Clockwise,
    /// The face is considered front-facing if its indices are counter-clockwise
    Reverse,
}

/// Specifies the primitive which the GPU should use for assembling geometry
#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    /// Useful for debugging and visualizing points in space,
    /// the geometry primitive is a single point.
    PointList,
    /// Useful for debugging and visualizing lines, debug boxes,
    /// the geometry primitive is a line.
    LineList,
    /// The most standard option, the geometry primitive is a triangle.
    TriangleList,
}

/// Specifies the blending mode for the GPU during the rasterization stage
#[derive(Debug, Clone, Copy)]
pub enum Blend {
    /// Specifies that the GPU will blend the new pixel with an old pixel in the framebuffer
    /// based on the new pixel's alpha value
    Alpha,
    /// Specifies that the GPU will replace the old pixel with the new pixel in the framebuffer
    Replace,
}

/// Specifies the depth testing function for the GPU during the depth testing stage
///
/// Depth testing refers to discarding or overwriting pixels based on their distance
/// from the camera by checking against the depth buffer
///
#[derive(Debug, Clone, Copy)]
pub enum Depth {
    /// The GPU will let the fragment pass if its depth value is less
    /// than the previous fragment's depth, therefore the new fragment is closer.
    Less,
    /// The GPU will let the fragment pass if its depth value is less or equal
    /// than the previous fragment's depth, therefore the new fragment is either
    /// the same distance away, or closer than the previous fragment.
    LessEqual,
    /// The GPU will let the fragment pass if its depth value is equal
    /// than the previous fragment's depth, therefore the new fragment is
    /// the same distance away.
    Equal,
    /// The GPU will let the fragment pass if its depth value is greater or equal
    /// than the previous fragment's depth, therefore the new fragment is either
    /// the same distance away, or farther than the previous fragment.
    GreaterEqual,
    /// The GPU will let the fragment pass if its depth value is greater
    /// than the previous fragment's depth, therefore the new fragment is farther.
    Greater,
    /// The GPU will always let the fragment pass
    Always,
    /// The GPU will never let the fragment pass
    Never,
}

/// Describes a wrapper around the raw [`wgpu::RenderPipeline`]
#[derive(Debug)]
pub struct Pipeline {
    raw: wgpu::RenderPipeline,
}

/// Describes a wrapper around the raw `wgpu::PipelineLayout`
#[derive(Debug)]
pub struct PipelineLayout {
    raw: wgpu::PipelineLayout,
}

/// Describes a [`Pipeline`]
///
/// A pipeline is a rendering configuration that specifies every stage of rendering
#[derive(Debug)]
pub struct PipelineDescriptor<'a> {
    /// The optional debugging label of the pipeline
    pub label: Option<&'a str>,
    /// The pipeline shader
    pub shader: &'a Shader,
    /// The pipeline layout specifying pipeline resources
    pub pipeline_layout: &'a PipelineLayout,
    /// The geometry layout, can be optional for procedurally generated geometry
    pub geometry_layout: Option<BufferLayout>,
    /// The instance layout, can be optional for cases where instanced rendering isn't needed
    pub instance_layout: Option<BufferLayout>,
    /// The drawing mode of the pipeline
    pub draw: Draw,
    /// The culling mode of the pipeline
    pub cull: Cull,
    /// The winding order geometry
    pub winding: Winding,
    /// The geometry primitive
    pub primitive: Primitive,
    /// The blending mode of fragments
    pub blend: Blend,
    /// The depth function to enable depth testing
    pub depth: Option<Depth>,
}

/// Describes a [`PipelineLayout`]
#[derive(Debug)]
pub struct PipelineLayoutDescriptor<'a> {
    /// The optional debugging label of the pipeline layout
    pub label: Option<&'a str>,
    /// The list of resource set layouts
    pub layouts: &'a [&'a BindGroupLayout],
}

impl Pipeline {
    /// Returns the internal [`wgpu::RenderPipeline`]
    pub fn raw(&self) -> &wgpu::RenderPipeline {
        &self.raw
    }
}

impl PipelineLayout {
    /// Returns the internal [`wgpu::PipelineLayout`]
    pub fn raw(&self) -> &wgpu::PipelineLayout {
        &self.raw
    }
}

impl<'a> PipelineDescriptor<'a> {
    /// Builds a new [`Pipeline`]
    /// - `device` is the raw [`wgpu::Device`] which is needed to build GPU resources
    pub fn build(self, device: &wgpu::Device) -> Pipeline {
        let buffer_layouts: &[wgpu::VertexBufferLayout] =
            match (self.geometry_layout, self.instance_layout) {
                (None, None) => panic!("Missing buffer layouts!"),
                (None, Some(layout)) => &[layout.raw()],
                (Some(layout), None) => &[layout.raw()],
                (Some(geometry_layout), Some(instance_layout)) => {
                    &[geometry_layout.raw(), instance_layout.raw()]
                }
            };
        Pipeline {
            raw: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: self.label,
                layout: Some(self.pipeline_layout.raw()),
                vertex: wgpu::VertexState {
                    module: self.shader.raw(),
                    entry_point: Some("vs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: buffer_layouts,
                },
                fragment: Some(wgpu::FragmentState {
                    module: self.shader.raw(),
                    entry_point: Some("fs_main"),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: TextureFormat::Standard.raw(),
                        blend: Some(self.blend.raw()),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: self.primitive.raw(),
                    front_face: self.winding.raw(),
                    cull_mode: self.cull.raw(),
                    polygon_mode: self.draw.raw(),
                    strip_index_format: if self.primitive.raw().is_strip() {
                        Some(wgpu::IndexFormat::Uint32)
                    } else {
                        None
                    },
                    unclipped_depth: false,
                    conservative: false,
                },
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                depth_stencil: self.depth.map(|mode| wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24PlusStencil8,
                    depth_write_enabled: true,
                    depth_compare: mode.raw(),
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multiview: None,
                cache: None,
            }),
        }
    }
}

impl<'a> PipelineLayoutDescriptor<'a> {
    /// Builds a [`PipelineLayout`]
    /// - `device` is the raw [`wgpu::Device`] which is needed to create GPU resources
    pub fn build(self, device: &wgpu::Device) -> PipelineLayout {
        let layouts: Vec<_> = self.layouts.iter().map(|layout| layout.raw()).collect();
        PipelineLayout {
            raw: device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: self.label,
                bind_group_layouts: Box::leak(layouts.into_boxed_slice()),
                push_constant_ranges: &[],
            }),
        }
    }
}

impl Draw {
    /// Maps the [`Draw`] to the internal [`wgpu::PolygonMode`]
    fn raw(self) -> wgpu::PolygonMode {
        match self {
            Draw::Fill => wgpu::PolygonMode::Fill,
            Draw::Points => wgpu::PolygonMode::Point,
            Draw::Wireframe => wgpu::PolygonMode::Line,
        }
    }
}

impl Cull {
    /// Maps the [`Cull`] to the internal [`wgpu::Face`]
    fn raw(self) -> Option<wgpu::Face> {
        match self {
            Cull::Front => Some(wgpu::Face::Front),
            Cull::Back => Some(wgpu::Face::Back),
            Cull::None => None,
        }
    }
}

impl Winding {
    /// Maps the [`Winding`] to the internal [`wgpu::FrontFace`]
    fn raw(self) -> wgpu::FrontFace {
        match self {
            Winding::Clockwise => wgpu::FrontFace::Cw,
            Winding::Reverse => wgpu::FrontFace::Ccw,
        }
    }
}

impl Primitive {
    /// Maps the [`Primitive`] to the internal [`wgpu::PrimitiveTopology`]
    fn raw(self) -> wgpu::PrimitiveTopology {
        match self {
            Primitive::PointList => wgpu::PrimitiveTopology::PointList,
            Primitive::LineList => wgpu::PrimitiveTopology::LineList,
            Primitive::TriangleList => wgpu::PrimitiveTopology::TriangleList,
        }
    }
}

impl Blend {
    /// Maps the [`Blend`] to the internal [`wgpu::BlendState`]
    fn raw(self) -> wgpu::BlendState {
        match self {
            Blend::Alpha => wgpu::BlendState::ALPHA_BLENDING,
            Blend::Replace => wgpu::BlendState::REPLACE,
        }
    }
}

impl Depth {
    /// Maps the [`Depth`] to the internal [`wgpu::CompareFunction`]
    fn raw(self) -> wgpu::CompareFunction {
        match self {
            Depth::Less => wgpu::CompareFunction::Less,
            Depth::LessEqual => wgpu::CompareFunction::LessEqual,
            Depth::Equal => wgpu::CompareFunction::Equal,
            Depth::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
            Depth::Greater => wgpu::CompareFunction::Greater,
            Depth::Always => wgpu::CompareFunction::Always,
            Depth::Never => wgpu::CompareFunction::Never,
        }
    }
}

#[derive(Debug, Default)]
pub struct PipelineBuilder<'a> {
    label: Option<&'a str>,
    shader: Option<&'a Shader>,
    layout: Option<&'a PipelineLayout>,
    draw: Option<Draw>,
    cull: Option<Cull>,
    depth: Option<Depth>,
    blend: Option<Blend>,
    winding: Option<Winding>,
    primitive: Option<Primitive>,
    geometry_layout: Option<BufferLayout>,
    instance_layout: Option<BufferLayout>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn shader(mut self, shader: &'a Shader) -> Self {
        self.shader = Some(shader);
        self
    }

    pub fn layout(mut self, layout: &'a PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn draw(mut self, draw: Draw) -> Self {
        self.draw = Some(draw);
        self
    }

    pub fn cull(mut self, cull: Cull) -> Self {
        self.cull = Some(cull);
        self
    }

    pub fn depth(mut self, depth: Depth) -> Self {
        self.depth = Some(depth);
        self
    }

    pub fn blend(mut self, blend: Blend) -> Self {
        self.blend = Some(blend);
        self
    }

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = Some(winding);
        self
    }

    pub fn primitive(mut self, primitive: Primitive) -> Self {
        self.primitive = Some(primitive);
        self
    }

    pub fn geometry_layout(mut self, layout: BufferLayout) -> Self {
        self.geometry_layout = Some(layout);
        self
    }

    pub fn instance_layout(mut self, layout: BufferLayout) -> Self {
        self.instance_layout = Some(layout);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> Pipeline {
        PipelineDescriptor {
            label: self.label,
            shader: self.shader.expect("Missing shader in pipeline"),
            pipeline_layout: self.layout.expect("Missing layout in pipeline"),
            geometry_layout: self.geometry_layout,
            instance_layout: self.instance_layout,
            draw: self.draw.expect("Missing draw mode in pipeline"),
            cull: self.cull.expect("Missing cull mode in pipeline"),
            blend: self.blend.expect("Missing blend mode in pipeline"),
            depth: self.depth,
            winding: self.winding.unwrap_or(Winding::Clockwise),
            primitive: self.primitive.unwrap_or(Primitive::TriangleList),
        }
        .build(device)
    }
}

#[derive(Debug, Default)]
pub struct PipelineLayoutBuilder<'a> {
    label: Option<&'a str>,
    layouts: Vec<&'a BindGroupLayout>,
}

impl<'a> PipelineLayoutBuilder<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn layout(mut self, layout: &'a BindGroupLayout) -> Self {
        self.layouts.push(layout);
        self
    }

    pub fn build(self, device: &wgpu::Device) -> PipelineLayout {
        PipelineLayoutDescriptor {
            label: self.label,
            layouts: &self.layouts,
        }
        .build(device)
    }
}
