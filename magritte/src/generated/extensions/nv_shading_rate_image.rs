use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Extent2D, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION")]
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME")]
pub const NV_SHADING_RATE_IMAGE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_shading_rate_image");
///[VkShadingRatePaletteEntryNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteEntryNV.html) - Shading rate image palette entry types
///# C Specifications
///The supported shading rate image palette entries are defined by
///[`ShadingRatePaletteEntryNV`]:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef enum VkShadingRatePaletteEntryNV {
///    VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV = 0,
///    VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV = 1,
///    VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV = 2,
///    VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV = 3,
///    VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV = 4,
///    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV = 5,
///    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV = 6,
///    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV = 7,
///    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV = 8,
///    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV = 9,
///    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV = 10,
///    VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV = 11,
///} VkShadingRatePaletteEntryNV;
///```
///# Description
///The following table indicates the width and height (in pixels) of each
///fragment generated using the indicated shading rate, as well as the maximum
///number of fragment shader invocations launched for each fragment.
///When processing regions of a primitive that have a shading rate of
///[`ShadingRatePaletteEntryNoInvocationsNv`], no fragments will be
///generated in that region.
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`CoarseSampleOrderCustomNV`]
/// - [`ShadingRatePaletteNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShadingRatePaletteEntryNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ShadingRatePaletteEntryNV {
    ///No documentation found
    ShadingRatePaletteEntryNoInvocationsNv = 0,
    ///No documentation found
    ShadingRatePaletteEntry16InvocationsPerPixelNv = 1,
    ///No documentation found
    ShadingRatePaletteEntry8InvocationsPerPixelNv = 2,
    ///No documentation found
    ShadingRatePaletteEntry4InvocationsPerPixelNv = 3,
    ///No documentation found
    ShadingRatePaletteEntry2InvocationsPerPixelNv = 4,
    ///No documentation found
    ShadingRatePaletteEntry1InvocationPerPixelNv = 5,
    ///No documentation found
    ShadingRatePaletteEntry1InvocationPer2X1PixelsNv = 6,
    ///No documentation found
    ShadingRatePaletteEntry1InvocationPer1X2PixelsNv = 7,
    ///No documentation found
    ShadingRatePaletteEntry1InvocationPer2X2PixelsNv = 8,
    ///No documentation found
    ShadingRatePaletteEntry1InvocationPer4X2PixelsNv = 9,
    ///No documentation found
    ShadingRatePaletteEntry1InvocationPer2X4PixelsNv = 10,
    ///No documentation found
    ShadingRatePaletteEntry1InvocationPer4X4PixelsNv = 11,
}
impl const Default for ShadingRatePaletteEntryNV {
    fn default() -> Self {
        ShadingRatePaletteEntryNoInvocationsNv
    }
}
impl ShadingRatePaletteEntryNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkCoarseSampleOrderTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderTypeNV.html) - Shading rate image sample ordering types
///# C Specifications
///The type [`CoarseSampleOrderTypeNV`] specifies the technique used to
///order coverage samples in fragments larger than one pixel, and is defined
///as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef enum VkCoarseSampleOrderTypeNV {
///    VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV = 0,
///    VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV = 1,
///    VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV = 2,
///    VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV = 3,
///} VkCoarseSampleOrderTypeNV;
///```
///# Description
/// - [`CoarseSampleOrderTypeDefaultNv`] specifies that coverage samples will be ordered in an
///   implementation-dependent manner.
/// - [`CoarseSampleOrderTypeCustomNv`] specifies that coverage samples will be ordered according to
///   the array of custom orderings provided in either the `pCustomSampleOrders` member of
///   [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] or the `pCustomSampleOrders` member of
///   [`CmdSetCoarseSampleOrderNV`].
/// - [`CoarseSampleOrderTypePixelMajorNv`] specifies that coverage samples will be ordered sequentially, sorted first by pixel coordinate (in row-major order) and then by [sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask).
/// - [`CoarseSampleOrderTypeSampleMajorNv`] specifies that coverage samples will be ordered sequentially, sorted first by [sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) and then by pixel coordinate (in row-major order).
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
/// - [`CmdSetCoarseSampleOrderNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCoarseSampleOrderTypeNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum CoarseSampleOrderTypeNV {
    ///[`CoarseSampleOrderTypeDefaultNv`] specifies that coverage
    ///samples will be ordered in an implementation-dependent manner.
    CoarseSampleOrderTypeDefaultNv = 0,
    ///[`CoarseSampleOrderTypeCustomNv`] specifies that coverage
    ///samples will be ordered according to the array of custom orderings
    ///provided in either the `pCustomSampleOrders` member of
    ///[`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] or the
    ///`pCustomSampleOrders` member of [`CmdSetCoarseSampleOrderNV`].
    CoarseSampleOrderTypeCustomNv = 1,
    ///[`CoarseSampleOrderTypePixelMajorNv`] specifies that coverage
    ///samples will be ordered sequentially, sorted first by pixel coordinate
    ///(in row-major order) and then by
    ///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask).
    CoarseSampleOrderTypePixelMajorNv = 2,
    ///[`CoarseSampleOrderTypeSampleMajorNv`] specifies that
    ///coverage samples will be ordered sequentially, sorted first by
    ///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) and then by
    ///pixel coordinate (in row-major order).
    CoarseSampleOrderTypeSampleMajorNv = 3,
}
impl const Default for CoarseSampleOrderTypeNV {
    fn default() -> Self {
        CoarseSampleOrderTypeDefaultNv
    }
}
impl CoarseSampleOrderTypeNV {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkShadingRatePaletteNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteNV.html) - Structure specifying a single shading rate palette
///# C Specifications
///The [`ShadingRatePaletteNV`] structure specifies to contents of a single
///shading rate image palette and is defined as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef struct VkShadingRatePaletteNV {
///    uint32_t                              shadingRatePaletteEntryCount;
///    const VkShadingRatePaletteEntryNV*    pShadingRatePaletteEntries;
///} VkShadingRatePaletteNV;
///```
///# Members
/// - [`shading_rate_palette_entry_count`] specifies the number of entries in the shading rate image
///   palette.
/// - [`p_shading_rate_palette_entries`] is a pointer to an array of [`ShadingRatePaletteEntryNV`]
///   enums defining the shading rate for each palette entry.
///# Description
///Valid Usage
/// - [`shading_rate_palette_entry_count`]**must** be between `1` and
///   [`PhysicalDeviceShadingRateImagePropertiesNV::shading_rate_palette_size`], inclusive
///Valid Usage (Implicit)
/// - [`p_shading_rate_palette_entries`]**must** be a valid pointer to an array of
///   [`shading_rate_palette_entry_count`] valid [`ShadingRatePaletteEntryNV`] values
/// - [`shading_rate_palette_entry_count`]**must** be greater than `0`
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`PipelineViewportShadingRateImageStateCreateInfoNV`]
/// - [`ShadingRatePaletteEntryNV`]
/// - [`CmdSetViewportShadingRatePaletteNV`]
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
pub struct ShadingRatePaletteNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`shading_rate_palette_entry_count`] specifies the number of entries in
    ///the shading rate image palette.
    shading_rate_palette_entry_count: u32,
    ///[`p_shading_rate_palette_entries`] is a pointer to an array of
    ///[`ShadingRatePaletteEntryNV`] enums defining the shading rate for
    ///each palette entry.
    p_shading_rate_palette_entries: *mut ShadingRatePaletteEntryNV,
}
///[VkPipelineViewportShadingRateImageStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html) - Structure specifying parameters controlling shading rate image usage
///# C Specifications
///If the [`p_next`] chain of [`PipelineViewportStateCreateInfo`] includes
///a [`PipelineViewportShadingRateImageStateCreateInfoNV`] structure, then
///that structure includes parameters controlling the shading rate.The
/// [`PipelineViewportShadingRateImageStateCreateInfoNV`] structure is
///defined as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkBool32                         shadingRateImageEnable;
///    uint32_t                         viewportCount;
///    const VkShadingRatePaletteNV*    pShadingRatePalettes;
///} VkPipelineViewportShadingRateImageStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shading_rate_image_enable`] specifies whether shading rate image and palettes are used
///   during rasterization.
/// - [`viewport_count`] specifies the number of per-viewport palettes used to translate values
///   stored in shading rate images.
/// - [`p_shading_rate_palettes`] is a pointer to an array of [`ShadingRatePaletteNV`] structures
///   defining the palette for each viewport. If the shading rate palette state is dynamic, this
///   member is ignored.
///# Description
///If this structure is not present, [`shading_rate_image_enable`] is considered
///to be [`FALSE`], and the shading rate image and palettes are not used.Valid Usage
/// - If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport)
///   feature is not enabled, [`viewport_count`]**must** be `0` or `1`
/// - [`viewport_count`]**must** be less than or equal to [`PhysicalDeviceLimits::max_viewports`]
/// - If [`shading_rate_image_enable`] is [`TRUE`], [`viewport_count`]**must** be greater or equal
///   to the [`viewport_count`] member of [`PipelineViewportStateCreateInfo`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`Bool32`]
/// - [`ShadingRatePaletteNV`]
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
pub struct PipelineViewportShadingRateImageStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`shading_rate_image_enable`] specifies whether shading rate image and
    ///palettes are used during rasterization.
    shading_rate_image_enable: Bool32,
    ///[`viewport_count`] specifies the number of per-viewport palettes used
    ///to translate values stored in shading rate images.
    viewport_count: u32,
    ///[`p_shading_rate_palettes`] is a pointer to an array of
    ///[`ShadingRatePaletteNV`] structures defining the palette for each
    ///viewport.
    ///If the shading rate palette state is dynamic, this member is ignored.
    p_shading_rate_palettes: *mut ShadingRatePaletteNV<'lt>,
}
///[VkPhysicalDeviceShadingRateImageFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImageFeaturesNV.html) - Structure describing shading rate image features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShadingRateImageFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef struct VkPhysicalDeviceShadingRateImageFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           shadingRateImage;
///    VkBool32           shadingRateCoarseSampleOrder;
///} VkPhysicalDeviceShadingRateImageFeaturesNV;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shading_rate_image`] indicates that the implementation supports the use of a shading rate
///   image to derive an effective shading rate for fragment processing. It also indicates that the
///   implementation supports the `ShadingRateNV` SPIR-V execution mode.
/// - [`shading_rate_coarse_sample_order`] indicates that the implementation supports a
///   user-configurable ordering of coverage samples in fragments larger than one pixel.
///See [Shading Rate Image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-shading-rate-image) for more
///information.If the [`PhysicalDeviceShadingRateImageFeaturesNV`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceShadingRateImageFeaturesNV`]**can** also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`Bool32`]
/// - [`StructureType`]
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
pub struct PhysicalDeviceShadingRateImageFeaturesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shading_rate_image`] indicates that the
    ///implementation supports the use of a shading rate image to derive an
    ///effective shading rate for fragment processing.
    ///It also indicates that the implementation supports the
    ///`ShadingRateNV` SPIR-V execution mode.
    shading_rate_image: Bool32,
    ///[`shading_rate_coarse_sample_order`] indicates that the implementation
    ///supports a user-configurable ordering of coverage samples in fragments
    ///larger than one pixel.
    shading_rate_coarse_sample_order: Bool32,
}
///[VkPhysicalDeviceShadingRateImagePropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShadingRateImagePropertiesNV.html) - Structure describing shading rate image limits that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShadingRateImagePropertiesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef struct VkPhysicalDeviceShadingRateImagePropertiesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkExtent2D         shadingRateTexelSize;
///    uint32_t           shadingRatePaletteSize;
///    uint32_t           shadingRateMaxCoarseSamples;
///} VkPhysicalDeviceShadingRateImagePropertiesNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shading_rate_texel_size`] indicates the width and height of the portion of the framebuffer
///   corresponding to each texel in the shading rate image.
/// - [`shading_rate_palette_size`] indicates the maximum number of palette entries supported for
///   the shading rate image.
/// - [`shading_rate_max_coarse_samples`] specifies the maximum number of coverage samples supported
///   in a single fragment. If the product of the fragment size derived from the base shading rate
///   and the number of coverage samples per pixel exceeds this limit, the final shading rate will
///   be adjusted so that its product does not exceed the limit.
///# Description
///If the [`PhysicalDeviceShadingRateImagePropertiesNV`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.These properties are related to the [shading
///rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-shading-rate-image) feature.Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`Extent2D`]
/// - [`StructureType`]
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
pub struct PhysicalDeviceShadingRateImagePropertiesNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseOutStructure<'lt>,
    ///[`shading_rate_texel_size`] indicates the
    ///width and height of the portion of the framebuffer corresponding to each
    ///texel in the shading rate image.
    shading_rate_texel_size: Extent2D,
    ///[`shading_rate_palette_size`] indicates
    ///the maximum number of palette entries supported for the shading rate
    ///image.
    shading_rate_palette_size: u32,
    ///[`shading_rate_max_coarse_samples`]
    ///specifies the maximum number of coverage samples supported in a single
    ///fragment.
    ///If the product of the fragment size derived from the base shading rate
    ///and the number of coverage samples per pixel exceeds this limit, the
    ///final shading rate will be adjusted so that its product does not exceed
    ///the limit.
    shading_rate_max_coarse_samples: u32,
}
///[VkCoarseSampleLocationNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleLocationNV.html) - Structure specifying parameters controlling shading rate image usage
///# C Specifications
///The [`CoarseSampleLocationNV`] structure identifies a specific pixel and
///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) for one of the
///coverage samples in a fragment that is larger than one pixel.
///This structure is defined as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef struct VkCoarseSampleLocationNV {
///    uint32_t    pixelX;
///    uint32_t    pixelY;
///    uint32_t    sample;
///} VkCoarseSampleLocationNV;
///```
///# Members
/// - [`pixel_x`] is added to the x coordinate of the upper-leftmost pixel of each fragment to
///   identify the pixel containing the coverage sample.
/// - [`pixel_y`] is added to the y coordinate of the upper-leftmost pixel of each fragment to
///   identify the pixel containing the coverage sample.
/// - [`sample`] is the number of the coverage sample in the pixel identified by [`pixel_x`] and
///   [`pixel_y`].
///# Description
///Valid Usage
/// - [`pixel_x`]**must** be less than the width (in pixels) of the fragment
/// - [`pixel_y`]**must** be less than the height (in pixels) of the fragment
/// - [`sample`]**must** be less than the number of coverage samples in each pixel belonging to the
///   fragment
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`CoarseSampleOrderCustomNV`]
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
pub struct CoarseSampleLocationNV {
    ///[`pixel_x`] is added to the x coordinate of the upper-leftmost pixel of
    ///each fragment to identify the pixel containing the coverage sample.
    pixel_x: u32,
    ///[`pixel_y`] is added to the y coordinate of the upper-leftmost pixel of
    ///each fragment to identify the pixel containing the coverage sample.
    pixel_y: u32,
    ///[`sample`] is the number of the coverage sample in the pixel
    ///identified by [`pixel_x`] and [`pixel_y`].
    sample: u32,
}
///[VkCoarseSampleOrderCustomNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderCustomNV.html) - Structure specifying parameters controlling shading rate image usage
///# C Specifications
///The [`CoarseSampleOrderCustomNV`] structure is defined as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef struct VkCoarseSampleOrderCustomNV {
///    VkShadingRatePaletteEntryNV        shadingRate;
///    uint32_t                           sampleCount;
///    uint32_t                           sampleLocationCount;
///    const VkCoarseSampleLocationNV*    pSampleLocations;
///} VkCoarseSampleOrderCustomNV;
///```
///# Members
/// - [`shading_rate`] is a shading rate palette entry that identifies the fragment width and height
///   for the combination of fragment area and per-pixel coverage sample count to control.
/// - [`sample_count`] identifies the per-pixel coverage sample count for the combination of
///   fragment area and coverage sample count to control.
/// - [`sample_location_count`] specifies the number of sample locations in the custom ordering.
/// - [`p_sample_locations`] is a pointer to an array of [`CoarseSampleLocationNV`] structures
///   specifying the location of each sample in the custom ordering.
///# Description
///The [`CoarseSampleOrderCustomNV`] structure is used with a coverage
///sample ordering type of `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV` to
///specify the order of coverage samples for one combination of fragment width,
///fragment height, and coverage sample count.When using a custom sample ordering, element *j* in
/// [`p_sample_locations`]
///specifies a specific pixel location and
///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) that corresponds to
///[coverage index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)*j* in the
///multi-pixel fragment.Valid Usage
/// - [`shading_rate`]**must** be a shading rate that generates fragments with more than one pixel
/// - [`sample_count`]**must** correspond to a sample count enumerated in [`SampleCountFlags`] whose
///   corresponding bit is set in [`PhysicalDeviceLimits::framebuffer_no_attachments_sample_counts`]
/// - [`sample_location_count`]**must** be equal to the product of [`sample_count`], the fragment
///   width for [`shading_rate`], and the fragment height for [`shading_rate`]
/// - [`sample_location_count`]**must** be less than or equal to the value of
///   [`PhysicalDeviceShadingRateImagePropertiesNV::shading_rate_max_coarse_samples`]
/// - The array [`p_sample_locations`]**must** contain exactly one entry for every combination of
///   valid values for `pixelX`, `pixelY`, and `sample` in the structure
///   [`CoarseSampleOrderCustomNV`]
///Valid Usage (Implicit)
/// - [`shading_rate`]**must** be a valid [`ShadingRatePaletteEntryNV`] value
/// - [`p_sample_locations`]**must** be a valid pointer to an array of
///   [`sample_location_count`][`CoarseSampleLocationNV`] structures
/// - [`sample_location_count`]**must** be greater than `0`
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`CoarseSampleLocationNV`]
/// - [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
/// - [`ShadingRatePaletteEntryNV`]
/// - [`CmdSetCoarseSampleOrderNV`]
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
pub struct CoarseSampleOrderCustomNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`shading_rate`] is a shading rate palette entry that identifies the
    ///fragment width and height for the combination of fragment area and
    ///per-pixel coverage sample count to control.
    shading_rate: ShadingRatePaletteEntryNV,
    ///[`sample_count`] identifies the per-pixel coverage sample count for the
    ///combination of fragment area and coverage sample count to control.
    sample_count: u32,
    ///[`sample_location_count`] specifies the number of sample locations in
    ///the custom ordering.
    sample_location_count: u32,
    ///[`p_sample_locations`] is a pointer to an array of
    ///[`CoarseSampleLocationNV`] structures specifying the location of
    ///each sample in the custom ordering.
    p_sample_locations: *mut CoarseSampleLocationNV,
}
///[VkPipelineViewportCoarseSampleOrderStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportCoarseSampleOrderStateCreateInfoNV.html) - Structure specifying parameters controlling sample order in coarse fragments
///# C Specifications
///If the [`p_next`] chain of [`PipelineViewportStateCreateInfo`] includes
///a [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] structure, then
///that structure includes parameters controlling the order of coverage samples
///in fragments larger than one pixel.The [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
/// structure is
///defined as:
///```c
///// Provided by VK_NV_shading_rate_image
///typedef struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkCoarseSampleOrderTypeNV             sampleOrderType;
///    uint32_t                              customSampleOrderCount;
///    const VkCoarseSampleOrderCustomNV*    pCustomSampleOrders;
///} VkPipelineViewportCoarseSampleOrderStateCreateInfoNV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`sample_order_type`] specifies the mechanism used to order coverage samples in fragments
///   larger than one pixel.
/// - [`custom_sample_order_count`] specifies the number of custom sample orderings to use when
///   ordering coverage samples.
/// - [`p_custom_sample_orders`] is a pointer to an array of
///   [`custom_sample_order_count`][`CoarseSampleOrderCustomNV`] structures, each structure
///   specifying the coverage sample order for a single combination of fragment area and coverage
///   sample count.
///# Description
///If this structure is not present, [`sample_order_type`] is considered to be
///`VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV`.If [`sample_order_type`] is
/// `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`, the
///coverage sample order used for any combination of fragment area and coverage
///sample count not enumerated in [`p_custom_sample_orders`] will be identical
///to that used for `VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV`.If the pipeline was created with
///`VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV`, the contents of this
///structure (if present) are ignored, and the coverage sample order is instead
///specified by [`CmdSetCoarseSampleOrderNV`].Valid Usage
/// - If [`sample_order_type`] is not `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`,
///   `customSamplerOrderCount`**must** be `0`
/// - The array [`p_custom_sample_orders`]**must** not contain two structures with matching values
///   for both the `shadingRate` and `sampleCount` members
///Valid Usage (Implicit)
/// - [`s_type`]**must** be
///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`
/// - [`sample_order_type`]**must** be a valid [`CoarseSampleOrderTypeNV`] value
/// - If [`custom_sample_order_count`] is not `0`, [`p_custom_sample_orders`]**must** be a valid
///   pointer to an array of [`custom_sample_order_count`] valid [`CoarseSampleOrderCustomNV`]
///   structures
///# Related
/// - [`VK_NV_shading_rate_image`]
/// - [`CoarseSampleOrderCustomNV`]
/// - [`CoarseSampleOrderTypeNV`]
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
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *mut BaseInStructure<'lt>,
    ///[`sample_order_type`] specifies the mechanism used to order coverage
    ///samples in fragments larger than one pixel.
    sample_order_type: CoarseSampleOrderTypeNV,
    ///[`custom_sample_order_count`] specifies the number of custom sample
    ///orderings to use when ordering coverage samples.
    custom_sample_order_count: u32,
    ///[`p_custom_sample_orders`] is a pointer to an array of
    ///[`custom_sample_order_count`][`CoarseSampleOrderCustomNV`]
    ///structures, each structure specifying the coverage sample order for a
    ///single combination of fragment area and coverage sample count.
    p_custom_sample_orders: *mut CoarseSampleOrderCustomNV<'lt>,
}
