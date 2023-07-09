use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION")]
pub const NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME")]
pub const NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_geometry_shader_passthrough");
