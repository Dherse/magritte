use crate::{
    cstr,
    vulkan1_0::{
        BaseOutStructure, Bool32, CommandBuffer, Device, Extent2D, RenderPass, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassShadingPipelineCreateInfoHUAWEI {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "renderPass")]
    render_pass: RenderPass,
    subpass: u32,
}
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxSubpassShadingWorkgroupSizeAspectRatio")]
    max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "subpassShading")]
    subpass_shading: Bool32,
}
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION")]
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME")]
pub const HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: &'static CStr = cstr!("VK_HUAWEI_subpass_shading");
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
pub type FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei = unsafe extern "system" fn(
    device: Device,
    renderpass: RenderPass,
    p_max_workgroup_size: *mut Extent2D,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
pub type FNCmdSubpassShadingHuawei = unsafe extern "system" fn(command_buffer: CommandBuffer);
