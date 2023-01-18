use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "advancedBlendCoherentOperations")]
    advanced_blend_coherent_operations: Bool32,
}
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "advancedBlendMaxColorAttachments")]
    advanced_blend_max_color_attachments: u32,
    #[doc(alias = "advancedBlendIndependentBlend")]
    advanced_blend_independent_blend: Bool32,
    #[doc(alias = "advancedBlendNonPremultipliedSrcColor")]
    advanced_blend_non_premultiplied_src_color: Bool32,
    #[doc(alias = "advancedBlendNonPremultipliedDstColor")]
    advanced_blend_non_premultiplied_dst_color: Bool32,
    #[doc(alias = "advancedBlendCorrelatedOverlap")]
    advanced_blend_correlated_overlap: Bool32,
    #[doc(alias = "advancedBlendAllOperations")]
    advanced_blend_all_operations: Bool32,
}
#[doc(alias = "VkPipelineColorBlendAdvancedStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "srcPremultiplied")]
    src_premultiplied: Bool32,
    #[doc(alias = "dstPremultiplied")]
    dst_premultiplied: Bool32,
    #[doc(alias = "blendOverlap")]
    blend_overlap: BlendOverlapEXT,
}
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION")]
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME")]
pub const EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_blend_operation_advanced");
#[doc(alias = "VkBlendOverlapEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct BlendOverlapEXT(i32);
impl BlendOverlapEXT {
    #[doc(alias = "VK_BLEND_OVERLAP_UNCORRELATED_EXT")]
    pub const UNCORRELATED: Self = Self(0);
    #[doc(alias = "VK_BLEND_OVERLAP_DISJOINT_EXT")]
    pub const DISJOINT: Self = Self(1);
    #[doc(alias = "VK_BLEND_OVERLAP_CONJOINT_EXT")]
    pub const CONJOINT: Self = Self(2);
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
            x if x == Self::UNCORRELATED.bits() => Some(Self(x)),
            x if x == Self::DISJOINT.bits() => Some(Self(x)),
            x if x == Self::CONJOINT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
