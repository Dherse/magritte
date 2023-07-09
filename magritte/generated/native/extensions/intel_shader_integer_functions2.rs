use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderIntegerFunctions2")]
    pub shader_integer_functions2: Bool32,
}
impl Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderIntegerFunctions2FeaturesIntel,
            p_next: unsafe { std::mem::zeroed() },
            shader_integer_functions2: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::intel_shader_integer_functions2::{
    INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME, INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION,
};
