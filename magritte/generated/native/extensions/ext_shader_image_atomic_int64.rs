use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderImageInt64Atomics")]
    pub shader_image_int64_atomics: Bool32,
    #[doc(alias = "sparseImageInt64Atomics")]
    pub sparse_image_int64_atomics: Bool32,
}
impl Default for PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderImageAtomicInt64FeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            shader_image_int64_atomics: unsafe { std::mem::zeroed() },
            sparse_image_int64_atomics: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_shader_image_atomic_int64::{
    EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME, EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION,
};
