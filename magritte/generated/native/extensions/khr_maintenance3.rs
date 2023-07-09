use crate::native::vulkan1_1::{
    DescriptorSetLayoutSupport, FNGetDescriptorSetLayoutSupport, PhysicalDeviceMaintenance3Properties,
};
///See [`PhysicalDeviceMaintenance3Properties`]
#[doc(alias = "VkPhysicalDeviceMaintenance3PropertiesKHR")]
pub type PhysicalDeviceMaintenance3PropertiesKHR = PhysicalDeviceMaintenance3Properties;
///See [`DescriptorSetLayoutSupport`]
#[doc(alias = "VkDescriptorSetLayoutSupportKHR")]
pub type DescriptorSetLayoutSupportKHR = DescriptorSetLayoutSupport;
pub use crate::common::extensions::khr_maintenance3::{
    KHR_MAINTENANCE3_EXTENSION_NAME, KHR_MAINTENANCE3_SPEC_VERSION, KHR_MAINTENANCE_3_EXTENSION_NAME,
    KHR_MAINTENANCE_3_SPEC_VERSION,
};
///See [`get_descriptor_set_layout_support`]
#[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
pub type FNGetDescriptorSetLayoutSupportKhr = FNGetDescriptorSetLayoutSupport;
