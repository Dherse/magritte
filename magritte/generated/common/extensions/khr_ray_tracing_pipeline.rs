use crate::{
    common::vulkan1_0::{DeviceAddress, DeviceSize},
    cstr,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkStridedDeviceAddressRegionKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct StridedDeviceAddressRegionKHR {
    #[doc(alias = "deviceAddress")]
    pub device_address: DeviceAddress,
    pub stride: DeviceSize,
    pub size: DeviceSize,
}
#[doc(alias = "VkTraceRaysIndirectCommandKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TraceRaysIndirectCommandKHR {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION")]
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_ray_tracing_pipeline");
#[doc(alias = "VkRayTracingShaderGroupTypeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum RayTracingShaderGroupTypeKHR {
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR")]
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV")]
    General = 0,
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR")]
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV")]
    TrianglesHitGroup = 1,
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR")]
    #[doc(alias = "VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV")]
    ProceduralHitGroup = 2,
}
impl Default for RayTracingShaderGroupTypeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl RayTracingShaderGroupTypeKHR {
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
            x if x == Self::General.bits() => Some(Self::General),
            x if x == Self::TrianglesHitGroup.bits() => Some(Self::TrianglesHitGroup),
            x if x == Self::ProceduralHitGroup.bits() => Some(Self::ProceduralHitGroup),
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
unsafe impl crate::conv::IntoLowLevel for RayTracingShaderGroupTypeKHR {
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
unsafe impl crate::conv::FromLowLevel for RayTracingShaderGroupTypeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkShaderGroupShaderKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ShaderGroupShaderKHR {
    #[doc(alias = "VK_SHADER_GROUP_SHADER_GENERAL_KHR")]
    General = 0,
    #[doc(alias = "VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR")]
    ClosestHit = 1,
    #[doc(alias = "VK_SHADER_GROUP_SHADER_ANY_HIT_KHR")]
    AnyHit = 2,
    #[doc(alias = "VK_SHADER_GROUP_SHADER_INTERSECTION_KHR")]
    Intersection = 3,
}
impl Default for ShaderGroupShaderKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ShaderGroupShaderKHR {
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
            x if x == Self::General.bits() => Some(Self::General),
            x if x == Self::ClosestHit.bits() => Some(Self::ClosestHit),
            x if x == Self::AnyHit.bits() => Some(Self::AnyHit),
            x if x == Self::Intersection.bits() => Some(Self::Intersection),
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
unsafe impl crate::conv::IntoLowLevel for ShaderGroupShaderKHR {
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
unsafe impl crate::conv::FromLowLevel for ShaderGroupShaderKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
