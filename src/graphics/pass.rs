use crate::graphics::{
    buffer::AnyBufferHandle, color::Color, pipeline::Pipeline, resource::ResourceSet,
    texture::Texture,
};

/// Describes a wrapper around the raw [`wgpu::RenderPass`]
pub struct RenderPass<'a> {
    raw: wgpu::RenderPass<'a>,
}

/// Describes a render pass
pub struct RenderPassDescriptor<'a> {
    /// The optional debugging label of this render pass
    pub label: Option<&'a str>,
    /// The color attachment of this render pass
    pub color_attachment: Option<Color>,
    /// The depth/stencil attachment of this render pass
    pub depth_stencil_attachment: Option<&'a Texture>,
}

impl<'a> RenderPass<'a> {
    /// Returns the raw [`wgpu::RenderPass`]
    pub fn raw(&self) -> &wgpu::RenderPass<'a> {
        &self.raw
    }

    /// Sets a geometry buffer in a specific slot
    /// - `slot` -> the slot to use for this buffer
    /// - `buffer` -> the geometry buffer to set
    pub fn use_geometry_buffer(&mut self, slot: u32, buffer: &dyn AnyBufferHandle) {
        self.raw.set_vertex_buffer(slot, buffer.as_slice());
    }

    /// Sets an index buffer to the render pass
    /// - `buffer` -> the index buffer to set
    pub fn use_index_buffer(&mut self, buffer: &dyn AnyBufferHandle) {
        self.raw
            .set_index_buffer(buffer.as_slice(), wgpu::IndexFormat::Uint32);
    }

    /// Sets an instance buffer in a specific slot
    /// - `slot` -> the slot to use for this buffer
    /// - `buffer` -> the instance buffer to set
    pub fn use_instance_buffer(&mut self, slot: u32, buffer: &dyn AnyBufferHandle) {
        self.raw.set_vertex_buffer(slot, buffer.as_slice());
    }

    /// Sets a [`ResourceSet`] to the render pass
    /// - `slot` -> the slot to use for this resource set
    /// - `set` -> the resource set
    pub fn use_resource_set(&mut self, slot: u32, set: &ResourceSet) {
        self.raw.set_bind_group(slot, set.raw(), &[]);
    }

    /// Sets a pipeline to the render pass
    /// - `pipeline` -> the pipeline to set
    pub fn use_pipeline(&mut self, pipeline: &Pipeline) {
        self.raw.set_pipeline(pipeline.raw());
    }

    /// Issues a draw call with the current render pass configuration
    /// - `vertex_count` -> how many vertices to draw
    /// - `instance_count` -> how many instances of the geometry to draw
    pub fn draw(&mut self, vertex_count: u32, instance_count: u32) {
        match (vertex_count, instance_count) {
            (0, 0) => panic!("Attempted to draw with a vertex count and instance count of 0"),
            (_, 0) => panic!("Attempted to draw with an instance count of 0"),
            (0, _) => panic!("Attempted to draw with a vertex count of 0"),
            (_, _) => (),
        }

        self.raw.draw(0..vertex_count, 0..instance_count);
    }

    /// Issues an indexed draw call with the current render pass configuration
    /// - `index_count` -> how many indices to draw
    /// - `instance_count` how many instances of the geometry to draw
    pub fn draw_indexed(&mut self, index_count: u32, instance_count: u32) {
        match (index_count, instance_count) {
            (0, 0) => panic!("Attempted to draw with an index count and instance count of 0"),
            (_, 0) => panic!("Attempted to draw with an instance count of 0"),
            (0, _) => panic!("Attempted to draw with an index count of 0"),
            (_, _) => (),
        }

        self.raw.draw_indexed(0..index_count, 0, 0..instance_count);
    }
}

impl<'a> RenderPassDescriptor<'a> {
    /// Builds a [`RenderPass`]
    /// - `frame` -> the texture view of the current frame we want to render to
    /// - `encoder` -> the command encoder that records the render pass
    pub fn build(
        self,
        frame: &wgpu::TextureView,
        encoder: &'a mut wgpu::CommandEncoder,
    ) -> RenderPass<'a> {
        RenderPass {
            raw: encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: self.label,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: frame,
                    ops: wgpu::Operations {
                        load: match self.color_attachment {
                            Some(color) => wgpu::LoadOp::Clear(color.raw()),
                            None => wgpu::LoadOp::Load,
                        },
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                    resolve_target: None,
                })],
                depth_stencil_attachment: self.depth_stencil_attachment.map(|depth_stencil| {
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth_stencil.view(),
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0),
                            store: wgpu::StoreOp::Store,
                        }),
                    }
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            }),
        }
    }
}
