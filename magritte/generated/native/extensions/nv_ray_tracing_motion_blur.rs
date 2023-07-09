pub use crate::common::extensions::nv_ray_tracing_motion_blur::{
    AccelerationStructureMatrixMotionInstanceNV, AccelerationStructureSrtMotionInstanceNV, SrtDataNV,
};
use crate::native::{
    extensions::khr_acceleration_structure::{AccelerationStructureInstanceKHR, DeviceOrHostAddressConstKHR},
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
#[doc(alias = "VkPhysicalDeviceRayTracingMotionBlurFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "rayTracingMotionBlur")]
    pub ray_tracing_motion_blur: Bool32,
    #[doc(alias = "rayTracingMotionBlurPipelineTraceRaysIndirect")]
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: Bool32,
}
impl Default for PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRayTracingMotionBlurFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            ray_tracing_motion_blur: unsafe { std::mem::zeroed() },
            ray_tracing_motion_blur_pipeline_trace_rays_indirect: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryMotionTrianglesDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "vertexData")]
    pub vertex_data: DeviceOrHostAddressConstKHR,
}
impl Default for AccelerationStructureGeometryMotionTrianglesDataNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureGeometryMotionTrianglesDataNv,
            p_next: unsafe { std::mem::zeroed() },
            vertex_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureMotionInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMotionInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "maxInstances")]
    pub max_instances: u32,
    pub flags: AccelerationStructureMotionInfoFlagsNV,
}
impl Default for AccelerationStructureMotionInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::AccelerationStructureMotionInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            max_instances: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureMotionInstanceNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AccelerationStructureMotionInstanceNV {
    #[doc(alias = "type")]
    pub type_: AccelerationStructureMotionInstanceTypeNV,
    pub flags: AccelerationStructureMotionInstanceFlagsNV,
    pub data: AccelerationStructureMotionInstanceDataNV,
}
impl Default for AccelerationStructureMotionInstanceNV {
    fn default() -> Self {
        Self {
            type_: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAccelerationStructureMotionInstanceDataNV")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union AccelerationStructureMotionInstanceDataNV {
    #[doc(alias = "staticInstance")]
    pub static_instance: AccelerationStructureInstanceKHR,
    #[doc(alias = "matrixMotionInstance")]
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    #[doc(alias = "srtMotionInstance")]
    pub srt_motion_instance: AccelerationStructureSrtMotionInstanceNV,
}
pub use crate::common::extensions::nv_ray_tracing_motion_blur::{
    AccelerationStructureMotionInfoFlagsNV, AccelerationStructureMotionInstanceFlagsNV,
    AccelerationStructureMotionInstanceTypeNV, NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME,
    NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION,
};
