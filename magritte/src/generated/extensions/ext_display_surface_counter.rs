use crate::{
    extensions::{
        khr_display::SurfaceTransformFlagsKHR,
        khr_surface::{CompositeAlphaFlagsKHR, SurfaceTransformFlagBitsKHR},
    },
    vulkan1_0::{BaseOutStructure, Extent2D, ImageUsageFlags, StructureType},
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION")]
pub const EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME")]
pub const EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_display_surface_counter");
///[VkSurfaceCapabilities2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2EXT.html) - Structure describing capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilities2EXT`] structure is defined as:
///```c
///// Provided by VK_EXT_display_surface_counter
///typedef struct VkSurfaceCapabilities2EXT {
///    VkStructureType                  sType;
///    void*                            pNext;
///    uint32_t                         minImageCount;
///    uint32_t                         maxImageCount;
///    VkExtent2D                       currentExtent;
///    VkExtent2D                       minImageExtent;
///    VkExtent2D                       maxImageExtent;
///    uint32_t                         maxImageArrayLayers;
///    VkSurfaceTransformFlagsKHR       supportedTransforms;
///    VkSurfaceTransformFlagBitsKHR    currentTransform;
///    VkCompositeAlphaFlagsKHR         supportedCompositeAlpha;
///    VkImageUsageFlags                supportedUsageFlags;
///    VkSurfaceCounterFlagsEXT         supportedSurfaceCounters;
///} VkSurfaceCapabilities2EXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_image_count`] is the minimum number of images the specified device supports for a
///   swapchain created for the surface, and will be at least one.
/// - [`max_image_count`] is the maximum number of images the specified device supports for a
///   swapchain created for the surface, and will be either 0, or greater than or equal to
///   [`min_image_count`]. A value of 0 means that there is no limit on the number of images, though
///   there **may** be limits related to the total amount of memory used by presentable images.
/// - [`current_extent`] is the current width and height of the surface, or the special value
///   (0xFFFFFFFF, 0xFFFFFFFF) indicating that the surface size will be determined by the extent of
///   a swapchain targeting the surface.
/// - [`min_image_extent`] contains the smallest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be less than or equal to
///   the corresponding `width` and `height` of [`current_extent`], unless [`current_extent`] has
///   the special value described above.
/// - [`max_image_extent`] contains the largest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be greater than or equal to
///   the corresponding `width` and `height` of [`min_image_extent`]. The `width` and `height` of
///   the extent will each be greater than or equal to the corresponding `width` and `height` of
///   [`current_extent`], unless [`current_extent`] has the special value described above.
/// - [`max_image_array_layers`] is the maximum number of layers presentable images **can** have for
///   a swapchain created for this device and surface, and will be at least one.
/// - [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] indicating the
///   presentation transforms supported for the surface on the specified device. At least one bit
///   will be set.
/// - [`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value indicating the surface’s
///   current transform relative to the presentation engine’s natural orientation.
/// - [`supported_composite_alpha`] is a bitmask of [`CompositeAlphaFlagBitsKHR`], representing the
///   alpha compositing modes supported by the presentation engine for the surface on the specified
///   device, and at least one bit will be set. Opaque composition **can** be achieved in any alpha
///   compositing mode by either using an image format that has no alpha component, or by ensuring
///   that all pixels in the presentable images have an alpha value of 1.0.
/// - [`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`] representing the ways the
///   application **can** use the presentable images of a swapchain created with [`PresentModeKHR`]
///   set to `VK_PRESENT_MODE_IMMEDIATE_KHR`, `VK_PRESENT_MODE_MAILBOX_KHR`,
///   `VK_PRESENT_MODE_FIFO_KHR` or `VK_PRESENT_MODE_FIFO_RELAXED_KHR` for the surface on the
///   specified device. `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`**must** be included in the set.
///   Implementations **may** support additional usages.
/// - [`supported_surface_counters`] is a bitmask of [`SurfaceCounterFlagBitsEXT`] indicating the
///   supported surface counter types.
///# Description
///Valid Usage
/// -  [`supported_surface_counters`]**must** not include `VK_SURFACE_COUNTER_VBLANK_BIT_EXT` unless the surface queried is a [display surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#wsi-display-surfaces)
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`
/// - [`p_next`]**must** be `NULL`
///# Related
/// - [`VK_EXT_display_surface_counter`]
/// - [`CompositeAlphaFlagsKHR`]
/// - [`Extent2D`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
/// - [`SurfaceCounterFlagsEXT`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`GetPhysicalDeviceSurfaceCapabilities2EXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SurfaceCapabilities2EXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`min_image_count`] is the minimum number of images the specified device
    ///supports for a swapchain created for the surface, and will be at least
    ///one.
    min_image_count: u32,
    ///[`max_image_count`] is the maximum number of images the specified device
    ///supports for a swapchain created for the surface, and will be either 0,
    ///or greater than or equal to [`min_image_count`].
    ///A value of 0 means that there is no limit on the number of images,
    ///though there **may** be limits related to the total amount of memory used
    ///by presentable images.
    max_image_count: u32,
    ///[`current_extent`] is the current width and height of the surface, or
    ///the special value (0xFFFFFFFF, 0xFFFFFFFF) indicating that the
    ///surface size will be determined by the extent of a swapchain targeting
    ///the surface.
    current_extent: Extent2D,
    ///[`min_image_extent`] contains the smallest valid swapchain extent for
    ///the surface on the specified device.
    ///The `width` and `height` of the extent will each be less than or
    ///equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    min_image_extent: Extent2D,
    ///[`max_image_extent`] contains the largest valid swapchain extent for the
    ///surface on the specified device.
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`min_image_extent`].
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    max_image_extent: Extent2D,
    ///[`max_image_array_layers`] is the maximum number of layers presentable
    ///images **can** have for a swapchain created for this device and surface,
    ///and will be at least one.
    max_image_array_layers: u32,
    ///[`supported_transforms`] is a bitmask of
    ///[`SurfaceTransformFlagBitsKHR`] indicating the presentation
    ///transforms supported for the surface on the specified device.
    ///At least one bit will be set.
    supported_transforms: SurfaceTransformFlagsKHR,
    ///[`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value
    ///indicating the surface’s current transform relative to the presentation
    ///engine’s natural orientation.
    current_transform: SurfaceTransformFlagBitsKHR,
    ///[`supported_composite_alpha`] is a bitmask of
    ///[`CompositeAlphaFlagBitsKHR`], representing the alpha compositing
    ///modes supported by the presentation engine for the surface on the
    ///specified device, and at least one bit will be set.
    ///Opaque composition **can** be achieved in any alpha compositing mode by
    ///either using an image format that has no alpha component, or by ensuring
    ///that all pixels in the presentable images have an alpha value of 1.0.
    supported_composite_alpha: CompositeAlphaFlagsKHR,
    ///[`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`]
    ///representing the ways the application **can** use the presentable images of
    ///a swapchain created
    ///with [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`,
    ///`VK_PRESENT_MODE_MAILBOX_KHR`, `VK_PRESENT_MODE_FIFO_KHR` or
    ///`VK_PRESENT_MODE_FIFO_RELAXED_KHR`
    ///for the surface on the specified device.
    ///`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT`**must** be included in the set.
    ///Implementations **may** support additional usages.
    supported_usage_flags: ImageUsageFlags,
    ///[`supported_surface_counters`] is a bitmask of
    ///[`SurfaceCounterFlagBitsEXT`] indicating the supported surface
    ///counter types.
    supported_surface_counters: SurfaceCounterFlagsEXT,
}
