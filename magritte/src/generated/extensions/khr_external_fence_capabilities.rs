//![VK_KHR_external_fence_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_capabilities.html) - instance extension
//!# Description
//!An application may wish to reference device fences in multiple Vulkan
//!logical devices or instances, in multiple processes, and/or in multiple
//!APIs.
//!This extension provides a set of capability queries and handle definitions
//!that allow an application to determine what types of “external” fence
//!handles an implementation supports for a given set of use cases.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_external_fence_capabilities]
//!   @critsec%0A<<Here describe the issue or question you have about the
//!   VK_KHR_external_fence_capabilities extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceExternalFencePropertiesKHR`]
//!# New structures
//! - [`ExternalFencePropertiesKHR`]
//! - [`PhysicalDeviceExternalFenceInfoKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceIdPropertiesKHR`]
//!# New enums
//! - [`ExternalFenceFeatureFlagBitsKHR`]
//! - [`ExternalFenceHandleTypeFlagBitsKHR`]
//!# New bitmasks
//! - [`ExternalFenceFeatureFlagsKHR`]
//! - [`ExternalFenceHandleTypeFlagsKHR`]
//!# New constants
//! - [`KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME`]
//! - [`KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION`]
//! - [`LUID_SIZE_KHR`]
//! - Extending [`ExternalFenceFeatureFlagBits`]:
//! - `VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR`
//! - `VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR`
//!
//! - Extending [`ExternalFenceHandleTypeFlagBits`]:
//! - `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR`
//! - `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR`
//! - `VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR`
//! - `VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR`
//!# Version History
//! - Revision 1, 2017-05-08 (Jesse Hall)
//! - Initial version
//!# Other info
//! * 2017-05-08
//! * No known IP claims.
//!*
//! - Promoted to Vulkan 1.1 Core
//!
//!*
//! - Jesse Hall, Google
//! - James Jones, NVIDIA
//! - Jeff Juliano, NVIDIA
//! - Cass Everitt, Oculus
//! - Contributors to `[`VK_KHR_external_semaphore_capabilities`]`
//!# Related
//! - [`LUID_SIZE_KHR`]
//! - [`ExternalFenceFeatureFlagBitsKHR`]
//! - [`ExternalFenceFeatureFlagsKHR`]
//! - [`ExternalFenceHandleTypeFlagBitsKHR`]
//! - [`ExternalFenceHandleTypeFlagsKHR`]
//! - [`ExternalFencePropertiesKHR`]
//! - [`PhysicalDeviceExternalFenceInfoKHR`]
//! - [`PhysicalDeviceIdPropertiesKHR`]
//! - [`GetPhysicalDeviceExternalFencePropertiesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_1::LUID_SIZE;
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_KHR_external_fence_capabilities");
///[VK_LUID_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_LUID_SIZE.html) - Length of a locally unique device identifier
///# C Specifications
///[`LUID_SIZE`] is the length in `uint8_t` values of an array
///containing a locally unique device identifier, as returned in
///[`PhysicalDeviceIdProperties`]::deviceLUID.
///```c
///#define VK_LUID_SIZE                      8U
///```
///or the equivalent
///```c
///#define VK_LUID_SIZE_KHR                  VK_LUID_SIZE
///```
///# Related
/// - [`VK_KHR_external_fence_capabilities`]
/// - [`VK_KHR_external_memory_capabilities`]
/// - [`VK_KHR_external_semaphore_capabilities`]
/// - [`crate::vulkan1_1`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_LUID_SIZE_KHR")]
pub const LUID_SIZE_KHR: u32 = LUID_SIZE;
