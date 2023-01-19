//!# [VK_NV_fragment_shading_rate_enums](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_fragment_shading_rate_enums.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shading_rate_enums/VK_NV_fragment_shading_rate_enums.md")]
use crate::{
    cstr,
    extensions::khr_fragment_shading_rate::FragmentShadingRateCombinerOpKHR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, SampleCountFlagBits, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shading_rate_enums/VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentShadingRateEnums")]
    fragment_shading_rate_enums: Bool32,
    #[doc(alias = "supersampleFragmentShadingRates")]
    supersample_fragment_shading_rates: Bool32,
    #[doc(alias = "noInvocationFragmentShadingRates")]
    no_invocation_fragment_shading_rates: Bool32,
}
///# [VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shading_rate_enums/VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV.md")]
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxFragmentShadingRateInvocationCount")]
    max_fragment_shading_rate_invocation_count: SampleCountFlagBits,
}
///# [VkPipelineFragmentShadingRateEnumStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shading_rate_enums/VkPipelineFragmentShadingRateEnumStateCreateInfoNV.md")]
#[doc(alias = "VkPipelineFragmentShadingRateEnumStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineFragmentShadingRateEnumStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "shadingRateType")]
    shading_rate_type: FragmentShadingRateTypeNV,
    #[doc(alias = "shadingRate")]
    shading_rate: FragmentShadingRateNV,
    #[doc(alias = "combinerOps")]
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
}
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_fragment_shading_rate_enums");
///# [VkFragmentShadingRateNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shading_rate_enums/VkFragmentShadingRateNV.md")]
#[doc(alias = "VkFragmentShadingRateNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FragmentShadingRateNV(i32);
impl FragmentShadingRateNV {
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV")]
    pub const N1_INVOCATION_PER_PIXEL: Self = Self(0);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV")]
    pub const N1_INVOCATION_PER1X2_PIXELS: Self = Self(1);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV")]
    pub const N1_INVOCATION_PER2X1_PIXELS: Self = Self(4);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV")]
    pub const N1_INVOCATION_PER2X2_PIXELS: Self = Self(5);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV")]
    pub const N1_INVOCATION_PER2X4_PIXELS: Self = Self(6);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV")]
    pub const N1_INVOCATION_PER4X2_PIXELS: Self = Self(9);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV")]
    pub const N1_INVOCATION_PER4X4_PIXELS: Self = Self(10);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV")]
    pub const N2_INVOCATIONS_PER_PIXEL: Self = Self(11);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV")]
    pub const N4_INVOCATIONS_PER_PIXEL: Self = Self(12);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV")]
    pub const N8_INVOCATIONS_PER_PIXEL: Self = Self(13);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV")]
    pub const N16_INVOCATIONS_PER_PIXEL: Self = Self(14);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV")]
    pub const NO_INVOCATIONS: Self = Self(15);
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
            x if x == Self::N1_INVOCATION_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER1X2_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER2X1_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER2X2_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER2X4_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER4X2_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N1_INVOCATION_PER4X4_PIXELS.bits() => Some(Self(x)),
            x if x == Self::N2_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N4_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N8_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::N16_INVOCATIONS_PER_PIXEL.bits() => Some(Self(x)),
            x if x == Self::NO_INVOCATIONS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkFragmentShadingRateTypeNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateTypeNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shading_rate_enums/VkFragmentShadingRateTypeNV.md")]
#[doc(alias = "VkFragmentShadingRateTypeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FragmentShadingRateTypeNV(i32);
impl FragmentShadingRateTypeNV {
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV")]
    pub const FRAGMENT_SIZE: Self = Self(0);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV")]
    pub const ENUMS: Self = Self(1);
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
            x if x == Self::FRAGMENT_SIZE.bits() => Some(Self(x)),
            x if x == Self::ENUMS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkCmdSetFragmentShadingRateEnumNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetFragmentShadingRateEnumNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_fragment_shading_rate_enums/vkCmdSetFragmentShadingRateEnumNV.md")]
#[doc(alias = "vkCmdSetFragmentShadingRateEnumNV")]
pub type FNCmdSetFragmentShadingRateEnumNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
);
