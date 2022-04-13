#![feature(arbitrary_self_types)]

use std::ffi::CStr;

#[doc(hidden)]
pub mod ffi;

pub mod allocation;
pub mod allocator;
pub mod defragmentation_context;
pub mod flags;
pub mod pool;
pub mod virtual_allocation;
pub mod virtual_block;

pub use allocator::VmaAllocator;

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
