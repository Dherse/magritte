pub use crate::common::extensions::ext_vertex_attribute_divisor::VertexInputBindingDivisorDescriptionEXT;
use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPipelineVertexInputDivisorStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "vertexBindingDivisorCount")]
    pub vertex_binding_divisor_count: u32,
    #[doc(alias = "pVertexBindingDivisors")]
    pub vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}
impl Default for PipelineVertexInputDivisorStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineVertexInputDivisorStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            vertex_binding_divisor_count: unsafe { std::mem::zeroed() },
            vertex_binding_divisors: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "maxVertexAttribDivisor")]
    pub max_vertex_attrib_divisor: u32,
}
impl Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVertexAttributeDivisorPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            max_vertex_attrib_divisor: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "vertexAttributeInstanceRateDivisor")]
    pub vertex_attribute_instance_rate_divisor: Bool32,
    #[doc(alias = "vertexAttributeInstanceRateZeroDivisor")]
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
}
impl Default for PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceVertexAttributeDivisorFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            vertex_attribute_instance_rate_divisor: unsafe { std::mem::zeroed() },
            vertex_attribute_instance_rate_zero_divisor: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_vertex_attribute_divisor::{
    EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME, EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION,
};
