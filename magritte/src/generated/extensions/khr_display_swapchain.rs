use crate::vulkan1_0::{BaseInStructure, Bool32, Rect2D, StructureType};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_display_swapchain");
///[VkDisplayPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDisplayPresentInfoKHR.html) - Structure describing parameters of a queue presentation to a swapchain
///# C Specifications
///The [`DisplayPresentInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_display_swapchain
///typedef struct VkDisplayPresentInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkRect2D           srcRect;
///    VkRect2D           dstRect;
///    VkBool32           persistent;
///} VkDisplayPresentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src_rect`] is a rectangular region of pixels to present. It **must** be a subset of the
///   image being presented. If [`DisplayPresentInfoKHR`] is not specified, this region will be
///   assumed to be the entire presentable image.
/// - [`dst_rect`] is a rectangular region within the visible region of the swapchain’s display
///   mode. If [`DisplayPresentInfoKHR`] is not specified, this region will be assumed to be the
///   entire visible region of the swapchain’s mode. If the specified rectangle is a subset of the
///   display mode’s visible region, content from display planes below the swapchain’s plane will be
///   visible outside the rectangle. If there are no planes below the swapchain’s, the area outside
///   the specified rectangle will be black. If portions of the specified rectangle are outside of
///   the display’s visible region, pixels mapping only to those portions of the rectangle will be
///   discarded.
/// - [`persistent`]: If this is [`TRUE`], the display engine will enable buffered mode on displays
///   that support it. This allows the display engine to stop sending content to the display until a
///   new image is presented. The display will instead maintain a copy of the last presented image.
///   This allows less power to be used, but **may** increase presentation latency. If
///   [`DisplayPresentInfoKHR`] is not specified, persistent mode will not be used.
///# Description
///If the extent of the [`src_rect`] and [`dst_rect`] are not equal, the
///presented pixels will be scaled accordingly.Valid Usage
/// - [`src_rect`]**must** specify a rectangular region that is a subset of the image being
///   presented
/// - [`dst_rect`]**must** specify a rectangular region that is a subset of the `visibleRegion`
///   parameter of the display mode the swapchain being presented uses
/// - If the `persistentContent` member of the [`DisplayPropertiesKHR`] structure returned by
///   [`GetPhysicalDeviceDisplayPropertiesKHR`] for the display the present operation targets is
///   [`FALSE`], then [`persistent`]**must** be [`FALSE`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR`
///# Related
/// - [`VK_KHR_display_swapchain`]
/// - [`Bool32`]
/// - [`Rect2D`]
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
pub struct DisplayPresentInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`src_rect`] is a rectangular region of pixels to present.
    ///It **must** be a subset of the image being presented.
    ///If [`DisplayPresentInfoKHR`] is not specified, this region will be
    ///assumed to be the entire presentable image.
    src_rect: Rect2D,
    ///[`dst_rect`] is a rectangular region within the visible region of the
    ///swapchain’s display mode.
    ///If [`DisplayPresentInfoKHR`] is not specified, this region will be
    ///assumed to be the entire visible region of the swapchain’s mode.
    ///If the specified rectangle is a subset of the display mode’s visible
    ///region, content from display planes below the swapchain’s plane will be
    ///visible outside the rectangle.
    ///If there are no planes below the swapchain’s, the area outside the
    ///specified rectangle will be black.
    ///If portions of the specified rectangle are outside of the display’s
    ///visible region, pixels mapping only to those portions of the rectangle
    ///will be discarded.
    dst_rect: Rect2D,
    ///[`persistent`]: If this is [`TRUE`], the display engine will
    ///enable buffered mode on displays that support it.
    ///This allows the display engine to stop sending content to the display
    ///until a new image is presented.
    ///The display will instead maintain a copy of the last presented image.
    ///This allows less power to be used, but **may** increase presentation
    ///latency.
    ///If [`DisplayPresentInfoKHR`] is not specified, persistent mode will
    ///not be used.
    persistent: Bool32,
}
