use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkVertexInputBindingDivisorDescriptionEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    binding: u32,
    divisor: u32,
}
#[doc(alias = "VkPipelineVertexInputDivisorStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "vertexBindingDivisorCount")]
    vertex_binding_divisor_count: u32,
    #[doc(alias = "pVertexBindingDivisors")]
    vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxVertexAttribDivisor")]
    max_vertex_attrib_divisor: u32,
}
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "vertexAttributeInstanceRateDivisor")]
    vertex_attribute_instance_rate_divisor: Bool32,
    #[doc(alias = "vertexAttributeInstanceRateZeroDivisor")]
    vertex_attribute_instance_rate_zero_divisor: Bool32,
}
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_vertex_attribute_divisor");
