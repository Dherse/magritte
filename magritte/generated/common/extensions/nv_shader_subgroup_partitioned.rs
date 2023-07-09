use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION")]
pub const NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME")]
pub const NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_shader_subgroup_partitioned");
