use crate::{
    extensions::khr_surface::{
        ColorSpaceKHR, CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR,
    },
    vulkan1_0::{
        BaseInStructure, Bool32, Extent2D, Format, ImageUsageFlags, Semaphore, SharingMode, StructureType,
        VulkanResultCodes,
    },
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_swapchain");
///[VkSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSwapchainCreateInfoKHR.html) - Structure specifying parameters of a newly created swapchain object
///# C Specifications
///The [`SwapchainCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_swapchain
///typedef struct VkSwapchainCreateInfoKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkSwapchainCreateFlagsKHR        flags;
///    VkSurfaceKHR                     surface;
///    uint32_t                         minImageCount;
///    VkFormat                         imageFormat;
///    VkColorSpaceKHR                  imageColorSpace;
///    VkExtent2D                       imageExtent;
///    uint32_t                         imageArrayLayers;
///    VkImageUsageFlags                imageUsage;
///    VkSharingMode                    imageSharingMode;
///    uint32_t                         queueFamilyIndexCount;
///    const uint32_t*                  pQueueFamilyIndices;
///    VkSurfaceTransformFlagBitsKHR    preTransform;
///    VkCompositeAlphaFlagBitsKHR      compositeAlpha;
///    VkPresentModeKHR                 presentMode;
///    VkBool32                         clipped;
///    VkSwapchainKHR                   oldSwapchain;
///} VkSwapchainCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`SwapchainCreateFlagBitsKHR`] indicating parameters of the
///   swapchain creation.
/// - [`surface`] is the surface onto which the swapchain will present images. If the creation
///   succeeds, the swapchain becomes associated with [`surface`].
/// - [`min_image_count`] is the minimum number of presentable images that the application needs.
///   The implementation will either create the swapchain with at least that many images, or it will
///   fail to create the swapchain.
/// - [`image_format`] is a [`Format`] value specifying the format the swapchain image(s) will be
///   created with.
/// - [`image_color_space`] is a [`ColorSpaceKHR`] value specifying the way the swapchain interprets
///   image data.
/// - [`image_extent`] is the size (in pixels) of the swapchain image(s). The behavior is
///   platform-dependent if the image extent does not match the surface’s `currentExtent` as
///   returned by [`GetPhysicalDeviceSurfaceCapabilitiesKHR`].
/// - [`image_array_layers`] is the number of views in a multiview/stereo surface. For
///   non-stereoscopic-3D applications, this value is 1.
/// - [`image_usage`] is a bitmask of [`ImageUsageFlagBits`] describing the intended usage of the
///   (acquired) swapchain images.
/// - [`image_sharing_mode`] is the sharing mode used for the image(s) of the swapchain.
/// - [`queue_family_index_count`] is the number of queue families having access to the image(s) of
///   the swapchain when [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
/// - [`p_queue_family_indices`] is a pointer to an array of queue family indices having access to
///   the images(s) of the swapchain when [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
/// - [`pre_transform`] is a [`SurfaceTransformFlagBitsKHR`] value describing the transform,
///   relative to the presentation engine’s natural orientation, applied to the image content prior
///   to presentation. If it does not match the `currentTransform` value returned by
///   [`GetPhysicalDeviceSurfaceCapabilitiesKHR`], the presentation engine will transform the image
///   content as part of the presentation operation.
/// - [`composite_alpha`] is a [`CompositeAlphaFlagBitsKHR`] value indicating the alpha compositing
///   mode to use when this surface is composited together with other surfaces on certain window
///   systems.
/// - [`present_mode`] is the presentation mode the swapchain will use. A swapchain’s present mode
///   determines how incoming present requests will be processed and queued internally.
/// - [`clipped`] specifies whether the Vulkan implementation is allowed to discard rendering
///   operations that affect regions of the surface that are not visible.  - If set to [`TRUE`], the
///   presentable images associated with the swapchain **may** not own all of their pixels. Pixels
///   in the presentable images that correspond to regions of the target surface obscured by another
///   window on the desktop, or subject to some other clipping mechanism will have undefined content
///   when read back. Fragment shaders **may** not execute for these pixels, and thus any side
///   effects they would have had will not occur. Setting [`TRUE`] does not guarantee any clipping
///   will occur, but allows more efficient presentation methods to be used on some platforms.  - If
///   set to [`FALSE`], presentable images associated with the swapchain will own all of the pixels
///   they contain.
/// - [`old_swapchain`] is [`crate::utils::Handle::null`], or the existing non-retired swapchain
///   currently associated with [`surface`]. Providing a valid [`old_swapchain`]**may** aid in the
///   resource reuse, and also allows the application to still present any images that are already
///   acquired from it.
///# Description
///Upon calling [`CreateSwapchainKHR`] with an [`old_swapchain`] that is
///not [`crate::utils::Handle::null`], [`old_swapchain`] is retired — even if creation
///of the new swapchain fails.
///The new swapchain is created in the non-retired state whether or not
///[`old_swapchain`] is [`crate::utils::Handle::null`].Upon calling [`CreateSwapchainKHR`] with an
/// [`old_swapchain`] that is
///not [`crate::utils::Handle::null`], any images from [`old_swapchain`] that are not
///acquired by the application **may** be freed by the implementation, which **may**
///occur even if creation of the new swapchain fails.
///The application **can** destroy [`old_swapchain`] to free all memory
///associated with [`old_swapchain`].Valid Usage
/// - [`surface`]**must** be a surface that is supported by the device as determined using
///   [`GetPhysicalDeviceSurfaceSupportKHR`]
/// - [`min_image_count`]**must** be less than or equal to the value returned in the `maxImageCount`
///   member of the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`GetPhysicalDeviceSurfaceCapabilitiesKHR`] for the surface if the returned `maxImageCount` is
///   not zero
/// - If [`present_mode`] is not `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` nor
///   `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`, then [`min_image_count`]**must** be greater
///   than or equal to the value returned in the [`min_image_count`] member of the
///   [`SurfaceCapabilitiesKHR`] structure returned by [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
///   for the surface
/// - [`min_image_count`]**must** be `1` if [`present_mode`] is either
///   `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`
/// - [`image_format`] and [`image_color_space`]**must** match the `format` and `colorSpace`
///   members, respectively, of one of the [`SurfaceFormatKHR`] structures returned by
///   [`GetPhysicalDeviceSurfaceFormatsKHR`] for the surface
/// - [`image_extent`]**must** be between `minImageExtent` and `maxImageExtent`, inclusive, where
///   `minImageExtent` and `maxImageExtent` are members of the [`SurfaceCapabilitiesKHR`] structure
///   returned by [`GetPhysicalDeviceSurfaceCapabilitiesKHR`] for the surface
/// - [`image_extent`] members `width` and `height`**must** both be non-zero
/// - [`image_array_layers`]**must** be greater than `0` and less than or equal to the
///   `maxImageArrayLayers` member of the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`GetPhysicalDeviceSurfaceCapabilitiesKHR`] for the surface
/// - If [`present_mode`] is `VK_PRESENT_MODE_IMMEDIATE_KHR`, `VK_PRESENT_MODE_MAILBOX_KHR`,
///   `VK_PRESENT_MODE_FIFO_KHR` or `VK_PRESENT_MODE_FIFO_RELAXED_KHR`, [`image_usage`]**must** be a
///   subset of the supported usage flags present in the `supportedUsageFlags` member of the
///   [`SurfaceCapabilitiesKHR`] structure returned by [`GetPhysicalDeviceSurfaceCapabilitiesKHR`]
///   for [`surface`]
/// - If [`present_mode`] is `VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR` or
///   `VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR`, [`image_usage`]**must** be a subset of the
///   supported usage flags present in the `sharedPresentSupportedUsageFlags` member of the
///   [`SharedPresentSurfaceCapabilitiesKHR`] structure returned by
///   [`GetPhysicalDeviceSurfaceCapabilities2KHR`] for [`surface`]
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, [`p_queue_family_indices`]**must**
///   be a valid pointer to an array of [`queue_family_index_count`]`uint32_t` values
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`,
///   [`queue_family_index_count`]**must** be greater than `1`
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, each element of
///   [`p_queue_family_indices`]**must** be unique and **must** be less than
///   `pQueueFamilyPropertyCount` returned by either [`GetPhysicalDeviceQueueFamilyProperties`] or
///   [`GetPhysicalDeviceQueueFamilyProperties2`] for the `physicalDevice` that was used to create
///   `device`
/// - [`pre_transform`]**must** be one of the bits present in the `supportedTransforms` member of
///   the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`GetPhysicalDeviceSurfaceCapabilitiesKHR`] for the surface
/// - [`composite_alpha`]**must** be one of the bits present in the `supportedCompositeAlpha` member
///   of the [`SurfaceCapabilitiesKHR`] structure returned by
///   [`GetPhysicalDeviceSurfaceCapabilitiesKHR`] for the surface
/// - [`present_mode`]**must** be one of the [`PresentModeKHR`] values returned by
///   [`GetPhysicalDeviceSurfacePresentModesKHR`] for the surface
/// - If the logical device was created with [`DeviceGroupDeviceCreateInfo::physical_device_count`]
///   equal to 1, [`flags`]**must** not contain
///   `VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
/// - If [`old_swapchain`] is not [`crate::utils::Handle::null`], [`old_swapchain`]**must** be a
///   non-retired swapchain associated with native window referred to by [`surface`]
/// - The [implied image creation parameters](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#swapchain-wsi-image-create-info)
///   of the swapchain **must** be supported as reported by
///   [`GetPhysicalDeviceImageFormatProperties`]
/// - If [`flags`] contains `VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR` then the [`p_next`] chain
///   **must** include a [`ImageFormatListCreateInfo`] structure with a `viewFormatCount` greater
///   than zero and `pViewFormats`**must** have an element equal to [`image_format`]
/// -    If a [`ImageFormatListCreateInfo`] structure was included in the [`p_next`] chain and [`ImageFormatListCreateInfo::view_format_count`] is not zero then all of the formats in [`ImageFormatListCreateInfo::p_view_formats`]**must** be compatible with the `format` as described in the [compatibility table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility)
/// - If [`flags`] does not contain `VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR` and the [`p_next`]
///   chain include a [`ImageFormatListCreateInfo`] structure then
///   [`ImageFormatListCreateInfo::view_format_count`]**must** be `0` or `1`
/// - If [`flags`] contains `VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR`, then
///   [`SurfaceProtectedCapabilitiesKHR::supports_protected`]**must** be [`TRUE`] in the
///   [`SurfaceProtectedCapabilitiesKHR`] structure returned by
///   [`GetPhysicalDeviceSurfaceCapabilities2KHR`] for [`surface`]
/// - If the [`p_next`] chain includes a [`SurfaceFullScreenExclusiveInfoEXT`] structure with its
///   `fullScreenExclusive` member set to `VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT`, and
///   [`surface`] was created using [`CreateWin32SurfaceKHR`], a
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`] structure **must** be included in the [`p_next`]
///   chain
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`DeviceGroupSwapchainCreateInfoKHR`],
///   [`ImageFormatListCreateInfo`], [`SurfaceFullScreenExclusiveInfoEXT`],
///   [`SurfaceFullScreenExclusiveWin32InfoEXT`], [`SwapchainCounterCreateInfoEXT`], or
///   [`SwapchainDisplayNativeHdrCreateInfoAMD`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - [`flags`]**must** be a valid combination of [`SwapchainCreateFlagBitsKHR`] values
/// - [`surface`]**must** be a valid [`SurfaceKHR`] handle
/// - [`image_format`]**must** be a valid [`Format`] value
/// - [`image_color_space`]**must** be a valid [`ColorSpaceKHR`] value
/// - [`image_usage`]**must** be a valid combination of [`ImageUsageFlagBits`] values
/// - [`image_usage`]**must** not be `0`
/// - [`image_sharing_mode`]**must** be a valid [`SharingMode`] value
/// - [`pre_transform`]**must** be a valid [`SurfaceTransformFlagBitsKHR`] value
/// - [`composite_alpha`]**must** be a valid [`CompositeAlphaFlagBitsKHR`] value
/// - [`present_mode`]**must** be a valid [`PresentModeKHR`] value
/// - If [`old_swapchain`] is not [`crate::utils::Handle::null`], [`old_swapchain`]**must** be a
///   valid [`SwapchainKHR`] handle
/// - If [`old_swapchain`] is a valid handle, it **must** have been created, allocated, or retrieved
///   from [`surface`]
/// - Both of [`old_swapchain`], and [`surface`] that are valid handles of non-ignored parameters
///   **must** have been created, allocated, or retrieved from the same [`Instance`]
///# Related
/// - [`VK_KHR_swapchain`]
/// - [`Bool32`]
/// - [`ColorSpaceKHR`]
/// - [`CompositeAlphaFlagBitsKHR`]
/// - [`Extent2D`]
/// - [`Format`]
/// - [`ImageUsageFlags`]
/// - [`PresentModeKHR`]
/// - [`SharingMode`]
/// - [`StructureType`]
/// - [`SurfaceKHR`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`SwapchainCreateFlagsKHR`]
/// - [`SwapchainKHR`]
/// - [`CreateSharedSwapchainsKHR`]
/// - [`CreateSwapchainKHR`]
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
pub struct SwapchainCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`SwapchainCreateFlagBitsKHR`]
    ///indicating parameters of the swapchain creation.
    flags: SwapchainCreateFlagsKHR,
    ///[`surface`] is the surface onto which the swapchain will present
    ///images.
    ///If the creation succeeds, the swapchain becomes associated with
    ///[`surface`].
    surface: SurfaceKHR,
    ///[`min_image_count`] is the minimum number of presentable images that the
    ///application needs.
    ///The implementation will either create the swapchain with at least that
    ///many images, or it will fail to create the swapchain.
    min_image_count: u32,
    ///[`image_format`] is a [`Format`] value specifying the format the
    ///swapchain image(s) will be created with.
    image_format: Format,
    ///[`image_color_space`] is a [`ColorSpaceKHR`] value specifying the
    ///way the swapchain interprets image data.
    image_color_space: ColorSpaceKHR,
    ///[`image_extent`] is the size (in pixels) of the swapchain image(s).
    ///The behavior is platform-dependent if the image extent does not match
    ///the surface’s `currentExtent` as returned by
    ///[`GetPhysicalDeviceSurfaceCapabilitiesKHR`].
    image_extent: Extent2D,
    ///[`image_array_layers`] is the number of views in a multiview/stereo
    ///surface.
    ///For non-stereoscopic-3D applications, this value is 1.
    image_array_layers: u32,
    ///[`image_usage`] is a bitmask of [`ImageUsageFlagBits`] describing
    ///the intended usage of the (acquired) swapchain images.
    image_usage: ImageUsageFlags,
    ///[`image_sharing_mode`] is the sharing mode used for the image(s) of the
    ///swapchain.
    image_sharing_mode: SharingMode,
    ///[`queue_family_index_count`] is the number of queue families having
    ///access to the image(s) of the swapchain when [`image_sharing_mode`] is
    ///`VK_SHARING_MODE_CONCURRENT`.
    queue_family_index_count: u32,
    ///[`p_queue_family_indices`] is a pointer to an array of queue family
    ///indices having access to the images(s) of the swapchain when
    ///[`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
    p_queue_family_indices: *mut u32,
    ///[`pre_transform`] is a [`SurfaceTransformFlagBitsKHR`] value
    ///describing the transform, relative to the presentation engine’s natural
    ///orientation, applied to the image content prior to presentation.
    ///If it does not match the `currentTransform` value returned by
    ///[`GetPhysicalDeviceSurfaceCapabilitiesKHR`], the presentation engine
    ///will transform the image content as part of the presentation operation.
    pre_transform: SurfaceTransformFlagBitsKHR,
    ///[`composite_alpha`] is a [`CompositeAlphaFlagBitsKHR`] value
    ///indicating the alpha compositing mode to use when this surface is
    ///composited together with other surfaces on certain window systems.
    composite_alpha: CompositeAlphaFlagBitsKHR,
    ///[`present_mode`] is the presentation mode the swapchain will use.
    ///A swapchain’s present mode determines how incoming present requests will
    ///be processed and queued internally.
    present_mode: PresentModeKHR,
    ///[`clipped`] specifies whether the Vulkan implementation is allowed to
    ///discard rendering operations that affect regions of the surface that are
    ///not visible.
    /// - If set to [`TRUE`], the presentable images associated with the swapchain **may** not own
    ///   all of their pixels. Pixels in the presentable images that correspond to regions of the
    ///   target surface obscured by another window on the desktop, or subject to some other
    ///   clipping mechanism will have undefined content when read back. Fragment shaders **may**
    ///   not execute for these pixels, and thus any side effects they would have had will not
    ///   occur. Setting [`TRUE`] does not guarantee any clipping will occur, but allows more
    ///   efficient presentation methods to be used on some platforms.
    /// - If set to [`FALSE`], presentable images associated with the swapchain will own all of the
    ///   pixels they contain.
    clipped: Bool32,
    ///[`old_swapchain`] is [`crate::utils::Handle::null`], or the existing non-retired
    ///swapchain currently associated with [`surface`].
    ///Providing a valid [`old_swapchain`]**may** aid in the resource reuse, and
    ///also allows the application to still present any images that are already
    ///acquired from it.
    old_swapchain: SwapchainKHR,
}
///[VkPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPresentInfoKHR.html) - Structure describing parameters of a queue presentation
///# C Specifications
///The [`PresentInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_swapchain
///typedef struct VkPresentInfoKHR {
///    VkStructureType          sType;
///    const void*              pNext;
///    uint32_t                 waitSemaphoreCount;
///    const VkSemaphore*       pWaitSemaphores;
///    uint32_t                 swapchainCount;
///    const VkSwapchainKHR*    pSwapchains;
///    const uint32_t*          pImageIndices;
///    VkResult*                pResults;
///} VkPresentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`wait_semaphore_count`] is the number of semaphores to wait for before issuing the present
///   request. The number **may** be zero.
/// - [`p_wait_semaphores`] is `NULL` or a pointer to an array of [`Semaphore`] objects with
///   [`wait_semaphore_count`] entries, and specifies the semaphores to wait for before issuing the
///   present request.
/// - [`swapchain_count`] is the number of swapchains being presented to by this command.
/// - [`p_swapchains`] is a pointer to an array of [`SwapchainKHR`] objects with [`swapchain_count`]
///   entries. A given swapchain **must** not appear in this list more than once.
/// - [`p_image_indices`] is a pointer to an array of indices into the array of each swapchain’s
///   presentable images, with [`swapchain_count`] entries. Each entry in this array identifies the
///   image to present on the corresponding entry in the [`p_swapchains`] array.
/// - [`p_results`] is a pointer to an array of [`VulkanResultCodes`] typed elements with
///   [`swapchain_count`] entries. Applications that do not need per-swapchain results **can** use
///   `NULL` for [`p_results`]. If non-`NULL`, each entry in [`p_results`] will be set to the
///   [`VulkanResultCodes`] for presenting the swapchain corresponding to the same index in
///   [`p_swapchains`].
///# Description
///Before an application **can** present an image, the image’s layout **must** be
///transitioned to the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
///layout, or for a shared presentable image the
///`VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
///layout.Valid Usage
/// - Each element of [`p_image_indices`]**must** be the index of a presentable image acquired from
///   the swapchain specified by the corresponding element of the [`p_swapchains`] array, and the
///   presented image subresource **must** be in the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR` or
///   `VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR` layout at the time the operation is executed on a
///   [`Device`]
/// - If a [`PresentIdKHR`] structure is included in the [`p_next`] chain, and the [`presentId`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-presentId)
///   feature is not enabled, each `presentIds` entry in that structure **must** be NULL
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PRESENT_INFO_KHR`
/// - Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain **must**
///   be either `NULL` or a pointer to a valid instance of [`DeviceGroupPresentInfoKHR`],
///   [`DisplayPresentInfoKHR`], [`PresentFrameTokenGGP`], [`PresentIdKHR`], [`PresentRegionsKHR`],
///   or [`PresentTimesInfoGOOGLE`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain **must** be unique
/// - If [`wait_semaphore_count`] is not `0`, [`p_wait_semaphores`]**must** be a valid pointer to an
///   array of [`wait_semaphore_count`] valid [`Semaphore`] handles
/// - [`p_swapchains`]**must** be a valid pointer to an array of [`swapchain_count`] valid
///   [`SwapchainKHR`] handles
/// - [`p_image_indices`]**must** be a valid pointer to an array of [`swapchain_count`]`uint32_t`
///   values
/// - If [`p_results`] is not `NULL`, [`p_results`]**must** be a valid pointer to an array of
///   [`swapchain_count`][`VulkanResultCodes`] values
/// - [`swapchain_count`]**must** be greater than `0`
/// - Both of the elements of [`p_swapchains`], and the elements of [`p_wait_semaphores`] that are
///   valid handles of non-ignored parameters **must** have been created, allocated, or retrieved
///   from the same [`Instance`]
///# Related
/// - [`VK_KHR_swapchain`]
/// - [`VulkanResultCodes`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
/// - [`QueuePresentKHR`]
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
pub struct PresentInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`wait_semaphore_count`] is the number of semaphores to wait for before
    ///issuing the present request.
    ///The number **may** be zero.
    wait_semaphore_count: u32,
    ///[`p_wait_semaphores`] is `NULL` or a pointer to an array of
    ///[`Semaphore`] objects with [`wait_semaphore_count`] entries, and
    ///specifies the semaphores to wait for before issuing the present request.
    p_wait_semaphores: *mut Semaphore,
    ///[`swapchain_count`] is the number of swapchains being presented to by
    ///this command.
    swapchain_count: u32,
    ///[`p_swapchains`] is a pointer to an array of [`SwapchainKHR`]
    ///objects with [`swapchain_count`] entries.
    ///A given swapchain **must** not appear in this list more than once.
    p_swapchains: *mut SwapchainKHR,
    ///[`p_image_indices`] is a pointer to an array of indices into the array
    ///of each swapchain’s presentable images, with [`swapchain_count`]
    ///entries.
    ///Each entry in this array identifies the image to present on the
    ///corresponding entry in the [`p_swapchains`] array.
    p_image_indices: *mut u32,
    ///[`p_results`] is a pointer to an array of [`VulkanResultCodes`] typed elements
    ///with [`swapchain_count`] entries.
    ///Applications that do not need per-swapchain results **can** use `NULL` for
    ///[`p_results`].
    ///If non-`NULL`, each entry in [`p_results`] will be set to the
    ///[`VulkanResultCodes`] for presenting the swapchain corresponding to the same
    ///index in [`p_swapchains`].
    p_results: *const VulkanResultCodes,
}
