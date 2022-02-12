//![VK_ARM_rasterization_order_attachment_access](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ARM_rasterization_order_attachment_access.html) - device extension
//!# Description
//!Renderpasses, and specifically
//![subpass
//!self-dependencies](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-barriers-subpass-self-dependencies) enable much of the same functionality as the framebuffer
//!fetch and pixel local storage extensions did for OpenGL ES.
//!But certain techniques such as programmable blending are awkward or
//!impractical to implement with these alone, in part because a self-dependency
//!is required every time a fragment will read a value at a given sample
//!coordinate.This extension extends the mechanism of input attachments to allow access to
//!framebuffer attachments when used as both input and color, or depth/stencil,
//!attachments from one fragment to the next, in rasterization order, without
//!explicit synchronization.See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more
//!information.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Jan-Harald Fredriksen [janharaldfredriksen-arm](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_ARM_rasterization_order_attachment_access]
//!   @janharaldfredriksen-arm%0A<<Here describe the issue or question you have about the
//!   VK_ARM_rasterization_order_attachment_access extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
//!# New enums
//! - [`PipelineColorBlendStateCreateFlagBits`]
//! - [`PipelineDepthStencilStateCreateFlagBits`]
//!# New constants
//! - [`ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME`]
//! - [`ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION`]
//! - Extending [`PipelineColorBlendStateCreateFlagBits`]:
//! - `VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM`
//! - Extending [`PipelineDepthStencilStateCreateFlagBits`]:
//! - `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`
//! - `VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM`
//! - Extending [`SubpassDescriptionFlagBits`]:
//! - `VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM`
//! - `VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM`
//! - `VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM`
//!# Known issues & F.A.Q
//!1) Is there any interaction with the `[`VK_KHR_dynamic_rendering`]`
//!extension?No.
//!This extension only affects reads from input attachments.
//!Render pass instances begun with [`CmdBeginRenderingKHR`] do not have
//!input attachments and a different mechanism will be needed to provide
//!similar functionality in this case.
//!# Version History
//! - Revision 1, 2021-11-12 (Jan-Harald Fredriksen)
//! - Initial draft
//!# Other info
//! * 2021-11-12
//! * No known IP claims.
//!*
//! - Tobias Hector, AMD
//! - Jan-Harald Fredriksen, Arm
//!# Related
//! - [`PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM`]
//! - [`PipelineColorBlendStateCreateFlagBits`]
//! - [`PipelineDepthStencilStateCreateFlagBits`]
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
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_ARM_rasterization_order_attachment_access");
