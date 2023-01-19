//!# [VK_NV_dedicated_allocation_image_aliasing](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_dedicated_allocation_image_aliasing.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_dedicated_allocation_image_aliasing/VK_NV_dedicated_allocation_image_aliasing.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_dedicated_allocation_image_aliasing/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "dedicatedAllocationImageAliasing")]
    dedicated_allocation_image_aliasing: Bool32,
}
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME: &'static CStr =
    cstr!("VK_NV_dedicated_allocation_image_aliasing");
