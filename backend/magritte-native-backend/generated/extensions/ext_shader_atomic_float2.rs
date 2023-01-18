use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderBufferFloat16Atomics")]
    shader_buffer_float16_atomics: Bool32,
    #[doc(alias = "shaderBufferFloat16AtomicAdd")]
    shader_buffer_float16_atomic_add: Bool32,
    #[doc(alias = "shaderBufferFloat16AtomicMinMax")]
    shader_buffer_float16_atomic_min_max: Bool32,
    #[doc(alias = "shaderBufferFloat32AtomicMinMax")]
    shader_buffer_float32_atomic_min_max: Bool32,
    #[doc(alias = "shaderBufferFloat64AtomicMinMax")]
    shader_buffer_float64_atomic_min_max: Bool32,
    #[doc(alias = "shaderSharedFloat16Atomics")]
    shader_shared_float16_atomics: Bool32,
    #[doc(alias = "shaderSharedFloat16AtomicAdd")]
    shader_shared_float16_atomic_add: Bool32,
    #[doc(alias = "shaderSharedFloat16AtomicMinMax")]
    shader_shared_float16_atomic_min_max: Bool32,
    #[doc(alias = "shaderSharedFloat32AtomicMinMax")]
    shader_shared_float32_atomic_min_max: Bool32,
    #[doc(alias = "shaderSharedFloat64AtomicMinMax")]
    shader_shared_float64_atomic_min_max: Bool32,
    #[doc(alias = "shaderImageFloat32AtomicMinMax")]
    shader_image_float32_atomic_min_max: Bool32,
    #[doc(alias = "sparseImageFloat32AtomicMinMax")]
    sparse_image_float32_atomic_min_max: Bool32,
}
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_shader_atomic_float2");
