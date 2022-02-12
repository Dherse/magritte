//![VK_EXT_color_write_enable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_color_write_enable.html) - device extension
//!# Description
//!This extension allows for selectively enabling and disabling writes to
//!output color attachments via a pipeline dynamic state.The intended use cases for this new state
//! are mostly identical to those of
//!colorWriteMask, such as selectively disabling writes to avoid feedback loops
//!between subpasses or bandwidth savings for unused outputs.
//!By making the state dynamic, one additional benefit is the ability to reduce
//!pipeline counts and pipeline switching via shaders that write a superset of
//!the desired data of which subsets are selected dynamically.
//!The reason for a new state, colorWriteEnable, rather than making
//!colorWriteMask dynamic is that, on many implementations, the more flexible
//!per-component semantics of the colorWriteMask state cannot be made dynamic
//!in a performant manner.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Sharif Elcott [selcott](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_color_write_enable]
//!   @selcott%0A<<Here describe the issue or question you have about the VK_EXT_color_write_enable
//!   extension>>)
//!# New functions & commands
//! - [`CmdSetColorWriteEnableEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceColorWriteEnableFeaturesEXT`]
//! - Extending [`PipelineColorBlendStateCreateInfo`]:
//! - [`PipelineColorWriteCreateInfoEXT`]
//!# New constants
//! - [`EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME`]
//! - [`EXT_COLOR_WRITE_ENABLE_SPEC_VERSION`]
//! - Extending [`DynamicState`]:
//! - `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT`
//! - `VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2020-01-25 (Sharif Elcott)
//! - Internal revisions
//!# Other info
//! * 2020-02-25
//! * No known IP claims.
//!*
//! - Sharif Elcott, Google
//! - Tobias Hector, AMD
//! - Piers Daniell, NVIDIA
//!# Related
//! - [`PhysicalDeviceColorWriteEnableFeaturesEXT`]
//! - [`PipelineColorWriteCreateInfoEXT`]
//! - [`CmdSetColorWriteEnableEXT`]
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
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION")]
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME")]
pub const EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_color_write_enable");
