pub use crate::common::extensions::ext_buffer_device_address::{
    EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME, EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceAddress, StructureType},
    vulkan1_2::BufferDeviceAddressInfo,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceBufferAddressFeaturesEXT")]
pub type PhysicalDeviceBufferAddressFeaturesEXT = PhysicalDeviceBufferDeviceAddressFeaturesEXT;
#[doc(alias = "VkBufferDeviceAddressInfoEXT")]
pub type BufferDeviceAddressInfoEXT = BufferDeviceAddressInfo;
#[doc(alias = "VkPhysicalDeviceBufferDeviceAddressFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    #[doc(alias = "bufferDeviceAddress")]
    pub buffer_device_address: bool,
    #[doc(alias = "bufferDeviceAddressCaptureReplay")]
    pub buffer_device_address_capture_replay: bool,
    #[doc(alias = "bufferDeviceAddressMultiDevice")]
    pub buffer_device_address_multi_device: bool,
}
impl PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    ///Get a reference to the `buffer_device_address` field.
    pub fn buffer_device_address(&self) -> &bool {
        &self.buffer_device_address
    }
    ///Get a reference to the `buffer_device_address_capture_replay` field.
    pub fn buffer_device_address_capture_replay(&self) -> &bool {
        &self.buffer_device_address_capture_replay
    }
    ///Get a reference to the `buffer_device_address_multi_device` field.
    pub fn buffer_device_address_multi_device(&self) -> &bool {
        &self.buffer_device_address_multi_device
    }
    ///Get a mutable reference to the `buffer_device_address` field.
    pub fn buffer_device_address_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address
    }
    ///Get a mutable reference to the `buffer_device_address_capture_replay` field.
    pub fn buffer_device_address_capture_replay_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address_capture_replay
    }
    ///Get a mutable reference to the `buffer_device_address_multi_device` field.
    pub fn buffer_device_address_multi_device_mut(&mut self) -> &mut bool {
        &mut self.buffer_device_address_multi_device
    }
    ///Sets the `buffer_device_address` field.
    pub fn set_buffer_device_address(&mut self, buffer_device_address: bool) -> &mut Self {
        self.buffer_device_address = buffer_device_address;
        self
    }
    ///Sets the `buffer_device_address_capture_replay` field.
    pub fn set_buffer_device_address_capture_replay(
        &mut self,
        buffer_device_address_capture_replay: bool,
    ) -> &mut Self {
        self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
        self
    }
    ///Sets the `buffer_device_address_multi_device` field.
    pub fn set_buffer_device_address_multi_device(&mut self, buffer_device_address_multi_device: bool) -> &mut Self {
        self.buffer_device_address_multi_device = buffer_device_address_multi_device;
        self
    }
    ///Sets the `buffer_device_address` field in a builder way.
    pub fn with_buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.buffer_device_address = buffer_device_address;
        self
    }
    ///Sets the `buffer_device_address_capture_replay` field in a builder way.
    pub fn with_buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.buffer_device_address_capture_replay = buffer_device_address_capture_replay;
        self
    }
    ///Sets the `buffer_device_address_multi_device` field in a builder way.
    pub fn with_buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.buffer_device_address_multi_device = buffer_device_address_multi_device;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_buffer_device_address::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
            s_type: StructureType::PhysicalDeviceBufferDeviceAddressFeaturesExt,
            p_next: std::ptr::null_mut(),
            buffer_device_address: self.buffer_device_address.into_low_level(context, bump),
            buffer_device_address_capture_replay: self
                .buffer_device_address_capture_replay
                .into_low_level(context, bump),
            buffer_device_address_multi_device: self.buffer_device_address_multi_device.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            buffer_device_address: crate::conv::FromLowLevel::from_low_level(context, value.buffer_device_address),
            buffer_device_address_capture_replay: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_device_address_capture_replay,
            ),
            buffer_device_address_multi_device: crate::conv::FromLowLevel::from_low_level(
                context,
                value.buffer_device_address_multi_device,
            ),
        }
    }
}
#[doc(alias = "VkBufferDeviceAddressCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferDeviceAddressCreateInfoEXT {
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
}
impl BufferDeviceAddressCreateInfoEXT {
    ///Get a reference to the `device_address` field.
    pub fn device_address(&self) -> &DeviceAddress {
        &self.device_address
    }
    ///Get a mutable reference to the `device_address` field.
    pub fn device_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.device_address
    }
    ///Sets the `device_address` field.
    pub fn set_device_address(&mut self, device_address: DeviceAddress) -> &mut Self {
        self.device_address = device_address;
        self
    }
    ///Sets the `device_address` field in a builder way.
    pub fn with_device_address(mut self, device_address: DeviceAddress) -> Self {
        self.device_address = device_address;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BufferDeviceAddressCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_buffer_device_address::BufferDeviceAddressCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_buffer_device_address::BufferDeviceAddressCreateInfoEXT {
            s_type: StructureType::BufferDeviceAddressCreateInfoExt,
            p_next: std::ptr::null(),
            device_address: self.device_address.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BufferDeviceAddressCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            device_address: crate::conv::FromLowLevel::from_low_level(context, value.device_address),
        }
    }
}
