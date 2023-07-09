pub use crate::common::extensions::khr_device_group_creation::{
    KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME, KHR_DEVICE_GROUP_CREATION_SPEC_VERSION, MAX_DEVICE_GROUP_SIZE_KHR,
};
use crate::vulkan1_1::{DeviceGroupDeviceCreateInfo, PhysicalDeviceGroupProperties};
#[doc(alias = "VkPhysicalDeviceGroupPropertiesKHR")]
pub type PhysicalDeviceGroupPropertiesKHR = PhysicalDeviceGroupProperties;
#[doc(alias = "VkDeviceGroupDeviceCreateInfoKHR")]
pub type DeviceGroupDeviceCreateInfoKHR = DeviceGroupDeviceCreateInfo;
