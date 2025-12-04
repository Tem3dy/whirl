use std::fmt::Debug;

use bytemuck::Pod;
use wgpu::{
    Device, Queue,
    util::{BufferInitDescriptor, DeviceExt},
};

/// Specifies the usage of the buffer on the GPU
///
/// All variants specify whether the buffer can be written to after creation
#[derive(Debug, Clone, Copy)]
pub enum BufferUsage {
    /// Specifies that the buffer will be used for vertex data
    Vertex { is_writable: bool },
    /// Specifies that the buffer will be used for index data
    Index { is_writable: bool },
    /// Specifies that the buffer will be used for small amounts of data in shaders
    Uniform { is_writable: bool },
    /// Specifies that the buffer will be used for large amounts of data in shaders
    Storage { is_writable: bool },
}

/// Describes a buffer wrapper
#[derive(Debug)]
pub struct Buffer {
    /// Represents the usage of the buffer on the GPU
    usage: BufferUsage,
    /// Represents the size of the buffer in bytes
    size: usize,
    /// Represents the internal [`wgpu::Buffer`]
    raw: wgpu::Buffer,
}

impl BufferUsage {
    /// Maps the [`BufferUsage`] to the internal [`wgpu::BufferUsages`]
    pub fn raw(self) -> wgpu::BufferUsages {
        match self {
            Self::Vertex { is_writable } => {
                if is_writable {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
                }
            }
            Self::Index { is_writable } => {
                if is_writable {
                    wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::INDEX
                }
            }
            Self::Uniform { is_writable } => {
                if is_writable {
                    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::UNIFORM
                }
            }
            Self::Storage { is_writable } => {
                if is_writable {
                    wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::STORAGE
                }
            }
        }
    }
}

impl Buffer {
    /// Creates a new buffer of a fixed size
    /// - `device` -> The raw [`wgpu::Device`] used to create a buffer on the GPU
    /// - `size` -> The size of the buffer in bytes
    /// - `usage` -> The usage of the buffer on the GPU
    pub fn new(device: &Device, size: usize, usage: BufferUsage) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: size as u64,
            usage: usage.raw(),
            mapped_at_creation: false,
        });
        Self {
            usage,
            size,
            raw: buffer,
        }
    }

    /// Creates a new buffer of a size that matches the size of the passed data slice
    /// - `device` -> The raw [`wgpu::Device`] used to create a buffer on the GPU
    /// - `data` -> The data slice containing any kind of buffer data
    /// - `usage` -> The usage of the buffer on the GPU
    pub fn with<T: Pod>(device: &Device, data: &[T], usage: BufferUsage) -> Self {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data),
            usage: usage.raw(),
        });
        Self {
            usage,
            size: size_of_val(data),
            raw: buffer,
        }
    }

    /// Writes data to an existing buffer on the GPU with a byte offset
    ///
    /// - `device` -> The raw [`wgpu::Device`] used for creating a new internal buffer in case
    ///   the new data size exceeds the current buffer size
    ///
    /// - `queue` -> The raw [`wgpu::Queue`] used for submitting the write command
    /// - `data` -> The data slice containing any kind of buffer data
    /// - `offset` -> The byte offset
    ///
    /// If the size of the new data exceeds the current buffer size,
    /// the internal buffer is replaced with the size of the new data
    pub fn write_offset<T: Pod>(
        &mut self,
        device: &Device,
        queue: &Queue,
        data: &[T],
        offset: u64,
    ) {
        if !self.is_writable() {
            panic!("Buffer is not writable");
        }

        // Properly handle the case when offset is > 0
        let new_size = size_of_val(data);
        if new_size > self.size {
            println!("Creating a new raw buffer to resize...");
            self.size = new_size;
            self.raw = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(data),
                usage: self.usage.raw(),
            })
        }

        queue.write_buffer(&self.raw, offset, bytemuck::cast_slice(data));
    }

    /// Writes data to an existing buffer on the GPU with an offset of zero bytes
    ///
    /// - `device` -> The raw [`wgpu::Device`] used for creating a new internal buffer in case
    ///   the new data size exceeds the current buffer size
    ///
    /// - `queue` -> The raw [`wgpu::Queue`] used for submitting the write command
    /// - `data` -> The data slice containing any kind of buffer data
    ///
    /// If the size of the new data exceeds the current buffer size,
    /// the internal buffer is replaced with the size of the new data
    pub fn write<T: Pod>(&mut self, device: &Device, queue: &Queue, data: &[T]) {
        self.write_offset(device, queue, data, 0);
    }

    /// Returns whether the buffer is writable or not
    pub fn is_writable(&self) -> bool {
        match self.usage {
            BufferUsage::Vertex { is_writable } => is_writable,
            BufferUsage::Index { is_writable } => is_writable,
            BufferUsage::Uniform { is_writable } => is_writable,
            BufferUsage::Storage { is_writable } => is_writable,
        }
    }

    /// Returns the [`BufferUsage`] of the buffer
    pub fn usage(&self) -> BufferUsage {
        self.usage
    }

    /// Returns the current size of the buffer in bytes
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the raw [`wgpu::Buffer`]
    pub fn raw(&self) -> &wgpu::Buffer {
        &self.raw
    }

    /// Returns the buffer as a [`wgpu::BufferSlice`]
    pub fn as_slice(&self) -> wgpu::BufferSlice<'_> {
        self.raw.slice(..)
    }
}
