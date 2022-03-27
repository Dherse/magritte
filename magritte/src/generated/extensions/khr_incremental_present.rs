use crate::vulkan1_0::{BaseInStructure, Extent2D, Offset2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION")]
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME")]
pub const KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_incremental_present");
///[VkPresentRegionsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionsKHR.html) - Structure hint of rectangular regions changed by vkQueuePresentKHR
///# C Specifications
///When the [`VK_KHR_incremental_present`] extension is enabled, additional
///fields **can** be specified that allow an application to specify that only
///certain rectangular regions of the presentable images of a swapchain are
///changed.
///This is an optimization hint that a presentation engine **may** use to only
///update the region of a surface that is actually changing.
///The application still **must** ensure that all pixels of a presented image
///contain the desired values, in case the presentation engine ignores this
///hint.
///An application **can** provide this hint by adding a [`PresentRegionsKHR`]
///structure to the [`p_next`] chain of the [`PresentInfoKHR`] structure.The [`PresentRegionsKHR`]
/// structure is defined as:
///```c
///// Provided by VK_KHR_incremental_present
///typedef struct VkPresentRegionsKHR {
///    VkStructureType              sType;
///    const void*                  pNext;
///    uint32_t                     swapchainCount;
///    const VkPresentRegionKHR*    pRegions;
///} VkPresentRegionsKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is the number of swapchains being presented to by this command.
/// - [`p_regions`] is `NULL` or a pointer to an array of [`PresentRegionKHR`] elements with
///   [`swapchain_count`] entries. If not `NULL`, each element of [`p_regions`] contains the region
///   that has changed since the last present to the swapchain in the corresponding entry in the
///   [`PresentInfoKHR::p_swapchains`] array.
///# Description
///Valid Usage
/// - [`swapchain_count`]**must** be the same value as [`PresentInfoKHR`]::[`swapchain_count`],
///   where [`PresentInfoKHR`] is included in the [`p_next`] chain of this [`PresentRegionsKHR`]
///   structure
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR`
/// - If [`p_regions`] is not `NULL`, [`p_regions`]**must** be a valid pointer to an array of
///   [`swapchain_count`] valid [`PresentRegionKHR`] structures
/// - [`swapchain_count`]**must** be greater than `0`
///# Related
/// - [`VK_KHR_incremental_present`]
/// - [`PresentRegionKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentRegionsKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`swapchain_count`] is the number of swapchains being presented to by
    ///this command.
    swapchain_count: u32,
    ///[`p_regions`] is `NULL` or a pointer to an array of
    ///[`PresentRegionKHR`] elements with [`swapchain_count`] entries.
    ///If not `NULL`, each element of [`p_regions`] contains the region that
    ///has changed since the last present to the swapchain in the corresponding
    ///entry in the [`PresentInfoKHR`]::`pSwapchains` array.
    p_regions: *mut PresentRegionKHR<'lt>,
}
///[VkPresentRegionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentRegionKHR.html) - Structure containing rectangular region changed by vkQueuePresentKHR for a given VkImage
///# C Specifications
///For a given image and swapchain, the region to present is specified by the
///[`PresentRegionKHR`] structure, which is defined as:
///```c
///// Provided by VK_KHR_incremental_present
///typedef struct VkPresentRegionKHR {
///    uint32_t                 rectangleCount;
///    const VkRectLayerKHR*    pRectangles;
///} VkPresentRegionKHR;
///```
///# Members
/// - [`rectangle_count`] is the number of rectangles in [`p_rectangles`], or zero if the entire
///   image has changed and should be presented.
/// - [`p_rectangles`] is either `NULL` or a pointer to an array of [`RectLayerKHR`] structures. The
///   [`RectLayerKHR`] structure is the framebuffer coordinates, plus layer, of a portion of a
///   presentable image that has changed and **must** be presented. If non-`NULL`, each entry in
///   [`p_rectangles`] is a rectangle of the given image that has changed since the last image was
///   presented to the given swapchain. The rectangles **must** be specified relative to
///   [`SurfaceCapabilitiesKHR::current_transform`], regardless of the swapchain’s `preTransform`.
///   The presentation engine will apply the `preTransform` transformation to the rectangles, along
///   with any further transformation it applies to the image content.
///# Description
///Valid Usage (Implicit)
/// - If [`rectangle_count`] is not `0`, and [`p_rectangles`] is not `NULL`,
///   [`p_rectangles`]**must** be a valid pointer to an array of [`rectangle_count`] valid
///   [`RectLayerKHR`] structures
///# Related
/// - [`VK_KHR_incremental_present`]
/// - [`PresentRegionsKHR`]
/// - [`RectLayerKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PresentRegionKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`rectangle_count`] is the number of rectangles in [`p_rectangles`],
    ///or zero if the entire image has changed and should be presented.
    rectangle_count: u32,
    ///[`p_rectangles`] is either `NULL` or a pointer to an array of
    ///[`RectLayerKHR`] structures.
    ///The [`RectLayerKHR`] structure is the framebuffer coordinates, plus
    ///layer, of a portion of a presentable image that has changed and **must** be
    ///presented.
    ///If non-`NULL`, each entry in [`p_rectangles`] is a rectangle of the
    ///given image that has changed since the last image was presented to the
    ///given swapchain.
    ///The rectangles **must** be specified relative to
    ///[`SurfaceCapabilitiesKHR`]::`currentTransform`, regardless of
    ///the swapchain’s `preTransform`.
    ///The presentation engine will apply the `preTransform` transformation
    ///to the rectangles, along with any further transformation it applies to
    ///the image content.
    p_rectangles: *mut RectLayerKHR,
}
///[VkRectLayerKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRectLayerKHR.html) - Structure containing a rectangle, including layer, changed by vkQueuePresentKHR for a given VkImage
///# C Specifications
///The [`RectLayerKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_incremental_present
///typedef struct VkRectLayerKHR {
///    VkOffset2D    offset;
///    VkExtent2D    extent;
///    uint32_t      layer;
///} VkRectLayerKHR;
///```
///# Members
/// - [`offset`] is the origin of the rectangle, in pixels.
/// - [`extent`] is the size of the rectangle, in pixels.
/// - [`layer`] is the layer of the image. For images with only one layer, the value of
///   [`layer`]**must** be 0.
///# Description
///Some platforms allow the size of a surface to change, and then scale the
///pixels of the image to fit the surface.
///[`RectLayerKHR`] specifies pixels of the swapchain’s image(s), which
///will be constant for the life of the swapchain.Valid Usage
/// - The sum of [`offset`] and [`extent`], after being transformed according to the `preTransform`
///   member of the [`SwapchainCreateInfoKHR`] structure, **must** be no greater than the
///   `imageExtent` member of the [`SwapchainCreateInfoKHR`] structure passed to
///   [`CreateSwapchainKHR`]
/// - [`layer`]**must** be less than the `imageArrayLayers` member of the [`SwapchainCreateInfoKHR`]
///   structure passed to [`CreateSwapchainKHR`]
///# Related
/// - [`VK_KHR_incremental_present`]
/// - [`Extent2D`]
/// - [`Offset2D`]
/// - [`PresentRegionKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RectLayerKHR {
    ///[`offset`] is the origin of the rectangle, in pixels.
    offset: Offset2D,
    ///[`extent`] is the size of the rectangle, in pixels.
    extent: Extent2D,
    ///[`layer`] is the layer of the image.
    ///For images with only one layer, the value of [`layer`]**must** be 0.
    layer: u32,
}
