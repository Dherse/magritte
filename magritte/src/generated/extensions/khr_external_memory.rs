use crate::vulkan1_1::QUEUE_FAMILY_EXTERNAL;
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_external_memory");
///[VK_QUEUE_FAMILY_EXTERNAL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_QUEUE_FAMILY_EXTERNAL.html) - External queue family index sentinel
///# C Specifications
///The special queue family index [`QUEUE_FAMILY_EXTERNAL`] represents any
///queue external to the resource’s current Vulkan instance, as long as the
///queue uses the same underlying
///device group or
///physical device, and the same driver version as the resource’s
///[`Device`], as indicated by
///[`PhysicalDeviceIdProperties::device_uuid`] and
///[`PhysicalDeviceIdProperties::driver_uuid`].
///```c
///#define VK_QUEUE_FAMILY_EXTERNAL          (~1U)
///```
///or the equivalent
///```c
///#define VK_QUEUE_FAMILY_EXTERNAL_KHR      VK_QUEUE_FAMILY_EXTERNAL
///```
///# Related
/// - [`VK_KHR_external_memory`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL_KHR")]
pub const QUEUE_FAMILY_EXTERNAL_KHR: u32 = QUEUE_FAMILY_EXTERNAL;
