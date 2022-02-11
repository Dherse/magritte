#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_fragment_shading_rate_enums");
///[VkFragmentShadingRateNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateNV.html) - Enumeration with fragment shading rates
///# C Specifications
///If the `fragmentShadingRateEnums` feature is enabled, fragment shading
///rates may be specified using the [`FragmentShadingRateNV`] enumerated
///type defined as:
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///typedef enum VkFragmentShadingRateNV {
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV = 0,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV = 1,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV = 4,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV = 5,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV = 6,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV = 9,
///    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV = 10,
///    VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV = 11,
///    VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV = 12,
///    VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV = 13,
///    VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV = 14,
///    VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV = 15,
///} VkFragmentShadingRateNV;
///```
///# Description
/// - [`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL`] specifies a
///fragment size of 1x1 pixels.
/// - [`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS`] specifies
///a fragment size of 1x2 pixels.
/// - [`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS`] specifies
///a fragment size of 2x1 pixels.
/// - [`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS`] specifies
///a fragment size of 2x2 pixels.
/// - [`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS`] specifies
///a fragment size of 2x4 pixels.
/// - [`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS`] specifies
///a fragment size of 4x2 pixels.
/// - [`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS`] specifies
///a fragment size of 4x4 pixels.
/// - [`FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL`] specifies a
///fragment size of 1x1 pixels, with two fragment shader invocations per
///fragment.
/// - [`FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL`] specifies a
///fragment size of 1x1 pixels, with four fragment shader invocations per
///fragment.
/// - [`FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL`] specifies a
///fragment size of 1x1 pixels, with eight fragment shader invocations per
///fragment.
/// - [`FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL`] specifies a
///fragment size of 1x1 pixels, with sixteen fragment shader invocations
///per fragment.
/// - [`FRAGMENT_SHADING_RATE_NO_INVOCATIONS`] specifies that any
///portions of a primitive that use that shading rate should be discarded
///without invoking any fragment shader.To use the shading rates
///[`FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL`],
///[`FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL`],
///[`FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL`], and
///[`FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL`] as a pipeline,
///primitive, or attachment shading rate, the
///`supersampleFragmentShadingRates` feature **must** be enabled.
///To use the shading rate [`FRAGMENT_SHADING_RATE_NO_INVOCATIONS`] as
///a pipeline, primitive, or attachment shading rate, the
///`noInvocationFragmentShadingRates` feature **must** be enabled.
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
/// - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
/// - [`CmdSetFragmentShadingRateEnumNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFragmentShadingRateNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FragmentShadingRateNV(i32);
impl const Default for FragmentShadingRateNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for FragmentShadingRateNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("FragmentShadingRateNV")
            .field(match *self {
                Self::FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL => &"FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL",
                Self::FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS => {
                    &"FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS"
                },
                Self::FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS => {
                    &"FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS"
                },
                Self::FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS => {
                    &"FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS"
                },
                Self::FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS => {
                    &"FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS"
                },
                Self::FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS => {
                    &"FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS"
                },
                Self::FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS => {
                    &"FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS"
                },
                Self::FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL => &"FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL",
                Self::FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL => &"FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL",
                Self::FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL => &"FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL",
                Self::FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL => {
                    &"FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL"
                },
                Self::FRAGMENT_SHADING_RATE_NO_INVOCATIONS => &"FRAGMENT_SHADING_RATE_NO_INVOCATIONS",
                other => unreachable!("invalid value for `FragmentShadingRateNV`: {:?}", other),
            })
            .finish()
    }
}
impl FragmentShadingRateNV {
    ///[`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels.
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL: Self = Self(0);
    ///[`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS`] specifies
    ///a fragment size of 1x2 pixels.
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS: Self = Self(1);
    ///[`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS`] specifies
    ///a fragment size of 2x1 pixels.
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS: Self = Self(4);
    ///[`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS`] specifies
    ///a fragment size of 2x2 pixels.
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS: Self = Self(5);
    ///[`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS`] specifies
    ///a fragment size of 2x4 pixels.
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS: Self = Self(6);
    ///[`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS`] specifies
    ///a fragment size of 4x2 pixels.
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS: Self = Self(9);
    ///[`FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS`] specifies
    ///a fragment size of 4x4 pixels.
    pub const FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS: Self = Self(10);
    ///[`FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with two fragment shader invocations per
    ///fragment.
    pub const FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL: Self = Self(11);
    ///[`FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with four fragment shader invocations per
    ///fragment.
    pub const FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL: Self = Self(12);
    ///[`FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with eight fragment shader invocations per
    ///fragment.
    pub const FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL: Self = Self(13);
    ///[`FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL`] specifies a
    ///fragment size of 1x1 pixels, with sixteen fragment shader invocations
    ///per fragment.
    pub const FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL: Self = Self(14);
    ///[`FRAGMENT_SHADING_RATE_NO_INVOCATIONS`] specifies that any
    ///portions of a primitive that use that shading rate should be discarded
    ///without invoking any fragment shader.
    pub const FRAGMENT_SHADING_RATE_NO_INVOCATIONS: Self = Self(15);
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
///[VkFragmentShadingRateTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateTypeNV.html) - Enumeration with fragment shading rate types
///# C Specifications
///The [`FragmentShadingRateTypeNV`] enumerated type specifies whether a
///graphics pipeline gets its pipeline fragment shading rates and combiners
///from the [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure
///or the [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure.
///```c
///// Provided by VK_NV_fragment_shading_rate_enums
///typedef enum VkFragmentShadingRateTypeNV {
///    VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV = 0,
///    VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV = 1,
///} VkFragmentShadingRateTypeNV;
///```
///# Description
/// - [`FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE`] specifies that a
///graphics pipeline should obtain its pipeline fragment shading rate and
///shading rate combiner state from the
///[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure and that
///any state specified by the
///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure
///should be ignored.
/// - [`FRAGMENT_SHADING_RATE_TYPE_ENUMS`] specifies that a graphics
///pipeline should obtain its pipeline fragment shading rate and shading
///rate combiner state from the
///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure and
///that any state specified by the
///[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure should
///be ignored.
///# Related
/// - [`VK_NV_fragment_shading_rate_enums`]
/// - [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFragmentShadingRateTypeNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct FragmentShadingRateTypeNV(i32);
impl const Default for FragmentShadingRateTypeNV {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for FragmentShadingRateTypeNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("FragmentShadingRateTypeNV")
            .field(match *self {
                Self::FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE => &"FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE",
                Self::FRAGMENT_SHADING_RATE_TYPE_ENUMS => &"FRAGMENT_SHADING_RATE_TYPE_ENUMS",
                other => unreachable!("invalid value for `FragmentShadingRateTypeNV`: {:?}", other),
            })
            .finish()
    }
}
impl FragmentShadingRateTypeNV {
    ///[`FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE`] specifies that a
    ///graphics pipeline should obtain its pipeline fragment shading rate and
    ///shading rate combiner state from the
    ///[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure and that
    ///any state specified by the
    ///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure
    ///should be ignored.
    pub const FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE: Self = Self(0);
    ///[`FRAGMENT_SHADING_RATE_TYPE_ENUMS`] specifies that a graphics
    ///pipeline should obtain its pipeline fragment shading rate and shading
    ///rate combiner state from the
    ///[`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure and
    ///that any state specified by the
    ///[`PipelineFragmentShadingRateStateCreateInfoKHR`] structure should
    ///be ignored.
    pub const FRAGMENT_SHADING_RATE_TYPE_ENUMS: Self = Self(1);
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
