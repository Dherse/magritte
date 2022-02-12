//![VK_KHR_pipeline_library](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_library.html) - device extension
//!# Description
//!A pipeline library is a special pipeline that cannot be bound, instead it
//!defines a set of shaders and shader groups which can be linked into other
//!pipelines.
//!This extension defines the infrastructure for pipeline libraries, but does
//!not specify the creation or usage of pipeline libraries.
//!This is left to additional dependent extensions.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Christoph Kubisch [pixeljetstream](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_pipeline_library]
//!   @pixeljetstream%0A<<Here describe the issue or question you have about the
//!   VK_KHR_pipeline_library extension>>)
//!# New structures
//! - [`PipelineLibraryCreateInfoKHR`]
//!# New constants
//! - [`KHR_PIPELINE_LIBRARY_EXTENSION_NAME`]
//! - [`KHR_PIPELINE_LIBRARY_SPEC_VERSION`]
//! - Extending [`PipelineCreateFlagBits`]:
//! - `VK_PIPELINE_CREATE_LIBRARY_BIT_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR`
//!# Version History
//! - Revision 1, 2020-01-08 (Christoph Kubisch)
//! - Initial draft.
//!# Other info
//! * 2020-01-08
//! * No known IP claims.
//!*
//! - See contributors to `[`VK_KHR_ray_tracing_pipeline`]`
//!# Related
//! - [`PipelineLibraryCreateInfoKHR`]
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
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION")]
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME")]
pub const KHR_PIPELINE_LIBRARY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_pipeline_library");
