pub use crate::common::extensions::khr_device_group::{
    DeviceGroupPresentModeFlagBitsKHR, DeviceGroupPresentModeFlagsKHR, KHR_DEVICE_GROUP_EXTENSION_NAME,
    KHR_DEVICE_GROUP_SPEC_VERSION,
};
#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::SwapchainKHR;
use crate::{
    context::Context,
    vulkan1_0::{Fence, Semaphore, StructureType, MAX_DEVICE_GROUP_SIZE},
    vulkan1_1::{
        BindBufferMemoryDeviceGroupInfo, BindImageMemoryDeviceGroupInfo, DeviceGroupBindSparseInfo,
        DeviceGroupCommandBufferBeginInfo, DeviceGroupRenderPassBeginInfo, DeviceGroupSubmitInfo,
        MemoryAllocateFlagBits, MemoryAllocateFlags, MemoryAllocateFlagsInfo, PeerMemoryFeatureFlagBits,
        PeerMemoryFeatureFlags,
    },
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkPeerMemoryFeatureFlagsKHR")]
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
#[doc(alias = "VkMemoryAllocateFlagsKHR")]
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
#[doc(alias = "VkPeerMemoryFeatureFlagBitsKHR")]
pub type PeerMemoryFeatureFlagBitsKHR = PeerMemoryFeatureFlagBits;
#[doc(alias = "VkMemoryAllocateFlagBitsKHR")]
pub type MemoryAllocateFlagBitsKHR = MemoryAllocateFlagBits;
#[doc(alias = "VkMemoryAllocateFlagsInfoKHR")]
pub type MemoryAllocateFlagsInfoKHR = MemoryAllocateFlagsInfo;
#[doc(alias = "VkBindBufferMemoryDeviceGroupInfoKHR")]
pub type BindBufferMemoryDeviceGroupInfoKHR = BindBufferMemoryDeviceGroupInfo;
#[doc(alias = "VkBindImageMemoryDeviceGroupInfoKHR")]
pub type BindImageMemoryDeviceGroupInfoKHR = BindImageMemoryDeviceGroupInfo;
#[doc(alias = "VkDeviceGroupRenderPassBeginInfoKHR")]
pub type DeviceGroupRenderPassBeginInfoKHR = DeviceGroupRenderPassBeginInfo;
#[doc(alias = "VkDeviceGroupCommandBufferBeginInfoKHR")]
pub type DeviceGroupCommandBufferBeginInfoKHR = DeviceGroupCommandBufferBeginInfo;
#[doc(alias = "VkDeviceGroupSubmitInfoKHR")]
pub type DeviceGroupSubmitInfoKHR = DeviceGroupSubmitInfo;
#[doc(alias = "VkDeviceGroupBindSparseInfoKHR")]
pub type DeviceGroupBindSparseInfoKHR = DeviceGroupBindSparseInfo;
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupPresentCapabilitiesKHR {
    #[doc(alias = "presentMask")]
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl DeviceGroupPresentCapabilitiesKHR {
    ///Get a reference to the `present_mask` field.
    pub fn present_mask(&self) -> [u32; MAX_DEVICE_GROUP_SIZE as usize] {
        self.present_mask
    }
    ///Get a reference to the `modes` field.
    pub fn modes(&self) -> DeviceGroupPresentModeFlagsKHR {
        self.modes
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupPresentCapabilitiesKHR {
    type LowLevel = crate::native::extensions::khr_device_group::DeviceGroupPresentCapabilitiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_device_group::DeviceGroupPresentCapabilitiesKHR {
            s_type: StructureType::DeviceGroupPresentCapabilitiesKhr,
            p_next: std::ptr::null_mut(),
            present_mask: self.present_mask.into_low_level(context, bump),
            modes: self.modes.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupPresentCapabilitiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            present_mask: crate::conv::FromLowLevel::from_low_level(context, value.present_mask),
            modes: crate::conv::FromLowLevel::from_low_level(context, value.modes),
        }
    }
}
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageSwapchainCreateInfoKHR {
    pub swapchain: Option<SwapchainKHR>,
}
impl ImageSwapchainCreateInfoKHR {
    ///Get a reference to the `swapchain` field.
    pub fn swapchain(&self) -> &Option<SwapchainKHR> {
        &self.swapchain
    }
    ///Get a mutable reference to the `swapchain` field.
    pub fn swapchain_mut(&mut self) -> &mut Option<SwapchainKHR> {
        &mut self.swapchain
    }
    ///Sets the `swapchain` field.
    pub fn set_swapchain(&mut self, swapchain: Option<SwapchainKHR>) -> &mut Self {
        self.swapchain = swapchain;
        self
    }
    ///Sets the `swapchain` field in a builder way.
    pub fn with_swapchain(mut self, swapchain: Option<SwapchainKHR>) -> Self {
        self.swapchain = swapchain;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ImageSwapchainCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_device_group::ImageSwapchainCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_device_group::ImageSwapchainCreateInfoKHR {
            s_type: StructureType::ImageSwapchainCreateInfoKhr,
            p_next: std::ptr::null(),
            swapchain: self
                .swapchain
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ImageSwapchainCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            swapchain: if value.swapchain == crate::native::extensions::khr_swapchain::SwapchainKHR::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.swapchain))
            },
        }
    }
}
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindImageMemorySwapchainInfoKHR {
    pub swapchain: SwapchainKHR,
    #[doc(alias = "imageIndex")]
    pub image_index: u32,
}
impl BindImageMemorySwapchainInfoKHR {
    ///Get a reference to the `swapchain` field.
    pub fn swapchain(&self) -> &SwapchainKHR {
        &self.swapchain
    }
    ///Get a reference to the `image_index` field.
    pub fn image_index(&self) -> u32 {
        self.image_index
    }
    ///Get a mutable reference to the `swapchain` field.
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Get a mutable reference to the `image_index` field.
    pub fn image_index_mut(&mut self) -> &mut u32 {
        &mut self.image_index
    }
    ///Sets the `swapchain` field.
    pub fn set_swapchain(&mut self, swapchain: SwapchainKHR) -> &mut Self {
        self.swapchain = swapchain;
        self
    }
    ///Sets the `image_index` field.
    pub fn set_image_index(&mut self, image_index: u32) -> &mut Self {
        self.image_index = image_index;
        self
    }
    ///Sets the `swapchain` field in a builder way.
    pub fn with_swapchain(mut self, swapchain: SwapchainKHR) -> Self {
        self.swapchain = swapchain;
        self
    }
    ///Sets the `image_index` field in a builder way.
    pub fn with_image_index(mut self, image_index: u32) -> Self {
        self.image_index = image_index;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BindImageMemorySwapchainInfoKHR {
    type LowLevel = crate::native::extensions::khr_device_group::BindImageMemorySwapchainInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_device_group::BindImageMemorySwapchainInfoKHR {
            s_type: StructureType::BindImageMemorySwapchainInfoKhr,
            p_next: std::ptr::null(),
            swapchain: self.swapchain.into_low_level(context, bump),
            image_index: self.image_index.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BindImageMemorySwapchainInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            swapchain: crate::conv::FromLowLevel::from_low_level(context, value.swapchain),
            image_index: crate::conv::FromLowLevel::from_low_level(context, value.image_index),
        }
    }
}
#[doc(alias = "VkAcquireNextImageInfoKHR")]
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcquireNextImageInfoKHR {
    pub swapchain: SwapchainKHR,
    pub timeout: u64,
    pub semaphore: Option<Semaphore>,
    pub fence: Option<Fence>,
    #[doc(alias = "deviceMask")]
    pub device_mask: u32,
}
impl AcquireNextImageInfoKHR {
    ///Get a reference to the `swapchain` field.
    pub fn swapchain(&self) -> &SwapchainKHR {
        &self.swapchain
    }
    ///Get a reference to the `timeout` field.
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
    ///Get a reference to the `semaphore` field.
    pub fn semaphore(&self) -> &Option<Semaphore> {
        &self.semaphore
    }
    ///Get a reference to the `fence` field.
    pub fn fence(&self) -> &Option<Fence> {
        &self.fence
    }
    ///Get a reference to the `device_mask` field.
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Get a mutable reference to the `swapchain` field.
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Get a mutable reference to the `timeout` field.
    pub fn timeout_mut(&mut self) -> &mut u64 {
        &mut self.timeout
    }
    ///Get a mutable reference to the `semaphore` field.
    pub fn semaphore_mut(&mut self) -> &mut Option<Semaphore> {
        &mut self.semaphore
    }
    ///Get a mutable reference to the `fence` field.
    pub fn fence_mut(&mut self) -> &mut Option<Fence> {
        &mut self.fence
    }
    ///Get a mutable reference to the `device_mask` field.
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Sets the `swapchain` field.
    pub fn set_swapchain(&mut self, swapchain: SwapchainKHR) -> &mut Self {
        self.swapchain = swapchain;
        self
    }
    ///Sets the `timeout` field.
    pub fn set_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }
    ///Sets the `semaphore` field.
    pub fn set_semaphore(&mut self, semaphore: Option<Semaphore>) -> &mut Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `fence` field.
    pub fn set_fence(&mut self, fence: Option<Fence>) -> &mut Self {
        self.fence = fence;
        self
    }
    ///Sets the `device_mask` field.
    pub fn set_device_mask(&mut self, device_mask: u32) -> &mut Self {
        self.device_mask = device_mask;
        self
    }
    ///Sets the `swapchain` field in a builder way.
    pub fn with_swapchain(mut self, swapchain: SwapchainKHR) -> Self {
        self.swapchain = swapchain;
        self
    }
    ///Sets the `timeout` field in a builder way.
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
    ///Sets the `semaphore` field in a builder way.
    pub fn with_semaphore(mut self, semaphore: Option<Semaphore>) -> Self {
        self.semaphore = semaphore;
        self
    }
    ///Sets the `fence` field in a builder way.
    pub fn with_fence(mut self, fence: Option<Fence>) -> Self {
        self.fence = fence;
        self
    }
    ///Sets the `device_mask` field in a builder way.
    pub fn with_device_mask(mut self, device_mask: u32) -> Self {
        self.device_mask = device_mask;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AcquireNextImageInfoKHR {
    type LowLevel = crate::native::extensions::khr_device_group::AcquireNextImageInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_device_group::AcquireNextImageInfoKHR {
            s_type: StructureType::AcquireNextImageInfoKhr,
            p_next: std::ptr::null(),
            swapchain: self.swapchain.into_low_level(context, bump),
            timeout: self.timeout.into_low_level(context, bump),
            semaphore: self
                .semaphore
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            fence: self
                .fence
                .as_ref()
                .map(|v| v.into_low_level(context, bump))
                .unwrap_or_else(Default::default),
            device_mask: self.device_mask.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AcquireNextImageInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            swapchain: crate::conv::FromLowLevel::from_low_level(context, value.swapchain),
            timeout: crate::conv::FromLowLevel::from_low_level(context, value.timeout),
            semaphore: if value.semaphore == crate::native::vulkan1_0::Semaphore::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.semaphore))
            },
            fence: if value.fence == crate::native::vulkan1_0::Fence::null() {
                None
            } else {
                Some(crate::conv::FromLowLevel::from_low_level(context, value.fence))
            },
            device_mask: crate::conv::FromLowLevel::from_low_level(context, value.device_mask),
        }
    }
}
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupPresentInfoKHR {
    #[doc(alias = "pDeviceMasks")]
    pub device_masks: SmallVec<[u32; 8]>,
    pub mode: DeviceGroupPresentModeFlagBitsKHR,
}
impl DeviceGroupPresentInfoKHR {
    ///Get a reference to the `device_masks` field.
    pub fn device_masks(&self) -> &SmallVec<[u32; 8]> {
        &self.device_masks
    }
    ///Get a reference to the `mode` field.
    pub fn mode(&self) -> DeviceGroupPresentModeFlagBitsKHR {
        self.mode
    }
    ///Get a mutable reference to the `device_masks` field.
    pub fn device_masks_mut(&mut self) -> &mut SmallVec<[u32; 8]> {
        &mut self.device_masks
    }
    ///Get a mutable reference to the `mode` field.
    pub fn mode_mut(&mut self) -> &mut DeviceGroupPresentModeFlagBitsKHR {
        &mut self.mode
    }
    ///Sets the `device_masks` field.
    pub fn set_device_masks(&mut self, device_masks: SmallVec<[u32; 8]>) -> &mut Self {
        self.device_masks = device_masks;
        self
    }
    ///Sets the `mode` field.
    pub fn set_mode(&mut self, mode: DeviceGroupPresentModeFlagBitsKHR) -> &mut Self {
        self.mode = mode;
        self
    }
    ///Sets the `device_masks` field in a builder way.
    pub fn with_device_masks(mut self, device_masks: SmallVec<[u32; 8]>) -> Self {
        self.device_masks = device_masks;
        self
    }
    ///Sets the `mode` field in a builder way.
    pub fn with_mode(mut self, mode: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        self.mode = mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupPresentInfoKHR {
    type LowLevel = crate::native::extensions::khr_device_group::DeviceGroupPresentInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_device_masks = self.device_masks.len() as u32;
        let device_masks = bump
            .alloc_slice_fill_iter(self.device_masks.iter().map(|x| x.into_low_level(context, bump)))
            .as_ptr()
            .cast();
        crate::native::extensions::khr_device_group::DeviceGroupPresentInfoKHR {
            s_type: StructureType::DeviceGroupPresentInfoKhr,
            p_next: std::ptr::null(),
            swapchain_count: len_device_masks,
            device_masks: device_masks,
            mode: self.mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupPresentInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let device_masks_len = value.swapchain_count;
        let mut device_masks = SmallVec::with_capacity(device_masks_len as usize);
        for i in 0..device_masks_len {
            device_masks.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.device_masks.add(i as usize).read(),
            ));
        }
        Self {
            device_masks: device_masks,
            mode: crate::conv::FromLowLevel::from_low_level(context, value.mode),
        }
    }
}
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl DeviceGroupSwapchainCreateInfoKHR {
    ///Get a reference to the `modes` field.
    pub fn modes(&self) -> DeviceGroupPresentModeFlagsKHR {
        self.modes
    }
    ///Get a mutable reference to the `modes` field.
    pub fn modes_mut(&mut self) -> &mut DeviceGroupPresentModeFlagsKHR {
        &mut self.modes
    }
    ///Sets the `modes` field.
    pub fn set_modes(&mut self, modes: DeviceGroupPresentModeFlagsKHR) -> &mut Self {
        self.modes = modes;
        self
    }
    ///Sets the `modes` field in a builder way.
    pub fn with_modes(mut self, modes: DeviceGroupPresentModeFlagsKHR) -> Self {
        self.modes = modes;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for DeviceGroupSwapchainCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_device_group::DeviceGroupSwapchainCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_device_group::DeviceGroupSwapchainCreateInfoKHR {
            s_type: StructureType::DeviceGroupSwapchainCreateInfoKhr,
            p_next: std::ptr::null(),
            modes: self.modes.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for DeviceGroupSwapchainCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            modes: crate::conv::FromLowLevel::from_low_level(context, value.modes),
        }
    }
}
