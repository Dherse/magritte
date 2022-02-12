//![VK_NV_external_memory_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_capabilities.html) - instance extension
//!# Description
//!Applications may wish to import memory from the Direct 3D API, or export
//!memory to other Vulkan instances.
//!This extension provides a set of capability queries that allow applications
//!determine what types of win32 memory handles an implementation supports for
//!a given set of use cases.
//!# Revision
//!1
//!# Dependencies
//! - *Deprecated* by
//!`[`VK_KHR_external_memory_capabilities`]`
//!extension
//! - Which in turn was *promoted* to
//![Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory_capabilities]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_NV_external_memory_capabilities extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
//!# New structures
//! - [`ExternalImageFormatPropertiesNV`]
//!# New enums
//! - [`ExternalMemoryFeatureFlagBitsNV`]
//! - [`ExternalMemoryHandleTypeFlagBitsNV`]
//!# New bitmasks
//! - [`ExternalMemoryFeatureFlagsNV`]
//! - [`ExternalMemoryHandleTypeFlagsNV`]
//!# New constants
//! - [`NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME`]
//! - [`NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) Why do so many external memory capabilities need to be queried on a
//!per-memory-handle-type basis?**RESOLVED**: This is because some handle types are based on
//! OS-native objects
//!that have far more limited capabilities than the very generic Vulkan memory
//!objects.
//!Not all memory handle types can name memory objects that support 3D images,
//!for example.
//!Some handle types cannot even support the deferred image and memory binding
//!behavior of Vulkan and require specifying the image when allocating or
//!importing the memory object.2) Does the [`ExternalImageFormatPropertiesNV`] struct need to
//! include a
//!list of memory type bits that support the given handle type?**RESOLVED**: No.
//!The memory types that do not support the handle types will simply be
//!filtered out of the results returned by [`GetImageMemoryRequirements`]
//!when a set of handle types was specified at image creation time.3) Should the non-opaque handle
//! types be moved to their own extension?**RESOLVED**: Perhaps.
//!However, defining the handle type bits does very little and does not require
//!any platform-specific types on its own, and it is easier to maintain the
//!bitmask values in a single extension for now.
//!Presumably more handle types could be added by separate extensions though,
//!and it would be midly weird to have some platform-specific ones defined in
//!the core spec and some in extensions
//!# Version History
//! - Revision 1, 2016-08-19 (James Jones)
//! - Initial version
//!# Other info
//! * 2016-08-19
//! * No known IP claims.
//!*
//! - Interacts with Vulkan 1.1.
//! - Interacts with `[`VK_KHR_dedicated_allocation`]`.
//! - Interacts with `[`VK_NV_dedicated_allocation`]`.
//!
//!*
//! - James Jones, NVIDIA
//!# Related
//! - [`ExternalImageFormatPropertiesNV`]
//! - [`ExternalMemoryFeatureFlagBitsNV`]
//! - [`ExternalMemoryFeatureFlagsNV`]
//! - [`ExternalMemoryHandleTypeFlagBitsNV`]
//! - [`ExternalMemoryHandleTypeFlagsNV`]
//! - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
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
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_external_memory_capabilities");
