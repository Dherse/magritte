use crate::native::vulkan1_2::{
    BufferDeviceAddressInfo, BufferOpaqueCaptureAddressCreateInfo, DeviceMemoryOpaqueCaptureAddressInfo,
    FNGetBufferDeviceAddress, FNGetBufferOpaqueCaptureAddress, FNGetDeviceMemoryOpaqueCaptureAddress,
    MemoryOpaqueCaptureAddressAllocateInfo, PhysicalDeviceBufferDeviceAddressFeatures,
};
///See [`PhysicalDeviceBufferDeviceAddressFeatures`]
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesKHR")]
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR = PhysicalDeviceBufferDeviceAddressFeatures;
///See [`BufferDeviceAddressInfo`]
#[doc(alias = "VkBufferDeviceAddressInfoKHR")]
pub type BufferDeviceAddressInfoKHR = BufferDeviceAddressInfo;
///See [`BufferOpaqueCaptureAddressCreateInfo`]
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfoKHR")]
pub type BufferOpaqueCaptureAddressCreateInfoKHR = BufferOpaqueCaptureAddressCreateInfo;
///See [`MemoryOpaqueCaptureAddressAllocateInfo`]
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfoKHR")]
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR = MemoryOpaqueCaptureAddressAllocateInfo;
///See [`DeviceMemoryOpaqueCaptureAddressInfo`]
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfoKHR")]
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR = DeviceMemoryOpaqueCaptureAddressInfo;
pub use crate::common::extensions::khr_buffer_device_address::{
    KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME, KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION,
};
///See [`get_buffer_opaque_capture_address`]
#[doc(alias = "vkGetBufferOpaqueCaptureAddressKHR")]
pub type FNGetBufferOpaqueCaptureAddressKhr = FNGetBufferOpaqueCaptureAddress;
///See [`get_buffer_device_address`]
#[doc(alias = "vkGetBufferDeviceAddressKHR")]
pub type FNGetBufferDeviceAddressKhr = FNGetBufferDeviceAddress;
///See [`get_device_memory_opaque_capture_address`]
#[doc(alias = "vkGetDeviceMemoryOpaqueCaptureAddressKHR")]
pub type FNGetDeviceMemoryOpaqueCaptureAddressKhr = FNGetDeviceMemoryOpaqueCaptureAddress;