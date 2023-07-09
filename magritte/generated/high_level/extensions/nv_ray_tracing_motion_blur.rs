pub use crate::common::extensions::nv_ray_tracing_motion_blur::{
    AccelerationStructureMatrixMotionInstanceNV, AccelerationStructureMotionInfoFlagsNV,
    AccelerationStructureMotionInstanceFlagsNV, AccelerationStructureMotionInstanceTypeNV,
    AccelerationStructureSrtMotionInstanceNV, SrtDataNV, NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME,
    NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION,
};
use crate::{
    context::Context,
    extensions::khr_acceleration_structure::{
        AccelerationStructureInstanceKHR, DeviceOrHostAddressConstKHR, GeometryInstanceFlagsKHR, TransformMatrixKHR,
    },
    vulkan1_0::StructureType,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceRayTracingMotionBlurFeaturesNV")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    #[doc(alias = "rayTracingMotionBlur")]
    pub ray_tracing_motion_blur: bool,
    #[doc(alias = "rayTracingMotionBlurPipelineTraceRaysIndirect")]
    pub ray_tracing_motion_blur_pipeline_trace_rays_indirect: bool,
}
impl PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    ///Get a reference to the `ray_tracing_motion_blur` field.
    pub fn ray_tracing_motion_blur(&self) -> &bool {
        &self.ray_tracing_motion_blur
    }
    ///Get a reference to the `ray_tracing_motion_blur_pipeline_trace_rays_indirect` field.
    pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect(&self) -> &bool {
        &self.ray_tracing_motion_blur_pipeline_trace_rays_indirect
    }
    ///Get a mutable reference to the `ray_tracing_motion_blur` field.
    pub fn ray_tracing_motion_blur_mut(&mut self) -> &mut bool {
        &mut self.ray_tracing_motion_blur
    }
    ///Get a mutable reference to the `ray_tracing_motion_blur_pipeline_trace_rays_indirect` field.
    pub fn ray_tracing_motion_blur_pipeline_trace_rays_indirect_mut(&mut self) -> &mut bool {
        &mut self.ray_tracing_motion_blur_pipeline_trace_rays_indirect
    }
    ///Sets the `ray_tracing_motion_blur` field.
    pub fn set_ray_tracing_motion_blur(&mut self, ray_tracing_motion_blur: bool) -> &mut Self {
        self.ray_tracing_motion_blur = ray_tracing_motion_blur;
        self
    }
    ///Sets the `ray_tracing_motion_blur_pipeline_trace_rays_indirect` field.
    pub fn set_ray_tracing_motion_blur_pipeline_trace_rays_indirect(
        &mut self,
        ray_tracing_motion_blur_pipeline_trace_rays_indirect: bool,
    ) -> &mut Self {
        self.ray_tracing_motion_blur_pipeline_trace_rays_indirect =
            ray_tracing_motion_blur_pipeline_trace_rays_indirect;
        self
    }
    ///Sets the `ray_tracing_motion_blur` field in a builder way.
    pub fn with_ray_tracing_motion_blur(mut self, ray_tracing_motion_blur: bool) -> Self {
        self.ray_tracing_motion_blur = ray_tracing_motion_blur;
        self
    }
    ///Sets the `ray_tracing_motion_blur_pipeline_trace_rays_indirect` field in a builder way.
    pub fn with_ray_tracing_motion_blur_pipeline_trace_rays_indirect(
        mut self,
        ray_tracing_motion_blur_pipeline_trace_rays_indirect: bool,
    ) -> Self {
        self.ray_tracing_motion_blur_pipeline_trace_rays_indirect =
            ray_tracing_motion_blur_pipeline_trace_rays_indirect;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing_motion_blur::PhysicalDeviceRayTracingMotionBlurFeaturesNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing_motion_blur::PhysicalDeviceRayTracingMotionBlurFeaturesNV {
            s_type: StructureType::PhysicalDeviceRayTracingMotionBlurFeaturesNv,
            p_next: std::ptr::null_mut(),
            ray_tracing_motion_blur: self.ray_tracing_motion_blur.into_low_level(context, bump),
            ray_tracing_motion_blur_pipeline_trace_rays_indirect: self
                .ray_tracing_motion_blur_pipeline_trace_rays_indirect
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceRayTracingMotionBlurFeaturesNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            ray_tracing_motion_blur: crate::conv::FromLowLevel::from_low_level(context, value.ray_tracing_motion_blur),
            ray_tracing_motion_blur_pipeline_trace_rays_indirect: crate::conv::FromLowLevel::from_low_level(
                context,
                value.ray_tracing_motion_blur_pipeline_trace_rays_indirect,
            ),
        }
    }
}
#[doc(alias = "VkAccelerationStructureGeometryMotionTrianglesDataNV")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureGeometryMotionTrianglesDataNV {
    #[doc(alias = "vertexData")]
    pub vertex_data: DeviceOrHostAddressConstKHR,
}
impl AccelerationStructureGeometryMotionTrianglesDataNV {
    ///Get a reference to the `vertex_data` field.
    pub fn vertex_data(&self) -> &DeviceOrHostAddressConstKHR {
        &self.vertex_data
    }
    ///Get a mutable reference to the `vertex_data` field.
    pub fn vertex_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.vertex_data
    }
    ///Sets the `vertex_data` field.
    pub fn set_vertex_data(&mut self, vertex_data: DeviceOrHostAddressConstKHR) -> &mut Self {
        self.vertex_data = vertex_data;
        self
    }
    ///Sets the `vertex_data` field in a builder way.
    pub fn with_vertex_data(mut self, vertex_data: DeviceOrHostAddressConstKHR) -> Self {
        self.vertex_data = vertex_data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureGeometryMotionTrianglesDataNV {
    type LowLevel =
        crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureGeometryMotionTrianglesDataNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureGeometryMotionTrianglesDataNV {
            s_type: StructureType::AccelerationStructureGeometryMotionTrianglesDataNv,
            p_next: std::ptr::null(),
            vertex_data: self.vertex_data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureGeometryMotionTrianglesDataNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            vertex_data: crate::conv::FromLowLevel::from_low_level(context, value.vertex_data),
        }
    }
}
#[doc(alias = "VkAccelerationStructureMotionInfoNV")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureMotionInfoNV {
    #[doc(alias = "maxInstances")]
    pub max_instances: u32,
    pub flags: AccelerationStructureMotionInfoFlagsNV,
}
impl AccelerationStructureMotionInfoNV {
    ///Get a reference to the `max_instances` field.
    pub fn max_instances(&self) -> u32 {
        self.max_instances
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> AccelerationStructureMotionInfoFlagsNV {
        self.flags
    }
    ///Get a mutable reference to the `max_instances` field.
    pub fn max_instances_mut(&mut self) -> &mut u32 {
        &mut self.max_instances
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut AccelerationStructureMotionInfoFlagsNV {
        &mut self.flags
    }
    ///Sets the `max_instances` field.
    pub fn set_max_instances(&mut self, max_instances: u32) -> &mut Self {
        self.max_instances = max_instances;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: AccelerationStructureMotionInfoFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `max_instances` field in a builder way.
    pub fn with_max_instances(mut self, max_instances: u32) -> Self {
        self.max_instances = max_instances;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: AccelerationStructureMotionInfoFlagsNV) -> Self {
        self.flags = flags;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMotionInfoNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInfoNV {
            s_type: StructureType::AccelerationStructureMotionInfoNv,
            p_next: std::ptr::null(),
            max_instances: self.max_instances.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMotionInfoNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_instances: crate::conv::FromLowLevel::from_low_level(context, value.max_instances),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
        }
    }
}
impl SrtDataNV {
    ///Get a reference to the `sx` field.
    pub fn sx(&self) -> f32 {
        self.sx
    }
    ///Get a reference to the `a` field.
    pub fn a(&self) -> f32 {
        self.a
    }
    ///Get a reference to the `b` field.
    pub fn b(&self) -> f32 {
        self.b
    }
    ///Get a reference to the `pvx` field.
    pub fn pvx(&self) -> f32 {
        self.pvx
    }
    ///Get a reference to the `sy` field.
    pub fn sy(&self) -> f32 {
        self.sy
    }
    ///Get a reference to the `c` field.
    pub fn c(&self) -> f32 {
        self.c
    }
    ///Get a reference to the `pvy` field.
    pub fn pvy(&self) -> f32 {
        self.pvy
    }
    ///Get a reference to the `sz` field.
    pub fn sz(&self) -> f32 {
        self.sz
    }
    ///Get a reference to the `pvz` field.
    pub fn pvz(&self) -> f32 {
        self.pvz
    }
    ///Get a reference to the `qx` field.
    pub fn qx(&self) -> f32 {
        self.qx
    }
    ///Get a reference to the `qy` field.
    pub fn qy(&self) -> f32 {
        self.qy
    }
    ///Get a reference to the `qz` field.
    pub fn qz(&self) -> f32 {
        self.qz
    }
    ///Get a reference to the `qw` field.
    pub fn qw(&self) -> f32 {
        self.qw
    }
    ///Get a reference to the `tx` field.
    pub fn tx(&self) -> f32 {
        self.tx
    }
    ///Get a reference to the `ty` field.
    pub fn ty(&self) -> f32 {
        self.ty
    }
    ///Get a reference to the `tz` field.
    pub fn tz(&self) -> f32 {
        self.tz
    }
    ///Get a mutable reference to the `sx` field.
    pub fn sx_mut(&mut self) -> &mut f32 {
        &mut self.sx
    }
    ///Get a mutable reference to the `a` field.
    pub fn a_mut(&mut self) -> &mut f32 {
        &mut self.a
    }
    ///Get a mutable reference to the `b` field.
    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.b
    }
    ///Get a mutable reference to the `pvx` field.
    pub fn pvx_mut(&mut self) -> &mut f32 {
        &mut self.pvx
    }
    ///Get a mutable reference to the `sy` field.
    pub fn sy_mut(&mut self) -> &mut f32 {
        &mut self.sy
    }
    ///Get a mutable reference to the `c` field.
    pub fn c_mut(&mut self) -> &mut f32 {
        &mut self.c
    }
    ///Get a mutable reference to the `pvy` field.
    pub fn pvy_mut(&mut self) -> &mut f32 {
        &mut self.pvy
    }
    ///Get a mutable reference to the `sz` field.
    pub fn sz_mut(&mut self) -> &mut f32 {
        &mut self.sz
    }
    ///Get a mutable reference to the `pvz` field.
    pub fn pvz_mut(&mut self) -> &mut f32 {
        &mut self.pvz
    }
    ///Get a mutable reference to the `qx` field.
    pub fn qx_mut(&mut self) -> &mut f32 {
        &mut self.qx
    }
    ///Get a mutable reference to the `qy` field.
    pub fn qy_mut(&mut self) -> &mut f32 {
        &mut self.qy
    }
    ///Get a mutable reference to the `qz` field.
    pub fn qz_mut(&mut self) -> &mut f32 {
        &mut self.qz
    }
    ///Get a mutable reference to the `qw` field.
    pub fn qw_mut(&mut self) -> &mut f32 {
        &mut self.qw
    }
    ///Get a mutable reference to the `tx` field.
    pub fn tx_mut(&mut self) -> &mut f32 {
        &mut self.tx
    }
    ///Get a mutable reference to the `ty` field.
    pub fn ty_mut(&mut self) -> &mut f32 {
        &mut self.ty
    }
    ///Get a mutable reference to the `tz` field.
    pub fn tz_mut(&mut self) -> &mut f32 {
        &mut self.tz
    }
    ///Sets the `sx` field.
    pub fn set_sx(&mut self, sx: f32) -> &mut Self {
        self.sx = sx;
        self
    }
    ///Sets the `a` field.
    pub fn set_a(&mut self, a: f32) -> &mut Self {
        self.a = a;
        self
    }
    ///Sets the `b` field.
    pub fn set_b(&mut self, b: f32) -> &mut Self {
        self.b = b;
        self
    }
    ///Sets the `pvx` field.
    pub fn set_pvx(&mut self, pvx: f32) -> &mut Self {
        self.pvx = pvx;
        self
    }
    ///Sets the `sy` field.
    pub fn set_sy(&mut self, sy: f32) -> &mut Self {
        self.sy = sy;
        self
    }
    ///Sets the `c` field.
    pub fn set_c(&mut self, c: f32) -> &mut Self {
        self.c = c;
        self
    }
    ///Sets the `pvy` field.
    pub fn set_pvy(&mut self, pvy: f32) -> &mut Self {
        self.pvy = pvy;
        self
    }
    ///Sets the `sz` field.
    pub fn set_sz(&mut self, sz: f32) -> &mut Self {
        self.sz = sz;
        self
    }
    ///Sets the `pvz` field.
    pub fn set_pvz(&mut self, pvz: f32) -> &mut Self {
        self.pvz = pvz;
        self
    }
    ///Sets the `qx` field.
    pub fn set_qx(&mut self, qx: f32) -> &mut Self {
        self.qx = qx;
        self
    }
    ///Sets the `qy` field.
    pub fn set_qy(&mut self, qy: f32) -> &mut Self {
        self.qy = qy;
        self
    }
    ///Sets the `qz` field.
    pub fn set_qz(&mut self, qz: f32) -> &mut Self {
        self.qz = qz;
        self
    }
    ///Sets the `qw` field.
    pub fn set_qw(&mut self, qw: f32) -> &mut Self {
        self.qw = qw;
        self
    }
    ///Sets the `tx` field.
    pub fn set_tx(&mut self, tx: f32) -> &mut Self {
        self.tx = tx;
        self
    }
    ///Sets the `ty` field.
    pub fn set_ty(&mut self, ty: f32) -> &mut Self {
        self.ty = ty;
        self
    }
    ///Sets the `tz` field.
    pub fn set_tz(&mut self, tz: f32) -> &mut Self {
        self.tz = tz;
        self
    }
    ///Sets the `sx` field in a builder way.
    pub fn with_sx(mut self, sx: f32) -> Self {
        self.sx = sx;
        self
    }
    ///Sets the `a` field in a builder way.
    pub fn with_a(mut self, a: f32) -> Self {
        self.a = a;
        self
    }
    ///Sets the `b` field in a builder way.
    pub fn with_b(mut self, b: f32) -> Self {
        self.b = b;
        self
    }
    ///Sets the `pvx` field in a builder way.
    pub fn with_pvx(mut self, pvx: f32) -> Self {
        self.pvx = pvx;
        self
    }
    ///Sets the `sy` field in a builder way.
    pub fn with_sy(mut self, sy: f32) -> Self {
        self.sy = sy;
        self
    }
    ///Sets the `c` field in a builder way.
    pub fn with_c(mut self, c: f32) -> Self {
        self.c = c;
        self
    }
    ///Sets the `pvy` field in a builder way.
    pub fn with_pvy(mut self, pvy: f32) -> Self {
        self.pvy = pvy;
        self
    }
    ///Sets the `sz` field in a builder way.
    pub fn with_sz(mut self, sz: f32) -> Self {
        self.sz = sz;
        self
    }
    ///Sets the `pvz` field in a builder way.
    pub fn with_pvz(mut self, pvz: f32) -> Self {
        self.pvz = pvz;
        self
    }
    ///Sets the `qx` field in a builder way.
    pub fn with_qx(mut self, qx: f32) -> Self {
        self.qx = qx;
        self
    }
    ///Sets the `qy` field in a builder way.
    pub fn with_qy(mut self, qy: f32) -> Self {
        self.qy = qy;
        self
    }
    ///Sets the `qz` field in a builder way.
    pub fn with_qz(mut self, qz: f32) -> Self {
        self.qz = qz;
        self
    }
    ///Sets the `qw` field in a builder way.
    pub fn with_qw(mut self, qw: f32) -> Self {
        self.qw = qw;
        self
    }
    ///Sets the `tx` field in a builder way.
    pub fn with_tx(mut self, tx: f32) -> Self {
        self.tx = tx;
        self
    }
    ///Sets the `ty` field in a builder way.
    pub fn with_ty(mut self, ty: f32) -> Self {
        self.ty = ty;
        self
    }
    ///Sets the `tz` field in a builder way.
    pub fn with_tz(mut self, tz: f32) -> Self {
        self.tz = tz;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for SrtDataNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing_motion_blur::SrtDataNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing_motion_blur::SrtDataNV {
            sx: self.sx.into_low_level(context, bump),
            a: self.a.into_low_level(context, bump),
            b: self.b.into_low_level(context, bump),
            pvx: self.pvx.into_low_level(context, bump),
            sy: self.sy.into_low_level(context, bump),
            c: self.c.into_low_level(context, bump),
            pvy: self.pvy.into_low_level(context, bump),
            sz: self.sz.into_low_level(context, bump),
            pvz: self.pvz.into_low_level(context, bump),
            qx: self.qx.into_low_level(context, bump),
            qy: self.qy.into_low_level(context, bump),
            qz: self.qz.into_low_level(context, bump),
            qw: self.qw.into_low_level(context, bump),
            tx: self.tx.into_low_level(context, bump),
            ty: self.ty.into_low_level(context, bump),
            tz: self.tz.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for SrtDataNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            sx: crate::conv::FromLowLevel::from_low_level(context, value.sx),
            a: crate::conv::FromLowLevel::from_low_level(context, value.a),
            b: crate::conv::FromLowLevel::from_low_level(context, value.b),
            pvx: crate::conv::FromLowLevel::from_low_level(context, value.pvx),
            sy: crate::conv::FromLowLevel::from_low_level(context, value.sy),
            c: crate::conv::FromLowLevel::from_low_level(context, value.c),
            pvy: crate::conv::FromLowLevel::from_low_level(context, value.pvy),
            sz: crate::conv::FromLowLevel::from_low_level(context, value.sz),
            pvz: crate::conv::FromLowLevel::from_low_level(context, value.pvz),
            qx: crate::conv::FromLowLevel::from_low_level(context, value.qx),
            qy: crate::conv::FromLowLevel::from_low_level(context, value.qy),
            qz: crate::conv::FromLowLevel::from_low_level(context, value.qz),
            qw: crate::conv::FromLowLevel::from_low_level(context, value.qw),
            tx: crate::conv::FromLowLevel::from_low_level(context, value.tx),
            ty: crate::conv::FromLowLevel::from_low_level(context, value.ty),
            tz: crate::conv::FromLowLevel::from_low_level(context, value.tz),
        }
    }
}
impl AccelerationStructureSrtMotionInstanceNV {
    ///Get a reference to the `transform_t0` field.
    pub fn transform_t0(&self) -> SrtDataNV {
        self.transform_t0
    }
    ///Get a reference to the `transform_t1` field.
    pub fn transform_t1(&self) -> SrtDataNV {
        self.transform_t1
    }
    ///Get a reference to the `instance_custom_index` field.
    pub fn instance_custom_index(&self) -> u32 {
        self.instance_custom_index
    }
    ///Get a reference to the `mask` field.
    pub fn mask(&self) -> u32 {
        self.mask
    }
    ///Get a reference to the `instance_shader_binding_table_record_offset` field.
    pub fn instance_shader_binding_table_record_offset(&self) -> u32 {
        self.instance_shader_binding_table_record_offset
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> GeometryInstanceFlagsKHR {
        self.flags
    }
    ///Get a reference to the `acceleration_structure_reference` field.
    pub fn acceleration_structure_reference(&self) -> u64 {
        self.acceleration_structure_reference
    }
    ///Get a mutable reference to the `transform_t0` field.
    pub fn transform_t0_mut(&mut self) -> &mut SrtDataNV {
        &mut self.transform_t0
    }
    ///Get a mutable reference to the `transform_t1` field.
    pub fn transform_t1_mut(&mut self) -> &mut SrtDataNV {
        &mut self.transform_t1
    }
    ///Get a mutable reference to the `instance_custom_index` field.
    pub fn instance_custom_index_mut(&mut self) -> &mut u32 {
        &mut self.instance_custom_index
    }
    ///Get a mutable reference to the `mask` field.
    pub fn mask_mut(&mut self) -> &mut u32 {
        &mut self.mask
    }
    ///Get a mutable reference to the `instance_shader_binding_table_record_offset` field.
    pub fn instance_shader_binding_table_record_offset_mut(&mut self) -> &mut u32 {
        &mut self.instance_shader_binding_table_record_offset
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut GeometryInstanceFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `acceleration_structure_reference` field.
    pub fn acceleration_structure_reference_mut(&mut self) -> &mut u64 {
        &mut self.acceleration_structure_reference
    }
    ///Sets the `transform_t0` field.
    pub fn set_transform_t0(&mut self, transform_t0: SrtDataNV) -> &mut Self {
        self.transform_t0 = transform_t0;
        self
    }
    ///Sets the `transform_t1` field.
    pub fn set_transform_t1(&mut self, transform_t1: SrtDataNV) -> &mut Self {
        self.transform_t1 = transform_t1;
        self
    }
    ///Sets the `instance_custom_index` field.
    pub fn set_instance_custom_index(&mut self, instance_custom_index: u32) -> &mut Self {
        self.instance_custom_index = instance_custom_index;
        self
    }
    ///Sets the `mask` field.
    pub fn set_mask(&mut self, mask: u32) -> &mut Self {
        self.mask = mask;
        self
    }
    ///Sets the `instance_shader_binding_table_record_offset` field.
    pub fn set_instance_shader_binding_table_record_offset(
        &mut self,
        instance_shader_binding_table_record_offset: u32,
    ) -> &mut Self {
        self.instance_shader_binding_table_record_offset = instance_shader_binding_table_record_offset;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: GeometryInstanceFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `acceleration_structure_reference` field.
    pub fn set_acceleration_structure_reference(&mut self, acceleration_structure_reference: u64) -> &mut Self {
        self.acceleration_structure_reference = acceleration_structure_reference;
        self
    }
    ///Sets the `transform_t0` field in a builder way.
    pub fn with_transform_t0(mut self, transform_t0: SrtDataNV) -> Self {
        self.transform_t0 = transform_t0;
        self
    }
    ///Sets the `transform_t1` field in a builder way.
    pub fn with_transform_t1(mut self, transform_t1: SrtDataNV) -> Self {
        self.transform_t1 = transform_t1;
        self
    }
    ///Sets the `instance_custom_index` field in a builder way.
    pub fn with_instance_custom_index(mut self, instance_custom_index: u32) -> Self {
        self.instance_custom_index = instance_custom_index;
        self
    }
    ///Sets the `mask` field in a builder way.
    pub fn with_mask(mut self, mask: u32) -> Self {
        self.mask = mask;
        self
    }
    ///Sets the `instance_shader_binding_table_record_offset` field in a builder way.
    pub fn with_instance_shader_binding_table_record_offset(
        mut self,
        instance_shader_binding_table_record_offset: u32,
    ) -> Self {
        self.instance_shader_binding_table_record_offset = instance_shader_binding_table_record_offset;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: GeometryInstanceFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `acceleration_structure_reference` field in a builder way.
    pub fn with_acceleration_structure_reference(mut self, acceleration_structure_reference: u64) -> Self {
        self.acceleration_structure_reference = acceleration_structure_reference;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureSrtMotionInstanceNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureSrtMotionInstanceNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureSrtMotionInstanceNV {
            transform_t0: self.transform_t0.into_low_level(context, bump),
            transform_t1: self.transform_t1.into_low_level(context, bump),
            instance_custom_index: self.instance_custom_index.into_low_level(context, bump),
            mask: self.mask.into_low_level(context, bump),
            instance_shader_binding_table_record_offset: self
                .instance_shader_binding_table_record_offset
                .into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            acceleration_structure_reference: self.acceleration_structure_reference.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureSrtMotionInstanceNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            transform_t0: crate::conv::FromLowLevel::from_low_level(context, value.transform_t0),
            transform_t1: crate::conv::FromLowLevel::from_low_level(context, value.transform_t1),
            instance_custom_index: crate::conv::FromLowLevel::from_low_level(context, value.instance_custom_index),
            mask: crate::conv::FromLowLevel::from_low_level(context, value.mask),
            instance_shader_binding_table_record_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.instance_shader_binding_table_record_offset,
            ),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            acceleration_structure_reference: crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structure_reference,
            ),
        }
    }
}
impl AccelerationStructureMatrixMotionInstanceNV {
    ///Get a reference to the `transform_t0` field.
    pub fn transform_t0(&self) -> TransformMatrixKHR {
        self.transform_t0
    }
    ///Get a reference to the `transform_t1` field.
    pub fn transform_t1(&self) -> TransformMatrixKHR {
        self.transform_t1
    }
    ///Get a reference to the `instance_custom_index` field.
    pub fn instance_custom_index(&self) -> u32 {
        self.instance_custom_index
    }
    ///Get a reference to the `mask` field.
    pub fn mask(&self) -> u32 {
        self.mask
    }
    ///Get a reference to the `instance_shader_binding_table_record_offset` field.
    pub fn instance_shader_binding_table_record_offset(&self) -> u32 {
        self.instance_shader_binding_table_record_offset
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> GeometryInstanceFlagsKHR {
        self.flags
    }
    ///Get a reference to the `acceleration_structure_reference` field.
    pub fn acceleration_structure_reference(&self) -> u64 {
        self.acceleration_structure_reference
    }
    ///Get a mutable reference to the `transform_t0` field.
    pub fn transform_t0_mut(&mut self) -> &mut TransformMatrixKHR {
        &mut self.transform_t0
    }
    ///Get a mutable reference to the `transform_t1` field.
    pub fn transform_t1_mut(&mut self) -> &mut TransformMatrixKHR {
        &mut self.transform_t1
    }
    ///Get a mutable reference to the `instance_custom_index` field.
    pub fn instance_custom_index_mut(&mut self) -> &mut u32 {
        &mut self.instance_custom_index
    }
    ///Get a mutable reference to the `mask` field.
    pub fn mask_mut(&mut self) -> &mut u32 {
        &mut self.mask
    }
    ///Get a mutable reference to the `instance_shader_binding_table_record_offset` field.
    pub fn instance_shader_binding_table_record_offset_mut(&mut self) -> &mut u32 {
        &mut self.instance_shader_binding_table_record_offset
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut GeometryInstanceFlagsKHR {
        &mut self.flags
    }
    ///Get a mutable reference to the `acceleration_structure_reference` field.
    pub fn acceleration_structure_reference_mut(&mut self) -> &mut u64 {
        &mut self.acceleration_structure_reference
    }
    ///Sets the `transform_t0` field.
    pub fn set_transform_t0(&mut self, transform_t0: TransformMatrixKHR) -> &mut Self {
        self.transform_t0 = transform_t0;
        self
    }
    ///Sets the `transform_t1` field.
    pub fn set_transform_t1(&mut self, transform_t1: TransformMatrixKHR) -> &mut Self {
        self.transform_t1 = transform_t1;
        self
    }
    ///Sets the `instance_custom_index` field.
    pub fn set_instance_custom_index(&mut self, instance_custom_index: u32) -> &mut Self {
        self.instance_custom_index = instance_custom_index;
        self
    }
    ///Sets the `mask` field.
    pub fn set_mask(&mut self, mask: u32) -> &mut Self {
        self.mask = mask;
        self
    }
    ///Sets the `instance_shader_binding_table_record_offset` field.
    pub fn set_instance_shader_binding_table_record_offset(
        &mut self,
        instance_shader_binding_table_record_offset: u32,
    ) -> &mut Self {
        self.instance_shader_binding_table_record_offset = instance_shader_binding_table_record_offset;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: GeometryInstanceFlagsKHR) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `acceleration_structure_reference` field.
    pub fn set_acceleration_structure_reference(&mut self, acceleration_structure_reference: u64) -> &mut Self {
        self.acceleration_structure_reference = acceleration_structure_reference;
        self
    }
    ///Sets the `transform_t0` field in a builder way.
    pub fn with_transform_t0(mut self, transform_t0: TransformMatrixKHR) -> Self {
        self.transform_t0 = transform_t0;
        self
    }
    ///Sets the `transform_t1` field in a builder way.
    pub fn with_transform_t1(mut self, transform_t1: TransformMatrixKHR) -> Self {
        self.transform_t1 = transform_t1;
        self
    }
    ///Sets the `instance_custom_index` field in a builder way.
    pub fn with_instance_custom_index(mut self, instance_custom_index: u32) -> Self {
        self.instance_custom_index = instance_custom_index;
        self
    }
    ///Sets the `mask` field in a builder way.
    pub fn with_mask(mut self, mask: u32) -> Self {
        self.mask = mask;
        self
    }
    ///Sets the `instance_shader_binding_table_record_offset` field in a builder way.
    pub fn with_instance_shader_binding_table_record_offset(
        mut self,
        instance_shader_binding_table_record_offset: u32,
    ) -> Self {
        self.instance_shader_binding_table_record_offset = instance_shader_binding_table_record_offset;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: GeometryInstanceFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `acceleration_structure_reference` field in a builder way.
    pub fn with_acceleration_structure_reference(mut self, acceleration_structure_reference: u64) -> Self {
        self.acceleration_structure_reference = acceleration_structure_reference;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMatrixMotionInstanceNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMatrixMotionInstanceNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMatrixMotionInstanceNV {
            transform_t0: self.transform_t0.into_low_level(context, bump),
            transform_t1: self.transform_t1.into_low_level(context, bump),
            instance_custom_index: self.instance_custom_index.into_low_level(context, bump),
            mask: self.mask.into_low_level(context, bump),
            instance_shader_binding_table_record_offset: self
                .instance_shader_binding_table_record_offset
                .into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            acceleration_structure_reference: self.acceleration_structure_reference.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMatrixMotionInstanceNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            transform_t0: crate::conv::FromLowLevel::from_low_level(context, value.transform_t0),
            transform_t1: crate::conv::FromLowLevel::from_low_level(context, value.transform_t1),
            instance_custom_index: crate::conv::FromLowLevel::from_low_level(context, value.instance_custom_index),
            mask: crate::conv::FromLowLevel::from_low_level(context, value.mask),
            instance_shader_binding_table_record_offset: crate::conv::FromLowLevel::from_low_level(
                context,
                value.instance_shader_binding_table_record_offset,
            ),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            acceleration_structure_reference: crate::conv::FromLowLevel::from_low_level(
                context,
                value.acceleration_structure_reference,
            ),
        }
    }
}
#[doc(alias = "VkAccelerationStructureMotionInstanceNV")]
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureMotionInstanceNV {
    #[doc(alias = "type")]
    pub type_: AccelerationStructureMotionInstanceTypeNV,
    pub flags: AccelerationStructureMotionInstanceFlagsNV,
    pub data: AccelerationStructureMotionInstanceDataNV,
}
impl AccelerationStructureMotionInstanceNV {
    ///Get a reference to the `type_` field.
    pub fn type_(&self) -> AccelerationStructureMotionInstanceTypeNV {
        self.type_
    }
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> AccelerationStructureMotionInstanceFlagsNV {
        self.flags
    }
    ///Get a reference to the `data` field.
    pub fn data(&self) -> AccelerationStructureMotionInstanceDataNV {
        self.data
    }
    ///Get a mutable reference to the `type_` field.
    pub fn type__mut(&mut self) -> &mut AccelerationStructureMotionInstanceTypeNV {
        &mut self.type_
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut AccelerationStructureMotionInstanceFlagsNV {
        &mut self.flags
    }
    ///Get a mutable reference to the `data` field.
    pub fn data_mut(&mut self) -> &mut AccelerationStructureMotionInstanceDataNV {
        &mut self.data
    }
    ///Sets the `type_` field.
    pub fn set_type_(&mut self, type_: AccelerationStructureMotionInstanceTypeNV) -> &mut Self {
        self.type_ = type_;
        self
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: AccelerationStructureMotionInstanceFlagsNV) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `data` field.
    pub fn set_data(&mut self, data: AccelerationStructureMotionInstanceDataNV) -> &mut Self {
        self.data = data;
        self
    }
    ///Sets the `type_` field in a builder way.
    pub fn with_type_(mut self, type_: AccelerationStructureMotionInstanceTypeNV) -> Self {
        self.type_ = type_;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: AccelerationStructureMotionInstanceFlagsNV) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `data` field in a builder way.
    pub fn with_data(mut self, data: AccelerationStructureMotionInstanceDataNV) -> Self {
        self.data = data;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMotionInstanceNV {
    type LowLevel = crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceNV;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceNV {
            type_: self.type_.into_low_level(context, bump),
            flags: self.flags.into_low_level(context, bump),
            data: self.data.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMotionInstanceNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            type_: crate::conv::FromLowLevel::from_low_level(context, value.type_),
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            data: crate::conv::FromLowLevel::from_low_level(context, value.data),
        }
    }
}
#[doc(alias = "VkAccelerationStructureMotionInstanceDataNV")]
#[derive(Clone, PartialEq, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccelerationStructureMotionInstanceDataNV {
    StaticInstance(AccelerationStructureInstanceKHR),
    MatrixMotionInstance(AccelerationStructureMatrixMotionInstanceNV),
    SrtMotionInstance(AccelerationStructureSrtMotionInstanceNV),
}
impl From<AccelerationStructureInstanceKHR> for AccelerationStructureMotionInstanceDataNV {
    fn from(v: AccelerationStructureInstanceKHR) -> Self {
        Self::StaticInstance(v)
    }
}
impl From<AccelerationStructureMatrixMotionInstanceNV> for AccelerationStructureMotionInstanceDataNV {
    fn from(v: AccelerationStructureMatrixMotionInstanceNV) -> Self {
        Self::MatrixMotionInstance(v)
    }
}
impl From<AccelerationStructureSrtMotionInstanceNV> for AccelerationStructureMotionInstanceDataNV {
    fn from(v: AccelerationStructureSrtMotionInstanceNV) -> Self {
        Self::SrtMotionInstance(v)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureMotionInstanceDataNV {
    type LowLevel = (
        AccelerationStructureMotionInstanceTypeNV,
        crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV,
    );
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        match self {
            Self::StaticInstance(v) => {
                crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV {
                    static_instance: (v.into_low_level(context, bump)),
                }
            },
            Self::MatrixMotionInstance(v) => {
                crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV {
                    matrix_motion_instance: (v.into_low_level(context, bump)),
                }
            },
            Self::SrtMotionInstance(v) => {
                crate::native::extensions::nv_ray_tracing_motion_blur::AccelerationStructureMotionInstanceDataNV {
                    srt_motion_instance: (v.into_low_level(context, bump)),
                }
            },
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureMotionInstanceDataNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        (variant, value): <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        match variant {
            STATIC => Self::StaticInstance(value.static_instance.into_high_level(context)),
            MATRIX_MOTION => Self::MatrixMotionInstance(value.matrix_motion_instance.into_high_level(context)),
            SRT_MOTION => Self::SrtMotionInstance(value.srt_motion_instance.into_high_level(context)),
        }
    }
}
