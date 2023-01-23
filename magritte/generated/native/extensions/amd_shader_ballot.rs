//!# [VK_AMD_shader_ballot](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_ballot.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_shader_ballot/VK_AMD_shader_ballot.md")]
use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_AMD_SHADER_BALLOT_SPEC_VERSION")]
pub const AMD_SHADER_BALLOT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_SHADER_BALLOT_EXTENSION_NAME")]
pub const AMD_SHADER_BALLOT_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_shader_ballot");
