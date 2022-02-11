use crate::vulkan1_1::MAX_DEVICE_GROUP_SIZE;
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_device_group_creation");
///[VK_MAX_DEVICE_GROUP_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DEVICE_GROUP_SIZE.html) - Length of a physical device handle array
///# C Specifications
///[`MAX_DEVICE_GROUP_SIZE`] is the length of an array containing
///[`PhysicalDevice`] handle values representing all physical devices in a
///group, as returned in
///[`PhysicalDeviceGroupProperties`]::physicalDevices.
///```c
///#define VK_MAX_DEVICE_GROUP_SIZE          32U
///```
///or the equivalent
///```c
///#define VK_MAX_DEVICE_GROUP_SIZE_KHR      VK_MAX_DEVICE_GROUP_SIZE
///```
///# Related
/// - [`VK_KHR_device_group_creation`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DEVICE_GROUP_SIZE_KHR")]
pub const MAX_DEVICE_GROUP_SIZE_KHR: u32 = MAX_DEVICE_GROUP_SIZE;
