//!# [VK_EXT_ycbcr_image_arrays](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_ycbcr_image_arrays.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_ycbcr_image_arrays/VK_EXT_ycbcr_image_arrays.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceYcbcrImageArraysFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_ycbcr_image_arrays/VkPhysicalDeviceYcbcrImageArraysFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceYcbcrImageArraysFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "ycbcrImageArrays")]
    ycbcr_image_arrays: Bool32,
}
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION")]
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME")]
pub const EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_ycbcr_image_arrays");
