use crate::native::vulkan1_1::{
    FNGetPhysicalDeviceFeatures2, FNGetPhysicalDeviceFormatProperties2, FNGetPhysicalDeviceImageFormatProperties2,
    FNGetPhysicalDeviceMemoryProperties2, FNGetPhysicalDeviceProperties2, FNGetPhysicalDeviceQueueFamilyProperties2,
    FNGetPhysicalDeviceSparseImageFormatProperties2, FormatProperties2, ImageFormatProperties2,
    PhysicalDeviceFeatures2, PhysicalDeviceImageFormatInfo2, PhysicalDeviceMemoryProperties2,
    PhysicalDeviceProperties2, PhysicalDeviceSparseImageFormatInfo2, QueueFamilyProperties2,
    SparseImageFormatProperties2,
};
///See [`PhysicalDeviceFeatures2`]
#[doc(alias = "VkPhysicalDeviceFeatures2KHR")]
pub type PhysicalDeviceFeatures2KHR = PhysicalDeviceFeatures2;
///See [`PhysicalDeviceProperties2`]
#[doc(alias = "VkPhysicalDeviceProperties2KHR")]
pub type PhysicalDeviceProperties2KHR = PhysicalDeviceProperties2;
///See [`FormatProperties2`]
#[doc(alias = "VkFormatProperties2KHR")]
pub type FormatProperties2KHR = FormatProperties2;
///See [`ImageFormatProperties2`]
#[doc(alias = "VkImageFormatProperties2KHR")]
pub type ImageFormatProperties2KHR = ImageFormatProperties2;
///See [`PhysicalDeviceImageFormatInfo2`]
#[doc(alias = "VkPhysicalDeviceImageFormatInfo2KHR")]
pub type PhysicalDeviceImageFormatInfo2KHR = PhysicalDeviceImageFormatInfo2;
///See [`QueueFamilyProperties2`]
#[doc(alias = "VkQueueFamilyProperties2KHR")]
pub type QueueFamilyProperties2KHR = QueueFamilyProperties2;
///See [`PhysicalDeviceMemoryProperties2`]
#[doc(alias = "VkPhysicalDeviceMemoryProperties2KHR")]
pub type PhysicalDeviceMemoryProperties2KHR = PhysicalDeviceMemoryProperties2;
///See [`SparseImageFormatProperties2`]
#[doc(alias = "VkSparseImageFormatProperties2KHR")]
pub type SparseImageFormatProperties2KHR = SparseImageFormatProperties2;
///See [`PhysicalDeviceSparseImageFormatInfo2`]
#[doc(alias = "VkPhysicalDeviceSparseImageFormatInfo2KHR")]
pub type PhysicalDeviceSparseImageFormatInfo2KHR = PhysicalDeviceSparseImageFormatInfo2;
pub use crate::common::extensions::khr_get_physical_device_properties2::{
    KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME, KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION,
};
///See [`get_physical_device_features2`]
#[doc(alias = "vkGetPhysicalDeviceFeatures2KHR")]
pub type FNGetPhysicalDeviceFeatures2Khr = FNGetPhysicalDeviceFeatures2;
///See [`get_physical_device_properties2`]
#[doc(alias = "vkGetPhysicalDeviceProperties2KHR")]
pub type FNGetPhysicalDeviceProperties2Khr = FNGetPhysicalDeviceProperties2;
///See [`get_physical_device_format_properties2`]
#[doc(alias = "vkGetPhysicalDeviceFormatProperties2KHR")]
pub type FNGetPhysicalDeviceFormatProperties2Khr = FNGetPhysicalDeviceFormatProperties2;
///See [`get_physical_device_image_format_properties2`]
#[doc(alias = "vkGetPhysicalDeviceImageFormatProperties2KHR")]
pub type FNGetPhysicalDeviceImageFormatProperties2Khr = FNGetPhysicalDeviceImageFormatProperties2;
///See [`get_physical_device_queue_family_properties2`]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyProperties2KHR")]
pub type FNGetPhysicalDeviceQueueFamilyProperties2Khr = FNGetPhysicalDeviceQueueFamilyProperties2;
///See [`get_physical_device_memory_properties2`]
#[doc(alias = "vkGetPhysicalDeviceMemoryProperties2KHR")]
pub type FNGetPhysicalDeviceMemoryProperties2Khr = FNGetPhysicalDeviceMemoryProperties2;
///See [`get_physical_device_sparse_image_format_properties2`]
#[doc(alias = "vkGetPhysicalDeviceSparseImageFormatProperties2KHR")]
pub type FNGetPhysicalDeviceSparseImageFormatProperties2Khr = FNGetPhysicalDeviceSparseImageFormatProperties2;
