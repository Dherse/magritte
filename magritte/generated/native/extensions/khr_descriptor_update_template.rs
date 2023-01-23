//!# [VK_KHR_descriptor_update_template](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_descriptor_update_template.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_descriptor_update_template/VK_KHR_descriptor_update_template.md")]
use crate::{
    cstr,
    vulkan1_0::{CommandBuffer, PipelineLayout},
    vulkan1_1::{
        DescriptorUpdateTemplate, DescriptorUpdateTemplateCreateFlags, DescriptorUpdateTemplateCreateInfo,
        DescriptorUpdateTemplateEntry, DescriptorUpdateTemplateType, FNCreateDescriptorUpdateTemplate,
        FNDestroyDescriptorUpdateTemplate, FNUpdateDescriptorSetWithTemplate,
    },
};
use std::ffi::CStr;
///See [`DescriptorUpdateTemplateCreateFlags`]
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlagsKHR")]
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;
///See [`DescriptorUpdateTemplate`]
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
///See [`DescriptorUpdateTemplateType`]
#[doc(alias = "VkDescriptorUpdateTemplateTypeKHR")]
pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;
///See [`DescriptorUpdateTemplateEntry`]
#[doc(alias = "VkDescriptorUpdateTemplateEntryKHR")]
pub type DescriptorUpdateTemplateEntryKHR = DescriptorUpdateTemplateEntry;
///See [`DescriptorUpdateTemplateCreateInfo`]
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfoKHR")]
pub type DescriptorUpdateTemplateCreateInfoKHR = DescriptorUpdateTemplateCreateInfo;
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_descriptor_update_template");
///See [`create_descriptor_update_template`]
#[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
pub type FNCreateDescriptorUpdateTemplateKhr = FNCreateDescriptorUpdateTemplate;
///See [`destroy_descriptor_update_template`]
#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
pub type FNDestroyDescriptorUpdateTemplateKhr = FNDestroyDescriptorUpdateTemplate;
///See [`update_descriptor_set_with_template`]
#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
pub type FNUpdateDescriptorSetWithTemplateKhr = FNUpdateDescriptorSetWithTemplate;
///# [vkCmdPushDescriptorSetWithTemplateKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_descriptor_update_template/vkCmdPushDescriptorSetWithTemplateKHR.md")]
#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
pub type FNCmdPushDescriptorSetWithTemplateKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    p_data: *const std::ffi::c_void,
);
