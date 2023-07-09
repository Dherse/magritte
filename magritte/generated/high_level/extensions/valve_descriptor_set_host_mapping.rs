//!# [VK_VALVE_descriptor_set_host_mapping](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VALVE_descriptor_set_host_mapping.html)
# ! [doc = include_str ! ("../../../../doc/extensions/valve_descriptor_set_host_mapping/VK_VALVE_descriptor_set_host_mapping.md")]
use crate::vulkan1_0::DescriptorSetLayout;
///# [VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.html)
# [doc = include_str ! ("../../../../doc/extensions/valve_descriptor_set_host_mapping/VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE.md")]
#[doc(alias = "VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    #[doc(alias = "descriptorSetHostMapping")]
    descriptor_set_host_mapping: bool,
}
impl PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    ///Get a reference to the `descriptor_set_host_mapping` field.
    pub fn descriptor_set_host_mapping(&self) -> bool {
        self.descriptor_set_host_mapping
    }
    ///Get a mutable reference to the `descriptor_set_host_mapping` field.
    pub fn descriptor_set_host_mapping_mut(&mut self) -> &mut bool {
        &mut self.descriptor_set_host_mapping
    }
    ///Sets the `descriptor_set_host_mapping` field.
    pub fn set_descriptor_set_host_mapping(&mut self, descriptor_set_host_mapping: bool) -> &mut Self {
        self.descriptor_set_host_mapping = descriptor_set_host_mapping;
        self
    }
    ///Sets the `descriptor_set_host_mapping` field in a builder way.
    pub fn with_descriptor_set_host_mapping(mut self, descriptor_set_host_mapping: bool) -> Self {
        self.descriptor_set_host_mapping = descriptor_set_host_mapping;
        self
    }
}
///# [VkDescriptorSetBindingReferenceVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetBindingReferenceVALVE.html)
# [doc = include_str ! ("../../../../doc/extensions/valve_descriptor_set_host_mapping/VkDescriptorSetBindingReferenceVALVE.md")]
#[doc(alias = "VkDescriptorSetBindingReferenceVALVE")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DescriptorSetBindingReferenceVALVE {
    #[doc(alias = "descriptorSetLayout")]
    descriptor_set_layout: Option<DescriptorSetLayout>,
    binding: u32,
}
impl DescriptorSetBindingReferenceVALVE {
    ///Get a reference to the `descriptor_set_layout` field.
    pub fn descriptor_set_layout(&self) -> &Option<DescriptorSetLayout> {
        &self.descriptor_set_layout
    }
    ///Get a reference to the `binding` field.
    pub fn binding(&self) -> u32 {
        self.binding
    }
    ///Get a mutable reference to the `descriptor_set_layout` field.
    pub fn descriptor_set_layout_mut(&mut self) -> &mut Option<DescriptorSetLayout> {
        &mut self.descriptor_set_layout
    }
    ///Get a mutable reference to the `binding` field.
    pub fn binding_mut(&mut self) -> &mut u32 {
        &mut self.binding
    }
    ///Sets the `descriptor_set_layout` field.
    pub fn set_descriptor_set_layout(&mut self, descriptor_set_layout: Option<DescriptorSetLayout>) -> &mut Self {
        self.descriptor_set_layout = descriptor_set_layout;
        self
    }
    ///Sets the `binding` field.
    pub fn set_binding(&mut self, binding: u32) -> &mut Self {
        self.binding = binding;
        self
    }
    ///Sets the `descriptor_set_layout` field in a builder way.
    pub fn with_descriptor_set_layout(mut self, descriptor_set_layout: Option<DescriptorSetLayout>) -> Self {
        self.descriptor_set_layout = descriptor_set_layout;
        self
    }
    ///Sets the `binding` field in a builder way.
    pub fn with_binding(mut self, binding: u32) -> Self {
        self.binding = binding;
        self
    }
}
///# [VkDescriptorSetLayoutHostMappingInfoVALVE](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayoutHostMappingInfoVALVE.html)
# [doc = include_str ! ("../../../../doc/extensions/valve_descriptor_set_host_mapping/VkDescriptorSetLayoutHostMappingInfoVALVE.md")]
#[doc(alias = "VkDescriptorSetLayoutHostMappingInfoVALVE")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DescriptorSetLayoutHostMappingInfoVALVE {
    #[doc(alias = "descriptorOffset")]
    descriptor_offset: usize,
    #[doc(alias = "descriptorSize")]
    descriptor_size: u32,
}
impl DescriptorSetLayoutHostMappingInfoVALVE {
    ///Get a reference to the `descriptor_offset` field.
    pub fn descriptor_offset(&self) -> usize {
        self.descriptor_offset
    }
    ///Get a reference to the `descriptor_size` field.
    pub fn descriptor_size(&self) -> u32 {
        self.descriptor_size
    }
    ///Get a mutable reference to the `descriptor_offset` field.
    pub fn descriptor_offset_mut(&mut self) -> &mut usize {
        &mut self.descriptor_offset
    }
    ///Get a mutable reference to the `descriptor_size` field.
    pub fn descriptor_size_mut(&mut self) -> &mut u32 {
        &mut self.descriptor_size
    }
    ///Sets the `descriptor_offset` field.
    pub fn set_descriptor_offset(&mut self, descriptor_offset: usize) -> &mut Self {
        self.descriptor_offset = descriptor_offset;
        self
    }
    ///Sets the `descriptor_size` field.
    pub fn set_descriptor_size(&mut self, descriptor_size: u32) -> &mut Self {
        self.descriptor_size = descriptor_size;
        self
    }
    ///Sets the `descriptor_offset` field in a builder way.
    pub fn with_descriptor_offset(mut self, descriptor_offset: usize) -> Self {
        self.descriptor_offset = descriptor_offset;
        self
    }
    ///Sets the `descriptor_size` field in a builder way.
    pub fn with_descriptor_size(mut self, descriptor_size: u32) -> Self {
        self.descriptor_size = descriptor_size;
        self
    }
}
