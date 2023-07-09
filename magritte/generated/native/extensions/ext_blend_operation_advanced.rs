use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "advancedBlendCoherentOperations")]
    pub advanced_blend_coherent_operations: Bool32,
}
impl Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceBlendOperationAdvancedFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            advanced_blend_coherent_operations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "advancedBlendMaxColorAttachments")]
    pub advanced_blend_max_color_attachments: u32,
    #[doc(alias = "advancedBlendIndependentBlend")]
    pub advanced_blend_independent_blend: Bool32,
    #[doc(alias = "advancedBlendNonPremultipliedSrcColor")]
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    #[doc(alias = "advancedBlendNonPremultipliedDstColor")]
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    #[doc(alias = "advancedBlendCorrelatedOverlap")]
    pub advanced_blend_correlated_overlap: Bool32,
    #[doc(alias = "advancedBlendAllOperations")]
    pub advanced_blend_all_operations: Bool32,
}
impl Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceBlendOperationAdvancedPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            advanced_blend_max_color_attachments: unsafe { std::mem::zeroed() },
            advanced_blend_independent_blend: unsafe { std::mem::zeroed() },
            advanced_blend_non_premultiplied_src_color: unsafe { std::mem::zeroed() },
            advanced_blend_non_premultiplied_dst_color: unsafe { std::mem::zeroed() },
            advanced_blend_correlated_overlap: unsafe { std::mem::zeroed() },
            advanced_blend_all_operations: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineColorBlendAdvancedStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "srcPremultiplied")]
    pub src_premultiplied: Bool32,
    #[doc(alias = "dstPremultiplied")]
    pub dst_premultiplied: Bool32,
    #[doc(alias = "blendOverlap")]
    pub blend_overlap: BlendOverlapEXT,
}
impl Default for PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineColorBlendAdvancedStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            src_premultiplied: unsafe { std::mem::zeroed() },
            dst_premultiplied: unsafe { std::mem::zeroed() },
            blend_overlap: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_blend_operation_advanced::{
    BlendOverlapEXT, EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME, EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION,
};
