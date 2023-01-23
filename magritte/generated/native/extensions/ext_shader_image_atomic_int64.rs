//!# [VK_EXT_shader_image_atomic_int64](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_image_atomic_int64.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_shader_image_atomic_int64/VK_EXT_shader_image_atomic_int64.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_shader_image_atomic_int64/VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderImageInt64Atomics")]
    shader_image_int64_atomics: Bool32,
    #[doc(alias = "sparseImageInt64Atomics")]
    sparse_image_int64_atomics: Bool32,
}
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_shader_image_atomic_int64");
