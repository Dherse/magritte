pub use crate::common::extensions::amd_shader_info::{
    ShaderInfoTypeAMD, ShaderResourceUsageAMD, ShaderStatisticsInfoAMD, AMD_SHADER_INFO_EXTENSION_NAME,
    AMD_SHADER_INFO_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::ShaderStageFlags};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
impl ShaderResourceUsageAMD {
    ///Get a reference to the `num_used_vgprs` field.
    pub fn num_used_vgprs(&self) -> u32 {
        self.num_used_vgprs
    }
    ///Get a reference to the `num_used_sgprs` field.
    pub fn num_used_sgprs(&self) -> u32 {
        self.num_used_sgprs
    }
    ///Get a reference to the `lds_size_per_local_work_group` field.
    pub fn lds_size_per_local_work_group(&self) -> u32 {
        self.lds_size_per_local_work_group
    }
    ///Get a reference to the `lds_usage_size_in_bytes` field.
    pub fn lds_usage_size_in_bytes(&self) -> usize {
        self.lds_usage_size_in_bytes
    }
    ///Get a reference to the `scratch_mem_usage_in_bytes` field.
    pub fn scratch_mem_usage_in_bytes(&self) -> usize {
        self.scratch_mem_usage_in_bytes
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ShaderResourceUsageAMD {
    type LowLevel = crate::native::extensions::amd_shader_info::ShaderResourceUsageAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_shader_info::ShaderResourceUsageAMD {
            num_used_vgprs: self.num_used_vgprs.into_low_level(context, bump),
            num_used_sgprs: self.num_used_sgprs.into_low_level(context, bump),
            lds_size_per_local_work_group: self.lds_size_per_local_work_group.into_low_level(context, bump),
            lds_usage_size_in_bytes: self.lds_usage_size_in_bytes.into_low_level(context, bump),
            scratch_mem_usage_in_bytes: self.scratch_mem_usage_in_bytes.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ShaderResourceUsageAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            num_used_vgprs: crate::conv::FromLowLevel::from_low_level(context, value.num_used_vgprs),
            num_used_sgprs: crate::conv::FromLowLevel::from_low_level(context, value.num_used_sgprs),
            lds_size_per_local_work_group: crate::conv::FromLowLevel::from_low_level(
                context,
                value.lds_size_per_local_work_group,
            ),
            lds_usage_size_in_bytes: crate::conv::FromLowLevel::from_low_level(context, value.lds_usage_size_in_bytes),
            scratch_mem_usage_in_bytes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.scratch_mem_usage_in_bytes,
            ),
        }
    }
}
impl ShaderStatisticsInfoAMD {
    ///Get a reference to the `shader_stage_mask` field.
    pub fn shader_stage_mask(&self) -> ShaderStageFlags {
        self.shader_stage_mask
    }
    ///Get a reference to the `resource_usage` field.
    pub fn resource_usage(&self) -> ShaderResourceUsageAMD {
        self.resource_usage
    }
    ///Get a reference to the `num_physical_vgprs` field.
    pub fn num_physical_vgprs(&self) -> u32 {
        self.num_physical_vgprs
    }
    ///Get a reference to the `num_physical_sgprs` field.
    pub fn num_physical_sgprs(&self) -> u32 {
        self.num_physical_sgprs
    }
    ///Get a reference to the `num_available_vgprs` field.
    pub fn num_available_vgprs(&self) -> u32 {
        self.num_available_vgprs
    }
    ///Get a reference to the `num_available_sgprs` field.
    pub fn num_available_sgprs(&self) -> u32 {
        self.num_available_sgprs
    }
    ///Get a reference to the `compute_work_group_size` field.
    pub fn compute_work_group_size(&self) -> [u32; 3 as usize] {
        self.compute_work_group_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ShaderStatisticsInfoAMD {
    type LowLevel = crate::native::extensions::amd_shader_info::ShaderStatisticsInfoAMD;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::amd_shader_info::ShaderStatisticsInfoAMD {
            shader_stage_mask: self.shader_stage_mask.into_low_level(context, bump),
            resource_usage: self.resource_usage.into_low_level(context, bump),
            num_physical_vgprs: self.num_physical_vgprs.into_low_level(context, bump),
            num_physical_sgprs: self.num_physical_sgprs.into_low_level(context, bump),
            num_available_vgprs: self.num_available_vgprs.into_low_level(context, bump),
            num_available_sgprs: self.num_available_sgprs.into_low_level(context, bump),
            compute_work_group_size: self.compute_work_group_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ShaderStatisticsInfoAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            shader_stage_mask: crate::conv::FromLowLevel::from_low_level(context, value.shader_stage_mask),
            resource_usage: crate::conv::FromLowLevel::from_low_level(context, value.resource_usage),
            num_physical_vgprs: crate::conv::FromLowLevel::from_low_level(context, value.num_physical_vgprs),
            num_physical_sgprs: crate::conv::FromLowLevel::from_low_level(context, value.num_physical_sgprs),
            num_available_vgprs: crate::conv::FromLowLevel::from_low_level(context, value.num_available_vgprs),
            num_available_sgprs: crate::conv::FromLowLevel::from_low_level(context, value.num_available_sgprs),
            compute_work_group_size: crate::conv::FromLowLevel::from_low_level(context, value.compute_work_group_size),
        }
    }
}
