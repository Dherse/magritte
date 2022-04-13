#![feature(arbitrary_self_types)]

use std::ffi::CStr;

#[doc(hidden)]
mod ffi;

mod allocation;
mod allocator;
mod defragmentation_context;
mod flags;
mod pool;
mod virtual_allocation;
mod virtual_block;

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
