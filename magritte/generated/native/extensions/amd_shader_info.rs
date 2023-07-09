pub use crate::common::extensions::amd_shader_info::{
    ShaderInfoTypeAMD, ShaderResourceUsageAMD, ShaderStatisticsInfoAMD, AMD_SHADER_INFO_EXTENSION_NAME,
    AMD_SHADER_INFO_SPEC_VERSION,
};
use crate::native::vulkan1_0::{Device, Pipeline, ShaderStageFlagBits, VulkanResultCodes};
#[doc(alias = "vkGetShaderInfoAMD")]
pub type FNGetShaderInfoAmd = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    shader_stage: ShaderStageFlagBits,
    info_type: ShaderInfoTypeAMD,
    p_info_size: *mut usize,
    p_info: *mut std::ffi::c_void,
) -> VulkanResultCodes;
