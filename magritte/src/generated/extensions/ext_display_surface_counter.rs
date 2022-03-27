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
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SurfaceCapabilities2EXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseOutStructure<'lt>,
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
impl<'lt> Default for SurfaceCapabilities2EXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            min_image_count: 0,
            max_image_count: 0,
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: 0,
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
            supported_surface_counters: Default::default(),
        }
    }
}
impl<'lt> SurfaceCapabilities2EXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::min_image_count`]
    pub fn min_image_count_raw(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the raw value of [`Self::max_image_count`]
    pub fn max_image_count_raw(&self) -> u32 {
        self.max_image_count
    }
    ///Gets the raw value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers_raw(&self) -> u32 {
        self.max_image_array_layers
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::min_image_count`]
    pub fn set_min_image_count_raw(&mut self, value: u32) -> &mut Self {
        self.min_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_count`]
    pub fn set_max_image_count_raw(&mut self, value: u32) -> &mut Self {
        self.max_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_array_layers`]
    pub fn set_max_image_array_layers_raw(&mut self, value: u32) -> &mut Self {
        self.max_image_array_layers = value;
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
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::min_image_count`]
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the value of [`Self::max_image_count`]
    pub fn max_image_count(&self) -> u32 {
        self.max_image_count
    }
    ///Gets the value of [`Self::current_extent`]
    pub fn current_extent(&self) -> Extent2D {
        self.current_extent
    }
    ///Gets the value of [`Self::min_image_extent`]
    pub fn min_image_extent(&self) -> Extent2D {
        self.min_image_extent
    }
    ///Gets the value of [`Self::max_image_extent`]
    pub fn max_image_extent(&self) -> Extent2D {
        self.max_image_extent
    }
    ///Gets the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers(&self) -> u32 {
        self.max_image_array_layers
    }
    ///Gets the value of [`Self::supported_transforms`]
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Gets the value of [`Self::current_transform`]
    pub fn current_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.current_transform
    }
    ///Gets the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha(&self) -> CompositeAlphaFlagsKHR {
        self.supported_composite_alpha
    }
    ///Gets the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags(&self) -> ImageUsageFlags {
        self.supported_usage_flags
    }
    ///Gets the value of [`Self::supported_surface_counters`]
    pub fn supported_surface_counters(&self) -> SurfaceCounterFlagsEXT {
        self.supported_surface_counters
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::min_image_count`]
    pub fn min_image_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::max_image_count`]
    pub fn max_image_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::current_extent`]
    pub fn current_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.current_extent
    }
    ///Gets a mutable reference to the value of [`Self::min_image_extent`]
    pub fn min_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_extent`]
    pub fn max_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::supported_transforms`]
    pub fn supported_transforms_mut(&mut self) -> &mut SurfaceTransformFlagsKHR {
        &mut self.supported_transforms
    }
    ///Gets a mutable reference to the value of [`Self::current_transform`]
    pub fn current_transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.current_transform
    }
    ///Gets a mutable reference to the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha_mut(&mut self) -> &mut CompositeAlphaFlagsKHR {
        &mut self.supported_composite_alpha
    }
    ///Gets a mutable reference to the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.supported_usage_flags
    }
    ///Gets a mutable reference to the value of [`Self::supported_surface_counters`]
    pub fn supported_surface_counters_mut(&mut self) -> &mut SurfaceCounterFlagsEXT {
        &mut self.supported_surface_counters
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::min_image_count`]
    pub fn set_min_image_count(&mut self, value: u32) -> &mut Self {
        self.min_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_count`]
    pub fn set_max_image_count(&mut self, value: u32) -> &mut Self {
        self.max_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::current_extent`]
    pub fn set_current_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.current_extent = value;
        self
    }
    ///Sets the raw value of [`Self::min_image_extent`]
    pub fn set_min_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_extent`]
    pub fn set_max_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_array_layers`]
    pub fn set_max_image_array_layers(&mut self, value: u32) -> &mut Self {
        self.max_image_array_layers = value;
        self
    }
    ///Sets the raw value of [`Self::supported_transforms`]
    pub fn set_supported_transforms(
        &mut self,
        value: crate::extensions::khr_display::SurfaceTransformFlagsKHR,
    ) -> &mut Self {
        self.supported_transforms = value;
        self
    }
    ///Sets the raw value of [`Self::current_transform`]
    pub fn set_current_transform(
        &mut self,
        value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> &mut Self {
        self.current_transform = value;
        self
    }
    ///Sets the raw value of [`Self::supported_composite_alpha`]
    pub fn set_supported_composite_alpha(
        &mut self,
        value: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    ) -> &mut Self {
        self.supported_composite_alpha = value;
        self
    }
    ///Sets the raw value of [`Self::supported_usage_flags`]
    pub fn set_supported_usage_flags(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.supported_usage_flags = value;
        self
    }
    ///Sets the raw value of [`Self::supported_surface_counters`]
    pub fn set_supported_surface_counters(
        &mut self,
        value: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
    ) -> &mut Self {
        self.supported_surface_counters = value;
        self
    }
}
