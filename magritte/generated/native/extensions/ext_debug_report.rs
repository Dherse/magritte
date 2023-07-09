#[cfg(feature = "VK_EXT_debug_marker")]
use crate::native::extensions::ext_debug_marker::DebugReportObjectTypeEXT;
use crate::native::vulkan1_0::{
    AllocationCallbacks, BaseInStructure, Bool32, Instance, StructureType, VulkanResultCodes,
};
use std::ffi::c_char;
#[doc(alias = "VkDebugReportCallbackCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugReportCallbackCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DebugReportFlagsEXT,
    #[doc(alias = "pfnCallback")]
    pub pfn_callback: PFNDebugReportCallbackEXT,
    #[doc(alias = "pUserData")]
    pub user_data: *mut std::ffi::c_void,
}
impl Default for DebugReportCallbackCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugReportCallbackCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            pfn_callback: unsafe { std::mem::zeroed() },
            user_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDebugReportCallbackEXT")]
#[repr(transparent)]
pub struct DebugReportCallbackEXT(u64);
impl DebugReportCallbackEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DebugReportCallbackEXT {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "PFN_vkDebugReportCallbackEXT")]
pub type PFNDebugReportCallbackEXT = unsafe extern "system" fn(
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const std::ffi::c_char,
    p_message: *const std::ffi::c_char,
    p_user_data: *mut std::ffi::c_void,
) -> Bool32;
pub use crate::common::extensions::ext_debug_report::{
    DebugReportFlagBitsEXT, DebugReportFlagsEXT, EXT_DEBUG_REPORT_EXTENSION_NAME, EXT_DEBUG_REPORT_SPEC_VERSION,
};
#[doc(alias = "vkCreateDebugReportCallbackEXT")]
pub type FNCreateDebugReportCallbackExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugReportCallbackCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_callback: *mut DebugReportCallbackEXT,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyDebugReportCallbackEXT")]
pub type FNDestroyDebugReportCallbackExt = unsafe extern "system" fn(
    instance: Instance,
    callback: DebugReportCallbackEXT,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkDebugReportMessageEXT")]
pub type FNDebugReportMessageExt = unsafe extern "system" fn(
    instance: Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const c_char,
    p_message: *const c_char,
);
