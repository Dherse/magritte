#[cfg(feature = "VK_KHR_device_group")]
use crate::native::extensions::khr_device_group::DeviceGroupPresentModeFlagsKHR;
use crate::native::{
    extensions::{
        khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR, khr_surface::PresentModeKHR,
        khr_swapchain::SwapchainKHR,
    },
    opaque::HMONITOR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, PhysicalDevice, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkSurfaceFullScreenExclusiveInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "fullScreenExclusive")]
    pub full_screen_exclusive: FullScreenExclusiveEXT,
}
impl Default for SurfaceFullScreenExclusiveInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SurfaceFullScreenExclusiveInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            full_screen_exclusive: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSurfaceFullScreenExclusiveWin32InfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub hmonitor: HMONITOR,
}
impl Default for SurfaceFullScreenExclusiveWin32InfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SurfaceFullScreenExclusiveWin32InfoExt,
            p_next: unsafe { std::mem::zeroed() },
            hmonitor: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSurfaceCapabilitiesFullScreenExclusiveEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "fullScreenExclusiveSupported")]
    pub full_screen_exclusive_supported: Bool32,
}
impl Default for SurfaceCapabilitiesFullScreenExclusiveEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SurfaceCapabilitiesFullScreenExclusiveExt,
            p_next: unsafe { std::mem::zeroed() },
            full_screen_exclusive_supported: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_full_screen_exclusive::{
    FullScreenExclusiveEXT, EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME, EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
pub type FNGetPhysicalDeviceSurfacePresentModes2Ext = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
pub type FNGetDeviceGroupSurfacePresentModes2Ext = unsafe extern "system" fn(
    device: Device,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
pub type FNAcquireFullScreenExclusiveModeExt =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes;
#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
pub type FNReleaseFullScreenExclusiveModeExt =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes;
