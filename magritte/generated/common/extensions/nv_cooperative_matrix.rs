use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION")]
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME")]
pub const NV_COOPERATIVE_MATRIX_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_cooperative_matrix");
#[doc(alias = "VkScopeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ScopeNV {
    Empty = 0,
    #[doc(alias = "VK_SCOPE_DEVICE_NV")]
    Device = 1,
    #[doc(alias = "VK_SCOPE_WORKGROUP_NV")]
    Workgroup = 2,
    #[doc(alias = "VK_SCOPE_SUBGROUP_NV")]
    Subgroup = 3,
    #[doc(alias = "VK_SCOPE_QUEUE_FAMILY_NV")]
    QueueFamily = 5,
}
impl Default for ScopeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ScopeNV {
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
            x if x == Self::Device.bits() => Some(Self::Device),
            x if x == Self::Workgroup.bits() => Some(Self::Workgroup),
            x if x == Self::Subgroup.bits() => Some(Self::Subgroup),
            x if x == Self::QueueFamily.bits() => Some(Self::QueueFamily),
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
unsafe impl crate::conv::IntoLowLevel for ScopeNV {
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
unsafe impl crate::conv::FromLowLevel for ScopeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkComponentTypeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ComponentTypeNV {
    #[doc(alias = "VK_COMPONENT_TYPE_FLOAT16_NV")]
    Float16 = 0,
    #[doc(alias = "VK_COMPONENT_TYPE_FLOAT32_NV")]
    Float32 = 1,
    #[doc(alias = "VK_COMPONENT_TYPE_FLOAT64_NV")]
    Float64 = 2,
    #[doc(alias = "VK_COMPONENT_TYPE_SINT8_NV")]
    Sint8 = 3,
    #[doc(alias = "VK_COMPONENT_TYPE_SINT16_NV")]
    Sint16 = 4,
    #[doc(alias = "VK_COMPONENT_TYPE_SINT32_NV")]
    Sint32 = 5,
    #[doc(alias = "VK_COMPONENT_TYPE_SINT64_NV")]
    Sint64 = 6,
    #[doc(alias = "VK_COMPONENT_TYPE_UINT8_NV")]
    Uint8 = 7,
    #[doc(alias = "VK_COMPONENT_TYPE_UINT16_NV")]
    Uint16 = 8,
    #[doc(alias = "VK_COMPONENT_TYPE_UINT32_NV")]
    Uint32 = 9,
    #[doc(alias = "VK_COMPONENT_TYPE_UINT64_NV")]
    Uint64 = 10,
}
impl Default for ComponentTypeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ComponentTypeNV {
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
            x if x == Self::Float16.bits() => Some(Self::Float16),
            x if x == Self::Float32.bits() => Some(Self::Float32),
            x if x == Self::Float64.bits() => Some(Self::Float64),
            x if x == Self::Sint8.bits() => Some(Self::Sint8),
            x if x == Self::Sint16.bits() => Some(Self::Sint16),
            x if x == Self::Sint32.bits() => Some(Self::Sint32),
            x if x == Self::Sint64.bits() => Some(Self::Sint64),
            x if x == Self::Uint8.bits() => Some(Self::Uint8),
            x if x == Self::Uint16.bits() => Some(Self::Uint16),
            x if x == Self::Uint32.bits() => Some(Self::Uint32),
            x if x == Self::Uint64.bits() => Some(Self::Uint64),
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
unsafe impl crate::conv::IntoLowLevel for ComponentTypeNV {
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
unsafe impl crate::conv::FromLowLevel for ComponentTypeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
