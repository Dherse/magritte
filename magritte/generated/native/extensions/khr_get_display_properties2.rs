use crate::native::{
    extensions::khr_display::{
        DisplayKHR, DisplayModeKHR, DisplayModePropertiesKHR, DisplayPlaneCapabilitiesKHR, DisplayPlanePropertiesKHR,
        DisplayPropertiesKHR,
    },
    vulkan1_0::{BaseInStructure, BaseOutStructure, PhysicalDevice, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkDisplayProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayProperties2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "displayProperties")]
    pub display_properties: DisplayPropertiesKHR,
}
impl Default for DisplayProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayProperties2Khr,
            p_next: unsafe { std::mem::zeroed() },
            display_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayPlaneProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneProperties2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "displayPlaneProperties")]
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}
impl Default for DisplayPlaneProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayPlaneProperties2Khr,
            p_next: unsafe { std::mem::zeroed() },
            display_plane_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayModeProperties2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayModeProperties2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "displayModeProperties")]
    pub display_mode_properties: DisplayModePropertiesKHR,
}
impl Default for DisplayModeProperties2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayModeProperties2Khr,
            p_next: unsafe { std::mem::zeroed() },
            display_mode_properties: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayPlaneInfo2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneInfo2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub mode: DisplayModeKHR,
    #[doc(alias = "planeIndex")]
    pub plane_index: u32,
}
impl Default for DisplayPlaneInfo2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayPlaneInfo2Khr,
            p_next: unsafe { std::mem::zeroed() },
            mode: unsafe { std::mem::zeroed() },
            plane_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayPlaneCapabilities2KHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlaneCapabilities2KHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}
impl Default for DisplayPlaneCapabilities2KHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayPlaneCapabilities2Khr,
            p_next: unsafe { std::mem::zeroed() },
            capabilities: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_get_display_properties2::{
    KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME, KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
pub type FNGetPhysicalDeviceDisplayPlaneProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDisplayModeProperties2KHR")]
pub type FNGetDisplayModeProperties2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
pub type FNGetDisplayPlaneCapabilities2Khr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> VulkanResultCodes;
