//![VK_EXT_depth_clip_enable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_enable.html) - device extension
//!# Description
//!This extension allows the depth clipping operation, that is normally
//!implicitly controlled by
//![`PipelineRasterizationStateCreateInfo::depth_clamp_enable`], to
//!instead be controlled explicitly by
//![`PipelineRasterizationDepthClipStateCreateInfoEXT::depth_clip_enable`].This is useful for
//! translating DX content which assumes depth clamping is
//!always enabled, but depth clip can be controlled by the DepthClipEnable
//!rasterization state (D3D12_RASTERIZER_DESC).
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_depth_clip_enable]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_depth_clip_enable extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceDepthClipEnableFeaturesEXT`]
//!
//! - Extending [`PipelineRasterizationStateCreateInfo`]:
//! - [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
//!# New bitmasks
//! - [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]
//!# New constants
//! - [`EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME`]
//! - [`EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT`
//! - `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT`
//!# Version History
//! - Revision 1, 2018-12-20 (Piers Daniell)
//! - Internal revisions
//!# Other info
//! * 2018-12-20
//!*
//! - Daniel Rakos, AMD
//! - Henri Verbeet, CodeWeavers
//! - Jeff Bolz, NVIDIA
//! - Philip Rebohle, DXVK
//! - Tobias Hector, AMD
//!# Related
//! - [`PhysicalDeviceDepthClipEnableFeaturesEXT`]
//! - [`PipelineRasterizationDepthClipStateCreateFlagsEXT`]
//! - [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
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
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_depth_clip_enable");
