//![VK_KHR_get_display_properties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_display_properties2.html) - instance extension
//!# Description
//!This extension provides new entry points to query device display properties
//!and capabilities in a way that can be easily extended by other extensions,
//!without introducing any further entry points.
//!This extension can be considered the `[`VK_KHR_display`]` equivalent of
//!the `[`VK_KHR_get_physical_device_properties2`]` extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_display_properties2]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_KHR_get_display_properties2 extension>>)
//!# New functions & commands
//! - [`GetDisplayModeProperties2KHR`]
//! - [`GetDisplayPlaneCapabilities2KHR`]
//! - [`GetPhysicalDeviceDisplayPlaneProperties2KHR`]
//! - [`GetPhysicalDeviceDisplayProperties2KHR`]
//!# New structures
//! - [`DisplayModeProperties2KHR`]
//! - [`DisplayPlaneCapabilities2KHR`]
//! - [`DisplayPlaneInfo2KHR`]
//! - [`DisplayPlaneProperties2KHR`]
//! - [`DisplayProperties2KHR`]
//!# New constants
//! - [`KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME`]
//! - [`KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`
//! - `VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`
//! - `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`
//! - `VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`
//! - `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`
//!# Known issues & F.A.Q
//!1) What should this extension be named?**RESOLVED**: [`VK_KHR_get_display_properties2`].
//!Other alternatives:
//! - `VK_KHR_display2`
//! - One extension, combined with `VK_KHR_surface_capabilites2`.2) Should extensible input structs
//!   be added for these new functions:**RESOLVED**:
//! - [`GetPhysicalDeviceDisplayProperties2KHR`]: No.
//!The only current input is a [`PhysicalDevice`].
//!Other inputs would not make sense.
//! - [`GetPhysicalDeviceDisplayPlaneProperties2KHR`]: No.
//!The only current input is a [`PhysicalDevice`].
//!Other inputs would not make sense.
//! - [`GetDisplayModeProperties2KHR`]: No.
//!The only current inputs are a [`PhysicalDevice`] and a
//![`DisplayModeKHR`].
//!Other inputs would not make sense.3) Should additional display query functions be
//! extended?**RESOLVED**:
//! - [`GetDisplayPlaneSupportedDisplaysKHR`]: No.
//!Extensions should instead extend
//![`GetDisplayPlaneCapabilitiesKHR`]().
//!# Version History
//! - Revision 1, 2017-02-21 (James Jones)
//! - Initial draft.
//!# Other info
//! * 2017-02-21
//! * No known IP claims.
//!*
//! - Ian Elliott, Google
//! - James Jones, NVIDIA
//!# Related
//! - [`DisplayModeProperties2KHR`]
//! - [`DisplayPlaneCapabilities2KHR`]
//! - [`DisplayPlaneInfo2KHR`]
//! - [`DisplayPlaneProperties2KHR`]
//! - [`DisplayProperties2KHR`]
//! - [`GetDisplayModeProperties2KHR`]
//! - [`GetDisplayPlaneCapabilities2KHR`]
//! - [`GetPhysicalDeviceDisplayPlaneProperties2KHR`]
//! - [`GetPhysicalDeviceDisplayProperties2KHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_get_display_properties2");
