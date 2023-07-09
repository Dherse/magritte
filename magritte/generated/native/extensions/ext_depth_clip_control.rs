use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceDepthClipControlFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "depthClipControl")]
    pub depth_clip_control: Bool32,
}
impl Default for PhysicalDeviceDepthClipControlFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDepthClipControlFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            depth_clip_control: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineViewportDepthClipControlCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "negativeOneToOne")]
    pub negative_one_to_one: Bool32,
}
impl Default for PipelineViewportDepthClipControlCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineViewportDepthClipControlCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            negative_one_to_one: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_depth_clip_control::{
    EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME, EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION,
};
