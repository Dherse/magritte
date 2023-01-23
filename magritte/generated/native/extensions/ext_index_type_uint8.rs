//!# [VK_EXT_index_type_uint8](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_index_type_uint8.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_index_type_uint8/VK_EXT_index_type_uint8.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceIndexTypeUint8FeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_index_type_uint8/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceIndexTypeUint8FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "indexTypeUint8")]
    index_type_uint8: Bool32,
}
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION")]
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME")]
pub const EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_index_type_uint8");
