use std::ffi::CStr;

pub mod buffer;
pub mod cache;
pub mod commands;
pub mod depth;
pub mod queue;
pub mod render_target;
pub mod renderpass;
pub mod shader;
pub mod surface;
pub mod vulkan;

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
