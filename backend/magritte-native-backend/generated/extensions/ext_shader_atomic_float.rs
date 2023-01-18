use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloatFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderBufferFloat32Atomics")]
    shader_buffer_float32_atomics: Bool32,
    #[doc(alias = "shaderBufferFloat32AtomicAdd")]
    shader_buffer_float32_atomic_add: Bool32,
    #[doc(alias = "shaderBufferFloat64Atomics")]
    shader_buffer_float64_atomics: Bool32,
    #[doc(alias = "shaderBufferFloat64AtomicAdd")]
    shader_buffer_float64_atomic_add: Bool32,
    #[doc(alias = "shaderSharedFloat32Atomics")]
    shader_shared_float32_atomics: Bool32,
    #[doc(alias = "shaderSharedFloat32AtomicAdd")]
    shader_shared_float32_atomic_add: Bool32,
    #[doc(alias = "shaderSharedFloat64Atomics")]
    shader_shared_float64_atomics: Bool32,
    #[doc(alias = "shaderSharedFloat64AtomicAdd")]
    shader_shared_float64_atomic_add: Bool32,
    #[doc(alias = "shaderImageFloat32Atomics")]
    shader_image_float32_atomics: Bool32,
    #[doc(alias = "shaderImageFloat32AtomicAdd")]
    shader_image_float32_atomic_add: Bool32,
    #[doc(alias = "sparseImageFloat32Atomics")]
    sparse_image_float32_atomics: Bool32,
    #[doc(alias = "sparseImageFloat32AtomicAdd")]
    sparse_image_float32_atomic_add: Bool32,
}
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_shader_atomic_float");
