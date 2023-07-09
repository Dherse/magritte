use crate::native::vulkan1_1::{
    DeviceGroupDeviceCreateInfo, FNEnumeratePhysicalDeviceGroups, PhysicalDeviceGroupProperties,
};
///See [`PhysicalDeviceGroupProperties`]
#[doc(alias = "VkPhysicalDeviceGroupPropertiesKHR")]
pub type PhysicalDeviceGroupPropertiesKHR = PhysicalDeviceGroupProperties;
///See [`DeviceGroupDeviceCreateInfo`]
#[doc(alias = "VkDeviceGroupDeviceCreateInfoKHR")]
pub type DeviceGroupDeviceCreateInfoKHR = DeviceGroupDeviceCreateInfo;
pub use crate::common::extensions::khr_device_group_creation::{
    KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME, KHR_DEVICE_GROUP_CREATION_SPEC_VERSION, MAX_DEVICE_GROUP_SIZE_KHR,
};
///See [`enumerate_physical_device_groups`]
#[doc(alias = "vkEnumeratePhysicalDeviceGroupsKHR")]
pub type FNEnumeratePhysicalDeviceGroupsKhr = FNEnumeratePhysicalDeviceGroups;
