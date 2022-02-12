//![VK_KHR_driver_properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_driver_properties.html) - device extension
//!# Description
//!This extension provides a new physical device query which allows retrieving
//!information about the driver implementation, allowing applications to
//!determine which physical device corresponds to which particular vendor’s
//!driver, and which conformance test suite version the driver implementation
//!is compliant with.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_driver_properties]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the
//!   VK_KHR_driver_properties extension>>)
//!# New structures
//! - [`ConformanceVersionKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceDriverPropertiesKHR`]
//!# New enums
//! - [`DriverIdKHR`]
//!# New constants
//! - [`KHR_DRIVER_PROPERTIES_EXTENSION_NAME`]
//! - [`KHR_DRIVER_PROPERTIES_SPEC_VERSION`]
//! - [`MAX_DRIVER_INFO_SIZE_KHR`]
//! - [`MAX_DRIVER_NAME_SIZE_KHR`]
//! - Extending [`DriverId`]:
//! - `VK_DRIVER_ID_AMD_OPEN_SOURCE_KHR`
//! - `VK_DRIVER_ID_AMD_PROPRIETARY_KHR`
//! - `VK_DRIVER_ID_ARM_PROPRIETARY_KHR`
//! - `VK_DRIVER_ID_BROADCOM_PROPRIETARY_KHR`
//! - `VK_DRIVER_ID_GGP_PROPRIETARY_KHR`
//! - `VK_DRIVER_ID_GOOGLE_SWIFTSHADER_KHR`
//! - `VK_DRIVER_ID_IMAGINATION_PROPRIETARY_KHR`
//! - `VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR`
//! - `VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR`
//! - `VK_DRIVER_ID_MESA_RADV_KHR`
//! - `VK_DRIVER_ID_NVIDIA_PROPRIETARY_KHR`
//! - `VK_DRIVER_ID_QUALCOMM_PROPRIETARY_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR`
//!# Version History
//! - Revision 1, 2018-04-11 (Daniel Rakos)
//! - Internal revisions
//!# Other info
//! * 2018-04-11
//!*
//! - Promoted to Vulkan 1.2 Core
//! * No known IP claims.
//!*
//! - Baldur Karlsson
//! - Matthaeus G. Chajdas, AMD
//! - Piers Daniell, NVIDIA
//! - Alexander Galazin, Arm
//! - Jesse Hall, Google
//! - Daniel Rakos, AMD
//!# Related
//! - [`MAX_DRIVER_INFO_SIZE_KHR`]
//! - [`MAX_DRIVER_NAME_SIZE_KHR`]
//! - [`ConformanceVersionKHR`]
//! - [`DriverIdKHR`]
//! - [`PhysicalDeviceDriverPropertiesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_2::{MAX_DRIVER_INFO_SIZE, MAX_DRIVER_NAME_SIZE};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION")]
pub const KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME")]
pub const KHR_DRIVER_PROPERTIES_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_driver_properties");
///[VK_MAX_DRIVER_NAME_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_NAME_SIZE.html) - Maximum length of a physical device driver name string
///# C Specifications
///[`MAX_DRIVER_NAME_SIZE`] is the length in `char` values of an array
///containing a driver name string, as returned in
///[`PhysicalDeviceDriverProperties`]::driverName.
///```c
///#define VK_MAX_DRIVER_NAME_SIZE           256U
///```
///or the equivalent
///```c
///#define VK_MAX_DRIVER_NAME_SIZE_KHR       VK_MAX_DRIVER_NAME_SIZE
///```
///# Related
/// - [`VK_KHR_driver_properties`]
/// - [`crate::vulkan1_2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DRIVER_NAME_SIZE_KHR")]
pub const MAX_DRIVER_NAME_SIZE_KHR: u32 = MAX_DRIVER_NAME_SIZE;
///[VK_MAX_DRIVER_INFO_SIZE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MAX_DRIVER_INFO_SIZE.html) - Length of a physical device driver information string
///# C Specifications
///[`MAX_DRIVER_INFO_SIZE`] is the length in `char` values of an array
///containing a driver information string, as returned in
///[`PhysicalDeviceDriverProperties`]::driverInfo.
///```c
///#define VK_MAX_DRIVER_INFO_SIZE           256U
///```
///or the equivalent
///```c
///#define VK_MAX_DRIVER_INFO_SIZE_KHR       VK_MAX_DRIVER_INFO_SIZE
///```
///# Related
/// - [`VK_KHR_driver_properties`]
/// - [`crate::vulkan1_2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VK_MAX_DRIVER_INFO_SIZE_KHR")]
pub const MAX_DRIVER_INFO_SIZE_KHR: u32 = MAX_DRIVER_INFO_SIZE;
