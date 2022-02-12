//![VK_KHR_display_swapchain](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_display_swapchain.html) - device extension
//!# Description
//!This extension provides an API to create a swapchain directly on a device’s
//!display without any underlying window system.
//!# Revision
//!10
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//! - Requires `[`VK_KHR_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_display_swapchain]
//!   @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_display_swapchain
//!   extension>>)
//!# New functions & commands
//! - [`CreateSharedSwapchainsKHR`]
//!# New structures
//! - Extending [`PresentInfoKHR`]:
//! - [`DisplayPresentInfoKHR`]
//!# New constants
//! - [`KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME`]
//! - [`KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION`]
//! - Extending [`VulkanResultCodes`]:
//! - `VK_ERROR_INCOMPATIBLE_DISPLAY_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Should swapchains sharing images each hold a reference to the images, or
//!should it be up to the application to destroy the swapchains and images in
//!an order that avoids the need for reference counting?**RESOLVED**: Take a reference.
//!The lifetime of presentable images is already complex enough.2) Should the `srcRect` and
//! `dstRect` parameters be specified as
//!part of the presentation command, or at swapchain creation time?**RESOLVED**: As part of the
//! presentation command.
//!This allows moving and scaling the image on the screen without the need to
//!respecify the mode or create a new swapchain and presentable images.3) Should `srcRect` and
//! `dstRect` be specified as rects, or separate
//!offset/extent values?**RESOLVED**: As rects.
//!Specifying them separately might make it easier for hardware to expose
//!support for one but not the other, but in such cases applications must just
//!take care to obey the reported capabilities and not use non-zero offsets or
//!extents that require scaling, as appropriate.4) How can applications create multiple swapchains
//! that use the same images?**RESOLVED**: By calling [`CreateSharedSwapchainsKHR`].An earlier
//! resolution used [`CreateSwapchainKHR`], chaining multiple
//![`SwapchainCreateInfoKHR`] structures through `pNext`.
//!In order to allow each swapchain to also allow other extension structs, a
//!level of indirection was used: [`SwapchainCreateInfoKHR::p_next`]
//!pointed to a different structure, which had both `sType` and `pNext`
//!members for additional extensions, and also had a pointer to the next
//![`SwapchainCreateInfoKHR`] structure.
//!The number of swapchains to be created could only be found by walking this
//!linked list of alternating structures, and the `pSwapchains` out
//!parameter was reinterpreted to be an array of [`SwapchainKHR`] handles.Another option considered
//! was a method to specify a “shared” swapchain
//!when creating a new swapchain, such that groups of swapchains using the same
//!images could be built up one at a time.
//!This was deemed unusable because drivers need to know all of the displays an
//!image will be used on when determining which internal formats and layouts to
//!use for that image.
//!# Version History
//! - Revision 1, 2015-07-29 (James Jones)
//! - Initial draft
//!
//! - Revision 2, 2015-08-21 (Ian Elliott)
//! - Renamed this extension and all of its enumerations, types, functions,
//!etc.
//!This makes it compliant with the proposed standard for Vulkan
//!extensions.
//! - Switched from “revision” to “version”, including use of the
//!VK_MAKE_VERSION macro in the header file.
//!
//! - Revision 3, 2015-09-01 (James Jones)
//! - Restore single-field revision number.
//!
//! - Revision 4, 2015-09-08 (James Jones)
//! - Allow creating multiple swap chains that share the same images using a
//!single call to vkCreateSwapchainKHR().
//!
//! - Revision 5, 2015-09-10 (Alon Or-bach)
//! - Removed underscores from SWAP_CHAIN in two enums.
//!
//! - Revision 6, 2015-10-02 (James Jones)
//! - Added support for smart panels/buffered displays.
//!
//! - Revision 7, 2015-10-26 (Ian Elliott)
//! - Renamed from VK_EXT_KHR_display_swapchain to VK_KHR_display_swapchain.
//!
//! - Revision 8, 2015-11-03 (Daniel Rakos)
//! - Updated sample code based on the changes to VK_KHR_swapchain.
//!
//! - Revision 9, 2015-11-10 (Jesse Hall)
//! - Replaced VkDisplaySwapchainCreateInfoKHR with
//!vkCreateSharedSwapchainsKHR, changing resolution of issue #4.
//!
//! - Revision 10, 2017-03-13 (James Jones)
//! - Closed all remaining issues.
//!The specification and implementations have been shipping with the
//!proposed resolutions for some time now.
//! - Removed the sample code and noted it has been integrated into the
//!official Vulkan SDK cube demo.
//!# Other info
//! * 2017-03-13
//! * No known IP claims.
//!*
//! - James Jones, NVIDIA
//! - Jeff Vigil, Qualcomm
//! - Jesse Hall, Google
//!# Related
//! - [`DisplayPresentInfoKHR`]
//! - [`CreateSharedSwapchainsKHR`]
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
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_display_swapchain");
