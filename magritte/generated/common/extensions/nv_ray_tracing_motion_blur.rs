use crate::{
    common::extensions::khr_acceleration_structure::{GeometryInstanceFlagsKHR, TransformMatrixKHR},
    cstr,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkSRTDataNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct SrtDataNV {
    pub sx: f32,
    pub a: f32,
    pub b: f32,
    pub pvx: f32,
    pub sy: f32,
    pub c: f32,
    pub pvy: f32,
    pub sz: f32,
    pub pvz: f32,
    pub qx: f32,
    pub qy: f32,
    pub qz: f32,
    pub qw: f32,
    pub tx: f32,
    pub ty: f32,
    pub tz: f32,
}
#[doc(alias = "VkAccelerationStructureSRTMotionInstanceNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureSrtMotionInstanceNV {
    #[doc(alias = "transformT0")]
    pub transform_t0: SrtDataNV,
    #[doc(alias = "transformT1")]
    pub transform_t1: SrtDataNV,
    #[doc(alias = "instanceCustomIndex")]
    pub instance_custom_index: u32,
    pub mask: u32,
    #[doc(alias = "instanceShaderBindingTableRecordOffset")]
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    #[doc(alias = "accelerationStructureReference")]
    pub acceleration_structure_reference: u64,
}
#[doc(alias = "VkAccelerationStructureMatrixMotionInstanceNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    #[doc(alias = "transformT0")]
    pub transform_t0: TransformMatrixKHR,
    #[doc(alias = "transformT1")]
    pub transform_t1: TransformMatrixKHR,
    #[doc(alias = "instanceCustomIndex")]
    pub instance_custom_index: u32,
    pub mask: u32,
    #[doc(alias = "instanceShaderBindingTableRecordOffset")]
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    #[doc(alias = "accelerationStructureReference")]
    pub acceleration_structure_reference: u64,
}
#[doc(alias = "VkAccelerationStructureMotionInfoFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureMotionInfoFlagsNV(u32);
impl AccelerationStructureMotionInfoFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for AccelerationStructureMotionInfoFlagsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMotionInfoFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMotionInfoFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccelerationStructureMotionInstanceFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureMotionInstanceFlagsNV(u32);
impl AccelerationStructureMotionInstanceFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for AccelerationStructureMotionInstanceFlagsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMotionInstanceFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMotionInstanceFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION")]
pub const NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME")]
pub const NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_ray_tracing_motion_blur");
#[doc(alias = "VkAccelerationStructureMotionInstanceTypeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum AccelerationStructureMotionInstanceTypeNV {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV")]
    Static = 0,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV")]
    MatrixMotion = 1,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV")]
    SrtMotion = 2,
}
impl Default for AccelerationStructureMotionInstanceTypeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AccelerationStructureMotionInstanceTypeNV {
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
            x if x == Self::Static.bits() => Some(Self::Static),
            x if x == Self::MatrixMotion.bits() => Some(Self::MatrixMotion),
            x if x == Self::SrtMotion.bits() => Some(Self::SrtMotion),
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
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMotionInstanceTypeNV {
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
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMotionInstanceTypeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
