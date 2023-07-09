pub use crate::common::extensions::khr_get_physical_device_properties2::{
    KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME, KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION,
};
use crate::vulkan1_1::{
    FormatProperties2, ImageFormatProperties2, PhysicalDeviceFeatures2, PhysicalDeviceImageFormatInfo2,
    PhysicalDeviceMemoryProperties2, PhysicalDeviceProperties2, PhysicalDeviceSparseImageFormatInfo2,
    QueueFamilyProperties2, SparseImageFormatProperties2,
};
#[doc(alias = "VkPhysicalDeviceFeatures2KHR")]
pub type PhysicalDeviceFeatures2KHR = PhysicalDeviceFeatures2;
#[doc(alias = "VkPhysicalDeviceProperties2KHR")]
pub type PhysicalDeviceProperties2KHR = PhysicalDeviceProperties2;
#[doc(alias = "VkFormatProperties2KHR")]
pub type FormatProperties2KHR = FormatProperties2;
#[doc(alias = "VkImageFormatProperties2KHR")]
pub type ImageFormatProperties2KHR = ImageFormatProperties2;
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2KHR")]
pub type PhysicalDeviceImageFormatInfo2KHR = PhysicalDeviceImageFormatInfo2;
#[doc(alias = "VkQueueFamilyProperties2KHR")]
pub type QueueFamilyProperties2KHR = QueueFamilyProperties2;
#[doc(alias = "VkPhysicalDeviceMemoryProperties2KHR")]
pub type PhysicalDeviceMemoryProperties2KHR = PhysicalDeviceMemoryProperties2;
#[doc(alias = "VkSparseImageFormatProperties2KHR")]
pub type SparseImageFormatProperties2KHR = SparseImageFormatProperties2;
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2KHR")]
pub type PhysicalDeviceSparseImageFormatInfo2KHR = PhysicalDeviceSparseImageFormatInfo2;
