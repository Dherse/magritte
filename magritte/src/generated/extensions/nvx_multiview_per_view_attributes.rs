//![VK_NVX_multiview_per_view_attributes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NVX_multiview_per_view_attributes.html) - device extension
//!# Description
//!This extension adds a new way to write shaders to be used with multiview
//!subpasses, where the attributes for all views are written out by a single
//!invocation of the
//![pre-rasterization shader
//!stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
//!Related SPIR-V and GLSL extensions `SPV_NVX_multiview_per_view_attributes`
//!and `GL_NVX_multiview_per_view_attributes` introduce per-view position and
//!viewport mask attributes arrays, and this extension defines how those
//!per-view attribute arrays are interpreted by Vulkan.
//!Pipelines using per-view attributes **may** only execute the
//![pre-rasterization shader
//!stages](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization) once for all views rather than once per-view, which reduces
//!redundant shading work.A subpass creation flag controls whether the subpass uses this extension.
//!A subpass **must** either exclusively use this extension or not use it at all.Some Vulkan
//! implementations only support the position attribute varying
//!between views in the X component.
//!A subpass can declare via a second creation flag whether all pipelines
//!compiled for this subpass will obey this restriction.Shaders that use the new per-view outputs
//! (e.g. `gl_PositionPerViewNV`)
//!**must** also write the non-per-view output (`gl_Position`), and the values
//!written **must** be such that `gl_Position =
//!gl_PositionPerViewNV[gl_ViewIndex]` for all views in the subpass.
//!Implementations are free to either use the per-view outputs or the
//!non-per-view outputs, whichever would be more efficient.If `[`VK_NV_viewport_array2`]` is not
//! also supported and enabled, the
//!per-view viewport mask **must** not be used.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_multiview`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NVX_multiview_per_view_attributes]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the
//!   VK_NVX_multiview_per_view_attributes extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`]
//!# New constants
//! - [`NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME`]
//! - [`NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX`
//!
//! - Extending [`SubpassDescriptionFlagBits`]:
//! - `VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX`
//! - `VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX`
//!# Version History
//! - Revision 1, 2017-01-13 (Jeff Bolz)
//! - Internal revisions
//!# Other info
//! * 2017-01-13
//! * No known IP claims.
//!*
//! - This extension requires
//![`SPV_NVX_multiview_per_view_attributes`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NVX_multiview_per_view_attributes.html)
//! - This extension provides API support for
//![`GL_NVX_multiview_per_view_attributes`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nvx/GL_NVX_multiview_per_view_attributes.txt)
//! - This extension interacts with `[`VK_NV_viewport_array2`]`.
//!
//!*
//! - Jeff Bolz, NVIDIA
//! - Daniel Koch, NVIDIA
//!# Related
//! - [`PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX`]
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
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NVX_multiview_per_view_attributes");
