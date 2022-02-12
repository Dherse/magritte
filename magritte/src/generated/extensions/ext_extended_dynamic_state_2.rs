//![VK_EXT_extended_dynamic_state2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state2.html) - device extension
//!# Description
//!This extension adds some more dynamic state to support applications that
//!need to reduce the number of pipeline state objects they compile and bind.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Vikram Kushwaha [vkushwaha-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_extended_dynamic_state2]
//!   @vkushwaha-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_extended_dynamic_state2 extension>>)
//!# New functions & commands
//! - [`CmdSetDepthBiasEnableEXT`]
//! - [`CmdSetLogicOpEXT`]
//! - [`CmdSetPatchControlPointsEXT`]
//! - [`CmdSetPrimitiveRestartEnableEXT`]
//! - [`CmdSetRasterizerDiscardEnableEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`]
//!# New constants
//! - [`EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME`]
//! - [`EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION`]
//! - Extending [`DynamicState`]:
//! - `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE_EXT`
//! - `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
//! - `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT`
//! - `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT`
//! - `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE_EXT`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2021-04-12 (Vikram Kushwaha)
//! - Internal revisions
//!# Other info
//! * 2021-04-12
//!*
//! - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//!*
//! - Vikram Kushwaha, NVIDIA
//! - Piers Daniell, NVIDIA
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceExtendedDynamicState2FeaturesEXT`]
//! - [`CmdSetDepthBiasEnableEXT`]
//! - [`CmdSetLogicOpEXT`]
//! - [`CmdSetPatchControlPointsEXT`]
//! - [`CmdSetPrimitiveRestartEnableEXT`]
//! - [`CmdSetRasterizerDiscardEnableEXT`]
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
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_extended_dynamic_state2");
