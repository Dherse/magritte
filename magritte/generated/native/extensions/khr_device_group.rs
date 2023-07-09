#[cfg(feature = "VK_KHR_surface")]
use crate::native::extensions::khr_surface::SurfaceKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::native::extensions::khr_swapchain::SwapchainKHR;
use crate::native::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Device, Fence, PhysicalDevice, Rect2D, Semaphore, StructureType,
        VulkanResultCodes, MAX_DEVICE_GROUP_SIZE,
    },
    vulkan1_1::{
        BindBufferMemoryDeviceGroupInfo, BindImageMemoryDeviceGroupInfo, DeviceGroupBindSparseInfo,
        DeviceGroupCommandBufferBeginInfo, DeviceGroupRenderPassBeginInfo, DeviceGroupSubmitInfo, FNCmdDispatchBase,
        FNCmdSetDeviceMask, FNGetDeviceGroupPeerMemoryFeatures, MemoryAllocateFlagBits, MemoryAllocateFlags,
        MemoryAllocateFlagsInfo, PeerMemoryFeatureFlagBits, PeerMemoryFeatureFlags,
    },
};
///See [`PeerMemoryFeatureFlags`]
#[doc(alias = "VkPeerMemoryFeatureFlagsKHR")]
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
///See [`MemoryAllocateFlags`]
#[doc(alias = "VkMemoryAllocateFlagsKHR")]
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
///See [`PeerMemoryFeatureFlagBits`]
#[doc(alias = "VkPeerMemoryFeatureFlagBitsKHR")]
pub type PeerMemoryFeatureFlagBitsKHR = PeerMemoryFeatureFlagBits;
///See [`MemoryAllocateFlagBits`]
#[doc(alias = "VkMemoryAllocateFlagBitsKHR")]
pub type MemoryAllocateFlagBitsKHR = MemoryAllocateFlagBits;
///See [`MemoryAllocateFlagsInfo`]
#[doc(alias = "VkMemoryAllocateFlagsInfoKHR")]
pub type MemoryAllocateFlagsInfoKHR = MemoryAllocateFlagsInfo;
///See [`BindBufferMemoryDeviceGroupInfo`]
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfoKHR")]
pub type BindBufferMemoryDeviceGroupInfoKHR = BindBufferMemoryDeviceGroupInfo;
///See [`BindImageMemoryDeviceGroupInfo`]
#[doc(alias = "VkBindImageMemoryDeviceGroupInfoKHR")]
pub type BindImageMemoryDeviceGroupInfoKHR = BindImageMemoryDeviceGroupInfo;
///See [`DeviceGroupRenderPassBeginInfo`]
#[doc(alias = "VkDeviceGroupRenderPassBeginInfoKHR")]
pub type DeviceGroupRenderPassBeginInfoKHR = DeviceGroupRenderPassBeginInfo;
///See [`DeviceGroupCommandBufferBeginInfo`]
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfoKHR")]
pub type DeviceGroupCommandBufferBeginInfoKHR = DeviceGroupCommandBufferBeginInfo;
///See [`DeviceGroupSubmitInfo`]
#[doc(alias = "VkDeviceGroupSubmitInfoKHR")]
pub type DeviceGroupSubmitInfoKHR = DeviceGroupSubmitInfo;
///See [`DeviceGroupBindSparseInfo`]
#[doc(alias = "VkDeviceGroupBindSparseInfoKHR")]
pub type DeviceGroupBindSparseInfoKHR = DeviceGroupBindSparseInfo;
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "presentMask")]
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl Default for DeviceGroupPresentCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupPresentCapabilitiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            present_mask: unsafe { std::mem::zeroed() },
            modes: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub swapchain: SwapchainKHR,
}
impl Default for ImageSwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImageSwapchainCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            swapchain: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub swapchain: SwapchainKHR,
    #[doc(alias = "imageIndex")]
    pub image_index: u32,
}
impl Default for BindImageMemorySwapchainInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::BindImageMemorySwapchainInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            swapchain: unsafe { std::mem::zeroed() },
            image_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAcquireNextImageInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AcquireNextImageInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub swapchain: SwapchainKHR,
    pub timeout: u64,
    pub semaphore: Semaphore,
    pub fence: Fence,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl Default for AcquireNextImageInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AcquireNextImageInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            swapchain: unsafe { std::mem::zeroed() },
            timeout: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            fence: unsafe { std::mem::zeroed() },
            device_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    pub swapchain_count: u32,
    #[doc(alias = "pDeviceMasks")]
    pub device_masks: *const u32,
    pub mode: DeviceGroupPresentModeFlagBitsKHR,
}
impl Default for DeviceGroupPresentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupPresentInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            swapchain_count: unsafe { std::mem::zeroed() },
            device_masks: unsafe { std::mem::zeroed() },
            mode: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl Default for DeviceGroupSwapchainCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceGroupSwapchainCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            modes: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_device_group::{
    DeviceGroupPresentModeFlagBitsKHR, DeviceGroupPresentModeFlagsKHR, KHR_DEVICE_GROUP_EXTENSION_NAME,
    KHR_DEVICE_GROUP_SPEC_VERSION,
};
///See [`get_device_group_peer_memory_features`]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
pub type FNGetDeviceGroupPeerMemoryFeaturesKhr = FNGetDeviceGroupPeerMemoryFeatures;
///See [`cmd_set_device_mask`]
#[doc(alias = "vkCmdSetDeviceMaskKHR")]
pub type FNCmdSetDeviceMaskKhr = FNCmdSetDeviceMask;
///See [`cmd_dispatch_base`]
#[doc(alias = "vkCmdDispatchBaseKHR")]
pub type FNCmdDispatchBaseKhr = FNCmdDispatchBase;
#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
pub type FNGetDeviceGroupPresentCapabilitiesKhr = unsafe extern "system" fn(
    device: Device,
    p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
pub type FNGetDeviceGroupSurfacePresentModesKhr = unsafe extern "system" fn(
    device: Device,
    surface: SurfaceKHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkAcquireNextImage2KHR")]
pub type FNAcquireNextImage2Khr = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const AcquireNextImageInfoKHR,
    p_image_index: *mut u32,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
pub type FNGetPhysicalDevicePresentRectanglesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_rect_count: *mut u32,
    p_rects: *mut Rect2D,
) -> VulkanResultCodes;
