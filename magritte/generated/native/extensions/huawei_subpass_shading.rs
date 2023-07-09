use crate::native::vulkan1_0::{
    BaseOutStructure, Bool32, CommandBuffer, Device, Extent2D, RenderPass, StructureType, VulkanResultCodes,
};
#[doc(alias = "VkSubpassShadingPipelineCreateInfoHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SubpassShadingPipelineCreateInfoHUAWEI {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "renderPass")]
    pub render_pass: RenderPass,
    pub subpass: u32,
}
impl Default for SubpassShadingPipelineCreateInfoHUAWEI {
    fn default() -> Self {
        Self {
            s_type: StructureType::SubpassShadingPipelineCreateInfoHuawei,
            p_next: unsafe { std::mem::zeroed() },
            render_pass: unsafe { std::mem::zeroed() },
            subpass: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubpassShadingPropertiesHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxSubpassShadingWorkgroupSizeAspectRatio")]
    pub max_subpass_shading_workgroup_size_aspect_ratio: u32,
}
impl Default for PhysicalDeviceSubpassShadingPropertiesHUAWEI {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSubpassShadingPropertiesHuawei,
            p_next: unsafe { std::mem::zeroed() },
            max_subpass_shading_workgroup_size_aspect_ratio: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceSubpassShadingFeaturesHUAWEI")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "subpassShading")]
    pub subpass_shading: Bool32,
}
impl Default for PhysicalDeviceSubpassShadingFeaturesHUAWEI {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceSubpassShadingFeaturesHuawei,
            p_next: unsafe { std::mem::zeroed() },
            subpass_shading: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::huawei_subpass_shading::{
    HUAWEI_SUBPASS_SHADING_EXTENSION_NAME, HUAWEI_SUBPASS_SHADING_SPEC_VERSION,
};
#[doc(alias = "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI")]
pub type FNGetDeviceSubpassShadingMaxWorkgroupSizeHuawei = unsafe extern "system" fn(
    device: Device,
    renderpass: RenderPass,
    p_max_workgroup_size: *mut Extent2D,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdSubpassShadingHUAWEI")]
pub type FNCmdSubpassShadingHuawei = unsafe extern "system" fn(command_buffer: CommandBuffer);
