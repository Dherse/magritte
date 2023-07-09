use crate::native::{
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
    vulkan1_3::PhysicalDeviceTexelBufferAlignmentProperties,
};
///See [`PhysicalDeviceTexelBufferAlignmentProperties`]
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT")]
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT = PhysicalDeviceTexelBufferAlignmentProperties;
#[doc(alias = "VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "texelBufferAlignment")]
    pub texel_buffer_alignment: Bool32,
}
impl Default for PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceTexelBufferAlignmentFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            texel_buffer_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_texel_buffer_alignment::{
    EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME, EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION,
};
