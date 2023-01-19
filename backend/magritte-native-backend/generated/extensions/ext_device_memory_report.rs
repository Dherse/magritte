//!# [VK_EXT_device_memory_report](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_device_memory_report.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_device_memory_report/VK_EXT_device_memory_report.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, DeviceSize, ObjectType, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDeviceMemoryReportFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_device_memory_report/VkPhysicalDeviceDeviceMemoryReportFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceDeviceMemoryReportFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDeviceMemoryReportFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceMemoryReport")]
    device_memory_report: Bool32,
}
///# [VkDeviceDeviceMemoryReportCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceDeviceMemoryReportCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_device_memory_report/VkDeviceDeviceMemoryReportCreateInfoEXT.md")]
#[doc(alias = "VkDeviceDeviceMemoryReportCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceDeviceMemoryReportCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: DeviceMemoryReportFlagsEXT,
    #[doc(alias = "pfnUserCallback")]
    pfn_user_callback: PFNDeviceMemoryReportCallbackEXT,
    #[doc(alias = "pUserData")]
    user_data: *mut std::ffi::c_void,
}
///# [VkDeviceMemoryReportCallbackDataEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportCallbackDataEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_device_memory_report/VkDeviceMemoryReportCallbackDataEXT.md")]
#[doc(alias = "VkDeviceMemoryReportCallbackDataEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryReportCallbackDataEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    flags: DeviceMemoryReportFlagsEXT,
    #[doc(alias = "type")]
    type_: DeviceMemoryReportEventTypeEXT,
    #[doc(alias = "memoryObjectId")]
    memory_object_id: u64,
    size: DeviceSize,
    #[doc(alias = "objectType")]
    object_type: ObjectType,
    #[doc(alias = "objectHandle")]
    object_handle: u64,
    #[doc(alias = "heapIndex")]
    heap_index: u32,
}
///# [PFN_vkDeviceMemoryReportCallbackEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/PFN_vkDeviceMemoryReportCallbackEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_device_memory_report/PFN_vkDeviceMemoryReportCallbackEXT.md")]
#[doc(alias = "PFN_vkDeviceMemoryReportCallbackEXT")]
pub type PFNDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
    p_callback_data: *const DeviceMemoryReportCallbackDataEXT,
    p_user_data: *mut std::ffi::c_void,
);
#[doc(alias = "VkDeviceMemoryReportFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceMemoryReportFlagsEXT(u32);
impl DeviceMemoryReportFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION")]
pub const EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME")]
pub const EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_device_memory_report");
///# [VkDeviceMemoryReportEventTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryReportEventTypeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_device_memory_report/VkDeviceMemoryReportEventTypeEXT.md")]
#[doc(alias = "VkDeviceMemoryReportEventTypeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DeviceMemoryReportEventTypeEXT(i32);
impl DeviceMemoryReportEventTypeEXT {
    #[doc(alias = "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT")]
    pub const ALLOCATE: Self = Self(0);
    #[doc(alias = "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT")]
    pub const FREE: Self = Self(1);
    #[doc(alias = "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT")]
    pub const IMPORT: Self = Self(2);
    #[doc(alias = "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT")]
    pub const UNIMPORT: Self = Self(3);
    #[doc(alias = "VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT")]
    pub const ALLOCATION_FAILED: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::ALLOCATE.bits() => Some(Self(x)),
            x if x == Self::FREE.bits() => Some(Self(x)),
            x if x == Self::IMPORT.bits() => Some(Self(x)),
            x if x == Self::UNIMPORT.bits() => Some(Self(x)),
            x if x == Self::ALLOCATION_FAILED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
