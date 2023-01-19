//!# [VK_EXT_texel_buffer_alignment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_texel_buffer_alignment.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_texel_buffer_alignment/VK_EXT_texel_buffer_alignment.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
    vulkan1_3::PhysicalDeviceTexelBufferAlignmentProperties,
};
use std::ffi::CStr;
///See [`PhysicalDeviceTexelBufferAlignmentProperties`]
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT")]
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT = PhysicalDeviceTexelBufferAlignmentProperties;
///# [VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_texel_buffer_alignment/VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "texelBufferAlignment")]
    texel_buffer_alignment: Bool32,
}
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_texel_buffer_alignment");
