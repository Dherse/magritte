use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceDepthClipEnableFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "depthClipEnable")]
    pub depth_clip_enable: Bool32,
}
impl Default for PhysicalDeviceDepthClipEnableFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDepthClipEnableFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            depth_clip_enable: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    #[doc(alias = "depthClipEnable")]
    pub depth_clip_enable: Bool32,
}
impl Default for PipelineRasterizationDepthClipStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRasterizationDepthClipStateCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            depth_clip_enable: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_depth_clip_enable::{
    PipelineRasterizationDepthClipStateCreateFlagsEXT, EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME,
    EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION,
};
