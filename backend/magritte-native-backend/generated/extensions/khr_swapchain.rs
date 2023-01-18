use crate::{
    cstr,
    extensions::khr_surface::{
        ColorSpaceKHR, CompositeAlphaFlagBitsKHR, PresentModeKHR, SurfaceKHR, SurfaceTransformFlagBitsKHR,
    },
    vulkan1_0::{
        AllocationCallbacks, BaseInStructure, Bool32, Device, Extent2D, Fence, Format, ImageUsageFlags,
        ImageViewCreateInfo, Queue, Semaphore, SharingMode, StructureType, VulkanResultCodes,
    },
};
use std::ffi::CStr;
#[doc(alias = "VkSwapchainCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: SwapchainCreateFlagsKHR,
    surface: SurfaceKHR,
    #[doc(alias = "minImageCount")]
    min_image_count: u32,
    #[doc(alias = "imageFormat")]
    image_format: Format,
    #[doc(alias = "imageColorSpace")]
    image_color_space: ColorSpaceKHR,
    #[doc(alias = "imageExtent")]
    image_extent: Extent2D,
    #[doc(alias = "imageArrayLayers")]
    image_array_layers: u32,
    #[doc(alias = "imageUsage")]
    image_usage: ImageUsageFlags,
    #[doc(alias = "imageSharingMode")]
    image_sharing_mode: SharingMode,
    #[doc(alias = "queueFamilyIndexCount")]
    queue_family_index_count: u32,
    #[doc(alias = "pQueueFamilyIndices")]
    queue_family_indices: *const u32,
    #[doc(alias = "preTransform")]
    pre_transform: SurfaceTransformFlagBitsKHR,
    #[doc(alias = "compositeAlpha")]
    composite_alpha: CompositeAlphaFlagBitsKHR,
    #[doc(alias = "presentMode")]
    present_mode: PresentModeKHR,
    clipped: Bool32,
    #[doc(alias = "oldSwapchain")]
    old_swapchain: SwapchainKHR,
}
#[doc(alias = "VkPresentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreCount")]
    wait_semaphore_count: u32,
    #[doc(alias = "pWaitSemaphores")]
    wait_semaphores: *const Semaphore,
    #[doc(alias = "swapchainCount")]
    swapchain_count: u32,
    #[doc(alias = "pSwapchains")]
    swapchains: *const SwapchainKHR,
    #[doc(alias = "pImageIndices")]
    image_indices: *const u32,
    #[doc(alias = "pResults")]
    results: *mut VulkanResultCodes,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkSwapchainKHR")]
#[repr(transparent)]
pub struct SwapchainKHR(u64);
impl SwapchainKHR {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for SwapchainKHR {
    fn default() -> Self {
        Self::null()
    }
}
# [doc = include_str ! ("../../../../doc/extensions/khr_swapchain/VkImage.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkImage")]
#[repr(transparent)]
pub struct SwapchainImage(u64);
impl SwapchainImage {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for SwapchainImage {
    fn default() -> Self {
        Self::null()
    }
}
# [doc = include_str ! ("../../../../doc/extensions/khr_swapchain/VkImageView.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkImageView")]
#[repr(transparent)]
pub struct SwapchainImageView(u64);
impl SwapchainImageView {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for SwapchainImageView {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VkSwapchainCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwapchainCreateFlagsKHR(u32);
impl SwapchainCreateFlagsKHR {
    #[doc(alias = "VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR")]
    pub const PROTECTED: Self = Self(2);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const MUTABLE_FORMAT: Self = Self(4);
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
        #[cfg(feature = "VK_KHR_device_group")]
        {
            all |= Self::SPLIT_INSTANCE_BIND_REGIONS;
        }
        {
            all |= Self::PROTECTED;
        }
        #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
        {
            all |= Self::MUTABLE_FORMAT;
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
impl const std::ops::BitOr for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for SwapchainCreateFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SwapchainCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SwapchainCreateFlagsKHR> for SwapchainCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = SwapchainCreateFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<SwapchainCreateFlagsKHR> for SwapchainCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SwapchainCreateFlagsKHR>>(iterator: T) -> SwapchainCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SwapchainCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for SwapchainCreateFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn from(bit: SwapchainCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = SwapchainCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = SwapchainCreateFlagBitsKHR>>(iterator: T) -> SwapchainCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<SwapchainCreateFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SwapchainCreateFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SwapchainCreateFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    #[cfg(feature = "VK_KHR_device_group")]
                    if self.0.contains(SwapchainCreateFlagsKHR::SPLIT_INSTANCE_BIND_REGIONS) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(SPLIT_INSTANCE_BIND_REGIONS))?;
                    }
                    if self.0.contains(SwapchainCreateFlagsKHR::PROTECTED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PROTECTED))?;
                    }
                    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
                    if self.0.contains(SwapchainCreateFlagsKHR::MUTABLE_FORMAT) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MUTABLE_FORMAT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SwapchainCreateFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_KHR_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
#[doc(alias = "VK_KHR_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_swapchain");
#[doc(alias = "VkSwapchainCreateFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct SwapchainCreateFlagBitsKHR(u32);
impl SwapchainCreateFlagBitsKHR {
    #[doc(alias = "VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR")]
    #[cfg(feature = "VK_KHR_device_group")]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR")]
    pub const PROTECTED: Self = Self(2);
    #[doc(alias = "VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR")]
    #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
    pub const MUTABLE_FORMAT: Self = Self(4);
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
            #[cfg(feature = "VK_KHR_device_group")]
            x if x == Self::SPLIT_INSTANCE_BIND_REGIONS.bits() => Some(Self(x)),
            x if x == Self::PROTECTED.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_swapchain_mutable_format")]
            x if x == Self::MUTABLE_FORMAT.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkCreateSwapchainKHR")]
pub type FNCreateSwapchainKhr = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SwapchainCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_swapchain: *mut SwapchainKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkDestroySwapchainKHR")]
pub type FNDestroySwapchainKhr =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR, p_allocator: *const AllocationCallbacks);
#[doc(alias = "vkGetSwapchainImagesKHR")]
pub type FNGetSwapchainImagesKhr = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut SwapchainImage,
) -> VulkanResultCodes;
#[doc(alias = "vkAcquireNextImageKHR")]
pub type FNAcquireNextImageKhr = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    p_image_index: *mut u32,
) -> VulkanResultCodes;
#[doc(alias = "vkQueuePresentKHR")]
pub type FNQueuePresentKhr =
    unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR) -> VulkanResultCodes;
# [doc = include_str ! ("../../../../doc/extensions/khr_swapchain/vkCreateImageView.md")]
#[doc(alias = "vkCreateImageView")]
pub type FNCreateSwapchainImageView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut SwapchainImageView,
) -> VulkanResultCodes;
