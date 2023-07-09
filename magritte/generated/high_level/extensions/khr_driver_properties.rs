pub use crate::common::extensions::khr_driver_properties::{
    KHR_DRIVER_PROPERTIES_EXTENSION_NAME, KHR_DRIVER_PROPERTIES_SPEC_VERSION, MAX_DRIVER_INFO_SIZE_KHR,
    MAX_DRIVER_NAME_SIZE_KHR,
};
use crate::vulkan1_2::{ConformanceVersion, DriverId, PhysicalDeviceDriverProperties};
#[doc(alias = "VkDriverIdKHR")]
pub type DriverIdKHR = DriverId;
#[doc(alias = "VkConformanceVersionKHR")]
pub type ConformanceVersionKHR = ConformanceVersion;
#[doc(alias = "VkPhysicalDeviceDriverPropertiesKHR")]
pub type PhysicalDeviceDriverPropertiesKHR = PhysicalDeviceDriverProperties;
