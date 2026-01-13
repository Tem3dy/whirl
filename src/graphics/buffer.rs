use std::fmt::Debug;

use bytemuck::Pod;
use wgpu::{
    BufferDescriptor, Device, Queue,
    util::{BufferInitDescriptor, DeviceExt},
};

/// A handle to a buffer on the GPU.
///
/// A buffer is used for any kind of data that needs to be provided to
/// the rendering pipeline, such as geometry data (vertices, indices),
/// or shader data that can change every frame (transformations, particle data).
///
/// A buffer may be of several kinds, such as:
/// - [`BufferUsage::Index`]
/// - [`BufferUsage::Vertex`]
/// - [`BufferUsage::Uniform`]
/// - [`BufferUsage::Storage`]
///
/// The buffer API is designed around the concepts of "items", where an item
/// is an instance of the type the buffer has, for example:
/// ```rust
/// #[repr(C)]
/// #[derive(Copy, Clone, Zeroable, Pod)]
/// struct Vertex {
///     position: [f32; 3],
/// }
///
/// fn create_geometry(device: &wgpu::Device) {
///     let geometry_buffer_handle = BufferHandle::<Vertex>::create(
///         device,
///         &[
///             Vertex { position: [ 0.0,  0.5, 0.0] },
///             Vertex { position: [-0.5, -0.5, 0.0] },
///             Vertex { position: [ 0.5, -0.5, 0.0] },
///         ],
///         BufferUsage::Vertex { is_writable: false, },
///         Some("Triangle geometry"),
///     );
///     ...
/// }
/// ```
/// This basic example creates a vertex buffer which holds items of type `Vertex`,
/// which is a struct with a single field that stores the 3D position of the vertex.
///
/// First of all, notice how the `Vertex` type is marked with `#[repr(C)]` and
/// derives both `Zeroable` and `Pod` from bytemuck.
///
/// The reason for this is that the GPU works with raw bytes, therefore it needs
/// some way to convert a high-level struct such as `Vertex` to a slice of bytes,
/// which is where `#[repr(C)]`, `Zeroable` and `Pod` come in.
///
/// For more information on those, I suggest looking them up in the `bytemuck` documentation.
///
/// All you need to know to use this API, is that any items stored inside a buffer must
/// implement the `Pod` trait, which requires that the type is also marked with `#[repr(C)]` and `Zeroable`,
/// as well as `Clone` and `Copy`, as can be seen in the example provided above.
///
/// In this example, the buffer holds exactly 3 items of type `Vertex`, which make up
/// a triangle in normalized space.
///
/// The thing with geometry and index buffers is that they usually not change once they've been created,
/// meaning they're just static data in the majority of cases, so the concepts of "items" isn't
/// very useful for them.
///
/// However, storing items really shines whenever we have an instance buffer, or an uniform/storage
/// buffer. The API provides several methods such as:
/// - [`BufferHandle::skip_and_write_item()`]
/// - [`BufferHandle::skip_and_update_item()`]
/// - [`BufferHandle::extend_with_item()`]
/// - [`BufferHandle::skip_and_write_item_list()`]
/// - [`BufferHandle::skip_and_update_item_list()`]
/// - [`BufferHandle::extend_with_item_list()`]
///
/// These exist to allow efficient partial writes/updates of items or even lists of items provided
/// they're in the correct order as they live in the buffer.
///
/// The API provides 2 kinds of methods,
/// - Flushing
/// - Non-flushing
///
/// The list of methods above is of the non-flushing kind, meaning for them to dispatch
/// on the GPU, one of the flush methods needs to be called, and there are 3 for flexibility:
/// - [`BufferHandle::flush()`]
/// - [`BufferHandle::skip_and_flush()`]
/// - [`BufferHandle::skip_and_flush_exact()`]
///
/// What this allows is, if you have a large amount of partial writes/updates,
/// you can do them entirely on the CPU first, and then just call flush once which will
/// reflect all of the changes you've made to the buffer, which is great for performance.
///
/// If the flexibility is not needed, you can simply just call their flushing counterparts:
/// - [`BufferHandle::skip_and_write_item_and_flush()`]
/// - [`BufferHandle::skip_and_update_item_and_flush()`]
/// - [`BufferHandle::extend_with_item_and_flush()`]
/// - [`BufferHandle::skip_and_write_item_list_and_flush()`]
/// - [`BufferHandle::skip_and_update_item_list_and_flush()`]
/// - [`BufferHandle::extend_with_item_list_and_flush()`]
///
/// There are also more methods that haven't been listed above, such as:
/// - [`BufferHandle::overwrite_and_flush()`]
/// - [`BufferHandle::truncate_and_flush()`]
/// - [`BufferHandle::nuke_and_flush()`]
///
/// All of that functionality is available if needed.
#[derive(Debug)]
pub struct BufferHandle<T: Pod> {
    item_list: Vec<T>,
    item_capacity: usize,
    usage: BufferUsage,
    raw: wgpu::Buffer,
}

impl<T: Pod> BufferHandle<T> {
    /// Allocates a new buffer that can hold initially hold `item_capacity` items,
    /// but the capacity can grow dynamically if needed.
    ///
    /// # Panics:
    /// - If `item_capacity` is equal to zero.
    pub fn allocate(
        device: &Device,
        item_capacity: usize,
        usage: BufferUsage,
        label: Option<&str>,
    ) -> Self {
        assert!(item_capacity > 0, "Item capacity cannot be zero!");
        Self {
            usage,
            item_capacity,
            raw: device.create_buffer(&wgpu::BufferDescriptor {
                label,
                size: item_capacity as u64 * size_of::<T>() as u64,
                usage: usage.raw(),
                mapped_at_creation: false,
            }),
            item_list: Vec::with_capacity(item_capacity),
        }
    }

    /// Creates a new buffer with the contents of `item_list`.
    ///
    /// The capacity of the buffer will initially be equal to the length of `item_list`,
    /// but can dynamically grow if needed.
    ///
    /// # Panics:
    /// - If `item_list` is an empty slice.
    pub fn create(
        device: &Device,
        item_list: &[T],
        usage: BufferUsage,
        label: Option<&str>,
    ) -> Self {
        assert!(
            !item_list.is_empty(),
            "Cannot create a buffer with an empty slice!"
        );
        Self {
            usage,
            item_capacity: item_list.len(),
            raw: device.create_buffer_init(&BufferInitDescriptor {
                label,
                contents: bytemuck::cast_slice(item_list),
                usage: usage.raw(),
            }),
            item_list: Vec::from(item_list),
        }
    }

    /// Skips `items_to_skip` items in the buffer and writes an item.
    ///
    /// If `items_to_skip` exceeds the item count of the buffer, the buffer is resized
    /// and the empty item slots are filled with zeroed data and the item gets written.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call the correct flush method such as:
    /// - [`BufferHandle::flush()`]
    /// - [`BufferHandle::skip_and_flush()`]
    /// - [`BufferHandle::skip_and_flush_exact()`]
    ///
    /// Or simply use [`BufferHandle::skip_and_write_item_and_flush()`] for an immediate flush.
    pub fn skip_and_write_item(&mut self, items_to_skip: usize, item: T) {
        self.skip_and_write_item_list(items_to_skip, &[item]);
    }

    /// Skips `items_to_skip` items in the buffer and writes a list of items.
    ///
    /// If `items_to_skip` exceeds the item count of the buffer, the buffer is resized
    /// and the empty item slots are filled with zeroed data and the list of items get written.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call the correct flush method such as:
    /// - [`BufferHandle::flush()`]
    /// - [`BufferHandle::skip_and_flush()`]
    /// - [`BufferHandle::skip_and_flush_exact()`]
    ///
    /// Or simply use [`BufferHandle::skip_and_write_item_list_and_flush()`] for an immediate flush.
    ///
    /// # Panics:
    /// - If the `item_list` is empty.
    pub fn skip_and_write_item_list(&mut self, items_to_skip: usize, item_list: &[T]) {
        assert!(
            !item_list.is_empty(),
            "Cannot write an empty slice to the buffer!"
        );
        let required_length = items_to_skip + item_list.len();
        if self.item_list.len() < required_length {
            self.item_list.resize(required_length, T::zeroed());
        }

        self.item_list[items_to_skip..required_length].copy_from_slice(item_list);
    }

    /// Skips `items_to_skip` items in the buffer and updates an existing item.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call the correct flush method such as:
    /// - [`BufferHandle::flush()`]
    /// - [`BufferHandle::skip_and_flush()`]
    /// - [`BufferHandle::skip_and_flush_exact()`]
    ///
    /// Or simply use [`BufferHandle::skip_and_update_item_and_flush()`] for an immediate flush.
    ///
    /// # Panics:
    /// - If `items_to_skip` is equal to or exceeds the length of the buffer.
    pub fn skip_and_update_item(&mut self, items_to_skip: usize, item: T) {
        self.skip_and_update_item_list(items_to_skip, &[item]);
    }

    /// Skips `items_to_skip` items in the buffer and updates an existing list of items.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call the correct flush method such as:
    /// - [`BufferHandle::flush()`]
    /// - [`BufferHandle::skip_and_flush()`]
    /// - [`BufferHandle::skip_and_flush_exact()`]
    ///
    /// Or simply use [`BufferHandle::skip_and_update_item_and_flush()`] for an immediate flush.
    ///
    /// # Panics:
    /// - If `items_to_skip` plus the length of the `item_list`
    ///   exceeds the length of the buffer.
    pub fn skip_and_update_item_list(&mut self, items_to_skip: usize, item_list: &[T]) {
        assert!(
            !item_list.is_empty(),
            "Cannot update the buffer with an empty slice!"
        );
        assert!(
            items_to_skip + item_list.len() <= self.item_list.len(),
            "Cannot update the buffer because it would overflow!"
        );
        let start_index = items_to_skip;
        let end_index = items_to_skip + item_list.len();
        self.item_list[start_index..end_index].copy_from_slice(item_list);
    }

    /// Extends the buffer with a list of items.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call the correct flush method such as:
    /// - [`BufferHandle::flush()`]
    /// - [`BufferHandle::skip_and_flush()`]
    /// - [`BufferHandle::skip_and_flush_exact()`]
    ///
    /// Or simply use [`BufferHandle::extend_with_item_list_and_flush()`] for an immediate flush.
    pub fn extend_with_item_list(&mut self, item_list: &[T]) {
        assert!(
            !item_list.is_empty(),
            "Cannot extend the buffer with an empty slice!"
        );
        self.skip_and_write_item_list(self.item_list.len(), item_list);
    }

    /// Extends the buffer with an item.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call the correct flush method such as:
    /// - [`BufferHandle::flush()`]
    /// - [`BufferHandle::skip_and_flush()`]
    /// - [`BufferHandle::skip_and_flush_exact()`]
    ///
    /// Or simply use [`BufferHandle::extend_with_item_and_flush()`] for an immediate flush.
    pub fn extend_with_item(&mut self, item: T) {
        self.extend_with_item_list(&[item]);
    }

    /// Truncates the buffer to a specified length.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call the correct flush method, such as:
    /// - [`BufferHandle::flush()`]
    /// - [`BufferHandle::skip_and_flush()`]
    /// - [`BufferHandle::skip_and_flush_exact()`]
    ///
    /// # Constraints:
    /// - `length` must be greater than zero.
    /// - `length` must be smaller than the item count of the buffer.
    pub fn truncate(&mut self, length: usize) {
        assert!(
            length > 0 && length < self.item_list.len(),
            "Cannot truncate buffer out of bounds!"
        );
        self.item_list.truncate(length);
    }

    /// Overwrites the buffer with the contents of `item_list`.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call [`BufferHandle::flush()`],
    /// or simply use [`BufferHandle::overwrite_and_flush()`] for an immediate flush.
    ///
    /// # Panics:
    /// - If `item_list` is empty.
    pub fn overwrite(&mut self, item_list: &[T]) {
        assert!(
            !item_list.is_empty(),
            "Cannot overwrite the buffer with an empty slice!"
        );
        self.item_list.clear();
        self.item_list.extend_from_slice(item_list);
    }

    /// Nukes the buffer, in other words, replaces the contents of the entire buffer
    /// with zeroed data.
    ///
    /// This function does not flush, meaning it does not update the GPU buffer after calling it.
    ///
    /// To accomplish that, call [`BufferHandle::flush()`],
    /// or simply use [`BufferHandle::nuke_and_flush()`] instead.
    ///
    /// What this function actually does is subject to change.
    pub fn nuke(&mut self) {
        self.item_list.fill(T::zeroed());
    }

    /// Skips `items_to_skip` items and writes an item, then flushes immediately.
    ///
    /// If `items_to_skip` exceeds the item count of the buffer, the buffer is resized
    /// and the empty item slots are filled with zeroed data and the item gets written.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    pub fn skip_and_write_item_and_flush(
        &mut self,
        device: &Device,
        queue: &Queue,
        items_to_skip: usize,
        item: T,
    ) {
        self.skip_and_write_item_list_and_flush(device, queue, items_to_skip, &[item]);
    }

    /// Skips `items_to_skip` items and writes a list of items, then flushes immediately.
    ///
    /// If `items_to_skip` exceeds the item count of the buffer, the buffer is resized
    /// and the empty item slots are filled with zeroed data and the list of items gets written.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If the `item_list` is empty.
    pub fn skip_and_write_item_list_and_flush(
        &mut self,
        device: &Device,
        queue: &Queue,
        items_to_skip: usize,
        item_list: &[T],
    ) {
        self.skip_and_write_item_list(items_to_skip, item_list);
        self.skip_and_flush_exact(device, queue, items_to_skip, item_list.len());
    }

    /// Skips `items_to_skip` items and updates an existing item, then flushes immediately.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If `items_to_skip` is equal to or exceeds the length of the buffer.
    pub fn skip_and_update_item_and_flush(
        &mut self,
        device: &Device,
        queue: &Queue,
        items_to_skip: usize,
        item: T,
    ) {
        self.skip_and_update_item_list_and_flush(device, queue, items_to_skip, &[item]);
    }

    /// Skips `items_to_skip` items and updates an existing list of items, then flushes immediately.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If `items_to_skip` plus the length of the `item_list`
    ///   is equal to or exceeds the length of the buffer.
    pub fn skip_and_update_item_list_and_flush(
        &mut self,
        device: &Device,
        queue: &Queue,
        items_to_skip: usize,
        item_list: &[T],
    ) {
        self.skip_and_update_item_list(items_to_skip, item_list);
        self.skip_and_flush_exact(device, queue, items_to_skip, item_list.len());
    }

    /// Extends the buffer with an item, then flushes immediately.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    pub fn extend_with_item_and_flush(&mut self, device: &Device, queue: &Queue, item: T) {
        self.extend_with_item_list_and_flush(device, queue, &[item]);
    }

    /// Extends the buffer with a list of items, then flushes immediately.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If `item_list` is empty.
    pub fn extend_with_item_list_and_flush(
        &mut self,
        device: &Device,
        queue: &Queue,
        item_list: &[T],
    ) {
        self.skip_and_write_item_list_and_flush(device, queue, self.item_list.len(), item_list);
    }

    /// Truncates the buffer to a specified length, then flushes immediately.
    ///
    /// # Constraints:
    /// - `length` must be greater than zero.
    /// - `length` must be smaller than the item count of the buffer.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    pub fn truncate_and_flush(&mut self, device: &Device, queue: &Queue, length: usize) {
        self.truncate(length);
        self.skip_and_flush(device, queue, length);
    }

    /// Overwrites the buffer with the contents of `item_list`, then flushes immediately.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If `item_list` is empty.
    pub fn overwrite_and_flush(&mut self, device: &Device, queue: &Queue, item_list: &[T]) {
        self.overwrite(item_list);
        self.flush(device, queue);
    }

    /// Nukes the buffer, in other words, replaces the contents of the entire buffer
    /// with zeroed data, then flushes immediately.
    ///
    /// What this function actually does is subject to change.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    pub fn nuke_and_flush(&mut self, device: &Device, queue: &Queue) {
        self.nuke();
        self.flush(device, queue);
    }

    /// Flushes the entire buffer.
    ///
    /// This effectively copies the CPU buffer to the GPU buffer.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If item capacity of the buffer is zero.
    pub fn flush(&mut self, device: &Device, queue: &Queue) {
        self.skip_and_flush(device, queue, 0);
    }

    /// Skips and flushes the rest of the buffer.
    ///
    /// This effectively copies a range of the CPU buffer to the GPU buffer.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If item capacity of the buffer is zero.
    pub fn skip_and_flush(&mut self, device: &Device, queue: &Queue, items_to_skip: usize) {
        self.skip_and_flush_exact(device, queue, items_to_skip, self.item_list.len());
    }

    /// Skips and flushes an exact amount of items.
    ///
    /// This effectively copies a range of the CPU buffer to the GPU buffer.
    ///
    /// # Panics:
    /// - If the buffer is not writable.
    /// - If the range of items to flush is out of bounds.
    pub fn skip_and_flush_exact(
        &mut self,
        device: &Device,
        queue: &Queue,
        items_to_skip: usize,
        items_to_flush: usize,
    ) {
        assert!(self.is_writable(), "Buffer is not writable!");
        assert!(
            items_to_skip + items_to_flush <= self.item_list.len(),
            "Cannot skip and flush exact because it would be out of bounds"
        );
        let mut needs_resizing = false;
        while self.item_list.len() > self.item_capacity {
            self.item_capacity *= 2;
            needs_resizing = true;
        }

        if needs_resizing {
            self.recreate_buffer(device);
            queue.write_buffer(&self.raw, 0, bytemuck::cast_slice(&self.item_list));
        }
        let start_index = items_to_skip;
        let end_index = items_to_skip + items_to_flush;
        queue.write_buffer(
            &self.raw,
            Self::items_to_bytes(items_to_skip),
            bytemuck::cast_slice(&self.item_list[start_index..end_index]),
        );
    }

    /// Converts the item count to bytes.
    pub fn item_count_to_bytes(&self) -> u64 {
        self.item_list.len() as u64 * size_of::<T>() as u64
    }

    /// Converts the item capacity to bytes.
    pub fn item_capacity_to_bytes(&self) -> u64 {
        self.item_capacity as u64 * size_of::<T>() as u64
    }

    /// Converts `items_to_skip` to bytes to skip.
    pub fn items_to_bytes(items_to_skip: usize) -> u64 {
        items_to_skip as u64 * size_of::<T>() as u64
    }

    /// Returns whether the buffer is writable or not
    pub fn is_writable(&self) -> bool {
        match self.usage {
            BufferUsage::Index { is_writable } => is_writable,
            BufferUsage::Vertex { is_writable } => is_writable,
            BufferUsage::Uniform { is_writable } => is_writable,
            BufferUsage::Storage { is_writable } => is_writable,
        }
    }

    /// Returns the [`BufferUsage`].
    pub fn usage(&self) -> BufferUsage {
        self.usage
    }

    /// Returns a slice of the contents.
    pub fn items(&self) -> &[T] {
        &self.item_list
    }

    /// Returns the item count.
    pub fn item_count(&self) -> usize {
        self.item_list.len()
    }

    /// Returns the item capacity.
    pub fn item_capacity(&self) -> usize {
        self.item_capacity
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.item_list.is_empty()
    }

    /// Returns the raw [`wgpu::Buffer`].
    pub fn raw(&self) -> &wgpu::Buffer {
        &self.raw
    }

    /// Returns the buffer as a [`wgpu::BufferSlice`].
    pub fn as_slice(&self) -> wgpu::BufferSlice<'_> {
        self.raw.slice(..)
    }

    /// Recreates the GPU buffer internally.
    fn recreate_buffer(&mut self, device: &Device) {
        self.raw = device.create_buffer(&BufferDescriptor {
            label: None,
            size: self.item_capacity_to_bytes(),
            usage: self.usage.raw(),
            mapped_at_creation: false,
        });
    }
}

/// Specifies the usage of the buffer on the GPU
///
/// All variants specify whether the buffer can be written to after creation
#[derive(Debug, Clone, Copy)]
pub enum BufferUsage {
    /// Specifies that the buffer will be used for index data
    Index { is_writable: bool },
    /// Specifies that the buffer will be used for vertex data
    Vertex { is_writable: bool },
    /// Specifies that the buffer will be used for small amounts of data in shaders
    Uniform { is_writable: bool },
    /// Specifies that the buffer will be used for large amounts of data in shaders
    Storage { is_writable: bool },
}

impl BufferUsage {
    /// Maps the [`BufferUsage`] to the internal [`wgpu::BufferUsages`]
    pub fn raw(self) -> wgpu::BufferUsages {
        match self {
            Self::Index { is_writable } => {
                if is_writable {
                    wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::INDEX
                }
            }
            Self::Vertex { is_writable } => {
                if is_writable {
                    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
                } else {
                    wgpu::BufferUsages::VERTEX
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

/// A trait that is used to erase the generic type of a buffer,
/// so it can be used in context where the type of the buffer is not known in advance.
pub trait AnyBufferHandle: Debug {
    /// Functionally the same as [`BufferHandle::raw()`].
    fn raw(&self) -> &wgpu::Buffer;
    /// Functionally the same as [`BufferHandle::as_slice()`].
    fn as_slice(&self) -> wgpu::BufferSlice<'_>;
}

impl<T: Debug + Pod> AnyBufferHandle for BufferHandle<T> {
    fn raw(&self) -> &wgpu::Buffer {
        self.raw()
    }

    fn as_slice(&self) -> wgpu::BufferSlice<'_> {
        self.as_slice()
    }
}
