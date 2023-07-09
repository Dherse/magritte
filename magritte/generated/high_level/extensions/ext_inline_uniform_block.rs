pub use crate::common::extensions::ext_inline_uniform_block::{
    EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME, EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION,
};
use crate::vulkan1_3::{
    DescriptorPoolInlineUniformBlockCreateInfo, PhysicalDeviceInlineUniformBlockFeatures,
    PhysicalDeviceInlineUniformBlockProperties, WriteDescriptorSetInlineUniformBlock,
};
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockFeaturesEXT")]
pub type PhysicalDeviceInlineUniformBlockFeaturesEXT = PhysicalDeviceInlineUniformBlockFeatures;
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockPropertiesEXT")]
pub type PhysicalDeviceInlineUniformBlockPropertiesEXT = PhysicalDeviceInlineUniformBlockProperties;
#[doc(alias = "VkWriteDescriptorSetInlineUniformBlockEXT")]
pub type WriteDescriptorSetInlineUniformBlockEXT = WriteDescriptorSetInlineUniformBlock;
#[doc(alias = "VkDescriptorPoolInlineUniformBlockCreateInfoEXT")]
pub type DescriptorPoolInlineUniformBlockCreateInfoEXT = DescriptorPoolInlineUniformBlockCreateInfo;
