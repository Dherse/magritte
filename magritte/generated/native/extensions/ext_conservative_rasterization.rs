//!# [VK_EXT_conservative_rasterization](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_conservative_rasterization.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_conservative_rasterization/VK_EXT_conservative_rasterization.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceConservativeRasterizationPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conservative_rasterization/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.md")]
#[doc(alias = "VkPhysicalDeviceConservativeRasterizationPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "primitiveOverestimationSize")]
    primitive_overestimation_size: f32,
    #[doc(alias = "maxExtraPrimitiveOverestimationSize")]
    max_extra_primitive_overestimation_size: f32,
    #[doc(alias = "extraPrimitiveOverestimationSizeGranularity")]
    extra_primitive_overestimation_size_granularity: f32,
    #[doc(alias = "primitiveUnderestimation")]
    primitive_underestimation: Bool32,
    #[doc(alias = "conservativePointAndLineRasterization")]
    conservative_point_and_line_rasterization: Bool32,
    #[doc(alias = "degenerateTrianglesRasterized")]
    degenerate_triangles_rasterized: Bool32,
    #[doc(alias = "degenerateLinesRasterized")]
    degenerate_lines_rasterized: Bool32,
    #[doc(alias = "fullyCoveredFragmentShaderInputVariable")]
    fully_covered_fragment_shader_input_variable: Bool32,
    #[doc(alias = "conservativeRasterizationPostDepthCoverage")]
    conservative_rasterization_post_depth_coverage: Bool32,
}
///# [VkPipelineRasterizationConservativeStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conservative_rasterization/VkPipelineRasterizationConservativeStateCreateInfoEXT.md")]
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    #[doc(alias = "conservativeRasterizationMode")]
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    #[doc(alias = "extraPrimitiveOverestimationSize")]
    extra_primitive_overestimation_size: f32,
}
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationConservativeStateCreateFlagsEXT(u32);
impl PipelineRasterizationConservativeStateCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_conservative_rasterization");
///# [VkConservativeRasterizationModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConservativeRasterizationModeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conservative_rasterization/VkConservativeRasterizationModeEXT.md")]
#[doc(alias = "VkConservativeRasterizationModeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ConservativeRasterizationModeEXT(i32);
impl ConservativeRasterizationModeEXT {
    #[doc(alias = "VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT")]
    pub const DISABLED: Self = Self(0);
    #[doc(alias = "VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT")]
    pub const OVERESTIMATE: Self = Self(1);
    #[doc(alias = "VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT")]
    pub const UNDERESTIMATE: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::DISABLED.bits() => Some(Self(x)),
            x if x == Self::OVERESTIMATE.bits() => Some(Self(x)),
            x if x == Self::UNDERESTIMATE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
