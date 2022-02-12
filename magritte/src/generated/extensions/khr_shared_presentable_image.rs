//![VK_KHR_shared_presentable_image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shared_presentable_image.html) - device extension
//!# Description
//!This extension extends `[`VK_KHR_swapchain`]` to enable creation of a
//!shared presentable image.
//!This allows the application to use the image while the presention engine is
//!accessing it, in order to reduce the latency between rendering and
//!presentation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//! - Requires `[`VK_KHR_get_surface_capabilities2`]`
//!# Contacts
//! - Alon Or-bach [alonorbach](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shared_presentable_image]
//!   @alonorbach%0A<<Here describe the issue or question you have about the
//!   VK_KHR_shared_presentable_image extension>>)
//!# New functions & commands
//! - [`GetSwapchainStatusKHR`]
//!# New structures
//! - Extending [`SurfaceCapabilities2KHR`]:
//! - [`SharedPresentSurfaceCapabilitiesKHR`]
//!# New constants
//! - [`KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME`]
//! - [`KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION`]
//! - Extending [`ImageLayout`]:
//! - `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
//!
//! - Extending [`PresentModeKHR`]:
//! - `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`
//! - `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR`
//!
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR`
//!# Known issues & F.A.Q
//!1) Should we allow a Vulkan WSI swapchain to toggle between normal usage and
//!shared presentation usage?**RESOLVED**: No.
//!WSI swapchains are typically recreated with new properties instead of having
//!their properties changed.
//!This can also save resources, assuming that fewer images are needed for
//!shared presentation, and assuming that most VR applications do not need to
//!switch between normal and shared usage.2) Should we have a query for determining how the
//! presentation engine
//!refresh is triggered?**RESOLVED**: Yes.
//!This is done via which presentation modes a surface supports.3) Should the object representing a
//! shared presentable image be an extension
//!of a [`SwapchainKHR`] or a separate object?**RESOLVED**: Extension of a swapchain due to overlap
//! in creation properties
//!and to allow common functionality between shared and normal presentable
//!images and swapchains.4) What should we call the extension and the new structures it
//! creates?**RESOLVED**: Shared presentable image / shared present.5) Should the `minImageCount`
//! and `presentMode` values of the
//![`SwapchainCreateInfoKHR`] be ignored, or required to be compatible
//!values?**RESOLVED**: `minImageCount` must be set to 1, and `presentMode`
//!should be set to either `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
//!`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`.6) What should the layout of the shared
//! presentable image be?**RESOLVED**: After acquiring the shared presentable image, the application
//!must transition it to the `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` layout
//!prior to it being used.
//!After this initial transition, any image usage that was requested during
//!swapchain creation **can** be performed on the image without layout transitions
//!being performed.7) Do we need a new API for the trigger to refresh new content?**RESOLVED**:
//! [`QueuePresentKHR`] to act as API to trigger a refresh, as
//!will allow combination with other compatible extensions to
//![`QueuePresentKHR`].8) How should an application detect a `VK_ERROR_OUT_OF_DATE_KHR` error
//!on a swapchain using the `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`
//!present mode?**RESOLVED**: Introduce [`GetSwapchainStatusKHR`] to allow applications to
//!query the status of a swapchain using a shared presentation mode.9) What should subsequent calls
//! to [`QueuePresentKHR`] for
//!`VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR` swapchains be defined to
//!do?**RESOLVED**: State that implementations may use it as a hint for updated
//!content.10) Can the ownership of a shared presentable image be transferred to a
//!different queue?**RESOLVED**: No.
//!It is not possible to transfer ownership of a shared presentable image
//!obtained from a swapchain created using `VK_SHARING_MODE_EXCLUSIVE`
//!after it has been presented.11) How should [`QueueSubmit`] behave if a command buffer uses an
//! image
//!from a `VK_ERROR_OUT_OF_DATE_KHR` swapchain?**RESOLVED**: [`QueueSubmit`] is expected to return
//! the
//!`VK_ERROR_DEVICE_LOST` error.12) Can Vulkan provide any guarantee on the order of rendering, to
//! enable
//!beam chasing?**RESOLVED**: This could be achieved via use of render passes to ensure strip
//!rendering.
//!# Version History
//! - Revision 1, 2017-03-20 (Alon Or-bach)
//! - Internal revisions
//!# Other info
//! * 2017-03-20
//! * No known IP claims.
//!*
//! - Alon Or-bach, Samsung Electronics
//! - Ian Elliott, Google
//! - Jesse Hall, Google
//! - Pablo Ceballos, Google
//! - Chris Forbes, Google
//! - Jeff Juliano, NVIDIA
//! - James Jones, NVIDIA
//! - Daniel Rakos, AMD
//! - Tobias Hector, Imagination Technologies
//! - Graham Connor, Imagination Technologies
//! - Michael Worcester, Imagination Technologies
//! - Cass Everitt, Oculus
//! - Johannes Van Waveren, Oculus
//!# Related
//! - [`SharedPresentSurfaceCapabilitiesKHR`]
//! - [`GetSwapchainStatusKHR`]
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
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_shared_presentable_image");
