use std::{borrow::Cow, error::Error, fs, path::Path};

/// Describes a wrapper around [`wgpu::ShaderModule`]
#[derive(Debug)]
pub struct Shader {
    /// The internal [`wgpu::ShaderModule`]
    raw: wgpu::ShaderModule,
}

impl Shader {
    /// Creates a new shader from the following arguments:
    /// - `device` is the raw [`wgpu::Device`]
    /// - `path` is the path of the shader file to read
    /// - `label` is an optional debugging label which is assigned to the shader unit
    pub fn new(
        device: &wgpu::Device,
        path: &Path,
        label: Option<&str>,
    ) -> Result<Self, Box<dyn Error>> {
        let source = fs::read_to_string(path)?;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label,
            source: wgpu::ShaderSource::Wgsl(Cow::Owned(source)),
        });
        Ok(Self { raw: shader })
    }

    /// Returns the raw [`wgpu::ShaderModule`] to use in pipeline creation
    pub fn raw(&self) -> &wgpu::ShaderModule {
        &self.raw
    }
}
