use crate::{
    cstr,
    extensions::khr_acceleration_structure::{
        AccelerationStructureInstanceKHR, DeviceOrHostAddressConstKHR, GeometryInstanceFlagsKHR, TransformMatrixKHR,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceRayTracingMotionBlurFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "rayTracingMotionBlur")]
    ray_tracing_motion_blur: Bool32,
    #[doc(alias = "rayTracingMotionBlurPipelineTraceRaysIndirect")]
    ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
}
#[doc(alias = "VkAccelerationStructureGeometryMotionTrianglesDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "vertexData")]
    vertex_data: DeviceOrHostAddressConstKHR,
}
#[doc(alias = "VkAccelerationStructureMotionInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMotionInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "maxInstances")]
    max_instances: u32,
    flags: AccelerationStructureMotionInfoFlagsNV,
}
#[doc(alias = "VkSRTDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SrtDataNV {
    sx: f32,
    a: f32,
    b: f32,
    pvx: f32,
    sy: f32,
    c: f32,
    pvy: f32,
    sz: f32,
    pvz: f32,
    qx: f32,
    qy: f32,
    qz: f32,
    qw: f32,
    tx: f32,
    ty: f32,
    tz: f32,
}
#[doc(alias = "VkAccelerationStructureSRTMotionInstanceNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureSrtMotionInstanceNV {
    #[doc(alias = "transformT0")]
    transform_t0: SrtDataNV,
    #[doc(alias = "transformT1")]
    transform_t1: SrtDataNV,
    #[doc(alias = "instanceCustomIndex")]
    instance_custom_index: u32,
    mask: u32,
    #[doc(alias = "instanceShaderBindingTableRecordOffset")]
    instance_shader_binding_table_record_offset: u32,
    flags: GeometryInstanceFlagsKHR,
    #[doc(alias = "accelerationStructureReference")]
    acceleration_structure_reference: u64,
}
#[doc(alias = "VkAccelerationStructureMatrixMotionInstanceNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMatrixMotionInstanceNV {
    #[doc(alias = "transformT0")]
    transform_t0: TransformMatrixKHR,
    #[doc(alias = "transformT1")]
    transform_t1: TransformMatrixKHR,
    #[doc(alias = "instanceCustomIndex")]
    instance_custom_index: u32,
    mask: u32,
    #[doc(alias = "instanceShaderBindingTableRecordOffset")]
    instance_shader_binding_table_record_offset: u32,
    flags: GeometryInstanceFlagsKHR,
    #[doc(alias = "accelerationStructureReference")]
    acceleration_structure_reference: u64,
}
#[doc(alias = "VkAccelerationStructureMotionInstanceNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
    #[doc(alias = "type")]
    type_: AccelerationStructureMotionInstanceTypeNV,
    flags: AccelerationStructureMotionInstanceFlagsNV,
    data: AccelerationStructureMotionInstanceDataNV,
}
#[doc(alias = "VkAccelerationStructureMotionInstanceDataNV")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union AccelerationStructureMotionInstanceDataNV {
    #[doc(alias = "staticInstance")]
    static_instance: AccelerationStructureInstanceKHR,
    #[doc(alias = "matrixMotionInstance")]
    matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    #[doc(alias = "srtMotionInstance")]
    srt_motion_instance: AccelerationStructureSrtMotionInstanceNV,
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
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION")]
pub const NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME")]
pub const NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_ray_tracing_motion_blur");
#[doc(alias = "VkAccelerationStructureMotionInstanceTypeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccelerationStructureMotionInstanceTypeNV(i32);
impl AccelerationStructureMotionInstanceTypeNV {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV")]
    pub const STATIC: Self = Self(0);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV")]
    pub const MATRIX_MOTION: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV")]
    pub const SRT_MOTION: Self = Self(2);
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
            x if x == Self::STATIC.bits() => Some(Self(x)),
            x if x == Self::MATRIX_MOTION.bits() => Some(Self(x)),
            x if x == Self::SRT_MOTION.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
