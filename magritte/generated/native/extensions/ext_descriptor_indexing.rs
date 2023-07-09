use crate::native::vulkan1_2::{
    DescriptorBindingFlagBits, DescriptorBindingFlags, DescriptorSetLayoutBindingFlagsCreateInfo,
    DescriptorSetVariableDescriptorCountAllocateInfo, DescriptorSetVariableDescriptorCountLayoutSupport,
    PhysicalDeviceDescriptorIndexingFeatures, PhysicalDeviceDescriptorIndexingProperties,
};
///See [`DescriptorBindingFlags`]
#[doc(alias = "VkDescriptorBindingFlagsEXT")]
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
///See [`DescriptorBindingFlagBits`]
#[doc(alias = "VkDescriptorBindingFlagBitsEXT")]
pub type DescriptorBindingFlagBitsEXT = DescriptorBindingFlagBits;
///See [`PhysicalDeviceDescriptorIndexingFeatures`]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingFeaturesEXT")]
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT = PhysicalDeviceDescriptorIndexingFeatures;
///See [`PhysicalDeviceDescriptorIndexingProperties`]
#[doc(alias = "VkPhysicalDeviceDescriptorIndexingPropertiesEXT")]
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT = PhysicalDeviceDescriptorIndexingProperties;
///See [`DescriptorSetLayoutBindingFlagsCreateInfo`]
#[doc(alias = "VkDescriptorSetLayoutBindingFlagsCreateInfoEXT")]
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT = DescriptorSetLayoutBindingFlagsCreateInfo;
///See [`DescriptorSetVariableDescriptorCountAllocateInfo`]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountAllocateInfoEXT")]
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT = DescriptorSetVariableDescriptorCountAllocateInfo;
///See [`DescriptorSetVariableDescriptorCountLayoutSupport`]
#[doc(alias = "VkDescriptorSetVariableDescriptorCountLayoutSupportEXT")]
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT = DescriptorSetVariableDescriptorCountLayoutSupport;
pub use crate::common::extensions::ext_descriptor_indexing::{
    EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME, EXT_DESCRIPTOR_INDEXING_SPEC_VERSION,
};
