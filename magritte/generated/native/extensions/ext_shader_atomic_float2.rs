use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderBufferFloat16Atomics")]
    pub shader_buffer_float16_atomics: Bool32,
    #[doc(alias = "shaderBufferFloat16AtomicAdd")]
    pub shader_buffer_float16_atomic_add: Bool32,
    #[doc(alias = "shaderBufferFloat16AtomicMinMax")]
    pub shader_buffer_float16_atomic_min_max: Bool32,
    #[doc(alias = "shaderBufferFloat32AtomicMinMax")]
    pub shader_buffer_float32_atomic_min_max: Bool32,
    #[doc(alias = "shaderBufferFloat64AtomicMinMax")]
    pub shader_buffer_float64_atomic_min_max: Bool32,
    #[doc(alias = "shaderSharedFloat16Atomics")]
    pub shader_shared_float16_atomics: Bool32,
    #[doc(alias = "shaderSharedFloat16AtomicAdd")]
    pub shader_shared_float16_atomic_add: Bool32,
    #[doc(alias = "shaderSharedFloat16AtomicMinMax")]
    pub shader_shared_float16_atomic_min_max: Bool32,
    #[doc(alias = "shaderSharedFloat32AtomicMinMax")]
    pub shader_shared_float32_atomic_min_max: Bool32,
    #[doc(alias = "shaderSharedFloat64AtomicMinMax")]
    pub shader_shared_float64_atomic_min_max: Bool32,
    #[doc(alias = "shaderImageFloat32AtomicMinMax")]
    pub shader_image_float32_atomic_min_max: Bool32,
    #[doc(alias = "sparseImageFloat32AtomicMinMax")]
    pub sparse_image_float32_atomic_min_max: Bool32,
}
impl Default for PhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderAtomicFloat2FeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            shader_buffer_float16_atomics: unsafe { std::mem::zeroed() },
            shader_buffer_float16_atomic_add: unsafe { std::mem::zeroed() },
            shader_buffer_float16_atomic_min_max: unsafe { std::mem::zeroed() },
            shader_buffer_float32_atomic_min_max: unsafe { std::mem::zeroed() },
            shader_buffer_float64_atomic_min_max: unsafe { std::mem::zeroed() },
            shader_shared_float16_atomics: unsafe { std::mem::zeroed() },
            shader_shared_float16_atomic_add: unsafe { std::mem::zeroed() },
            shader_shared_float16_atomic_min_max: unsafe { std::mem::zeroed() },
            shader_shared_float32_atomic_min_max: unsafe { std::mem::zeroed() },
            shader_shared_float64_atomic_min_max: unsafe { std::mem::zeroed() },
            shader_image_float32_atomic_min_max: unsafe { std::mem::zeroed() },
            sparse_image_float32_atomic_min_max: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_shader_atomic_float2::{
    EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME, EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION,
};
