//![VK_KHR_imageless_framebuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_imageless_framebuffer.html) - device extension
//!# Description
//!This extension allows framebuffers to be created without the need for
//!creating images first, allowing more flexibility in how they are used, and
//!avoiding the need for many of the confusing compatibility rules.Framebuffers are now created
//! with a small amount of additional metadata
//!about the image views that will be used in
//![`FramebufferAttachmentsCreateInfoKHR`], and the actual image views are
//!provided at render pass begin time via
//![`RenderPassAttachmentBeginInfoKHR`].
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to
//![Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_maintenance2`]`
//! - Requires `[`VK_KHR_image_format_list`]`
//!# Contacts
//! - Tobias Hector [tobias](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_imageless_framebuffer]
//!   @tobias%0A<<Here describe the issue or question you have about the
//!   VK_KHR_imageless_framebuffer extension>>)
//!# New structures
//! - [`FramebufferAttachmentImageInfoKHR`]
//! - Extending [`FramebufferCreateInfo`]:
//! - [`FramebufferAttachmentsCreateInfoKHR`]
//!
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceImagelessFramebufferFeaturesKHR`]
//!
//! - Extending [`RenderPassBeginInfo`]:
//! - [`RenderPassAttachmentBeginInfoKHR`]
//!# New constants
//! - [`KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME`]
//! - [`KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION`]
//! - Extending [`FramebufferCreateFlagBits`]:
//! - `VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR`
//! - `VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR`
//!# Version History
//! - Revision 1, 2018-12-14 (Tobias Hector)
//! - Internal revisions
//!# Other info
//! * 2018-12-14
//!*
//! - Promoted to Vulkan 1.2 Core
//!
//!*
//! - Tobias Hector
//! - Graham Wihlidal
//!# Related
//! - [`FramebufferAttachmentImageInfoKHR`]
//! - [`FramebufferAttachmentsCreateInfoKHR`]
//! - [`PhysicalDeviceImagelessFramebufferFeaturesKHR`]
//! - [`RenderPassAttachmentBeginInfoKHR`]
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
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION")]
pub const KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME")]
pub const KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_imageless_framebuffer");
