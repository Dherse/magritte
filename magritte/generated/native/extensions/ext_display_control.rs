use crate::native::{
    extensions::{
        ext_display_surface_counter::{SurfaceCounterFlagBitsEXT, SurfaceCounterFlagsEXT},
        khr_display::DisplayKHR,
        khr_swapchain::SwapchainKHR,
    },
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkDisplayPowerInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPowerInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "powerState")]
    pub power_state: DisplayPowerStateEXT,
}
impl Default for DisplayPowerInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayPowerInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            power_state: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceEventInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceEventInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "deviceEvent")]
    pub device_event: DeviceEventTypeEXT,
}
impl Default for DeviceEventInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceEventInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            device_event: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDisplayEventInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayEventInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "displayEvent")]
    pub display_event: DisplayEventTypeEXT,
}
impl Default for DisplayEventInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::DisplayEventInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            display_event: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSwapchainCounterCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SwapchainCounterCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "surfaceCounters")]
    pub surface_counters: SurfaceCounterFlagsEXT,
}
impl Default for SwapchainCounterCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::SwapchainCounterCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            surface_counters: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_display_control::{
    DeviceEventTypeEXT, DisplayEventTypeEXT, DisplayPowerStateEXT, EXT_DISPLAY_CONTROL_EXTENSION_NAME,
    EXT_DISPLAY_CONTROL_SPEC_VERSION,
};
#[doc(alias = "vkDisplayPowerControlEXT")]
pub type FNDisplayPowerControlExt = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_power_info: *const DisplayPowerInfoEXT,
) -> VulkanResultCodes;
#[doc(alias = "vkRegisterDeviceEventEXT")]
pub type FNRegisterDeviceEventExt = unsafe extern "system" fn(
    device: Device,
    p_device_event_info: *const DeviceEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> VulkanResultCodes;
#[doc(alias = "vkRegisterDisplayEventEXT")]
pub type FNRegisterDisplayEventExt = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_event_info: *const DisplayEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> VulkanResultCodes;
#[doc(alias = "vkGetSwapchainCounterEXT")]
pub type FNGetSwapchainCounterExt = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagBitsEXT,
    p_counter_value: *mut u64,
) -> VulkanResultCodes;
