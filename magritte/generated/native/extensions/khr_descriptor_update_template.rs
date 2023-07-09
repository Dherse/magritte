use crate::native::{
    vulkan1_0::{CommandBuffer, PipelineLayout},
    vulkan1_1::{
        DescriptorUpdateTemplate, DescriptorUpdateTemplateCreateFlags, DescriptorUpdateTemplateCreateInfo,
        DescriptorUpdateTemplateEntry, DescriptorUpdateTemplateType, FNCreateDescriptorUpdateTemplate,
        FNDestroyDescriptorUpdateTemplate, FNUpdateDescriptorSetWithTemplate,
    },
};
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
pub use crate::common::extensions::khr_descriptor_update_template::{
    KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME, KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION,
};
///See [`create_descriptor_update_template`]
#[doc(alias = "vkCreateDescriptorUpdateTemplateKHR")]
pub type FNCreateDescriptorUpdateTemplateKhr = FNCreateDescriptorUpdateTemplate;
///See [`destroy_descriptor_update_template`]
#[doc(alias = "vkDestroyDescriptorUpdateTemplateKHR")]
pub type FNDestroyDescriptorUpdateTemplateKhr = FNDestroyDescriptorUpdateTemplate;
///See [`update_descriptor_set_with_template`]
#[doc(alias = "vkUpdateDescriptorSetWithTemplateKHR")]
pub type FNUpdateDescriptorSetWithTemplateKhr = FNUpdateDescriptorSetWithTemplate;
#[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
pub type FNCmdPushDescriptorSetWithTemplateKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    p_data: *const std::ffi::c_void,
);
