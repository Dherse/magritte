//!# [VK_KHR_maintenance3](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance3.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_maintenance3/VK_KHR_maintenance3.md")]
use crate::{
    cstr,
    vulkan1_1::{DescriptorSetLayoutSupport, FNGetDescriptorSetLayoutSupport, PhysicalDeviceMaintenance3Properties},
};
use std::ffi::CStr;
///See [`PhysicalDeviceMaintenance3Properties`]
#[doc(alias = "VkPhysicalDeviceMaintenance3PropertiesKHR")]
pub type PhysicalDeviceMaintenance3PropertiesKHR = PhysicalDeviceMaintenance3Properties;
///See [`DescriptorSetLayoutSupport`]
#[doc(alias = "VkDescriptorSetLayoutSupportKHR")]
pub type DescriptorSetLayoutSupportKHR = DescriptorSetLayoutSupport;
#[doc(alias = "VK_KHR_MAINTENANCE_3_SPEC_VERSION")]
pub const KHR_MAINTENANCE_3_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_MAINTENANCE_3_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_3_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_maintenance3");
///See [`KHR_MAINTENANCE_3_SPEC_VERSION`]
#[doc(alias = "VK_KHR_MAINTENANCE3_SPEC_VERSION")]
pub const KHR_MAINTENANCE3_SPEC_VERSION: u32 = KHR_MAINTENANCE_3_SPEC_VERSION;
///See [`KHR_MAINTENANCE_3_EXTENSION_NAME`]
#[doc(alias = "VK_KHR_MAINTENANCE3_EXTENSION_NAME")]
pub const KHR_MAINTENANCE3_EXTENSION_NAME: &'static CStr = KHR_MAINTENANCE_3_EXTENSION_NAME;
///See [`get_descriptor_set_layout_support`]
#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
pub type FNGetDescriptorSetLayoutSupportKhr = FNGetDescriptorSetLayoutSupport;
