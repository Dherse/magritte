use crate::{
    cstr,
    vulkan1_2::{
        DescriptorBindingFlagBits, DescriptorBindingFlags, DescriptorSetLayoutBindingFlagsCreateInfo,
        DescriptorSetVariableDescriptorCountAllocateInfo, DescriptorSetVariableDescriptorCountLayoutSupport,
        PhysicalDeviceDescriptorIndexingFeatures, PhysicalDeviceDescriptorIndexingProperties,
    },
};
use std::ffi::CStr;
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
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION")]
pub const EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME")]
pub const EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_descriptor_indexing");
