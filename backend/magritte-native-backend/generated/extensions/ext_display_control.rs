//!# [VK_EXT_display_control](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_display_control.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VK_EXT_display_control.md")]
use crate::{
    cstr,
    extensions::{
        ext_display_surface_counter::{SurfaceCounterFlagBitsEXT, SurfaceCounterFlagsEXT},
        khr_display::DisplayKHR,
        khr_swapchain::SwapchainKHR,
    },
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkDisplayPowerInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VkDisplayPowerInfoEXT.md")]
#[doc(alias = "VkDisplayPowerInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPowerInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "powerState")]
    power_state: DisplayPowerStateEXT,
}
///# [VkDeviceEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceEventInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VkDeviceEventInfoEXT.md")]
#[doc(alias = "VkDeviceEventInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceEventInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "deviceEvent")]
    device_event: DeviceEventTypeEXT,
}
///# [VkDisplayEventInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayEventInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VkDisplayEventInfoEXT.md")]
#[doc(alias = "VkDisplayEventInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayEventInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "displayEvent")]
    display_event: DisplayEventTypeEXT,
}
///# [VkSwapchainCounterCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSwapchainCounterCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VkSwapchainCounterCreateInfoEXT.md")]
#[doc(alias = "VkSwapchainCounterCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "surfaceCounters")]
    surface_counters: SurfaceCounterFlagsEXT,
}
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_SPEC_VERSION")]
pub const EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME")]
pub const EXT_DISPLAY_CONTROL_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_display_control");
///# [VkDisplayPowerStateEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPowerStateEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VkDisplayPowerStateEXT.md")]
#[doc(alias = "VkDisplayPowerStateEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DisplayPowerStateEXT(i32);
impl DisplayPowerStateEXT {
    #[doc(alias = "VK_DISPLAY_POWER_STATE_OFF_EXT")]
    pub const OFF: Self = Self(0);
    #[doc(alias = "VK_DISPLAY_POWER_STATE_SUSPEND_EXT")]
    pub const SUSPEND: Self = Self(1);
    #[doc(alias = "VK_DISPLAY_POWER_STATE_ON_EXT")]
    pub const ON: Self = Self(2);
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
            x if x == Self::OFF.bits() => Some(Self(x)),
            x if x == Self::SUSPEND.bits() => Some(Self(x)),
            x if x == Self::ON.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkDeviceEventTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceEventTypeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VkDeviceEventTypeEXT.md")]
#[doc(alias = "VkDeviceEventTypeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DeviceEventTypeEXT(i32);
impl DeviceEventTypeEXT {
    #[doc(alias = "VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT")]
    pub const DISPLAY_HOTPLUG: Self = Self(0);
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
            x if x == Self::DISPLAY_HOTPLUG.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkDisplayEventTypeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayEventTypeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/VkDisplayEventTypeEXT.md")]
#[doc(alias = "VkDisplayEventTypeEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DisplayEventTypeEXT(i32);
impl DisplayEventTypeEXT {
    #[doc(alias = "VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT")]
    pub const FIRST_PIXEL_OUT: Self = Self(0);
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
            x if x == Self::FIRST_PIXEL_OUT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkDisplayPowerControlEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDisplayPowerControlEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/vkDisplayPowerControlEXT.md")]
#[doc(alias = "vkDisplayPowerControlEXT")]
pub type FNDisplayPowerControlExt = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_power_info: *const DisplayPowerInfoEXT,
) -> VulkanResultCodes;
///# [vkRegisterDeviceEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDeviceEventEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/vkRegisterDeviceEventEXT.md")]
#[doc(alias = "vkRegisterDeviceEventEXT")]
pub type FNRegisterDeviceEventExt = unsafe extern "system" fn(
    device: Device,
    p_device_event_info: *const DeviceEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> VulkanResultCodes;
///# [vkRegisterDisplayEventEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/vkRegisterDisplayEventEXT.md")]
#[doc(alias = "vkRegisterDisplayEventEXT")]
pub type FNRegisterDisplayEventExt = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_event_info: *const DisplayEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> VulkanResultCodes;
///# [vkGetSwapchainCounterEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainCounterEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_display_control/vkGetSwapchainCounterEXT.md")]
#[doc(alias = "vkGetSwapchainCounterEXT")]
pub type FNGetSwapchainCounterExt = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagBitsEXT,
    p_counter_value: *mut u64,
) -> VulkanResultCodes;