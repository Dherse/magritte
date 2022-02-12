//![VK_EXT_extended_dynamic_state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_extended_dynamic_state.html) - device extension
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
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_extended_dynamic_state]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_extended_dynamic_state extension>>)
//!# New functions & commands
//! - [`CmdBindVertexBuffers2EXT`]
//! - [`CmdSetCullModeEXT`]
//! - [`CmdSetDepthBoundsTestEnableEXT`]
//! - [`CmdSetDepthCompareOpEXT`]
//! - [`CmdSetDepthTestEnableEXT`]
//! - [`CmdSetDepthWriteEnableEXT`]
//! - [`CmdSetFrontFaceEXT`]
//! - [`CmdSetPrimitiveTopologyEXT`]
//! - [`CmdSetScissorWithCountEXT`]
//! - [`CmdSetStencilOpEXT`]
//! - [`CmdSetStencilTestEnableEXT`]
//! - [`CmdSetViewportWithCountEXT`]
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`]
//!# New constants
//! - [`EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME`]
//! - [`EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION`]
//! - Extending [`DynamicState`]:
//! - `VK_DYNAMIC_STATE_CULL_MODE_EXT`
//! - `VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT`
//! - `VK_DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT`
//! - `VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT`
//! - `VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT`
//! - `VK_DYNAMIC_STATE_FRONT_FACE_EXT`
//! - `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT`
//! - `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT`
//! - `VK_DYNAMIC_STATE_STENCIL_OP_EXT`
//! - `VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT`
//! - `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT`
//! - `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT`
//!# Version History
//! - Revision 1, 2019-12-09 (Piers Daniell)
//! - Internal revisions
//!# Other info
//! * 2019-12-09
//!*
//! - Promoted to Vulkan 1.3 Core
//! * No known IP claims.
//!*
//! - Dan Ginsburg, Valve Corporation
//! - Graeme Leese, Broadcom
//! - Hans-Kristian Arntzen, Valve Corporation
//! - Jan-Harald Fredriksen, Arm Limited
//! - Jason Ekstrand, Intel
//! - Jeff Bolz, NVIDIA
//! - Jesse Hall, Google
//! - Philip Rebohle, Valve Corporation
//! - Stuart Smith, Imagination Technologies
//! - Tobias Hector, AMD
//!# Related
//! - [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`]
//! - [`CmdBindVertexBuffers2EXT`]
//! - [`CmdSetCullModeEXT`]
//! - [`CmdSetDepthBoundsTestEnableEXT`]
//! - [`CmdSetDepthCompareOpEXT`]
//! - [`CmdSetDepthTestEnableEXT`]
//! - [`CmdSetDepthWriteEnableEXT`]
//! - [`CmdSetFrontFaceEXT`]
//! - [`CmdSetPrimitiveTopologyEXT`]
//! - [`CmdSetScissorWithCountEXT`]
//! - [`CmdSetStencilOpEXT`]
//! - [`CmdSetStencilTestEnableEXT`]
//! - [`CmdSetViewportWithCountEXT`]
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
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_extended_dynamic_state");
