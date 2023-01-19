//!# [VK_INTEL_shader_integer_functions2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_INTEL_shader_integer_functions2.html)
# ! [doc = include_str ! ("../../../../doc/extensions/intel_shader_integer_functions2/VK_INTEL_shader_integer_functions2.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_shader_integer_functions2/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.md")]
#[doc(alias = "VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderIntegerFunctions2")]
    shader_integer_functions2: Bool32,
}
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: &'static CStr = cstr!("VK_INTEL_shader_integer_functions2");
