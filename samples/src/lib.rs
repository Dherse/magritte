use std::ffi::CStr;

pub mod buffer;
pub mod commands;
pub mod depth;
pub mod queue;
pub mod renderpass;
pub mod shader;
pub mod surface;
pub mod vulkan;
pub mod cache;

pub trait AsCStr {
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
