use crate::native::vulkan1_0::{BaseInStructure, CommandBuffer, Device, StructureType, VulkanResultCodes};
use std::ffi::c_char;
#[doc(alias = "VkDebugMarkerObjectNameInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerObjectNameInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    #[doc(alias = "pObjectName")]
    pub object_name: *const c_char,
}
impl Default for DebugMarkerObjectNameInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugMarkerObjectNameInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            object_type: unsafe { std::mem::zeroed() },
            object: unsafe { std::mem::zeroed() },
            object_name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDebugMarkerObjectTagInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerObjectTagInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    #[doc(alias = "tagName")]
    pub tag_name: u64,
    #[doc(alias = "tagSize")]
    pub tag_size: usize,
    #[doc(alias = "pTag")]
    pub tag: *const std::ffi::c_void,
}
impl Default for DebugMarkerObjectTagInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugMarkerObjectTagInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            object_type: unsafe { std::mem::zeroed() },
            object: unsafe { std::mem::zeroed() },
            tag_name: unsafe { std::mem::zeroed() },
            tag_size: unsafe { std::mem::zeroed() },
            tag: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDebugMarkerMarkerInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugMarkerMarkerInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pMarkerName")]
    pub marker_name: *const c_char,
    pub color: [f32; 4 as usize],
}
impl Default for DebugMarkerMarkerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugMarkerMarkerInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            marker_name: unsafe { std::mem::zeroed() },
            color: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_debug_marker::{
    DebugReportObjectTypeEXT, EXT_DEBUG_MARKER_EXTENSION_NAME, EXT_DEBUG_MARKER_SPEC_VERSION,
};
#[doc(alias = "vkDebugMarkerSetObjectNameEXT")]
pub type FNDebugMarkerSetObjectNameExt =
    unsafe extern "system" fn(device: Device, p_name_info: *const DebugMarkerObjectNameInfoEXT) -> VulkanResultCodes;
#[doc(alias = "vkDebugMarkerSetObjectTagEXT")]
pub type FNDebugMarkerSetObjectTagExt =
    unsafe extern "system" fn(device: Device, p_tag_info: *const DebugMarkerObjectTagInfoEXT) -> VulkanResultCodes;
#[doc(alias = "vkCmdDebugMarkerBeginEXT")]
pub type FNCmdDebugMarkerBeginExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_marker_info: *const DebugMarkerMarkerInfoEXT);
#[doc(alias = "vkCmdDebugMarkerEndEXT")]
pub type FNCmdDebugMarkerEndExt = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc(alias = "vkCmdDebugMarkerInsertEXT")]
pub type FNCmdDebugMarkerInsertExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_marker_info: *const DebugMarkerMarkerInfoEXT);
