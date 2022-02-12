//![VK_KHR_incremental_present](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_incremental_present.html) - device extension
//!# Description
//!This device extension extends [`QueuePresentKHR`], from the
//!`[`VK_KHR_swapchain`]` extension, allowing an application to specify a
//!list of rectangular, modified regions of each image to present.
//!This should be used in situations where an application is only changing a
//!small portion of the presentable images within a swapchain, since it enables
//!the presentation engine to avoid wasting time presenting parts of the
//!surface that have not changed.This extension is leveraged from the
//! `EGL_KHR_swap_buffers_with_damage`
//!extension.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_swapchain`]`
//!# Contacts
//! - Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_incremental_present]
//!   @ianelliottus%0A<<Here describe the issue or question you have about the
//!   VK_KHR_incremental_present extension>>)
//!# New structures
//! - [`PresentRegionKHR`]
//! - [`RectLayerKHR`]
//! - Extending [`PresentInfoKHR`]:
//! - [`PresentRegionsKHR`]
//!# New constants
//! - [`KHR_INCREMENTAL_PRESENT_EXTENSION_NAME`]
//! - [`KHR_INCREMENTAL_PRESENT_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`
//!# Known issues & F.A.Q
//!1) How should we handle steroescopic-3D swapchains? We need to add a layer
//!for each rectangle.
//!One approach is to create another struct containing the [`Rect2D`] plus
//!layer, and have [`PresentRegionsKHR`] point to an array of that struct.
//!Another approach is to have two parallel arrays, `pRectangles` and
//!`pLayers`, where `pRectangles`[i] and `pLayers`[i] must be used
//!together.
//!Which approach should we use, and if the array of a new structure, what
//!should that be called?**RESOLVED**: Create a new structure, which is a [`Rect2D`] plus a layer,
//!and will be called [`RectLayerKHR`].2) Where is the origin of the [`RectLayerKHR`]?**RESOLVED**:
//! The upper left corner of the presentable image(s) of the
//!swapchain, per the definition of framebuffer coordinates.3) Does the rectangular region,
//! [`RectLayerKHR`], specify pixels of the
//!swapchain’s image(s), or of the surface?**RESOLVED**: Of the image(s).
//!Some presentation engines may scale the pixels of a swapchain’s image(s) to
//!the size of the surface.
//!The size of the swapchain’s image(s) will be consistent, where the size of
//!the surface may vary over time.4) What if all of the rectangles for a given swapchain contain a
//! width
//!and/or height of zero?**RESOLVED**: The application is indicating that no pixels changed since
//! the
//!last present.
//!The presentation engine may use such a hint and not update any pixels for
//!the swapchain.
//!However, all other semantics of [`QueuePresentKHR`] must still be
//!honored, including waiting for semaphores to signal.5) When the swapchain is created with
//![`SwapchainCreateInfoKHR::pre_transform`] set to a value other than
//!`VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR`, should the rectangular region,
//![`RectLayerKHR`], be transformed to align with the `preTransform`?**RESOLVED**: No.
//!The rectangular region in [`RectLayerKHR`] should not be tranformed.
//!As such, it may not align with the extents of the swapchain’s image(s).
//!It is the responsibility of the presentation engine to transform the
//!rectangular region.
//!This matches the behavior of the Android presentation engine, which set the
//!precedent.
//!# Version History
//! - Revision 1, 2016-11-02 (Ian Elliott)
//! - Internal revisions
//!
//! - Revision 2, 2021-03-18 (Ian Elliott)
//! - Clarified alignment of rectangles for presentation engines that support
//!transformed swapchains.
//!# Other info
//! * 2016-11-02
//! * No known IP claims.
//!*
//! - Ian Elliott, Google
//! - Jesse Hall, Google
//! - Alon Or-bach, Samsung
//! - James Jones, NVIDIA
//! - Daniel Rakos, AMD
//! - Ray Smith, ARM
//! - Mika Isojarvi, Google
//! - Jeff Juliano, NVIDIA
//! - Jeff Bolz, NVIDIA
//!# Related
//! - [`PresentRegionKHR`]
//! - [`PresentRegionsKHR`]
//! - [`RectLayerKHR`]
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
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION")]
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME")]
pub const KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_incremental_present");
