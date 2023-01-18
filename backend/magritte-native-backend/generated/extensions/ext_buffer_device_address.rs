use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceAddress, StructureType},
    vulkan1_2::{BufferDeviceAddressInfo, FNGetBufferDeviceAddress},
};
use std::ffi::CStr;
///See [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`]
#[doc(alias = "VkPhysicalDeviceBufferAddressFeaturesEXT")]
pub type PhysicalDeviceBufferAddressFeaturesEXT = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
///See [`BufferDeviceAddressInfo`]
#[doc(alias = "VkBufferDeviceAddressInfoEXT")]
pub type BufferDeviceAddressInfoEXT = BufferDeviceAddressInfo;
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "bufferDeviceAddress")]
    buffer_device_address: Bool32,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    buffer_device_address_capture_replay: Bool32,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    buffer_device_address_multi_device: Bool32,
}
#[doc(alias = "VkBufferDeviceAddressCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "deviceAddress")]
    device_address: DeviceAddress,
}
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_buffer_device_address");
///See [`get_buffer_device_address`]
#[doc(alias = "vkGetBufferDeviceAddressEXT")]
pub type FNGetBufferDeviceAddressExt = FNGetBufferDeviceAddress;
