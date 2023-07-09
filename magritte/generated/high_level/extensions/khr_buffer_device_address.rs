pub use crate::common::extensions::khr_buffer_device_address::{
    KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME, KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION,
};
use crate::vulkan1_2::{
    BufferDeviceAddressInfo, BufferOpaqueCaptureAddressCreateInfo, DeviceMemoryOpaqueCaptureAddressInfo,
    MemoryOpaqueCaptureAddressAllocateInfo, PhysicalDeviceBufferDeviceAddressFeatures,
};
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesKHR")]
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR = PhysicalDeviceBufferDeviceAddressFeatures;
#[doc(alias = "VkBufferDeviceAddressInfoKHR")]
pub type BufferDeviceAddressInfoKHR = BufferDeviceAddressInfo;
#[doc(alias = "VkBufferOpaqueCaptureAddressCreateInfoKHR")]
pub type BufferOpaqueCaptureAddressCreateInfoKHR = BufferOpaqueCaptureAddressCreateInfo;
#[doc(alias = "VkMemoryOpaqueCaptureAddressAllocateInfoKHR")]
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR = MemoryOpaqueCaptureAddressAllocateInfo;
#[doc(alias = "VkDeviceMemoryOpaqueCaptureAddressInfoKHR")]
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR = DeviceMemoryOpaqueCaptureAddressInfo;
