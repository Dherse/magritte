pub use crate::common::extensions::khr_maintenance3::{
    KHR_MAINTENANCE3_EXTENSION_NAME, KHR_MAINTENANCE3_SPEC_VERSION, KHR_MAINTENANCE_3_EXTENSION_NAME,
    KHR_MAINTENANCE_3_SPEC_VERSION,
};
use crate::vulkan1_1::{DescriptorSetLayoutSupport, PhysicalDeviceMaintenance3Properties};
#[doc(alias = "VkPhysicalDeviceMaintenance3PropertiesKHR")]
pub type PhysicalDeviceMaintenance3PropertiesKHR = PhysicalDeviceMaintenance3Properties;
#[doc(alias = "VkDescriptorSetLayoutSupportKHR")]
pub type DescriptorSetLayoutSupportKHR = DescriptorSetLayoutSupport;
