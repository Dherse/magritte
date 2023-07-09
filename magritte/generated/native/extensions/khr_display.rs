pub use crate::common::extensions::khr_display::{DisplayModeParametersKHR, DisplayPlaneCapabilitiesKHR};
use crate::native::{
    extensions::khr_surface::{SurfaceKHR, SurfaceTransformFlagBitsKHR},
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Extent2D, Instance, PhysicalDevice, StructureType,
        VulkanResultCodes,
    },
};
use std::ffi::c_char;
#[doc(alias = "VkDisplayPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    #[doc(alias = "displayName")]
    pub display_name: *const c_char,
    #[doc(alias = "physicalDimensions")]
    pub physical_dimensions: Extent2D,
    #[doc(alias = "physicalResolution")]
    pub physical_resolution: Extent2D,
    #[doc(alias = "supportedTransforms")]
    pub supported_transforms: SurfaceTransformFlagsKHR,
    #[doc(alias = "planeReorderPossible")]
    pub plane_reorder_possible: Bool32,
    #[doc(alias = "persistentContent")]
    pub persistent_content: Bool32,
}
impl Default for DisplayPropertiesKHR {
    fn default() -> Self {
        Self {
            display: unsafe { std::mem::zeroed() },
            display_name: unsafe { std::mem::zeroed() },
            physical_dimensions: unsafe { std::mem::zeroed() },
            physical_resolution: unsafe { std::mem::zeroed() },
            supported_transforms: unsafe { std::mem::zeroed() },
            plane_reorder_possible: unsafe { std::mem::zeroed() },
            persistent_content: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayPlanePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPlanePropertiesKHR {
    #[doc(alias = "currentDisplay")]
    pub current_display: DisplayKHR,
    #[doc(alias = "currentStackIndex")]
    pub current_stack_index: u32,
}
impl Default for DisplayPlanePropertiesKHR {
    fn default() -> Self {
        Self {
            current_display: unsafe { std::mem::zeroed() },
            current_stack_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayModePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayModePropertiesKHR {
    #[doc(alias = "displayMode")]
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}
impl Default for DisplayModePropertiesKHR {
    fn default() -> Self {
        Self {
            display_mode: unsafe { std::mem::zeroed() },
            parameters: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayModeCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayModeCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}
impl Default for DisplayModeCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayModeCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            parameters: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplaySurfaceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplaySurfaceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    #[doc(alias = "displayMode")]
    pub display_mode: DisplayModeKHR,
    #[doc(alias = "planeIndex")]
    pub plane_index: u32,
    #[doc(alias = "planeStackIndex")]
    pub plane_stack_index: u32,
    pub transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "globalAlpha")]
    pub global_alpha: f32,
    #[doc(alias = "alphaMode")]
    pub alpha_mode: DisplayPlaneAlphaFlagBitsKHR,
    #[doc(alias = "imageExtent")]
    pub image_extent: Extent2D,
}
impl Default for DisplaySurfaceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplaySurfaceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            display_mode: unsafe { std::mem::zeroed() },
            plane_index: unsafe { std::mem::zeroed() },
            plane_stack_index: unsafe { std::mem::zeroed() },
            transform: unsafe { std::mem::zeroed() },
            global_alpha: unsafe { std::mem::zeroed() },
            alpha_mode: unsafe { std::mem::zeroed() },
            image_extent: unsafe { std::mem::zeroed() },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDisplayKHR")]
#[repr(transparent)]
pub struct DisplayKHR(u64);
impl DisplayKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DisplayKHR {
    fn default() -> Self {
        Self::null()
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkDisplayModeKHR")]
#[repr(transparent)]
pub struct DisplayModeKHR(u64);
impl DisplayModeKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for DisplayModeKHR {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::khr_display::{
    DisplayModeCreateFlagsKHR, DisplayPlaneAlphaFlagBitsKHR, DisplayPlaneAlphaFlagsKHR, DisplaySurfaceCreateFlagsKHR,
    SurfaceTransformFlagsKHR, KHR_DISPLAY_EXTENSION_NAME, KHR_DISPLAY_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceDisplayPropertiesKHR")]
pub type FNGetPhysicalDeviceDisplayPropertiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPropertiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceDisplayPlanePropertiesKHR")]
pub type FNGetPhysicalDeviceDisplayPlanePropertiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlanePropertiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDisplayPlaneSupportedDisplaysKHR")]
pub type FNGetDisplayPlaneSupportedDisplaysKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    plane_index: u32,
    p_display_count: *mut u32,
    p_displays: *mut DisplayKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDisplayModePropertiesKHR")]
pub type FNGetDisplayModePropertiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModePropertiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateDisplayModeKHR")]
pub type FNCreateDisplayModeKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_create_info: *const DisplayModeCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_mode: *mut DisplayModeKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDisplayPlaneCapabilitiesKHR")]
pub type FNGetDisplayPlaneCapabilitiesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkCreateDisplayPlaneSurfaceKHR")]
pub type FNCreateDisplayPlaneSurfaceKhr = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DisplaySurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> VulkanResultCodes;
