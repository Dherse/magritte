//!# [VK_HUAWEI_subpass_shading](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_HUAWEI_subpass_shading.html)
# ! [doc = include_str ! ("../../../../doc/extensions/huawei_subpass_shading/VK_HUAWEI_subpass_shading.md")]
use crate::{
    cstr,
    vulkan1_0::{
        BaseOutStructure, Bool32, CommandBuffer, Device, Extent2D, RenderPass, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
///# [VkSubpassShadingPipelineCreateInfoHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSubpassShadingPipelineCreateInfoHUAWEI.html)
# [doc = include_str ! ("../../../../doc/extensions/huawei_subpass_shading/VkSubpassShadingPipelineCreateInfoHUAWEI.md")]
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
///# [VkPhysicalDeviceSubpassShadingPropertiesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.html)
# [doc = include_str ! ("../../../../doc/extensions/huawei_subpass_shading/VkPhysicalDeviceSubpassShadingPropertiesHUAWEI.md")]
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
///# [VkPhysicalDeviceSubpassShadingFeaturesHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.html)
# [doc = include_str ! ("../../../../doc/extensions/huawei_subpass_shading/VkPhysicalDeviceSubpassShadingFeaturesHUAWEI.md")]
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
///# [vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html)
# [doc = include_str ! ("../../../../doc/extensions/huawei_subpass_shading/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.md")]
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
pub type FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei = unsafe extern "system" fn(
    device: Device,
    renderpass: RenderPass,
    p_max_workgroup_size: *mut Extent2D,
) -> VulkanResultCodes;
///# [vkCmdSubpassShadingHUAWEI](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSubpassShadingHUAWEI.html)
# [doc = include_str ! ("../../../../doc/extensions/huawei_subpass_shading/vkCmdSubpassShadingHUAWEI.md")]
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
pub type FNCmdSubpassShadingHuawei = unsafe extern "system" fn(command_buffer: CommandBuffer);
