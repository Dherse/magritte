pub use crate::common::extensions::ext_descriptor_indexing::{
    EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME, EXT_DESCRIPTOR_INDEXING_SPEC_VERSION,
};
use crate::vulkan1_2::{
    DescriptorBindingFlagBits, DescriptorBindingFlags, DescriptorSetLayoutBindingFlagsCreateInfo,
    DescriptorSetVariableDescriptorCountAllocateInfo, DescriptorSetVariableDescriptorCountLayoutSupport,
    PhysicalDeviceDescriptorIndexingFeatures, PhysicalDeviceDescriptorIndexingProperties,
};
#[doc(alias = "VkDescriptorBindingFlagsEXT")]
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
#[doc(alias = "VkDescriptorBindingFlagBitsEXT")]
pub type DescriptorBindingFlagBitsEXT = DescriptorBindingFlagBits;
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeaturesEXT")]
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT = PhysicalDeviceDescriptorIndexingFeatures;
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingPropertiesEXT")]
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT = PhysicalDeviceDescriptorIndexingProperties;
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfoEXT")]
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT = DescriptorSetLayoutBindingFlagsCreateInfo;
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfoEXT")]
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT = DescriptorSetVariableDescriptorCountAllocateInfo;
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupportEXT")]
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT = DescriptorSetVariableDescriptorCountLayoutSupport;
