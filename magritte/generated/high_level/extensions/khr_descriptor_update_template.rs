pub use crate::common::extensions::khr_descriptor_update_template::{
    KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME, KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION,
};
use crate::vulkan1_1::{
    DescriptorUpdateTemplate, DescriptorUpdateTemplateCreateFlags, DescriptorUpdateTemplateCreateInfo,
    DescriptorUpdateTemplateEntry, DescriptorUpdateTemplateType,
};
#[doc(alias = "VkDescriptorUpdateTemplateCreateFlagsKHR")]
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
#[doc(alias = "VkDescriptorUpdateTemplateTypeKHR")]
pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;
#[doc(alias = "VkDescriptorUpdateTemplateEntryKHR")]
pub type DescriptorUpdateTemplateEntryKHR = DescriptorUpdateTemplateEntry;
#[doc(alias = "VkDescriptorUpdateTemplateCreateInfoKHR")]
pub type DescriptorUpdateTemplateCreateInfoKHR = DescriptorUpdateTemplateCreateInfo;
