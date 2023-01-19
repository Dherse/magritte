//!# [VK_EXT_full_screen_exclusive](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/VK_EXT_full_screen_exclusive.md")]
#[cfg(feature = "VK_KHR_device_group")]
use crate::extensions::khr_device_group::DeviceGroupPresentModeFlagsKHR;
use crate::{
    cstr,
    extensions::{
        khr_get_surface_capabilities2::PhysicalDeviceSurfaceInfo2KHR, khr_surface::PresentModeKHR,
        khr_swapchain::SwapchainKHR,
    },
    opaque::HMONITOR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Device, PhysicalDevice, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkSurfaceFullScreenExclusiveInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/VkSurfaceFullScreenExclusiveInfoEXT.md")]
#[doc(alias = "VkSurfaceFullScreenExclusiveInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "fullScreenExclusive")]
    full_screen_exclusive: FullScreenExclusiveEXT,
}
///# [VkSurfaceFullScreenExclusiveWin32InfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceFullScreenExclusiveWin32InfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/VkSurfaceFullScreenExclusiveWin32InfoEXT.md")]
#[doc(alias = "VkSurfaceFullScreenExclusiveWin32InfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    hmonitor: HMONITOR,
}
///# [VkSurfaceCapabilitiesFullScreenExclusiveEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilitiesFullScreenExclusiveEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/VkSurfaceCapabilitiesFullScreenExclusiveEXT.md")]
#[doc(alias = "VkSurfaceCapabilitiesFullScreenExclusiveEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "fullScreenExclusiveSupported")]
    full_screen_exclusive_supported: Bool32,
}
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_full_screen_exclusive");
///# [VkFullScreenExclusiveEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFullScreenExclusiveEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/VkFullScreenExclusiveEXT.md")]
#[doc(alias = "VkFullScreenExclusiveEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FullScreenExclusiveEXT(i32);
impl FullScreenExclusiveEXT {
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT")]
    pub const ALLOWED: Self = Self(1);
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT")]
    pub const DISALLOWED: Self = Self(2);
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT")]
    pub const APPLICATION_CONTROLLED: Self = Self(3);
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
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::ALLOWED.bits() => Some(Self(x)),
            x if x == Self::DISALLOWED.bits() => Some(Self(x)),
            x if x == Self::APPLICATION_CONTROLLED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkGetPhysicalDeviceSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/vkGetPhysicalDeviceSurfacePresentModes2EXT.md")]
#[doc(alias = "vkGetPhysicalDeviceSurfacePresentModes2EXT")]
pub type FNGetPhysicalDeviceSurfacePresentModes2Ext = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> VulkanResultCodes;
///# [vkGetDeviceGroupSurfacePresentModes2EXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/vkGetDeviceGroupSurfacePresentModes2EXT.md")]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModes2EXT")]
pub type FNGetDeviceGroupSurfacePresentModes2Ext = unsafe extern "system" fn(
    device: Device,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> VulkanResultCodes;
///# [vkAcquireFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/vkAcquireFullScreenExclusiveModeEXT.md")]
#[doc(alias = "vkAcquireFullScreenExclusiveModeEXT")]
pub type FNAcquireFullScreenExclusiveModeExt =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes;
///# [vkReleaseFullScreenExclusiveModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_full_screen_exclusive/vkReleaseFullScreenExclusiveModeEXT.md")]
#[doc(alias = "vkReleaseFullScreenExclusiveModeEXT")]
pub type FNReleaseFullScreenExclusiveModeExt =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes;
