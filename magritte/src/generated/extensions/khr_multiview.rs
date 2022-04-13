//![VK_KHR_multiview](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_multiview.html) - device extension
//!# Description
//!This extension has the same goal as the OpenGL ES `GL_OVR_multiview`
//!extension.
//!Multiview is a rendering technique originally designed for VR where it is
//!more efficient to record a single set of commands to be executed with
//!slightly different behavior for each “view”.It includes a concise way to declare a render pass
//! with multiple views, and
//!gives implementations freedom to render the views in the most efficient way
//!possible.
//!This is done with a multiview configuration specified during [render pass](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass) creation with the [`RenderPassMultiviewCreateInfo`] passed
//!into [`RenderPassCreateInfo::p_next`].This extension enables the use of the
//![`SPV_KHR_multiview`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_multiview.html) shader extension,
//!which adds a new `ViewIndex` built-in type that allows shaders to control
//!what to do for each view.
//!If using GLSL there is also the
//![`GL_EXT_multiview`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_multiview.txt) extension that
//!introduces a `highp int gl_ViewIndex;` built-in variable for vertex,
//!tessellation, geometry, and fragment shaders.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_multiview]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_multiview
//!   extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceMultiviewFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceMultiviewPropertiesKHR`]
//! - Extending [`RenderPassCreateInfo`]:  - [`RenderPassMultiviewCreateInfoKHR`]
//!# New constants
//! - [`KHR_MULTIVIEW_EXTENSION_NAME`]
//! - [`KHR_MULTIVIEW_SPEC_VERSION`]
//! - Extending [`DependencyFlagBits`]:  - `VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR`
//!# Version History
//! - Revision 1, 2016-10-28 (Jeff Bolz)  - Internal revisions
//!# Other info
//! * 2016-10-28
//! * No known IP claims.
//! * - Promoted to Vulkan 1.1 Core  - This extension requires [`SPV_KHR_multiview`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_multiview.html)
//!   - This extension provides API support for [`GL_EXT_multiview`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GL_EXT_multiview.txt)
//! * - Jeff Bolz, NVIDIA
//!# Related
//! - [`PhysicalDeviceMultiviewFeaturesKHR`]
//! - [`PhysicalDeviceMultiviewPropertiesKHR`]
//! - [`RenderPassMultiviewCreateInfoKHR`]
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
#[doc(alias = "VK_KHR_MULTIVIEW_SPEC_VERSION")]
pub const KHR_MULTIVIEW_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_MULTIVIEW_EXTENSION_NAME")]
pub const KHR_MULTIVIEW_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_multiview");
