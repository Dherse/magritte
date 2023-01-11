#![feature(arbitrary_self_types)]

#[doc(hidden)]
mod ffi;

mod allocation;
mod allocator;
mod buffer;
mod defragmentation_context;
mod flags;
mod image;
mod pool;

pub use allocation::Allocation;
pub use allocator::Allocator;
pub use buffer::{BufferUsage, VmaBuffer};
pub use defragmentation_context::DefragmentationContext;
pub use ffi::{
    AllocationCreateInfo, AllocatorInfo, Budget, DetailedStatistics, MemoryUsage, PoolCreateInfo, Statistics,
    TotalStatistics,
};
pub use flags::{AllocationCreateFlags, DefragmentationFlags, PoolCreateFlags};
pub use image::{ImageUsage, VmaImage};
pub use pool::{MemoryCorruptionState, Pool};
