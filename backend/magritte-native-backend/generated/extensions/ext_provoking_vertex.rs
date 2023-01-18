use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceProvokingVertexFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "provokingVertexLast")]
    provoking_vertex_last: Bool32,
    #[doc(alias = "transformFeedbackPreservesProvokingVertex")]
    transform_feedback_preserves_provoking_vertex: Bool32,
}
#[doc(alias = "VkPhysicalDeviceProvokingVertexPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "provokingVertexModePerPipeline")]
    provoking_vertex_mode_per_pipeline: Bool32,
    #[doc(alias = "transformFeedbackPreservesTriangleFanProvokingVertex")]
    transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
}
#[doc(alias = "VkPipelineRasterizationProvokingVertexStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "provokingVertexMode")]
    provoking_vertex_mode: ProvokingVertexModeEXT,
}
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_SPEC_VERSION")]
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME")]
pub const EXT_PROVOKING_VERTEX_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_provoking_vertex");
#[doc(alias = "VkProvokingVertexModeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ProvokingVertexModeEXT(i32);
impl ProvokingVertexModeEXT {
    #[doc(alias = "VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT")]
    pub const FIRST_VERTEX: Self = Self(0);
    #[doc(alias = "VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT")]
    pub const LAST_VERTEX: Self = Self(1);
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
            x if x == Self::FIRST_VERTEX.bits() => Some(Self(x)),
            x if x == Self::LAST_VERTEX.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
