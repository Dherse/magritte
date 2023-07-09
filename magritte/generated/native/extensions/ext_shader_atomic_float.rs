use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderAtomicFloatFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderBufferFloat32Atomics")]
    pub shader_buffer_float32_atomics: Bool32,
    #[doc(alias = "shaderBufferFloat32AtomicAdd")]
    pub shader_buffer_float32_atomic_add: Bool32,
    #[doc(alias = "shaderBufferFloat64Atomics")]
    pub shader_buffer_float64_atomics: Bool32,
    #[doc(alias = "shaderBufferFloat64AtomicAdd")]
    pub shader_buffer_float64_atomic_add: Bool32,
    #[doc(alias = "shaderSharedFloat32Atomics")]
    pub shader_shared_float32_atomics: Bool32,
    #[doc(alias = "shaderSharedFloat32AtomicAdd")]
    pub shader_shared_float32_atomic_add: Bool32,
    #[doc(alias = "shaderSharedFloat64Atomics")]
    pub shader_shared_float64_atomics: Bool32,
    #[doc(alias = "shaderSharedFloat64AtomicAdd")]
    pub shader_shared_float64_atomic_add: Bool32,
    #[doc(alias = "shaderImageFloat32Atomics")]
    pub shader_image_float32_atomics: Bool32,
    #[doc(alias = "shaderImageFloat32AtomicAdd")]
    pub shader_image_float32_atomic_add: Bool32,
    #[doc(alias = "sparseImageFloat32Atomics")]
    pub sparse_image_float32_atomics: Bool32,
    #[doc(alias = "sparseImageFloat32AtomicAdd")]
    pub sparse_image_float32_atomic_add: Bool32,
}
impl Default for PhysicalDeviceShaderAtomicFloatFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderAtomicFloatFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            shader_buffer_float32_atomics: unsafe { std::mem::zeroed() },
            shader_buffer_float32_atomic_add: unsafe { std::mem::zeroed() },
            shader_buffer_float64_atomics: unsafe { std::mem::zeroed() },
            shader_buffer_float64_atomic_add: unsafe { std::mem::zeroed() },
            shader_shared_float32_atomics: unsafe { std::mem::zeroed() },
            shader_shared_float32_atomic_add: unsafe { std::mem::zeroed() },
            shader_shared_float64_atomics: unsafe { std::mem::zeroed() },
            shader_shared_float64_atomic_add: unsafe { std::mem::zeroed() },
            shader_image_float32_atomics: unsafe { std::mem::zeroed() },
            shader_image_float32_atomic_add: unsafe { std::mem::zeroed() },
            sparse_image_float32_atomics: unsafe { std::mem::zeroed() },
            sparse_image_float32_atomic_add: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_shader_atomic_float::{
    EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME, EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION,
};
