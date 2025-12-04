use std::{error::Error, fmt, path::PathBuf};

use image::{EncodableLayout, ImageReader};

/// Describes a wrapper around [`wgpu::Texture`] with more information
#[derive(Debug)]
pub struct Texture {
    /// Represents the raw [`wgpu::Texture`]
    raw: wgpu::Texture,
    /// Represents the raw [`wgpu::TextureView`]
    raw_view: wgpu::TextureView,
    /// Represents the dimensions (width, height, depth) of the texture
    size: TextureSize,
}

/// Describes a texture
#[derive(Debug)]
pub struct TextureDescriptor<'a> {
    /// The optional debugging label of this texture
    pub label: Option<&'a str>,
    /// The dimension of this texture (1D, 2D, 3D)
    pub dimension: TextureDimension,
    /// The usage of this texture (image binding, storage binding, render attachment)
    pub usage: TextureUsage,
    /// The data source of this texture (file, depth, stencil, blank, bytes)
    pub source: TextureSource,
}

/// Describes the size of a texture
#[derive(Debug, Clone, Copy)]
pub struct TextureSize {
    /// The width of this texture (in pixels)
    pub width: u32,
    /// The height of this texture (in pixels)
    pub height: u32,
    /// The depth of this texture (in pixels), this is 1 for 2D textures
    pub depth: u32,
}

/// Specifies the dimension of the texture
#[derive(Debug, Clone, Copy)]
pub enum TextureDimension {
    /// Represents a 1D texture (an array of pixels)
    D1,
    /// Represents a 2D texture (a grid of pixels)
    D2,
    /// Represents a 3D texture (a cube of pixels)
    D3,
}

/// Specifies the format of the texture
#[derive(Debug, Clone, Copy)]
pub enum TextureFormat {
    /// The standard RGBA format
    Standard,
    /// A signed format which is normalized
    SignedNormalized,
    /// An unsigned format which is normalized
    UnsignedNormalized,
    /// A signed integer format
    Signed,
    /// An unsigned integer format
    Unsigned,
    /// A depth buffer format
    Depth,
    /// A stencil buffer format
    Stencil,
    /// A combined depth + stencil buffer format
    DepthStencil,
}

/// Specifies the usage of the texture
///
/// Each of these usages are either CPU writable or readable (or both)
#[derive(Debug, Clone, Copy)]
pub enum TextureUsage {
    /// The texture is to be sampled in a shader
    Image {
        is_writable: bool,
        is_readable: bool,
    },
    /// The texture serves as a storage for pixel data
    Storage {
        is_writable: bool,
        is_readable: bool,
    },
    /// The texture serves as a render attachment
    Attachment {
        is_writable: bool,
        is_readable: bool,
    },
}

/// Specifies the source of the texture
#[derive(Debug, Clone)]
pub enum TextureSource {
    /// The texture's source data comes from a file (png, jpeg, bmp)
    File { path: PathBuf },
    /// The texture's source data is an empty depth buffer of specified dimensions
    Depth { width: u32, height: u32 },
    /// The texture's source data is an empty stencil buffer of specified dimensions
    Stencil { width: u32, height: u32 },
    /// The texture's source data is a combined depth + stencil buffer of specified dimensions
    DepthStencil { width: u32, height: u32 },
    /// The texture's source data is a blank canvas of specified dimensions (and a format)
    Blank {
        width: u32,
        height: u32,
        format: TextureFormat,
    },
    /// The texture's source data is a collection of pixels (and a format) of specified dimensions
    Bytes {
        width: u32,
        height: u32,
        format: TextureFormat,
        bytes: Vec<u8>,
    },
}

/// Specifies a texture error that may have occurred.
#[derive(Debug)]
pub enum TextureError {
    /// The image file the texture was meant to be created from failed to open
    OpenFailure {
        /// The file the texture tried to read from
        file: PathBuf,
        /// The underlying cause of the failure
        cause: Box<dyn Error>,
    },
    /// The image data the texture was meant to be created from couldn't be decoded
    DecodeFailure {
        /// The file the texture tried to decode from
        file: PathBuf,
        /// The underlying cause of the failure
        cause: Box<dyn Error>,
    },
    /// The texture couldn't be uploaded to the GPU
    WriteFailure {
        /// In the case of an image source, this represents the file that the texture came from
        file: PathBuf,
        /// The underlying cause of the failure
        cause: &'static str,
    },
    /// The texture is of an illegal size
    IllegalSize {
        /// The illegal size information
        size: (u32, u32),
        /// Specifies which extent (width/height) was of an illegal size
        cause: &'static str,
    },
}

impl Texture {
    /// Returns a reference to the raw [`wgpu::Texture`]
    pub fn raw(&self) -> &wgpu::Texture {
        &self.raw
    }

    /// Returns a reference to the raw [`wgpu::TextureView`]
    pub fn view(&self) -> &wgpu::TextureView {
        &self.raw_view
    }

    /// Returns the size of the texture contained in a [`TextureSize`]
    pub fn size(&self) -> TextureSize {
        self.size
    }
}

impl<'a> TextureDescriptor<'a> {
    /// Attempts to build a [`Texture`] from this descriptor, returns a [`TextureError`] upon failure
    /// - `device` -> the [`wgpu::Device`] needed to create this GPU resource
    /// - `queue` -> the [`wgpu::Queue`] needed to write the image data to this texture on the GPU
    pub fn build(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Texture, TextureError> {
        match self.source.clone() {
            TextureSource::File { path } => Self::into_file(self, device, queue, path),
            TextureSource::Depth { width, height } => Self::into_depth(self, device, width, height),
            TextureSource::Stencil { width, height } => {
                Self::into_stencil(self, device, width, height)
            }
            TextureSource::DepthStencil { width, height } => {
                Self::into_depth_stencil(self, device, width, height)
            }
            TextureSource::Blank {
                width,
                height,
                format,
            } => Self::into_blank(self, device, width, height, format),
            TextureSource::Bytes {
                width,
                height,
                format,
                bytes,
            } => Self::into_bytes(self, device, queue, width, height, format, bytes),
        }
    }

    fn upload_texture(
        queue: &wgpu::Queue,
        texture: &Texture,
        texture_size: TextureSize,
        pixel_size: u32,
        mip_level: u32,
        bytes: &[u8],
    ) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: texture.raw(),
                mip_level,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pixel_size * texture_size.width),
                rows_per_image: Some(texture_size.height),
            },
            texture_size.raw(),
        );
    }

    fn into_args(self, device: &wgpu::Device, size: TextureSize, format: TextureFormat) -> Texture {
        let raw_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: self.label, 
            size: size.raw(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: self.dimension.raw(),
            format: format.raw(),
            usage: self.usage.raw(),
            view_formats: &[],
        });
        Texture {
            raw_view: raw_texture.create_view(&wgpu::TextureViewDescriptor::default()),
            raw: raw_texture,
            size,
        }
    }

    fn into_file(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        path: PathBuf,
    ) -> Result<Texture, TextureError> {
        let image_reader = match ImageReader::open(&path) {
            Ok(image_reader) => image_reader,
            Err(cause) => {
                return Err(TextureError::OpenFailure {
                    file: path,
                    cause: Box::new(cause),
                });
            }
        };
        let image = match image_reader.decode() {
            Ok(image) => image,
            Err(cause) => {
                return Err(TextureError::DecodeFailure {
                    file: path,
                    cause: Box::new(cause),
                });
            }
        };
        let image_size = TextureSize {
            width: image.width(),
            height: image.height(),
            depth: 1,
        };
        if let TextureUsage::Image {
            is_writable,
            is_readable: _is_readable,
        } = self.usage
            && !is_writable
        {
            return Err(TextureError::WriteFailure {
                file: path,
                cause: "Texture is not writable",
            });
        }
        let texture = self.into_args(device, image_size, TextureFormat::Standard);
        Self::upload_texture(
            queue,
            &texture,
            image_size,
            4,
            0,
            image.into_rgba8().as_bytes(),
        );
        Ok(texture)
    }

    fn into_depth(
        self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureError> {
        Self::err_on_zero(width, height)?;
        let size = TextureSize {
            width,
            height,
            depth: 1,
        };
        Ok(self.into_args(device, size, TextureFormat::Depth))
    }

    fn into_stencil(
        self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureError> {
        Self::err_on_zero(width, height)?;
        Ok(self.into_args(
            device,
            TextureSize {
                width,
                height,
                depth: 1,
            },
            TextureFormat::Stencil,
        ))
    }

    fn into_depth_stencil(
        self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
    ) -> Result<Texture, TextureError> {
        Self::err_on_zero(width, height)?;
        Ok(self.into_args(
            device,
            TextureSize {
                width,
                height,
                depth: 1,
            },
            TextureFormat::DepthStencil,
        ))
    }

    fn into_blank(
        self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Result<Texture, TextureError> {
        Self::err_on_zero(width, height)?;
        Ok(self.into_args(
            device,
            TextureSize {
                width,
                height,
                depth: 1,
            },
            format,
        ))
    }

    pub fn into_bytes(
        self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        format: TextureFormat,
        bytes: Vec<u8>,
    ) -> Result<Texture, TextureError> {
        Self::err_on_zero(width, height)?;
        let texture_size = TextureSize {
            width,
            height,
            depth: 1,
        };
        let texture = self.into_args(device, texture_size, format);
        Self::upload_texture(queue, &texture, texture_size, 4, 0, &bytes);
        Ok(texture)
    }

    fn err_on_zero(width: u32, height: u32) -> Result<(), TextureError> {
        if width == 0 {
            return Err(TextureError::IllegalSize {
                size: (width, height),
                cause: "Texture width cannot be zero",
            });
        }

        if height == 0 {
            return Err(TextureError::IllegalSize {
                size: (width, height),
                cause: "Texture height cannot be zero",
            });
        }

        Ok(())
    }
}

impl TextureSize {
    /// Maps the high level [`TextureSize`] to a [`wgpu::Extent3d`]
    pub fn raw(self) -> wgpu::Extent3d {
        wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: self.depth,
        }
    }
}

impl TextureDimension {
    /// Maps the high level [`TextureDimension`] to a [`wgpu::TextureDimension`]
    pub fn raw(self) -> wgpu::TextureDimension {
        match self {
            TextureDimension::D1 => wgpu::TextureDimension::D1,
            TextureDimension::D2 => wgpu::TextureDimension::D2,
            TextureDimension::D3 => wgpu::TextureDimension::D3,
        }
    }
}

impl TextureFormat {
    /// Maps the high level [`TextureFormat`] to a [`wgpu::TextureFormat`]
    pub fn raw(self) -> wgpu::TextureFormat {
        match self {
            TextureFormat::Standard => wgpu::TextureFormat::Rgba8UnormSrgb,
            TextureFormat::SignedNormalized => wgpu::TextureFormat::Rgba8Snorm,
            TextureFormat::UnsignedNormalized => wgpu::TextureFormat::Rgba8Unorm,
            TextureFormat::Signed => wgpu::TextureFormat::Rgba8Sint,
            TextureFormat::Unsigned => wgpu::TextureFormat::Rgba8Uint,
            TextureFormat::Depth => wgpu::TextureFormat::Depth32Float,
            TextureFormat::Stencil => wgpu::TextureFormat::Stencil8,
            TextureFormat::DepthStencil => wgpu::TextureFormat::Depth24PlusStencil8,
        }
    }
}

impl TextureUsage {
    /// Maps the high level [`TextureUsage`] to a [`wgpu::TextureUsages`]
    pub fn raw(self) -> wgpu::TextureUsages {
        let writable = wgpu::TextureUsages::COPY_DST;
        let readable = wgpu::TextureUsages::COPY_SRC;
        match self {
            TextureUsage::Image {
                is_writable,
                is_readable,
            } => match (is_writable, is_readable) {
                (true, true) => wgpu::TextureUsages::TEXTURE_BINDING | writable | readable,
                (true, false) => wgpu::TextureUsages::TEXTURE_BINDING | writable,
                (false, true) => wgpu::TextureUsages::TEXTURE_BINDING | readable,
                (false, false) => wgpu::TextureUsages::TEXTURE_BINDING,
            },
            TextureUsage::Storage {
                is_writable,
                is_readable,
            } => match (is_writable, is_readable) {
                (true, true) => wgpu::TextureUsages::STORAGE_BINDING | writable | readable,
                (true, false) => wgpu::TextureUsages::STORAGE_BINDING | writable,
                (false, true) => wgpu::TextureUsages::STORAGE_BINDING | readable,
                (false, false) => wgpu::TextureUsages::STORAGE_BINDING,
            },
            TextureUsage::Attachment {
                is_writable,
                is_readable,
            } => match (is_writable, is_readable) {
                (true, true) => wgpu::TextureUsages::RENDER_ATTACHMENT | writable | readable,
                (true, false) => wgpu::TextureUsages::RENDER_ATTACHMENT | writable,
                (false, true) => wgpu::TextureUsages::RENDER_ATTACHMENT | readable,
                (false, false) => wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        }
    }
}

impl fmt::Display for TextureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextureError::OpenFailure { file, cause } => {
                write!(
                    f,
                    "Couldn't open texture from file {:?}:\n\t{}",
                    file, cause
                )
            }
            TextureError::DecodeFailure { file, cause } => {
                write!(
                    f,
                    "Couldn't decode texture from file {:?}:\n\t{}",
                    file, cause
                )
            }
            TextureError::WriteFailure { file, cause } => {
                write!(
                    f,
                    "Couldn't write texture from file {:?}:\n\t{}",
                    file, cause
                )
            }
            TextureError::IllegalSize { size, cause } => {
                write!(f, "Illegal texture size: {:?}:\n\t{}", size, cause)
            }
        }
    }
}
