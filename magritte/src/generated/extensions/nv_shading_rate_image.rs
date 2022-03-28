//![VK_NV_shading_rate_image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_shading_rate_image.html) - device extension
//!# Description
//!This extension allows applications to use a variable shading rate when
//!processing fragments of rasterized primitives.
//!By default, Vulkan will spawn one fragment shader for each pixel covered by
//!a primitive.
//!In this extension, applications can bind a *shading rate image* that can be
//!used to vary the number of fragment shader invocations across the
//!framebuffer.
//!Some portions of the screen may be configured to spawn up to 16 fragment
//!shaders for each pixel, while other portions may use a single fragment
//!shader invocation for a 4x4 block of pixels.
//!This can be useful for use cases like eye tracking, where the portion of the
//!framebuffer that the user is looking at directly can be processed at high
//!frequency, while distant corners of the image can be processed at lower
//!frequency.
//!Each texel in the shading rate image represents a fixed-size rectangle in
//!the framebuffer, covering 16x16 pixels in the initial implementation of this
//!extension.
//!When rasterizing a primitive covering one of these rectangles, the Vulkan
//!implementation reads a texel in the bound shading rate image and looks up
//!the fetched value in a palette to determine a base shading rate.In addition to the API support
//! controlling rasterization, this extension
//!also adds Vulkan support for the
//![`SPV_NV_shading_rate`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shading_rate.html) extension to
//!SPIR-V.
//!That extension provides two fragment shader variable decorations that allow
//!fragment shaders to determine the shading rate used for processing the
//!fragment:
//! - `FragmentSizeNV`, which indicates the width and height of the set of pixels processed by the
//!   fragment shader.
//! - `InvocationsPerPixel`, which indicates the maximum number of fragment shader invocations that
//!   could be spawned for the pixel(s) covered by the fragment.
//!When using SPIR-V in conjunction with the OpenGL Shading Language (GLSL),
//!the fragment shader capabilities are provided by the
//!`GL_NV_shading_rate_image` language extension and correspond to the built-in
//!variables `gl_FragmentSizeNV` and `gl_InvocationsPerPixelNV`,
//!respectively.
//!# Revision
//!3
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_shading_rate_image]
//!   @nvpbrown%0A<<Here describe the issue or question you have about the VK_NV_shading_rate_image
//!   extension>>)
//!# New functions & commands
//! - [`CmdBindShadingRateImageNV`]
//! - [`CmdSetCoarseSampleOrderNV`]
//! - [`CmdSetViewportShadingRatePaletteNV`]
//!# New structures
//! - [`CoarseSampleLocationNV`]
//! - [`CoarseSampleOrderCustomNV`]
//! - [`ShadingRatePaletteNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceShadingRateImageFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceShadingRateImagePropertiesNV`]
//! - Extending [`PipelineViewportStateCreateInfo`]:  -
//!   [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]  -
//!   [`PipelineViewportShadingRateImageStateCreateInfoNV`]
//!# New enums
//! - [`CoarseSampleOrderTypeNV`]
//! - [`ShadingRatePaletteEntryNV`]
//!# New constants
//! - [`NV_SHADING_RATE_IMAGE_EXTENSION_NAME`]
//! - [`NV_SHADING_RATE_IMAGE_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV`
//! - Extending [`DynamicState`]:  - `VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV`  -
//!   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV`
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV`
//! - Extending [`ImageUsageFlagBits`]:  - `VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!(1) When using shading rates specifying “coarse” fragments covering
//!    multiple pixels, we will generate a combined coverage mask that combines
//!    the coverage masks of all pixels covered by the fragment.
//!    By default, these masks are combined in an implementation-dependent
//!    order.
//!    Should we provide a mechanism allowing applications to query or specify
//!    an exact order? **RESOLVED** : Yes, this feature is useful for cases where most of the
//! fragment
//!shader can be evaluated once for an entire coarse fragment, but where some
//!per-pixel computations are also required.
//!For example, a per-pixel alpha test may want to kill all the samples for
//!some pixels in a coarse fragment.
//!This sort of test can be implemented using an output sample mask, but such a
//!shader would need to know which bit in the mask corresponds to each sample
//!in the coarse fragment.
//!We are including a mechanism to allow aplications to specify the orders of
//!coverage samples for each shading rate and sample count, either as static
//!pipeline state or dynamically via a command buffer.
//!This portion of the extension has its own feature bit.We will not be providing a query to
//! determine the implementation-dependent
//!default ordering.
//!The thinking here is that if an application cares enough about the coarse
//!fragment sample ordering to perform such a query, it could instead just set
//!its own order, also using custom per-pixel sample locations if required.(2) For the pipeline
//! stage
//!`VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`, should we specify a
//!precise location in the pipeline the shading rate image is accessed (after
//!geometry shading, but before the early fragment tests) or leave it
//!under-specified in case there are other implementations that access the
//!image in a different pipeline location? **RESOLVED**  We are specifying the pipeline stage to be
//! between the final
//![pre-rasterization shader
//!stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization) (`VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`) and before the first
//!stage used for fragment processing
//!(`VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT`), which seems to be the
//!natural place to access the shading rate image.(3) How do centroid-sampled variables work with
//! fragments larger than one
//!pixel? **RESOLVED**  For single-pixel fragments, fragment shader inputs decorated with
//!`Centroid` are sampled at an implementation-dependent location in the
//!intersection of the area of the primitive being rasterized and the area of
//!the pixel that corresponds to the fragment.
//!With multi-pixel fragments, we follow a similar pattern, using the
//!intersection of the primitive and the  **set**  of pixels corresponding to the
//!fragment.One important thing to keep in mind when using such “coarse” shading rates
//!is that fragment attributes are sampled at the center of the fragment by
//!default, regardless of the set of pixels/samples covered by the fragment.
//!For fragments with a size of 4x4 pixels, this center location will be more
//!than two pixels (1.5 * sqrt(2)) away from the center of the pixels at the
//!corners of the fragment.
//!When rendering a primitive that covers only a small part of a coarse
//!fragment, sampling a color outside the primitive can produce overly bright
//!or dark color values if the color values have a large gradient.
//!To deal with this, an application can use centroid sampling on attributes
//!where “extrapolation” artifacts can lead to overly bright or dark pixels.
//!Note that this same problem also exists for multisampling with single-pixel
//!fragments, but is less severe because it only affects certain samples of a
//!pixel and such bright/dark samples may be averaged with other samples that
//!do not have a similar problem.
//!# Version History
//! - Revision 3, 2019-07-18 (Mathias Schott)  - Fully list extension interfaces in this appendix.
//! - Revision 2, 2018-09-13 (Pat Brown)  - Miscellaneous edits preparing the specification for
//!   publication.
//! - Revision 1, 2018-08-08 (Pat Brown)  - Internal revisions
//!# Other info
//! * 2019-07-18
//! * - This extension requires [`SPV_NV_shading_rate`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shading_rate.html)
//!   - This extension provides API support for [`GL_NV_shading_rate_image`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_shading_rate_image.txt)
//! * - Pat Brown, NVIDIA  - Carsten Rohde, NVIDIA  - Jeff Bolz, NVIDIA  - Daniel Koch, NVIDIA  -
//!   Mathias Schott, NVIDIA  - Matthew Netsch, Qualcomm Technologies, Inc.
//!# Related
//! - [`CoarseSampleLocationNV`]
//! - [`CoarseSampleOrderCustomNV`]
//! - [`CoarseSampleOrderTypeNV`]
//! - [`PhysicalDeviceShadingRateImageFeaturesNV`]
//! - [`PhysicalDeviceShadingRateImagePropertiesNV`]
//! - [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
//! - [`PipelineViewportShadingRateImageStateCreateInfoNV`]
//! - [`ShadingRatePaletteEntryNV`]
//! - [`ShadingRatePaletteNV`]
//! - [`CmdBindShadingRateImageNV`]
//! - [`CmdSetCoarseSampleOrderNV`]
//! - [`CmdSetViewportShadingRatePaletteNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
#[doc(alias = "VkShadingRatePaletteEntryNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
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
        Self::ShadingRatePaletteEntryNoInvocationsNv
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
#[doc(alias = "VkCoarseSampleOrderTypeNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
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
        Self::CoarseSampleOrderTypeDefaultNv
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
/// - [`shading_rate_palette_entries`] is a pointer to an array of [`ShadingRatePaletteEntryNV`]
///   enums defining the shading rate for each palette entry.
///# Description
///## Valid Usage
/// - [`shading_rate_palette_entry_count`] **must**  be between `1` and
///   [`PhysicalDeviceShadingRateImagePropertiesNV::shading_rate_palette_size`], inclusive
///
///## Valid Usage (Implicit)
/// - [`shading_rate_palette_entries`] **must**  be a valid pointer to an array of
///   [`shading_rate_palette_entry_count`] valid [`ShadingRatePaletteEntryNV`] values
/// - [`shading_rate_palette_entry_count`] **must**  be greater than `0`
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
#[doc(alias = "VkShadingRatePaletteNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ShadingRatePaletteNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`shading_rate_palette_entry_count`] specifies the number of entries in
    ///the shading rate image palette.
    pub shading_rate_palette_entry_count: u32,
    ///[`shading_rate_palette_entries`] is a pointer to an array of
    ///[`ShadingRatePaletteEntryNV`] enums defining the shading rate for
    ///each palette entry.
    pub shading_rate_palette_entries: *const ShadingRatePaletteEntryNV,
}
impl<'lt> Default for ShadingRatePaletteNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            shading_rate_palette_entry_count: 0,
            shading_rate_palette_entries: std::ptr::null(),
        }
    }
}
impl<'lt> ShadingRatePaletteNV<'lt> {
    ///Gets the raw value of [`Self::shading_rate_palette_entries`]
    pub fn shading_rate_palette_entries_raw(&self) -> *const ShadingRatePaletteEntryNV {
        self.shading_rate_palette_entries
    }
    ///Sets the raw value of [`Self::shading_rate_palette_entries`]
    pub fn set_shading_rate_palette_entries_raw(&mut self, value: *const ShadingRatePaletteEntryNV) -> &mut Self {
        self.shading_rate_palette_entries = value;
        self
    }
    ///Gets the value of [`Self::shading_rate_palette_entry_count`]
    pub fn shading_rate_palette_entry_count(&self) -> u32 {
        self.shading_rate_palette_entry_count
    }
    ///Gets the value of [`Self::shading_rate_palette_entries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn shading_rate_palette_entries(&self) -> &[ShadingRatePaletteEntryNV] {
        std::slice::from_raw_parts(
            self.shading_rate_palette_entries,
            self.shading_rate_palette_entry_count as usize,
        )
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_palette_entry_count`]
    pub fn shading_rate_palette_entry_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::shading_rate_palette_entry_count`]
    pub fn set_shading_rate_palette_entry_count(&mut self, value: u32) -> &mut Self {
        self.shading_rate_palette_entry_count = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_palette_entries`]
    pub fn set_shading_rate_palette_entries(
        &mut self,
        value: &'lt [crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.shading_rate_palette_entries = value.as_ptr();
        self.shading_rate_palette_entry_count = len_;
        self
    }
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
/// - [`shading_rate_palettes`] is a pointer to an array of [`ShadingRatePaletteNV`] structures
///   defining the palette for each viewport. If the shading rate palette state is dynamic, this
///   member is ignored.
///# Description
///If this structure is not present, [`shading_rate_image_enable`] is considered
///to be [`FALSE`], and the shading rate image and palettes are not used.
///## Valid Usage
/// - If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport)
///   feature is not enabled, [`viewport_count`] **must**  be `0` or `1`
/// - [`viewport_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_viewports`]
/// - If [`shading_rate_image_enable`] is [`TRUE`], [`viewport_count`] **must**  be greater or equal
///   to the [`viewport_count`] member of [`PipelineViewportStateCreateInfo`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
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
#[doc(alias = "VkPipelineViewportShadingRateImageStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`shading_rate_image_enable`] specifies whether shading rate image and
    ///palettes are used during rasterization.
    pub shading_rate_image_enable: Bool32,
    ///[`viewport_count`] specifies the number of per-viewport palettes used
    ///to translate values stored in shading rate images.
    pub viewport_count: u32,
    ///[`shading_rate_palettes`] is a pointer to an array of
    ///[`ShadingRatePaletteNV`] structures defining the palette for each
    ///viewport.
    ///If the shading rate palette state is dynamic, this member is ignored.
    pub shading_rate_palettes: *const ShadingRatePaletteNV<'lt>,
}
impl<'lt> Default for PipelineViewportShadingRateImageStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            shading_rate_image_enable: 0,
            viewport_count: 0,
            shading_rate_palettes: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineViewportShadingRateImageStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::shading_rate_image_enable`]
    pub fn shading_rate_image_enable_raw(&self) -> Bool32 {
        self.shading_rate_image_enable
    }
    ///Gets the raw value of [`Self::shading_rate_palettes`]
    pub fn shading_rate_palettes_raw(&self) -> *const ShadingRatePaletteNV<'lt> {
        self.shading_rate_palettes
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_image_enable`]
    pub fn set_shading_rate_image_enable_raw(&mut self, value: Bool32) -> &mut Self {
        self.shading_rate_image_enable = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_palettes`]
    pub fn set_shading_rate_palettes_raw(&mut self, value: *const ShadingRatePaletteNV<'lt>) -> &mut Self {
        self.shading_rate_palettes = value;
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
    ///Gets the value of [`Self::shading_rate_image_enable`]
    pub fn shading_rate_image_enable(&self) -> bool {
        unsafe { std::mem::transmute(self.shading_rate_image_enable as u8) }
    }
    ///Gets the value of [`Self::viewport_count`]
    pub fn viewport_count(&self) -> u32 {
        self.viewport_count
    }
    ///Gets the value of [`Self::shading_rate_palettes`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn shading_rate_palettes(&self) -> &[ShadingRatePaletteNV<'lt>] {
        std::slice::from_raw_parts(self.shading_rate_palettes, self.viewport_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_image_enable`]
    pub fn shading_rate_image_enable_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shading_rate_image_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shading_rate_image_enable as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::viewport_count`]
    pub fn viewport_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::shading_rate_image_enable`]
    pub fn set_shading_rate_image_enable(&mut self, value: bool) -> &mut Self {
        self.shading_rate_image_enable = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::viewport_count`]
    pub fn set_viewport_count(&mut self, value: u32) -> &mut Self {
        self.viewport_count = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_palettes`]
    pub fn set_shading_rate_palettes(
        &mut self,
        value: &'lt [crate::extensions::nv_shading_rate_image::ShadingRatePaletteNV<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.shading_rate_palettes = value.as_ptr();
        self.viewport_count = len_;
        self
    }
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
///[`PhysicalDeviceShadingRateImageFeaturesNV`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`
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
#[doc(alias = "VkPhysicalDeviceShadingRateImageFeaturesNV")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImageFeaturesNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shading_rate_image`] indicates that the
    ///implementation supports the use of a shading rate image to derive an
    ///effective shading rate for fragment processing.
    ///It also indicates that the implementation supports the
    ///`ShadingRateNV` SPIR-V execution mode.
    pub shading_rate_image: Bool32,
    ///[`shading_rate_coarse_sample_order`] indicates that the implementation
    ///supports a user-configurable ordering of coverage samples in fragments
    ///larger than one pixel.
    pub shading_rate_coarse_sample_order: Bool32,
}
impl<'lt> Default for PhysicalDeviceShadingRateImageFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shading_rate_image: 0,
            shading_rate_coarse_sample_order: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShadingRateImageFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::shading_rate_image`]
    pub fn shading_rate_image_raw(&self) -> Bool32 {
        self.shading_rate_image
    }
    ///Gets the raw value of [`Self::shading_rate_coarse_sample_order`]
    pub fn shading_rate_coarse_sample_order_raw(&self) -> Bool32 {
        self.shading_rate_coarse_sample_order
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_image`]
    pub fn set_shading_rate_image_raw(&mut self, value: Bool32) -> &mut Self {
        self.shading_rate_image = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_coarse_sample_order`]
    pub fn set_shading_rate_coarse_sample_order_raw(&mut self, value: Bool32) -> &mut Self {
        self.shading_rate_coarse_sample_order = value;
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
    ///Gets the value of [`Self::shading_rate_image`]
    pub fn shading_rate_image(&self) -> bool {
        unsafe { std::mem::transmute(self.shading_rate_image as u8) }
    }
    ///Gets the value of [`Self::shading_rate_coarse_sample_order`]
    pub fn shading_rate_coarse_sample_order(&self) -> bool {
        unsafe { std::mem::transmute(self.shading_rate_coarse_sample_order as u8) }
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
    ///Gets a mutable reference to the value of [`Self::shading_rate_image`]
    pub fn shading_rate_image_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shading_rate_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shading_rate_image as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_coarse_sample_order`]
    pub fn shading_rate_coarse_sample_order_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.shading_rate_coarse_sample_order as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.shading_rate_coarse_sample_order as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the raw value of [`Self::shading_rate_image`]
    pub fn set_shading_rate_image(&mut self, value: bool) -> &mut Self {
        self.shading_rate_image = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_coarse_sample_order`]
    pub fn set_shading_rate_coarse_sample_order(&mut self, value: bool) -> &mut Self {
        self.shading_rate_coarse_sample_order = value as u8 as u32;
        self
    }
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
///rate image](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-shading-rate-image) feature.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`
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
#[doc(alias = "VkPhysicalDeviceShadingRateImagePropertiesNV")]
#[derive(Debug, Eq, Ord, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceShadingRateImagePropertiesNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shading_rate_texel_size`] indicates the
    ///width and height of the portion of the framebuffer corresponding to each
    ///texel in the shading rate image.
    pub shading_rate_texel_size: Extent2D,
    ///[`shading_rate_palette_size`] indicates
    ///the maximum number of palette entries supported for the shading rate
    ///image.
    pub shading_rate_palette_size: u32,
    ///[`shading_rate_max_coarse_samples`]
    ///specifies the maximum number of coverage samples supported in a single
    ///fragment.
    ///If the product of the fragment size derived from the base shading rate
    ///and the number of coverage samples per pixel exceeds this limit, the
    ///final shading rate will be adjusted so that its product does not exceed
    ///the limit.
    pub shading_rate_max_coarse_samples: u32,
}
impl<'lt> Default for PhysicalDeviceShadingRateImagePropertiesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            shading_rate_texel_size: Default::default(),
            shading_rate_palette_size: 0,
            shading_rate_max_coarse_samples: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShadingRateImagePropertiesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
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
    ///Gets the value of [`Self::shading_rate_texel_size`]
    pub fn shading_rate_texel_size(&self) -> Extent2D {
        self.shading_rate_texel_size
    }
    ///Gets the value of [`Self::shading_rate_palette_size`]
    pub fn shading_rate_palette_size(&self) -> u32 {
        self.shading_rate_palette_size
    }
    ///Gets the value of [`Self::shading_rate_max_coarse_samples`]
    pub fn shading_rate_max_coarse_samples(&self) -> u32 {
        self.shading_rate_max_coarse_samples
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
    ///Gets a mutable reference to the value of [`Self::shading_rate_texel_size`]
    pub fn shading_rate_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.shading_rate_texel_size
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_palette_size`]
    pub fn shading_rate_palette_size_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate_max_coarse_samples`]
    pub fn shading_rate_max_coarse_samples_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::shading_rate_texel_size`]
    pub fn set_shading_rate_texel_size(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.shading_rate_texel_size = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_palette_size`]
    pub fn set_shading_rate_palette_size(&mut self, value: u32) -> &mut Self {
        self.shading_rate_palette_size = value;
        self
    }
    ///Sets the raw value of [`Self::shading_rate_max_coarse_samples`]
    pub fn set_shading_rate_max_coarse_samples(&mut self, value: u32) -> &mut Self {
        self.shading_rate_max_coarse_samples = value;
        self
    }
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
///## Valid Usage
/// - [`pixel_x`] **must**  be less than the width (in pixels) of the fragment
/// - [`pixel_y`] **must**  be less than the height (in pixels) of the fragment
/// - [`sample`] **must**  be less than the number of coverage samples in each pixel belonging to
///   the fragment
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
#[doc(alias = "VkCoarseSampleLocationNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CoarseSampleLocationNV {
    ///[`pixel_x`] is added to the x coordinate of the upper-leftmost pixel of
    ///each fragment to identify the pixel containing the coverage sample.
    pub pixel_x: u32,
    ///[`pixel_y`] is added to the y coordinate of the upper-leftmost pixel of
    ///each fragment to identify the pixel containing the coverage sample.
    pub pixel_y: u32,
    ///[`sample`] is the number of the coverage sample in the pixel
    ///identified by [`pixel_x`] and [`pixel_y`].
    pub sample: u32,
}
impl Default for CoarseSampleLocationNV {
    fn default() -> Self {
        Self {
            pixel_x: 0,
            pixel_y: 0,
            sample: 0,
        }
    }
}
impl CoarseSampleLocationNV {
    ///Gets the value of [`Self::pixel_x`]
    pub fn pixel_x(&self) -> u32 {
        self.pixel_x
    }
    ///Gets the value of [`Self::pixel_y`]
    pub fn pixel_y(&self) -> u32 {
        self.pixel_y
    }
    ///Gets the value of [`Self::sample`]
    pub fn sample(&self) -> u32 {
        self.sample
    }
    ///Gets a mutable reference to the value of [`Self::pixel_x`]
    pub fn pixel_x_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::pixel_y`]
    pub fn pixel_y_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::sample`]
    pub fn sample_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::pixel_x`]
    pub fn set_pixel_x(&mut self, value: u32) -> &mut Self {
        self.pixel_x = value;
        self
    }
    ///Sets the raw value of [`Self::pixel_y`]
    pub fn set_pixel_y(&mut self, value: u32) -> &mut Self {
        self.pixel_y = value;
        self
    }
    ///Sets the raw value of [`Self::sample`]
    pub fn set_sample(&mut self, value: u32) -> &mut Self {
        self.sample = value;
        self
    }
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
/// - [`sample_locations`] is a pointer to an array of [`CoarseSampleLocationNV`] structures
///   specifying the location of each sample in the custom ordering.
///# Description
///The [`CoarseSampleOrderCustomNV`] structure is used with a coverage
///sample ordering type of `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV` to
///specify the order of coverage samples for one combination of fragment width,
///fragment height, and coverage sample count.When using a custom sample ordering, element *j* in
/// [`sample_locations`]
///specifies a specific pixel location and
///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) that corresponds to
///[coverage index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)*j* in the
///multi-pixel fragment.
///## Valid Usage
/// - [`shading_rate`] **must**  be a shading rate that generates fragments with more than one pixel
/// - [`sample_count`] **must**  correspond to a sample count enumerated in [`SampleCountFlags`]
///   whose corresponding bit is set in
///   [`PhysicalDeviceLimits::framebuffer_no_attachments_sample_counts`]
/// - [`sample_location_count`] **must**  be equal to the product of [`sample_count`], the fragment
///   width for [`shading_rate`], and the fragment height for [`shading_rate`]
/// - [`sample_location_count`] **must**  be less than or equal to the value of
///   [`PhysicalDeviceShadingRateImagePropertiesNV::shading_rate_max_coarse_samples`]
/// - The array [`sample_locations`] **must**  contain exactly one entry for every combination of
///   valid values for `pixelX`, `pixelY`, and `sample` in the structure
///   [`CoarseSampleOrderCustomNV`]
///
///## Valid Usage (Implicit)
/// - [`shading_rate`] **must**  be a valid [`ShadingRatePaletteEntryNV`] value
/// - [`sample_locations`] **must**  be a valid pointer to an array of
///   [`sample_location_count`][`CoarseSampleLocationNV`] structures
/// - [`sample_location_count`] **must**  be greater than `0`
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
#[doc(alias = "VkCoarseSampleOrderCustomNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CoarseSampleOrderCustomNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`shading_rate`] is a shading rate palette entry that identifies the
    ///fragment width and height for the combination of fragment area and
    ///per-pixel coverage sample count to control.
    pub shading_rate: ShadingRatePaletteEntryNV,
    ///[`sample_count`] identifies the per-pixel coverage sample count for the
    ///combination of fragment area and coverage sample count to control.
    pub sample_count: u32,
    ///[`sample_location_count`] specifies the number of sample locations in
    ///the custom ordering.
    pub sample_location_count: u32,
    ///[`sample_locations`] is a pointer to an array of
    ///[`CoarseSampleLocationNV`] structures specifying the location of
    ///each sample in the custom ordering.
    pub sample_locations: *const CoarseSampleLocationNV,
}
impl<'lt> Default for CoarseSampleOrderCustomNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            shading_rate: Default::default(),
            sample_count: 0,
            sample_location_count: 0,
            sample_locations: std::ptr::null(),
        }
    }
}
impl<'lt> CoarseSampleOrderCustomNV<'lt> {
    ///Gets the raw value of [`Self::sample_locations`]
    pub fn sample_locations_raw(&self) -> *const CoarseSampleLocationNV {
        self.sample_locations
    }
    ///Sets the raw value of [`Self::sample_locations`]
    pub fn set_sample_locations_raw(&mut self, value: *const CoarseSampleLocationNV) -> &mut Self {
        self.sample_locations = value;
        self
    }
    ///Gets the value of [`Self::shading_rate`]
    pub fn shading_rate(&self) -> ShadingRatePaletteEntryNV {
        self.shading_rate
    }
    ///Gets the value of [`Self::sample_count`]
    pub fn sample_count(&self) -> u32 {
        self.sample_count
    }
    ///Gets the value of [`Self::sample_location_count`]
    pub fn sample_location_count(&self) -> u32 {
        self.sample_location_count
    }
    ///Gets the value of [`Self::sample_locations`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn sample_locations(&self) -> &[CoarseSampleLocationNV] {
        std::slice::from_raw_parts(self.sample_locations, self.sample_location_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::shading_rate`]
    pub fn shading_rate_mut(&mut self) -> &mut ShadingRatePaletteEntryNV {
        &mut self.shading_rate
    }
    ///Gets a mutable reference to the value of [`Self::sample_count`]
    pub fn sample_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Gets a mutable reference to the value of [`Self::sample_location_count`]
    pub fn sample_location_count_mut(&mut self) -> &mut u32 {
        &mut getter
    }
    ///Sets the raw value of [`Self::shading_rate`]
    pub fn set_shading_rate(
        &mut self,
        value: crate::extensions::nv_shading_rate_image::ShadingRatePaletteEntryNV,
    ) -> &mut Self {
        self.shading_rate = value;
        self
    }
    ///Sets the raw value of [`Self::sample_count`]
    pub fn set_sample_count(&mut self, value: u32) -> &mut Self {
        self.sample_count = value;
        self
    }
    ///Sets the raw value of [`Self::sample_location_count`]
    pub fn set_sample_location_count(&mut self, value: u32) -> &mut Self {
        self.sample_location_count = value;
        self
    }
    ///Sets the raw value of [`Self::sample_locations`]
    pub fn set_sample_locations(
        &mut self,
        value: &'lt [crate::extensions::nv_shading_rate_image::CoarseSampleLocationNV],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.sample_locations = value.as_ptr();
        self.sample_location_count = len_;
        self
    }
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
/// - [`custom_sample_orders`] is a pointer to an array of
///   [`custom_sample_order_count`][`CoarseSampleOrderCustomNV`] structures, each structure
///   specifying the coverage sample order for a single combination of fragment area and coverage
///   sample count.
///# Description
///If this structure is not present, [`sample_order_type`] is considered to be
///`VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV`.If [`sample_order_type`] is
/// `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`, the
///coverage sample order used for any combination of fragment area and coverage
///sample count not enumerated in [`custom_sample_orders`] will be identical
///to that used for `VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV`.If the pipeline was created with
///`VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV`, the contents of this
///structure (if present) are ignored, and the coverage sample order is instead
///specified by [`CmdSetCoarseSampleOrderNV`].
///## Valid Usage
/// - If [`sample_order_type`] is not `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV`,
///   `customSamplerOrderCount` **must**  be `0`
/// - The array [`custom_sample_orders`] **must**  not contain two structures with matching values
///   for both the `shadingRate` and `sampleCount` members
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`
/// - [`sample_order_type`] **must**  be a valid [`CoarseSampleOrderTypeNV`] value
/// - If [`custom_sample_order_count`] is not `0`, [`custom_sample_orders`] **must**  be a valid
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
#[doc(alias = "VkPipelineViewportCoarseSampleOrderStateCreateInfoNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNV<'lt> {
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`sample_order_type`] specifies the mechanism used to order coverage
    ///samples in fragments larger than one pixel.
    pub sample_order_type: CoarseSampleOrderTypeNV,
    ///[`custom_sample_order_count`] specifies the number of custom sample
    ///orderings to use when ordering coverage samples.
    pub custom_sample_order_count: u32,
    ///[`custom_sample_orders`] is a pointer to an array of
    ///[`custom_sample_order_count`][`CoarseSampleOrderCustomNV`]
    ///structures, each structure specifying the coverage sample order for a
    ///single combination of fragment area and coverage sample count.
    pub custom_sample_orders: *const CoarseSampleOrderCustomNV<'lt>,
}
impl<'lt> Default for PipelineViewportCoarseSampleOrderStateCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            sample_order_type: Default::default(),
            custom_sample_order_count: 0,
            custom_sample_orders: std::ptr::null(),
        }
    }
}
impl<'lt> PipelineViewportCoarseSampleOrderStateCreateInfoNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::custom_sample_orders`]
    pub fn custom_sample_orders_raw(&self) -> *const CoarseSampleOrderCustomNV<'lt> {
        self.custom_sample_orders
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::custom_sample_orders`]
    pub fn set_custom_sample_orders_raw(&mut self, value: *const CoarseSampleOrderCustomNV<'lt>) -> &mut Self {
        self.custom_sample_orders = value;
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
    ///Gets the value of [`Self::sample_order_type`]
    pub fn sample_order_type(&self) -> CoarseSampleOrderTypeNV {
        self.sample_order_type
    }
    ///Gets the value of [`Self::custom_sample_order_count`]
    pub fn custom_sample_order_count(&self) -> u32 {
        self.custom_sample_order_count
    }
    ///Gets the value of [`Self::custom_sample_orders`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn custom_sample_orders(&self) -> &[CoarseSampleOrderCustomNV<'lt>] {
        std::slice::from_raw_parts(self.custom_sample_orders, self.custom_sample_order_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::sample_order_type`]
    pub fn sample_order_type_mut(&mut self) -> &mut CoarseSampleOrderTypeNV {
        &mut self.sample_order_type
    }
    ///Gets a mutable reference to the value of [`Self::custom_sample_order_count`]
    pub fn custom_sample_order_count_mut(&mut self) -> &mut u32 {
        &mut getter
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
    ///Sets the raw value of [`Self::sample_order_type`]
    pub fn set_sample_order_type(
        &mut self,
        value: crate::extensions::nv_shading_rate_image::CoarseSampleOrderTypeNV,
    ) -> &mut Self {
        self.sample_order_type = value;
        self
    }
    ///Sets the raw value of [`Self::custom_sample_order_count`]
    pub fn set_custom_sample_order_count(&mut self, value: u32) -> &mut Self {
        self.custom_sample_order_count = value;
        self
    }
    ///Sets the raw value of [`Self::custom_sample_orders`]
    pub fn set_custom_sample_orders(
        &mut self,
        value: &'lt [crate::extensions::nv_shading_rate_image::CoarseSampleOrderCustomNV<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.custom_sample_orders = value.as_ptr();
        self.custom_sample_order_count = len_;
        self
    }
}
