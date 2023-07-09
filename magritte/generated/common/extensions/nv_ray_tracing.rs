use crate::{common::vulkan1_0::SHADER_UNUSED_KHR, cstr};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_ray_tracing");
///See [`SHADER_UNUSED_KHR`]
#[doc(alias = "VK_SHADER_UNUSED_NV")]
pub const SHADER_UNUSED_NV: u32 = SHADER_UNUSED_KHR;
#[doc(alias = "VkAccelerationStructureMemoryRequirementsTypeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum AccelerationStructureMemoryRequirementsTypeNV {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV")]
    Object = 0,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV")]
    BuildScratch = 1,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV")]
    UpdateScratch = 2,
}
impl Default for AccelerationStructureMemoryRequirementsTypeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AccelerationStructureMemoryRequirementsTypeNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Object.bits() => Some(Self::Object),
            x if x == Self::BuildScratch.bits() => Some(Self::BuildScratch),
            x if x == Self::UpdateScratch.bits() => Some(Self::UpdateScratch),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMemoryRequirementsTypeNV {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMemoryRequirementsTypeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
