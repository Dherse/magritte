use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceSize, ObjectType, StructureType};
#[doc(alias = "VkPhysicalDeviceDeviceMemoryReportFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceMemoryReport")]
    pub device_memory_report: Bool32,
}
impl Default for PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDeviceMemoryReportFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            device_memory_report: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceDeviceMemoryReportCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DeviceMemoryReportFlagsEXT,
    #[doc(alias = "pfnUserCallback")]
    pub pfn_user_callback: PFNDeviceMemoryReportCallbackEXT,
    #[doc(alias = "pUserData")]
    pub user_data: *mut std::ffi::c_void,
}
impl Default for DeviceDeviceMemoryReportCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceDeviceMemoryReportCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            pfn_user_callback: unsafe { std::mem::zeroed() },
            user_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceMemoryReportCallbackDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub flags: DeviceMemoryReportFlagsEXT,
    #[doc(alias = "type")]
    pub type_: DeviceMemoryReportEventTypeEXT,
    #[doc(alias = "memoryObjectId")]
    pub memory_object_id: u64,
    pub size: DeviceSize,
    #[doc(alias = "objectType")]
    pub object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    pub object_handle: u64,
    #[doc(alias = "heapIndex")]
    pub heap_index: u32,
}
impl Default for DeviceMemoryReportCallbackDataEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceMemoryReportCallbackDataExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            memory_object_id: unsafe { std::mem::zeroed() },
            size: unsafe { std::mem::zeroed() },
            object_type: unsafe { std::mem::zeroed() },
            object_handle: unsafe { std::mem::zeroed() },
            heap_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "PFN_vkDeviceMemoryReportCallbackEXT")]
pub type PFNDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
    p_callback_data: *const DeviceMemoryReportCallbackDataEXT,
    p_user_data: *mut std::ffi::c_void,
);
pub use crate::common::extensions::ext_device_memory_report::{
    DeviceMemoryReportEventTypeEXT, DeviceMemoryReportFlagsEXT, EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME,
    EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION,
};
