use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_pipeline_executable_properties");
#[doc(alias = "VkPipelineExecutableStatisticFormatKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum PipelineExecutableStatisticFormatKHR {
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR")]
    Bool32 = 0,
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR")]
    Int64 = 1,
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR")]
    Uint64 = 2,
    #[doc(alias = "VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR")]
    Float64 = 3,
}
impl Default for PipelineExecutableStatisticFormatKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl PipelineExecutableStatisticFormatKHR {
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
            x if x == Self::Bool32.bits() => Some(Self::Bool32),
            x if x == Self::Int64.bits() => Some(Self::Int64),
            x if x == Self::Uint64.bits() => Some(Self::Uint64),
            x if x == Self::Float64.bits() => Some(Self::Float64),
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
unsafe impl crate::conv::IntoLowLevel for PipelineExecutableStatisticFormatKHR {
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
unsafe impl crate::conv::FromLowLevel for PipelineExecutableStatisticFormatKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
