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
