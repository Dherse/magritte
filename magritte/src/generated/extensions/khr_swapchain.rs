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
/// - [`queue_family_indices`] is a pointer to an array of queue family indices having access to the
///   images(s) of the swapchain when [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
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
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, [`queue_family_indices`]**must** be
///   a valid pointer to an array of [`queue_family_index_count`]`uint32_t` values
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`,
///   [`queue_family_index_count`]**must** be greater than `1`
/// - If [`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`, each element of
///   [`queue_family_indices`]**must** be unique and **must** be less than
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
/// -    If a [`ImageFormatListCreateInfo`] structure was included in the [`p_next`] chain and [`ImageFormatListCreateInfo::view_format_count`] is not zero then all of the formats in [`ImageFormatListCreateInfo::view_formats`]**must** be compatible with the `format` as described in the [compatibility table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-compatibility)
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
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SwapchainCreateInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
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
    ///[`queue_family_indices`] is a pointer to an array of queue family
    ///indices having access to the images(s) of the swapchain when
    ///[`image_sharing_mode`] is `VK_SHARING_MODE_CONCURRENT`.
    queue_family_indices: *const u32,
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
impl<'lt> Default for SwapchainCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            flags: Default::default(),
            surface: Default::default(),
            min_image_count: 0,
            image_format: Default::default(),
            image_color_space: Default::default(),
            image_extent: Default::default(),
            image_array_layers: 0,
            image_usage: Default::default(),
            image_sharing_mode: Default::default(),
            queue_family_index_count: 0,
            queue_family_indices: std::ptr::null(),
            pre_transform: Default::default(),
            composite_alpha: Default::default(),
            present_mode: Default::default(),
            clipped: 0,
            old_swapchain: Default::default(),
        }
    }
}
impl<'lt> SwapchainCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::min_image_count`]
    pub fn min_image_count_raw(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the raw value of [`Self::image_array_layers`]
    pub fn image_array_layers_raw(&self) -> u32 {
        self.image_array_layers
    }
    ///Gets the raw value of [`Self::queue_family_index_count`]
    pub fn queue_family_index_count_raw(&self) -> u32 {
        self.queue_family_index_count
    }
    ///Gets the raw value of [`Self::queue_family_indices`]
    pub fn queue_family_indices_raw(&self) -> *const u32 {
        self.queue_family_indices
    }
    ///Gets the raw value of [`Self::clipped`]
    pub fn clipped_raw(&self) -> Bool32 {
        self.clipped
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::min_image_count`]
    pub fn set_min_image_count_raw(&mut self, value: u32) -> &mut Self {
        self.min_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::image_array_layers`]
    pub fn set_image_array_layers_raw(&mut self, value: u32) -> &mut Self {
        self.image_array_layers = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_index_count`]
    pub fn set_queue_family_index_count_raw(&mut self, value: u32) -> &mut Self {
        self.queue_family_index_count = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_indices`]
    pub fn set_queue_family_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.queue_family_indices = value;
        self
    }
    ///Sets the raw value of [`Self::clipped`]
    pub fn set_clipped_raw(&mut self, value: Bool32) -> &mut Self {
        self.clipped = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> SwapchainCreateFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::surface`]
    pub fn surface(&self) -> SurfaceKHR {
        self.surface
    }
    ///Gets the value of [`Self::min_image_count`]
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the value of [`Self::image_format`]
    pub fn image_format(&self) -> Format {
        self.image_format
    }
    ///Gets the value of [`Self::image_color_space`]
    pub fn image_color_space(&self) -> ColorSpaceKHR {
        self.image_color_space
    }
    ///Gets the value of [`Self::image_extent`]
    pub fn image_extent(&self) -> Extent2D {
        self.image_extent
    }
    ///Gets the value of [`Self::image_array_layers`]
    pub fn image_array_layers(&self) -> u32 {
        self.image_array_layers
    }
    ///Gets the value of [`Self::image_usage`]
    pub fn image_usage(&self) -> ImageUsageFlags {
        self.image_usage
    }
    ///Gets the value of [`Self::image_sharing_mode`]
    pub fn image_sharing_mode(&self) -> SharingMode {
        self.image_sharing_mode
    }
    ///Gets the value of [`Self::queue_family_index_count`]
    pub fn queue_family_index_count(&self) -> u32 {
        self.queue_family_index_count
    }
    ///Gets the value of [`Self::queue_family_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn queue_family_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.queue_family_indices, self.queue_family_index_count as usize)
    }
    ///Gets the value of [`Self::pre_transform`]
    pub fn pre_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.pre_transform
    }
    ///Gets the value of [`Self::composite_alpha`]
    pub fn composite_alpha(&self) -> CompositeAlphaFlagBitsKHR {
        self.composite_alpha
    }
    ///Gets the value of [`Self::present_mode`]
    pub fn present_mode(&self) -> PresentModeKHR {
        self.present_mode
    }
    ///Gets the value of [`Self::clipped`]
    pub fn clipped(&self) -> bool {
        unsafe { std::mem::transmute(self.clipped as u8) }
    }
    ///Gets the value of [`Self::old_swapchain`]
    pub fn old_swapchain(&self) -> SwapchainKHR {
        self.old_swapchain
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut SwapchainCreateFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::surface`]
    pub fn surface_mut(&mut self) -> &mut SurfaceKHR {
        &mut self.surface
    }
    ///Gets a mutable reference to the value of [`Self::min_image_count`]
    pub fn min_image_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::image_format`]
    pub fn image_format_mut(&mut self) -> &mut Format {
        &mut self.image_format
    }
    ///Gets a mutable reference to the value of [`Self::image_color_space`]
    pub fn image_color_space_mut(&mut self) -> &mut ColorSpaceKHR {
        &mut self.image_color_space
    }
    ///Gets a mutable reference to the value of [`Self::image_extent`]
    pub fn image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.image_extent
    }
    ///Gets a mutable reference to the value of [`Self::image_array_layers`]
    pub fn image_array_layers_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::image_usage`]
    pub fn image_usage_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.image_usage
    }
    ///Gets a mutable reference to the value of [`Self::image_sharing_mode`]
    pub fn image_sharing_mode_mut(&mut self) -> &mut SharingMode {
        &mut self.image_sharing_mode
    }
    ///Gets a mutable reference to the value of [`Self::queue_family_index_count`]
    pub fn queue_family_index_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::pre_transform`]
    pub fn pre_transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.pre_transform
    }
    ///Gets a mutable reference to the value of [`Self::composite_alpha`]
    pub fn composite_alpha_mut(&mut self) -> &mut CompositeAlphaFlagBitsKHR {
        &mut self.composite_alpha
    }
    ///Gets a mutable reference to the value of [`Self::present_mode`]
    pub fn present_mode_mut(&mut self) -> &mut PresentModeKHR {
        &mut self.present_mode
    }
    ///Gets a mutable reference to the value of [`Self::clipped`]
    pub fn clipped_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.clipped as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.clipped as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::old_swapchain`]
    pub fn old_swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.old_swapchain
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_swapchain::SwapchainCreateFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::surface`]
    pub fn set_surface(&mut self, value: crate::extensions::khr_surface::SurfaceKHR) -> &mut Self {
        self.surface = value;
        self
    }
    ///Sets the raw value of [`Self::min_image_count`]
    pub fn set_min_image_count(&mut self, value: u32) -> &mut Self {
        self.min_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::image_format`]
    pub fn set_image_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.image_format = value;
        self
    }
    ///Sets the raw value of [`Self::image_color_space`]
    pub fn set_image_color_space(&mut self, value: crate::extensions::khr_surface::ColorSpaceKHR) -> &mut Self {
        self.image_color_space = value;
        self
    }
    ///Sets the raw value of [`Self::image_extent`]
    pub fn set_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::image_array_layers`]
    pub fn set_image_array_layers(&mut self, value: u32) -> &mut Self {
        self.image_array_layers = value;
        self
    }
    ///Sets the raw value of [`Self::image_usage`]
    pub fn set_image_usage(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.image_usage = value;
        self
    }
    ///Sets the raw value of [`Self::image_sharing_mode`]
    pub fn set_image_sharing_mode(&mut self, value: crate::vulkan1_0::SharingMode) -> &mut Self {
        self.image_sharing_mode = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_index_count`]
    pub fn set_queue_family_index_count(&mut self, value: u32) -> &mut Self {
        self.queue_family_index_count = value;
        self
    }
    ///Sets the raw value of [`Self::queue_family_indices`]
    pub fn set_queue_family_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.queue_family_indices = value.as_ptr();
        self.queue_family_index_count = len_;
        self
    }
    ///Sets the raw value of [`Self::pre_transform`]
    pub fn set_pre_transform(
        &mut self,
        value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> &mut Self {
        self.pre_transform = value;
        self
    }
    ///Sets the raw value of [`Self::composite_alpha`]
    pub fn set_composite_alpha(
        &mut self,
        value: crate::extensions::khr_surface::CompositeAlphaFlagBitsKHR,
    ) -> &mut Self {
        self.composite_alpha = value;
        self
    }
    ///Sets the raw value of [`Self::present_mode`]
    pub fn set_present_mode(&mut self, value: crate::extensions::khr_surface::PresentModeKHR) -> &mut Self {
        self.present_mode = value;
        self
    }
    ///Sets the raw value of [`Self::clipped`]
    pub fn set_clipped(&mut self, value: bool) -> &mut Self {
        self.clipped = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::old_swapchain`]
    pub fn set_old_swapchain(&mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> &mut Self {
        self.old_swapchain = value;
        self
    }
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
/// - [`wait_semaphores`] is `NULL` or a pointer to an array of [`Semaphore`] objects with
///   [`wait_semaphore_count`] entries, and specifies the semaphores to wait for before issuing the
///   present request.
/// - [`swapchain_count`] is the number of swapchains being presented to by this command.
/// - [`swapchains`] is a pointer to an array of [`SwapchainKHR`] objects with [`swapchain_count`]
///   entries. A given swapchain **must** not appear in this list more than once.
/// - [`image_indices`] is a pointer to an array of indices into the array of each swapchain’s
///   presentable images, with [`swapchain_count`] entries. Each entry in this array identifies the
///   image to present on the corresponding entry in the [`swapchains`] array.
/// - [`results`] is a pointer to an array of [`VulkanResultCodes`] typed elements with
///   [`swapchain_count`] entries. Applications that do not need per-swapchain results **can** use
///   `NULL` for [`results`]. If non-`NULL`, each entry in [`results`] will be set to the
///   [`VulkanResultCodes`] for presenting the swapchain corresponding to the same index in
///   [`swapchains`].
///# Description
///Before an application **can** present an image, the image’s layout **must** be
///transitioned to the `VK_IMAGE_LAYOUT_PRESENT_SRC_KHR`
///layout, or for a shared presentable image the
///`VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR`
///layout.Valid Usage
/// - Each element of [`image_indices`]**must** be the index of a presentable image acquired from
///   the swapchain specified by the corresponding element of the [`swapchains`] array, and the
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
/// - If [`wait_semaphore_count`] is not `0`, [`wait_semaphores`]**must** be a valid pointer to an
///   array of [`wait_semaphore_count`] valid [`Semaphore`] handles
/// - [`swapchains`]**must** be a valid pointer to an array of [`swapchain_count`] valid
///   [`SwapchainKHR`] handles
/// - [`image_indices`]**must** be a valid pointer to an array of [`swapchain_count`]`uint32_t`
///   values
/// - If [`results`] is not `NULL`, [`results`]**must** be a valid pointer to an array of
///   [`swapchain_count`][`VulkanResultCodes`] values
/// - [`swapchain_count`]**must** be greater than `0`
/// - Both of the elements of [`swapchains`], and the elements of [`wait_semaphores`] that are valid
///   handles of non-ignored parameters **must** have been created, allocated, or retrieved from the
///   same [`Instance`]
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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PresentInfoKHR<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`wait_semaphore_count`] is the number of semaphores to wait for before
    ///issuing the present request.
    ///The number **may** be zero.
    wait_semaphore_count: u32,
    ///[`wait_semaphores`] is `NULL` or a pointer to an array of
    ///[`Semaphore`] objects with [`wait_semaphore_count`] entries, and
    ///specifies the semaphores to wait for before issuing the present request.
    wait_semaphores: *const Semaphore,
    ///[`swapchain_count`] is the number of swapchains being presented to by
    ///this command.
    swapchain_count: u32,
    ///[`swapchains`] is a pointer to an array of [`SwapchainKHR`]
    ///objects with [`swapchain_count`] entries.
    ///A given swapchain **must** not appear in this list more than once.
    swapchains: *const SwapchainKHR,
    ///[`image_indices`] is a pointer to an array of indices into the array
    ///of each swapchain’s presentable images, with [`swapchain_count`]
    ///entries.
    ///Each entry in this array identifies the image to present on the
    ///corresponding entry in the [`swapchains`] array.
    image_indices: *const u32,
    ///[`results`] is a pointer to an array of [`VulkanResultCodes`] typed elements
    ///with [`swapchain_count`] entries.
    ///Applications that do not need per-swapchain results **can** use `NULL` for
    ///[`results`].
    ///If non-`NULL`, each entry in [`results`] will be set to the
    ///[`VulkanResultCodes`] for presenting the swapchain corresponding to the same
    ///index in [`swapchains`].
    results: *mut VulkanResultCodes,
}
impl<'lt> Default for PresentInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            wait_semaphore_count: 0,
            wait_semaphores: std::ptr::null(),
            swapchain_count: 0,
            swapchains: std::ptr::null(),
            image_indices: std::ptr::null(),
            results: std::ptr::null_mut(),
        }
    }
}
impl<'lt> PresentInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::wait_semaphore_count`]
    pub fn wait_semaphore_count_raw(&self) -> u32 {
        self.wait_semaphore_count
    }
    ///Gets the raw value of [`Self::wait_semaphores`]
    pub fn wait_semaphores_raw(&self) -> *const Semaphore {
        self.wait_semaphores
    }
    ///Gets the raw value of [`Self::swapchain_count`]
    pub fn swapchain_count_raw(&self) -> u32 {
        self.swapchain_count
    }
    ///Gets the raw value of [`Self::swapchains`]
    pub fn swapchains_raw(&self) -> *const SwapchainKHR {
        self.swapchains
    }
    ///Gets the raw value of [`Self::image_indices`]
    pub fn image_indices_raw(&self) -> *const u32 {
        self.image_indices
    }
    ///Gets the raw value of [`Self::results`]
    pub fn results_raw(&self) -> &*mut VulkanResultCodes {
        &self.results
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_count`]
    pub fn set_wait_semaphore_count_raw(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphores`]
    pub fn set_wait_semaphores_raw(&mut self, value: *const Semaphore) -> &mut Self {
        self.wait_semaphores = value;
        self
    }
    ///Sets the raw value of [`Self::swapchain_count`]
    pub fn set_swapchain_count_raw(&mut self, value: u32) -> &mut Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the raw value of [`Self::swapchains`]
    pub fn set_swapchains_raw(&mut self, value: *const SwapchainKHR) -> &mut Self {
        self.swapchains = value;
        self
    }
    ///Sets the raw value of [`Self::image_indices`]
    pub fn set_image_indices_raw(&mut self, value: *const u32) -> &mut Self {
        self.image_indices = value;
        self
    }
    ///Sets the raw value of [`Self::results`]
    pub fn set_results_raw(&mut self, value: *mut VulkanResultCodes) -> &mut Self {
        self.results = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::wait_semaphore_count`]
    pub fn wait_semaphore_count(&self) -> u32 {
        self.wait_semaphore_count
    }
    ///Gets the value of [`Self::wait_semaphores`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn wait_semaphores(&self) -> &[Semaphore] {
        std::slice::from_raw_parts(self.wait_semaphores, self.wait_semaphore_count as usize)
    }
    ///Gets the value of [`Self::swapchain_count`]
    pub fn swapchain_count(&self) -> u32 {
        self.swapchain_count
    }
    ///Gets the value of [`Self::swapchains`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn swapchains(&self) -> &[SwapchainKHR] {
        std::slice::from_raw_parts(self.swapchains, self.swapchain_count as usize)
    }
    ///Gets the value of [`Self::image_indices`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn image_indices(&self) -> &[u32] {
        std::slice::from_raw_parts(self.image_indices, self.swapchain_count as usize)
    }
    ///Gets the value of [`Self::results`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn results(&self) -> &[VulkanResultCodes] {
        std::slice::from_raw_parts(self.results, self.swapchain_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::wait_semaphore_count`]
    pub fn wait_semaphore_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::swapchain_count`]
    pub fn swapchain_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::results`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn results_mut(&mut self) -> &mut [VulkanResultCodes] {
        std::slice::from_raw_parts_mut(self.results, self.swapchain_count as usize)
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphore_count`]
    pub fn set_wait_semaphore_count(&mut self, value: u32) -> &mut Self {
        self.wait_semaphore_count = value;
        self
    }
    ///Sets the raw value of [`Self::wait_semaphores`]
    pub fn set_wait_semaphores(&mut self, value: &'lt [crate::vulkan1_0::Semaphore]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.wait_semaphores = value.as_ptr();
        self.wait_semaphore_count = len_;
        self
    }
    ///Sets the raw value of [`Self::swapchain_count`]
    pub fn set_swapchain_count(&mut self, value: u32) -> &mut Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the raw value of [`Self::swapchains`]
    pub fn set_swapchains(&mut self, value: &'lt [crate::extensions::khr_swapchain::SwapchainKHR]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.swapchains = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the raw value of [`Self::image_indices`]
    pub fn set_image_indices(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.image_indices = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the raw value of [`Self::results`]
    pub fn set_results(&mut self, value: &'lt mut [crate::vulkan1_0::VulkanResultCodes]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.results = value.as_mut_ptr();
        self.swapchain_count = len_;
        self
    }
}
