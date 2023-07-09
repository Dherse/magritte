use crate::native::{
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceAddress, StructureType},
    vulkan1_2::{BufferDeviceAddressInfo, FNGetBufferDeviceAddress},
};
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
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "bufferDeviceAddress")]
    pub buffer_device_address: Bool32,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    pub buffer_device_address_capture_replay: Bool32,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    pub buffer_device_address_multi_device: Bool32,
}
impl Default for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceBufferDeviceAddressFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            buffer_device_address: unsafe { std::mem::zeroed() },
            buffer_device_address_capture_replay: unsafe { std::mem::zeroed() },
            buffer_device_address_multi_device: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBufferDeviceAddressCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
}
impl Default for BufferDeviceAddressCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::BufferDeviceAddressCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            device_address: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_buffer_device_address::{
    EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME, EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION,
};
///See [`get_buffer_device_address`]
#[doc(alias = "vkGetBufferDeviceAddressEXT")]
pub type FNGetBufferDeviceAddressExt = FNGetBufferDeviceAddress;
