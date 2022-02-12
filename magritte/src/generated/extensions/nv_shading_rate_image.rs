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
//! - `FragmentSizeNV`, which indicates the width and height of the set of
//!pixels processed by the fragment shader.
//! - `InvocationsPerPixel`, which indicates the maximum number of fragment
//!shader invocations that could be spawned for the pixel(s) covered by the
//!fragment.When using SPIR-V in conjunction with the OpenGL Shading Language (GLSL),
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
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDeviceShadingRateImageFeaturesNV`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDeviceShadingRateImagePropertiesNV`]
//! - Extending [`PipelineViewportStateCreateInfo`]:
//! - [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
//! - [`PipelineViewportShadingRateImageStateCreateInfoNV`]
//!# New enums
//! - [`CoarseSampleOrderTypeNV`]
//! - [`ShadingRatePaletteEntryNV`]
//!# New constants
//! - [`NV_SHADING_RATE_IMAGE_EXTENSION_NAME`]
//! - [`NV_SHADING_RATE_IMAGE_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:
//! - `VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV`
//! - Extending [`DynamicState`]:
//! - `VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV`
//! - `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV`
//! - Extending [`ImageLayout`]:
//! - `VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV`
//! - Extending [`ImageUsageFlagBits`]:
//! - `VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV`
//! - Extending [`PipelineStageFlagBits`]:
//! - `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV`
//! - `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV`
//! - `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`
//!# Known issues & F.A.Q
//!(1) When using shading rates specifying “coarse” fragments covering
//!    multiple pixels, we will generate a combined coverage mask that combines
//!    the coverage masks of all pixels covered by the fragment.
//!    By default, these masks are combined in an implementation-dependent
//!    order.
//!    Should we provide a mechanism allowing applications to query or specify
//!    an exact order?**RESOLVED**: Yes, this feature is useful for cases where most of the fragment
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
//!image in a different pipeline location?**RESOLVED** We are specifying the pipeline stage to be
//! between the final
//![pre-rasterization shader
//!stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization) (`VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`) and before the first
//!stage used for fragment processing
//!(`VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT`), which seems to be the
//!natural place to access the shading rate image.(3) How do centroid-sampled variables work with
//! fragments larger than one
//!pixel?**RESOLVED** For single-pixel fragments, fragment shader inputs decorated with
//!`Centroid` are sampled at an implementation-dependent location in the
//!intersection of the area of the primitive being rasterized and the area of
//!the pixel that corresponds to the fragment.
//!With multi-pixel fragments, we follow a similar pattern, using the
//!intersection of the primitive and the **set** of pixels corresponding to the
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
//! - Revision 3, 2019-07-18 (Mathias Schott)
//! - Fully list extension interfaces in this appendix.
//! - Revision 2, 2018-09-13 (Pat Brown)
//! - Miscellaneous edits preparing the specification for publication.
//! - Revision 1, 2018-08-08 (Pat Brown)
//! - Internal revisions
//!# Other info
//! * 2019-07-18
//!*
//! - This extension requires
//![`SPV_NV_shading_rate`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shading_rate.html)
//! - This extension provides API support for
//![`GL_NV_shading_rate_image`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_shading_rate_image.txt)
//!*
//! - Pat Brown, NVIDIA
//! - Carsten Rohde, NVIDIA
//! - Jeff Bolz, NVIDIA
//! - Daniel Koch, NVIDIA
//! - Mathias Schott, NVIDIA
//! - Matthew Netsch, Qualcomm Technologies, Inc.
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
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
///[`SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS`], no fragments will be
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ShadingRatePaletteEntryNV(i32);
impl const Default for ShadingRatePaletteEntryNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for ShadingRatePaletteEntryNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("ShadingRatePaletteEntryNV")
            .field(match *self {
                Self::SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS => &"SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS",
                Self::SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL => {
                    &"SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL => {
                    &"SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL => {
                    &"SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL => {
                    &"SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL => {
                    &"SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS => {
                    &"SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS => {
                    &"SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS => {
                    &"SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS => {
                    &"SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS => {
                    &"SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS"
                },
                Self::SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS => {
                    &"SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS"
                },
                other => unreachable!("invalid value for `ShadingRatePaletteEntryNV`: {:?}", other),
            })
            .finish()
    }
}
impl ShadingRatePaletteEntryNV {
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS: Self = Self(0);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL: Self = Self(1);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL: Self = Self(2);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL: Self = Self(3);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL: Self = Self(4);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL: Self = Self(5);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS: Self = Self(6);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS: Self = Self(7);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS: Self = Self(8);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS: Self = Self(9);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS: Self = Self(10);
    ///No documentation found
    pub const SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS: Self = Self(11);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
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
/// - [`COARSE_SAMPLE_ORDER_TYPE_DEFAULT`] specifies that coverage
///samples will be ordered in an implementation-dependent manner.
/// - [`COARSE_SAMPLE_ORDER_TYPE_CUSTOM`] specifies that coverage
///samples will be ordered according to the array of custom orderings
///provided in either the `pCustomSampleOrders` member of
///[`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] or the
///`pCustomSampleOrders` member of [`CmdSetCoarseSampleOrderNV`].
/// - [`COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR`] specifies that coverage
///samples will be ordered sequentially, sorted first by pixel coordinate
///(in row-major order) and then by
///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask).
/// - [`COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR`] specifies that
///coverage samples will be ordered sequentially, sorted first by
///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) and then by
///pixel coordinate (in row-major order).
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
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CoarseSampleOrderTypeNV(i32);
impl const Default for CoarseSampleOrderTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for CoarseSampleOrderTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("CoarseSampleOrderTypeNV")
            .field(match *self {
                Self::COARSE_SAMPLE_ORDER_TYPE_DEFAULT => &"COARSE_SAMPLE_ORDER_TYPE_DEFAULT",
                Self::COARSE_SAMPLE_ORDER_TYPE_CUSTOM => &"COARSE_SAMPLE_ORDER_TYPE_CUSTOM",
                Self::COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR => &"COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR",
                Self::COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR => &"COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR",
                other => unreachable!("invalid value for `CoarseSampleOrderTypeNV`: {:?}", other),
            })
            .finish()
    }
}
impl CoarseSampleOrderTypeNV {
    ///[`COARSE_SAMPLE_ORDER_TYPE_DEFAULT`] specifies that coverage
    ///samples will be ordered in an implementation-dependent manner.
    pub const COARSE_SAMPLE_ORDER_TYPE_DEFAULT: Self = Self(0);
    ///[`COARSE_SAMPLE_ORDER_TYPE_CUSTOM`] specifies that coverage
    ///samples will be ordered according to the array of custom orderings
    ///provided in either the `pCustomSampleOrders` member of
    ///[`PipelineViewportCoarseSampleOrderStateCreateInfoNV`] or the
    ///`pCustomSampleOrders` member of [`CmdSetCoarseSampleOrderNV`].
    pub const COARSE_SAMPLE_ORDER_TYPE_CUSTOM: Self = Self(1);
    ///[`COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR`] specifies that coverage
    ///samples will be ordered sequentially, sorted first by pixel coordinate
    ///(in row-major order) and then by
    ///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask).
    pub const COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR: Self = Self(2);
    ///[`COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR`] specifies that
    ///coverage samples will be ordered sequentially, sorted first by
    ///[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) and then by
    ///pixel coordinate (in row-major order).
    pub const COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR: Self = Self(3);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
