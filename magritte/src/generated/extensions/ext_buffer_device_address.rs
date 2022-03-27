use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceAddress, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_buffer_device_address");
///[VkPhysicalDeviceBufferDeviceAddressFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBufferDeviceAddressFeaturesEXT.html) - Structure describing buffer address features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_buffer_device_address
///typedef struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           bufferDeviceAddress;
///    VkBool32           bufferDeviceAddressCaptureReplay;
///    VkBool32           bufferDeviceAddressMultiDevice;
///} VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
///```
///
///```c
///// Provided by VK_EXT_buffer_device_address
///typedef VkPhysicalDeviceBufferDeviceAddressFeaturesEXT VkPhysicalDeviceBufferAddressFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`buffer_device_address`] indicates that the implementation supports accessing buffer memory
///   in shaders as storage buffers via an address queried from [`GetBufferDeviceAddressEXT`].
/// - [`buffer_device_address_capture_replay`] indicates that the implementation supports saving and
///   reusing buffer addresses, e.g. for trace capture and replay.
/// - [`buffer_device_address_multi_device`] indicates that the implementation supports the
///   [`buffer_device_address`] feature for logical devices created with multiple physical devices.
///   If this feature is not supported, buffer addresses **must** not be queried on a logical device
///   created with more than one physical device.
///If the [`PhysicalDeviceBufferDeviceAddressFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceBufferDeviceAddressFeaturesEXT`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT`
///# Related
/// - [`VK_EXT_buffer_device_address`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`buffer_device_address`] indicates
    ///that the implementation supports accessing buffer memory in shaders as
    ///storage buffers via an address queried from
    ///[`GetBufferDeviceAddressEXT`].
    buffer_device_address: Bool32,
    ///[`buffer_device_address_capture_replay`] indicates that the implementation
    ///supports saving and reusing buffer addresses, e.g. for trace capture and
    ///replay.
    buffer_device_address_capture_replay: Bool32,
    ///[`buffer_device_address_multi_device`] indicates that the implementation
    ///supports the [`buffer_device_address`] feature for logical devices
    ///created with multiple physical devices.
    ///If this feature is not supported, buffer addresses **must** not be queried
    ///on a logical device created with more than one physical device.
    buffer_device_address_multi_device: Bool32,
}
///[VkBufferDeviceAddressCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferDeviceAddressCreateInfoEXT.html) - Request a specific address for a buffer
///# C Specifications
///Alternatively, to
///request a specific device address for a buffer, add a
///[`BufferDeviceAddressCreateInfoEXT`] structure to the [`p_next`] chain
///of the [`BufferCreateInfo`] structure.
///The [`BufferDeviceAddressCreateInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_buffer_device_address
///typedef struct VkBufferDeviceAddressCreateInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkDeviceAddress    deviceAddress;
///} VkBufferDeviceAddressCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`device_address`] is the device address requested for the buffer.
///# Description
///If [`device_address`] is zero, no specific address is requested.If [`device_address`] is not
/// zero, then it **must** be an address retrieved
///from an identically created buffer on the same implementation.
///The buffer **must** also be bound to an identically created
///[`DeviceMemory`] object.If this structure is not present, it is as if [`device_address`] is
/// zero.Apps **should** avoid creating buffers with app-provided addresses and
///implementation-provided addresses in the same process, to reduce the
///likelihood of `VK_ERROR_INVALID_DEVICE_ADDRESS_EXT` errors.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT`
///# Related
/// - [`VK_EXT_buffer_device_address`]
/// - [`DeviceAddress`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct BufferDeviceAddressCreateInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`device_address`] is the device address requested for the buffer.
    device_address: DeviceAddress,
}
