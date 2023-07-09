use crate::native::vulkan1_3::{
    DescriptorPoolInlineUniformBlockCreateInfo, PhysicalDeviceInlineUniformBlockFeatures,
    PhysicalDeviceInlineUniformBlockProperties, WriteDescriptorSetInlineUniformBlock,
};
///See [`PhysicalDeviceInlineUniformBlockFeatures`]
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockFeaturesEXT")]
pub type PhysicalDeviceInlineUniformBlockFeaturesEXT = PhysicalDeviceInlineUniformBlockFeatures;
///See [`PhysicalDeviceInlineUniformBlockProperties`]
#[doc(alias = "VkPhysicalDeviceInlineUniformBlockPropertiesEXT")]
pub type PhysicalDeviceInlineUniformBlockPropertiesEXT = PhysicalDeviceInlineUniformBlockProperties;
///See [`WriteDescriptorSetInlineUniformBlock`]
#[doc(alias = "VkWriteDescriptorSetInlineUniformBlockEXT")]
pub type WriteDescriptorSetInlineUniformBlockEXT = WriteDescriptorSetInlineUniformBlock;
///See [`DescriptorPoolInlineUniformBlockCreateInfo`]
#[doc(alias = "VkDescriptorPoolInlineUniformBlockCreateInfoEXT")]
pub type DescriptorPoolInlineUniformBlockCreateInfoEXT = DescriptorPoolInlineUniformBlockCreateInfo;
pub use crate::common::extensions::ext_inline_uniform_block::{
    EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME, EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION,
};
