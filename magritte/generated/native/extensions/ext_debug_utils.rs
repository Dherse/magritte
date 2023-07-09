use crate::native::vulkan1_0::{
    AllocationCallbacks, BaseInStructure, Bool32, CommandBuffer, Device, Instance, ObjectType, Queue, StructureType,
    VulkanResultCodes,
};
use std::ffi::c_char;
#[doc(alias = "VkDebugUtilsObjectNameInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsObjectNameInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    pub object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    pub object_handle: u64,
    #[doc(alias = "pObjectName")]
    pub object_name: *const c_char,
}
impl Default for DebugUtilsObjectNameInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugUtilsObjectNameInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            object_type: unsafe { std::mem::zeroed() },
            object_handle: unsafe { std::mem::zeroed() },
            object_name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDebugUtilsObjectTagInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsObjectTagInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "objectType")]
    pub object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    pub object_handle: u64,
    #[doc(alias = "tagName")]
    pub tag_name: u64,
    #[doc(alias = "tagSize")]
    pub tag_size: usize,
    #[doc(alias = "pTag")]
    pub tag: *const std::ffi::c_void,
}
impl Default for DebugUtilsObjectTagInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugUtilsObjectTagInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            object_type: unsafe { std::mem::zeroed() },
            object_handle: unsafe { std::mem::zeroed() },
            tag_name: unsafe { std::mem::zeroed() },
            tag_size: unsafe { std::mem::zeroed() },
            tag: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDebugUtilsLabelEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsLabelEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pLabelName")]
    pub label_name: *const c_char,
    pub color: [f32; 4 as usize],
}
impl Default for DebugUtilsLabelEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugUtilsLabelExt,
            p_next: unsafe { std::mem::zeroed() },
            label_name: unsafe { std::mem::zeroed() },
            color: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDebugUtilsMessengerCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    #[doc(alias = "messageSeverity")]
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    #[doc(alias = "messageType")]
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    #[doc(alias = "pfnUserCallback")]
    pub pfn_user_callback: PFNDebugUtilsMessengerCallbackEXT,
    #[doc(alias = "pUserData")]
    pub user_data: *mut std::ffi::c_void,
}
impl Default for DebugUtilsMessengerCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugUtilsMessengerCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            message_severity: unsafe { std::mem::zeroed() },
            message_type: unsafe { std::mem::zeroed() },
            pfn_user_callback: unsafe { std::mem::zeroed() },
            user_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDebugUtilsMessengerCallbackDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    #[doc(alias = "pMessageIdName")]
    pub message_id_name: *const c_char,
    #[doc(alias = "messageIdNumber")]
    pub message_id_number: i32,
    #[doc(alias = "pMessage")]
    pub message: *const c_char,
    #[doc(alias = "queueLabelCount")]
    pub queue_label_count: u32,
    #[doc(alias = "pQueueLabels")]
    pub queue_labels: *const DebugUtilsLabelEXT,
    #[doc(alias = "cmdBufLabelCount")]
    pub cmd_buf_label_count: u32,
    #[doc(alias = "pCmdBufLabels")]
    pub cmd_buf_labels: *const DebugUtilsLabelEXT,
    #[doc(alias = "objectCount")]
    pub object_count: u32,
    #[doc(alias = "pObjects")]
    pub objects: *const DebugUtilsObjectNameInfoEXT,
}
impl Default for DebugUtilsMessengerCallbackDataEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DebugUtilsMessengerCallbackDataExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            message_id_name: unsafe { std::mem::zeroed() },
            message_id_number: unsafe { std::mem::zeroed() },
            message: unsafe { std::mem::zeroed() },
            queue_label_count: unsafe { std::mem::zeroed() },
            queue_labels: unsafe { std::mem::zeroed() },
            cmd_buf_label_count: unsafe { std::mem::zeroed() },
            cmd_buf_labels: unsafe { std::mem::zeroed() },
            object_count: unsafe { std::mem::zeroed() },
            objects: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[repr(transparent)]
pub struct DebugUtilsMessengerEXT(u64);
impl DebugUtilsMessengerEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DebugUtilsMessengerEXT {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "PFN_vkDebugUtilsMessengerCallbackEXT")]
pub type PFNDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut std::ffi::c_void,
) -> Bool32;
pub use crate::common::extensions::ext_debug_utils::{
    DebugUtilsMessageSeverityFlagBitsEXT, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagBitsEXT,
    DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessengerCallbackDataFlagsEXT, DebugUtilsMessengerCreateFlagsEXT,
    EXT_DEBUG_UTILS_EXTENSION_NAME, EXT_DEBUG_UTILS_SPEC_VERSION,
};
#[doc(alias = "vkSetDebugUtilsObjectNameEXT")]
pub type FNSetDebugUtilsObjectNameExt =
    unsafe extern "system" fn(device: Device, p_name_info: *const DebugUtilsObjectNameInfoEXT) -> VulkanResultCodes;
#[doc(alias = "vkSetDebugUtilsObjectTagEXT")]
pub type FNSetDebugUtilsObjectTagExt =
    unsafe extern "system" fn(device: Device, p_tag_info: *const DebugUtilsObjectTagInfoEXT) -> VulkanResultCodes;
#[doc(alias = "vkQueueBeginDebugUtilsLabelEXT")]
pub type FNQueueBeginDebugUtilsLabelExt =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT);
#[doc(alias = "vkQueueEndDebugUtilsLabelEXT")]
pub type FNQueueEndDebugUtilsLabelExt = unsafe extern "system" fn(queue: Queue);
#[doc(alias = "vkQueueInsertDebugUtilsLabelEXT")]
pub type FNQueueInsertDebugUtilsLabelExt =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT);
#[doc(alias = "vkCreateDebugUtilsMessengerEXT")]
pub type FNCreateDebugUtilsMessengerExt = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_messenger: *mut DebugUtilsMessengerEXT,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroyDebugUtilsMessengerEXT")]
pub type FNDestroyDebugUtilsMessengerExt = unsafe extern "system" fn(
    instance: Instance,
    messenger: DebugUtilsMessengerEXT,
    p_allocator: *const AllocationCallbacks,
);
#[doc(alias = "vkSubmitDebugUtilsMessageEXT")]
pub type FNSubmitDebugUtilsMessageExt = unsafe extern "system" fn(
    instance: Instance,
    message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
);
#[doc(alias = "vkCmdBeginDebugUtilsLabelEXT")]
pub type FNCmdBeginDebugUtilsLabelExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT);
#[doc(alias = "vkCmdEndDebugUtilsLabelEXT")]
pub type FNCmdEndDebugUtilsLabelExt = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc(alias = "vkCmdInsertDebugUtilsLabelEXT")]
pub type FNCmdInsertDebugUtilsLabelExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT);
