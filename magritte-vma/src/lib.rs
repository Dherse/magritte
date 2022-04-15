#![feature(arbitrary_self_types)]

use std::ffi::CStr;

#[doc(hidden)]
mod ffi;

mod allocation;
mod allocator;
mod buffer;
mod defragmentation_context;
mod flags;
mod pool;
mod image;

pub use allocation::Allocation;
pub use allocator::Allocator;
pub use defragmentation_context::DefragmentationContext;
pub use buffer::{VmaBuffer, BufferUsage};
pub use image::{VmaImage, ImageUsage};
pub use ffi::{
    AllocationCreateInfo, AllocatorInfo, Budget, DetailedStatistics, PoolCreateInfo, Statistics, TotalStatistics,
    MemoryUsage,
};
pub use flags::{AllocationCreateFlags, PoolCreateFlags, DefragmentationFlags};
pub use pool::{MemoryCorruptionState, Pool};

pub(crate) trait AsCStr {
    fn as_cstr(&self) -> &CStr;
}

impl AsCStr for &[i8] {
    fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }
}

impl<const N: usize> AsCStr for &[i8; N] {
    fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }
}
