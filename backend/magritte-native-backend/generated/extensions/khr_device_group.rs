//!# [VK_KHR_device_group](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VK_KHR_device_group.md")]
#[cfg(feature = "VK_KHR_surface")]
use crate::extensions::khr_surface::SurfaceKHR;
#[cfg(feature = "VK_KHR_swapchain")]
use crate::extensions::khr_swapchain::SwapchainKHR;
use crate::{
    cstr,
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
use std::ffi::CStr;
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
///# [VkDeviceGroupPresentCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkDeviceGroupPresentCapabilitiesKHR.md")]
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "presentMask")]
    present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    modes: DeviceGroupPresentModeFlagsKHR,
}
///# [VkImageSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageSwapchainCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkImageSwapchainCreateInfoKHR.md")]
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    swapchain: SwapchainKHR,
}
///# [VkBindImageMemorySwapchainInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkBindImageMemorySwapchainInfoKHR.md")]
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    swapchain: SwapchainKHR,
    #[doc(alias = "imageIndex")]
    image_index: u32,
}
///# [VkAcquireNextImageInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireNextImageInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkAcquireNextImageInfoKHR.md")]
#[doc(alias = "VkAcquireNextImageInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AcquireNextImageInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    #[doc(alias = "deviceMask")]
    device_mask: u32,
}
///# [VkDeviceGroupPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkDeviceGroupPresentInfoKHR.md")]
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    swapchain_count: u32,
    #[doc(alias = "pDeviceMasks")]
    device_masks: *const u32,
    mode: DeviceGroupPresentModeFlagBitsKHR,
}
///# [VkDeviceGroupSwapchainCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkDeviceGroupSwapchainCreateInfoKHR.md")]
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    modes: DeviceGroupPresentModeFlagsKHR,
}
///# [VkDeviceGroupPresentModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkDeviceGroupPresentModeFlagBitsKHR.md")]
#[doc(alias = "VkDeviceGroupPresentModeFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceGroupPresentModeFlagsKHR(u32);
impl DeviceGroupPresentModeFlagsKHR {
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL: Self = Self(1);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const REMOTE: Self = Self(2);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const SUM: Self = Self(4);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL_MULTI_DEVICE: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    #[allow(unused_mut)]
    pub const fn all() -> Self {
        let mut all = Self::empty();
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::LOCAL;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::REMOTE;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::SUM;
        }
        #[cfg(feature = "VK_KHR_swapchain")]
        {
            all |= Self::LOCAL_MULTI_DEVICE;
        }
        all
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DeviceGroupPresentModeFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DeviceGroupPresentModeFlagsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn extend<T: IntoIterator<Item = DeviceGroupPresentModeFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<DeviceGroupPresentModeFlagsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DeviceGroupPresentModeFlagsKHR>>(
        iterator: T,
    ) -> DeviceGroupPresentModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DeviceGroupPresentModeFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for DeviceGroupPresentModeFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from(bit: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn extend<T: IntoIterator<Item = DeviceGroupPresentModeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from_iter<T: IntoIterator<Item = DeviceGroupPresentModeFlagBitsKHR>>(
        iterator: T,
    ) -> DeviceGroupPresentModeFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<DeviceGroupPresentModeFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DeviceGroupPresentModeFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DeviceGroupPresentModeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::LOCAL) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOCAL))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::REMOTE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(REMOTE))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::SUM) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SUM))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain")]
                    if self.0.contains(DeviceGroupPresentModeFlagsKHR::LOCAL_MULTI_DEVICE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOCAL_MULTI_DEVICE))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DeviceGroupPresentModeFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_KHR_DEVICE_GROUP_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_KHR_DEVICE_GROUP_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_device_group");
///# [VkDeviceGroupPresentModeFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/VkDeviceGroupPresentModeFlagBitsKHR.md")]
#[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct DeviceGroupPresentModeFlagBitsKHR(u32);
impl DeviceGroupPresentModeFlagBitsKHR {
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL: Self = Self(1);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const REMOTE: Self = Self(2);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const SUM: Self = Self(4);
    #[doc(alias = "VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain")]
    pub const LOCAL_MULTI_DEVICE: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        match bits {
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::LOCAL.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::REMOTE.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::SUM.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain")]
            x if x == Self::LOCAL_MULTI_DEVICE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///See [`get_device_group_peer_memory_features`]
#[doc(alias = "vkGetDeviceGroupPeerMemoryFeaturesKHR")]
pub type FNGetDeviceGroupPeerMemoryFeaturesKhr = FNGetDeviceGroupPeerMemoryFeatures;
///See [`cmd_set_device_mask`]
#[doc(alias = "vkCmdSetDeviceMaskKHR")]
pub type FNCmdSetDeviceMaskKhr = FNCmdSetDeviceMask;
///See [`cmd_dispatch_base`]
#[doc(alias = "vkCmdDispatchBaseKHR")]
pub type FNCmdDispatchBaseKhr = FNCmdDispatchBase;
///# [vkGetDeviceGroupPresentCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/vkGetDeviceGroupPresentCapabilitiesKHR.md")]
#[doc(alias = "vkGetDeviceGroupPresentCapabilitiesKHR")]
pub type FNGetDeviceGroupPresentCapabilitiesKhr = unsafe extern "system" fn(
    device: Device,
    p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
) -> VulkanResultCodes;
///# [vkGetDeviceGroupSurfacePresentModesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/vkGetDeviceGroupSurfacePresentModesKHR.md")]
#[doc(alias = "vkGetDeviceGroupSurfacePresentModesKHR")]
pub type FNGetDeviceGroupSurfacePresentModesKhr = unsafe extern "system" fn(
    device: Device,
    surface: SurfaceKHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> VulkanResultCodes;
///# [vkAcquireNextImage2KHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/vkAcquireNextImage2KHR.md")]
#[doc(alias = "vkAcquireNextImage2KHR")]
pub type FNAcquireNextImage2Khr = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const AcquireNextImageInfoKHR,
    p_image_index: *mut u32,
) -> VulkanResultCodes;
///# [vkGetPhysicalDevicePresentRectanglesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_device_group/vkGetPhysicalDevicePresentRectanglesKHR.md")]
#[doc(alias = "vkGetPhysicalDevicePresentRectanglesKHR")]
pub type FNGetPhysicalDevicePresentRectanglesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_rect_count: *mut u32,
    p_rects: *mut Rect2D,
) -> VulkanResultCodes;
