//![VK_KHR_device_group](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_device_group.html) - device extension
//!# Description
//!This extension provides functionality to use a logical device that consists
//!of multiple physical devices, as created with the
//!`[`VK_KHR_device_group_creation`]` extension.
//!A device group can allocate memory across the subdevices, bind memory from
//!one subdevice to a resource on another subdevice, record command buffers
//!where some work executes on an arbitrary subset of the subdevices, and
//!potentially present a swapchain image from one or more subdevices.
//!# Revision
//!4
//!# Dependencies
//! - *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_device_group_creation`]`
//!# Contacts
//! - Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_device_group]
//!   @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_device_group
//!   extension>>)
//!# New functions & commands
//! - [`CmdDispatchBaseKHR`]
//! - [`CmdSetDeviceMaskKHR`]
//! - [`GetDeviceGroupPeerMemoryFeaturesKHR`]
//!If [`VK_KHR_surface`] is supported:
//! - [`GetDeviceGroupPresentCapabilitiesKHR`]
//! - [`GetDeviceGroupSurfacePresentModesKHR`]
//! - [`GetPhysicalDevicePresentRectanglesKHR`]
//!If [`VK_KHR_swapchain`] is supported:
//! - [`AcquireNextImage2KHR`]
//!# New structures
//! - Extending [`BindSparseInfo`]:  - [`DeviceGroupBindSparseInfoKHR`]
//! - Extending [`CommandBufferBeginInfo`]:  - [`DeviceGroupCommandBufferBeginInfoKHR`]
//! - Extending [`MemoryAllocateInfo`]:  - [`MemoryAllocateFlagsInfoKHR`]
//! - Extending [`RenderPassBeginInfo`], [`RenderingInfo`]:  - [`DeviceGroupRenderPassBeginInfoKHR`]
//! - Extending [`SubmitInfo`]:  - [`DeviceGroupSubmitInfoKHR`]
//!If [`VK_KHR_bind_memory2`] is supported:
//! - Extending [`BindBufferMemoryInfo`]:  - [`BindBufferMemoryDeviceGroupInfoKHR`]
//! - Extending [`BindImageMemoryInfo`]:  - [`BindImageMemoryDeviceGroupInfoKHR`]
//!If [`VK_KHR_surface`] is supported:
//! - [`DeviceGroupPresentCapabilitiesKHR`]
//!If [`VK_KHR_swapchain`] is supported:
//! - [`AcquireNextImageInfoKHR`]
//! - Extending [`BindImageMemoryInfo`]:  - [`BindImageMemorySwapchainInfoKHR`]
//! - Extending [`ImageCreateInfo`]:  - [`ImageSwapchainCreateInfoKHR`]
//! - Extending [`PresentInfoKHR`]:  - [`DeviceGroupPresentInfoKHR`]
//! - Extending [`SwapchainCreateInfoKHR`]:  - [`DeviceGroupSwapchainCreateInfoKHR`]
//!# New enums
//! - [`MemoryAllocateFlagBitsKHR`]
//! - [`PeerMemoryFeatureFlagBitsKHR`]
//!If [`VK_KHR_surface`] is supported:
//! - [`DeviceGroupPresentModeFlagBitsKHR`]
//!# New bitmasks
//! - [`MemoryAllocateFlagsKHR`]
//! - [`PeerMemoryFeatureFlagsKHR`]
//!If [`VK_KHR_surface`] is supported:
//! - [`DeviceGroupPresentModeFlagsKHR`]
//!# New constants
//! - [`KHR_DEVICE_GROUP_EXTENSION_NAME`]
//! - [`KHR_DEVICE_GROUP_SPEC_VERSION`]
//! - Extending [`DependencyFlagBits`]:  - `VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR`
//! - Extending [`MemoryAllocateFlagBits`]:  - `VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR`
//! - Extending [`PeerMemoryFeatureFlagBits`]:  - `VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR`  -
//!   `VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR`  - `VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR`  -
//!   `VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR`
//! - Extending [`PipelineCreateFlagBits`]:  - `VK_PIPELINE_CREATE_DISPATCH_BASE_KHR`  -
//!   `VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR`
//!If [`VK_KHR_bind_memory2`] is supported:
//! - Extending [`ImageCreateFlagBits`]:  - `VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR`
//!   - `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR`
//!If [`VK_KHR_surface`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
//!If [`VK_KHR_swapchain`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
//! - Extending [`SwapchainCreateFlagBitsKHR`]:  -
//!   `VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR`
//!# Version History
//! - Revision 1, 2016-10-19 (Jeff Bolz)  - Internal revisions
//! - Revision 2, 2017-05-19 (Tobias Hector)  - Removed extended memory bind functions to
//!   VK_KHR_bind_memory2, added dependency on that extension, and device-group-specific structs for
//!   those functions.
//! - Revision 3, 2017-10-06 (Ian Elliott)  - Corrected Vulkan 1.1 interactions with the WSI
//!   extensions. All Vulkan 1.1 WSI interactions are with the VK_KHR_swapchain extension.
//! - Revision 4, 2017-10-10 (Jeff Bolz)  - Rename “SFR” bits and structure members to use the
//!   phrase “split instance bind regions”.
//!# Other info
//! * 2017-10-10
//! * No known IP claims.
//! * - This extension requires [`SPV_KHR_device_group`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_device_group.html)
//!   - Promoted to Vulkan 1.1 Core
//! * - Jeff Bolz, NVIDIA  - Tobias Hector, Imagination Technologies
//!# Related
//! - [`DeviceGroupBindSparseInfoKHR`]
//! - [`DeviceGroupCommandBufferBeginInfoKHR`]
//! - [`DeviceGroupRenderPassBeginInfoKHR`]
//! - [`DeviceGroupSubmitInfoKHR`]
//! - [`MemoryAllocateFlagBitsKHR`]
//! - [`MemoryAllocateFlagsInfoKHR`]
//! - [`MemoryAllocateFlagsKHR`]
//! - [`PeerMemoryFeatureFlagBitsKHR`]
//! - [`PeerMemoryFeatureFlagsKHR`]
//! - [`CmdDispatchBaseKHR`]
//! - [`CmdSetDeviceMaskKHR`]
//! - [`GetDeviceGroupPeerMemoryFeaturesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Fence, Semaphore, StructureType},
    vulkan1_1::MAX_DEVICE_GROUP_SIZE,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_DEVICE_GROUP_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_device_group");
///[VkDeviceGroupPresentModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) - Bitmask specifying supported device group present modes
///# C Specifications
///Bits which  **may**  be set in
///[`DeviceGroupPresentCapabilitiesKHR::modes`], indicating which
///device group presentation modes are supported, are:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///typedef enum VkDeviceGroupPresentModeFlagBitsKHR {
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = 0x00000001,
///    VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = 0x00000002,
///    VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = 0x00000004,
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008,
///} VkDeviceGroupPresentModeFlagBitsKHR;
///```
///# Description
/// - [`DeviceGroupPresentModeLocalKhr`] specifies that any physical device with a presentation
///   engine  **can**  present its own swapchain images.
/// - [`DeviceGroupPresentModeRemoteKhr`] specifies that any physical device with a presentation
///   engine  **can**  present swapchain images from any physical device in its `presentMask`.
/// - [`DeviceGroupPresentModeSumKhr`] specifies that any physical device with a presentation engine
///   **can**  present the sum of swapchain images from any physical devices in its `presentMask`.
/// - [`DeviceGroupPresentModeLocalMultiDeviceKhr`] specifies that multiple physical devices with a
///   presentation engine  **can**  each present their own swapchain images.
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_surface`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentInfoKHR`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentModeFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum DeviceGroupPresentModeFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`DeviceGroupPresentModeLocalKhr`] specifies that any
    ///physical device with a presentation engine  **can**  present its own
    ///swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    DeviceGroupPresentModeLocalKhr = 1,
    ///[`DeviceGroupPresentModeRemoteKhr`] specifies that any
    ///physical device with a presentation engine  **can**  present swapchain images
    ///from any physical device in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    DeviceGroupPresentModeRemoteKhr = 2,
    ///[`DeviceGroupPresentModeSumKhr`] specifies that any
    ///physical device with a presentation engine  **can**  present the sum of
    ///swapchain images from any physical devices in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    DeviceGroupPresentModeSumKhr = 4,
    ///[`DeviceGroupPresentModeLocalMultiDeviceKhr`] specifies
    ///that multiple physical devices with a presentation engine  **can**  each
    ///present their own swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    DeviceGroupPresentModeLocalMultiDeviceKhr = 8,
}
impl const Default for DeviceGroupPresentModeFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl DeviceGroupPresentModeFlagBitsKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkDeviceGroupPresentModeFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentModeFlagBitsKHR.html) - Bitmask specifying supported device group present modes
///# C Specifications
///Bits which  **may**  be set in
///[`DeviceGroupPresentCapabilitiesKHR::modes`], indicating which
///device group presentation modes are supported, are:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///typedef enum VkDeviceGroupPresentModeFlagBitsKHR {
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = 0x00000001,
///    VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = 0x00000002,
///    VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = 0x00000004,
///    VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = 0x00000008,
///} VkDeviceGroupPresentModeFlagBitsKHR;
///```
///# Description
/// - [`DeviceGroupPresentModeLocalKhr`] specifies that any physical device with a presentation
///   engine  **can**  present its own swapchain images.
/// - [`DeviceGroupPresentModeRemoteKhr`] specifies that any physical device with a presentation
///   engine  **can**  present swapchain images from any physical device in its `presentMask`.
/// - [`DeviceGroupPresentModeSumKhr`] specifies that any physical device with a presentation engine
///   **can**  present the sum of swapchain images from any physical devices in its `presentMask`.
/// - [`DeviceGroupPresentModeLocalMultiDeviceKhr`] specifies that multiple physical devices with a
///   presentation engine  **can**  each present their own swapchain images.
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_surface`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentInfoKHR`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentModeFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DeviceGroupPresentModeFlagsKHR(u32);
impl const Default for DeviceGroupPresentModeFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from(from: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl DeviceGroupPresentModeFlagsKHR {
    ///[`DeviceGroupPresentModeLocalKhr`] specifies that any
    ///physical device with a presentation engine  **can**  present its own
    ///swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    pub const DEVICE_GROUP_PRESENT_MODE_LOCAL_KHR: Self = Self(1);
    ///[`DeviceGroupPresentModeRemoteKhr`] specifies that any
    ///physical device with a presentation engine  **can**  present swapchain images
    ///from any physical device in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    pub const DEVICE_GROUP_PRESENT_MODE_REMOTE_KHR: Self = Self(2);
    ///[`DeviceGroupPresentModeSumKhr`] specifies that any
    ///physical device with a presentation engine  **can**  present the sum of
    ///swapchain images from any physical devices in its `presentMask`.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    pub const DEVICE_GROUP_PRESENT_MODE_SUM_KHR: Self = Self(4);
    ///[`DeviceGroupPresentModeLocalMultiDeviceKhr`] specifies
    ///that multiple physical devices with a presentation engine  **can**  each
    ///present their own swapchain images.
    ///
    ///Provided by [`crate::extensions::khr_swapchain`]
    pub const DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_KHR: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::DEVICE_GROUP_PRESENT_MODE_LOCAL_KHR
            | Self::DEVICE_GROUP_PRESENT_MODE_REMOTE_KHR
            | Self::DEVICE_GROUP_PRESENT_MODE_SUM_KHR
            | Self::DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_KHR
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
impl std::ops::BitOrAssign for DeviceGroupPresentModeFlagsKHR {
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
impl std::ops::BitXorAssign for DeviceGroupPresentModeFlagsKHR {
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
impl std::ops::BitAndAssign for DeviceGroupPresentModeFlagsKHR {
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
impl std::ops::SubAssign for DeviceGroupPresentModeFlagsKHR {
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
impl Extend<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn extend<T: IntoIterator<Item = DeviceGroupPresentModeFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DeviceGroupPresentModeFlagBitsKHR>>::from(i));
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
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DeviceGroupPresentModeFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(DeviceGroupPresentModeFlagsKHR::DEVICE_GROUP_PRESENT_MODE_LOCAL_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEVICE_GROUP_PRESENT_MODE_LOCAL_KHR))?;
                    }
                    if self
                        .0
                        .contains(DeviceGroupPresentModeFlagsKHR::DEVICE_GROUP_PRESENT_MODE_REMOTE_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEVICE_GROUP_PRESENT_MODE_REMOTE_KHR))?;
                    }
                    if self
                        .0
                        .contains(DeviceGroupPresentModeFlagsKHR::DEVICE_GROUP_PRESENT_MODE_SUM_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEVICE_GROUP_PRESENT_MODE_SUM_KHR))?;
                    }
                    if self
                        .0
                        .contains(DeviceGroupPresentModeFlagsKHR::DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_KHR))?;
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
///[VkDeviceGroupPresentCapabilitiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentCapabilitiesKHR.html) - Present capabilities from other physical devices
///# C Specifications
///The [`DeviceGroupPresentCapabilitiesKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_surface
///typedef struct VkDeviceGroupPresentCapabilitiesKHR {
///    VkStructureType                     sType;
///    void*                               pNext;
///    uint32_t                            presentMask[VK_MAX_DEVICE_GROUP_SIZE];
///    VkDeviceGroupPresentModeFlagsKHR    modes;
///} VkDeviceGroupPresentCapabilitiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`present_mask`] is an array of [`MAX_DEVICE_GROUP_SIZE`]`uint32_t` masks, where the mask at
///   element i is non-zero if physical device i has a presentation engine, and where bit j is set
///   in element i if physical device i **can**  present swapchain images from physical device j. If
///   element i is non-zero, then bit i **must**  be set.
/// - [`modes`] is a bitmask of [`DeviceGroupPresentModeFlagBitsKHR`] indicating which device group
///   presentation modes are supported.
///# Description
///[`modes`] always has `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR` set.The present mode flags are
/// also used when presenting an image, in
///[`DeviceGroupPresentInfoKHR::mode`].If a device group only includes a single physical device,
/// then [`modes`] **must**  equal `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_surface`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
/// - [`StructureType`]
/// - [`GetDeviceGroupPresentCapabilitiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentCapabilitiesKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupPresentCapabilitiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`present_mask`] is an array of [`MAX_DEVICE_GROUP_SIZE`]`uint32_t` masks, where the mask at
    /// element i is non-zero if physical device i has a presentation engine, and where bit j
    ///is set in element i if physical device i **can**  present
    ///swapchain images from physical device j.
    ///If element i is non-zero, then bit i **must**  be set.
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE as usize],
    ///[`modes`] is a bitmask of [`DeviceGroupPresentModeFlagBitsKHR`]
    ///indicating which device group presentation modes are supported.
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl<'lt> Default for DeviceGroupPresentCapabilitiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            present_mask: [0; MAX_DEVICE_GROUP_SIZE as usize],
            modes: Default::default(),
        }
    }
}
impl<'lt> DeviceGroupPresentCapabilitiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::present_mask`]
    pub fn present_mask(&self) -> &[u32; MAX_DEVICE_GROUP_SIZE as usize] {
        &self.present_mask
    }
    ///Gets the value of [`Self::modes`]
    pub fn modes(&self) -> DeviceGroupPresentModeFlagsKHR {
        self.modes
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::present_mask`]
    pub fn present_mask_mut(&mut self) -> &mut [u32; MAX_DEVICE_GROUP_SIZE as usize] {
        &mut self.present_mask
    }
    ///Gets a mutable reference to the value of [`Self::modes`]
    pub fn modes_mut(&mut self) -> &mut DeviceGroupPresentModeFlagsKHR {
        &mut self.modes
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::present_mask`]
    pub fn set_present_mask(&mut self, value: [u32; crate::vulkan1_1::MAX_DEVICE_GROUP_SIZE as usize]) -> &mut Self {
        self.present_mask = value;
        self
    }
    ///Sets the raw value of [`Self::modes`]
    pub fn set_modes(
        &mut self,
        value: crate::extensions::khr_device_group::DeviceGroupPresentModeFlagsKHR,
    ) -> &mut Self {
        self.modes = value;
        self
    }
}
///[VkImageSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageSwapchainCreateInfoKHR.html) - Specify that an image will be bound to swapchain memory
///# C Specifications
///If the [`p_next`] chain of [`ImageCreateInfo`] includes a
///[`ImageSwapchainCreateInfoKHR`] structure, then that structure includes
///a swapchain handle indicating that the image will be bound to memory from
///that swapchain.The [`ImageSwapchainCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkImageSwapchainCreateInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSwapchainKHR     swapchain;
///} VkImageSwapchainCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain`] is [`crate::utils::Handle::null`] or a handle of a swapchain that the image will
///   be bound to.
///# Description
///## Valid Usage
/// -    If [`swapchain`] is not [`crate::utils::Handle::null`], the fields of [`ImageCreateInfo`] **must**  match the [implied image creation parameters](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#swapchain-wsi-image-create-info) of the swapchain
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR`
/// - If [`swapchain`] is not [`crate::utils::Handle::null`], [`swapchain`] **must**  be a valid
///   [`SwapchainKHR`] handle
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkImageSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct ImageSwapchainCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain`] is [`crate::utils::Handle::null`] or a handle of a swapchain that
    ///the image will be bound to.
    pub swapchain: SwapchainKHR,
}
impl<'lt> Default for ImageSwapchainCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            swapchain: Default::default(),
        }
    }
}
impl<'lt> ImageSwapchainCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain`]
    pub fn swapchain(&self) -> SwapchainKHR {
        self.swapchain
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain`]
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::swapchain`]
    pub fn set_swapchain(&mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> &mut Self {
        self.swapchain = value;
        self
    }
}
///[VkBindImageMemorySwapchainInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBindImageMemorySwapchainInfoKHR.html) - Structure specifying swapchain image memory to bind to
///# C Specifications
///If the [`p_next`] chain of [`BindImageMemoryInfo`] includes a
///[`BindImageMemorySwapchainInfoKHR`] structure, then that structure
///includes a swapchain handle and image index indicating that the image will
///be bound to memory from that swapchain.The [`BindImageMemorySwapchainInfoKHR`] structure is
/// defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkBindImageMemorySwapchainInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSwapchainKHR     swapchain;
///    uint32_t           imageIndex;
///} VkBindImageMemorySwapchainInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain`] is [`crate::utils::Handle::null`] or a swapchain handle.
/// - [`image_index`] is an image index within [`swapchain`].
///# Description
///If [`swapchain`] is not `NULL`, the [`swapchain`] and [`image_index`]
///are used to determine the memory that the image is bound to, instead of
///`memory` and `memoryOffset`.Memory  **can**  be bound to a swapchain and use the
/// `pDeviceIndices` or
///`pSplitInstanceBindRegions` members of
///[`BindImageMemoryDeviceGroupInfo`].
///## Valid Usage
/// - [`image_index`] **must**  be less than the number of images in [`swapchain`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR`
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBindImageMemorySwapchainInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct BindImageMemorySwapchainInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain`] is [`crate::utils::Handle::null`] or a swapchain handle.
    pub swapchain: SwapchainKHR,
    ///[`image_index`] is an image index within [`swapchain`].
    pub image_index: u32,
}
impl<'lt> Default for BindImageMemorySwapchainInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            image_index: 0,
        }
    }
}
impl<'lt> BindImageMemorySwapchainInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain`]
    pub fn swapchain(&self) -> SwapchainKHR {
        self.swapchain
    }
    ///Gets the value of [`Self::image_index`]
    pub fn image_index(&self) -> u32 {
        self.image_index
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain`]
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Gets a mutable reference to the value of [`Self::image_index`]
    pub fn image_index_mut(&mut self) -> &mut u32 {
        &mut self.image_index
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::swapchain`]
    pub fn set_swapchain(&mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> &mut Self {
        self.swapchain = value;
        self
    }
    ///Sets the raw value of [`Self::image_index`]
    pub fn set_image_index(&mut self, value: u32) -> &mut Self {
        self.image_index = value;
        self
    }
}
///[VkAcquireNextImageInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAcquireNextImageInfoKHR.html) - Structure specifying parameters of the acquire
///# C Specifications
///The [`AcquireNextImageInfoKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkAcquireNextImageInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkSwapchainKHR     swapchain;
///    uint64_t           timeout;
///    VkSemaphore        semaphore;
///    VkFence            fence;
///    uint32_t           deviceMask;
///} VkAcquireNextImageInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain`] is a non-retired swapchain from which an image is acquired.
/// - [`timeout`] specifies how long the function waits, in nanoseconds, if no image is available.
/// - [`semaphore`] is [`crate::utils::Handle::null`] or a semaphore to signal.
/// - [`fence`] is [`crate::utils::Handle::null`] or a fence to signal.
/// - [`device_mask`] is a mask of physical devices for which the swapchain image will be ready to
///   use when the semaphore or fence is signaled.
///# Description
///If [`AcquireNextImageKHR`] is used, the device mask is considered to
///include all physical devices in the logical device.
///## Valid Usage
/// - [`swapchain`] **must**  not be in the retired state
/// - If [`semaphore`] is not [`crate::utils::Handle::null`] it  **must**  be unsignaled
/// - If [`semaphore`] is not [`crate::utils::Handle::null`] it  **must**  not have any uncompleted
///   signal or wait operations pending
/// - If [`fence`] is not [`crate::utils::Handle::null`] it  **must**  be unsignaled and  **must**
///   not be associated with any other queue command that has not yet completed execution on that
///   queue
/// - [`semaphore`] and [`fence`] **must**  not both be equal to [`crate::utils::Handle::null`]
/// - [`device_mask`] **must**  be a valid device mask
/// - [`device_mask`] **must**  not be zero
/// - [`semaphore`] **must**  have a [`SemaphoreType`] of `VK_SEMAPHORE_TYPE_BINARY`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
/// - If [`semaphore`] is not [`crate::utils::Handle::null`], [`semaphore`] **must**  be a valid
///   [`Semaphore`] handle
/// - If [`fence`] is not [`crate::utils::Handle::null`], [`fence`] **must**  be a valid [`Fence`]
///   handle
/// - Each of [`fence`], [`semaphore`], and [`swapchain`] that are valid handles of non-ignored
///   parameters  **must**  have been created, allocated, or retrieved from the same [`Instance`]
///
///## Host Synchronization
/// - Host access to [`swapchain`] **must**  be externally synchronized
/// - Host access to [`semaphore`] **must**  be externally synchronized
/// - Host access to [`fence`] **must**  be externally synchronized
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`Fence`]
/// - [`Semaphore`]
/// - [`StructureType`]
/// - [`SwapchainKHR`]
/// - [`AcquireNextImage2KHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAcquireNextImageInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AcquireNextImageInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain`] is a non-retired swapchain from which an image is
    ///acquired.
    pub swapchain: SwapchainKHR,
    ///[`timeout`] specifies how long the function waits, in nanoseconds, if
    ///no image is available.
    pub timeout: u64,
    ///[`semaphore`] is [`crate::utils::Handle::null`] or a semaphore to signal.
    pub semaphore: Semaphore,
    ///[`fence`] is [`crate::utils::Handle::null`] or a fence to signal.
    pub fence: Fence,
    ///[`device_mask`] is a mask of physical devices for which the swapchain
    ///image will be ready to use when the semaphore or fence is signaled.
    pub device_mask: u32,
}
impl<'lt> Default for AcquireNextImageInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            swapchain: Default::default(),
            timeout: 0,
            semaphore: Default::default(),
            fence: Default::default(),
            device_mask: 0,
        }
    }
}
impl<'lt> AcquireNextImageInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain`]
    pub fn swapchain(&self) -> SwapchainKHR {
        self.swapchain
    }
    ///Gets the value of [`Self::timeout`]
    pub fn timeout(&self) -> u64 {
        self.timeout
    }
    ///Gets the value of [`Self::semaphore`]
    pub fn semaphore(&self) -> Semaphore {
        self.semaphore
    }
    ///Gets the value of [`Self::fence`]
    pub fn fence(&self) -> Fence {
        self.fence
    }
    ///Gets the value of [`Self::device_mask`]
    pub fn device_mask(&self) -> u32 {
        self.device_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain`]
    pub fn swapchain_mut(&mut self) -> &mut SwapchainKHR {
        &mut self.swapchain
    }
    ///Gets a mutable reference to the value of [`Self::timeout`]
    pub fn timeout_mut(&mut self) -> &mut u64 {
        &mut self.timeout
    }
    ///Gets a mutable reference to the value of [`Self::semaphore`]
    pub fn semaphore_mut(&mut self) -> &mut Semaphore {
        &mut self.semaphore
    }
    ///Gets a mutable reference to the value of [`Self::fence`]
    pub fn fence_mut(&mut self) -> &mut Fence {
        &mut self.fence
    }
    ///Gets a mutable reference to the value of [`Self::device_mask`]
    pub fn device_mask_mut(&mut self) -> &mut u32 {
        &mut self.device_mask
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::swapchain`]
    pub fn set_swapchain(&mut self, value: crate::extensions::khr_swapchain::SwapchainKHR) -> &mut Self {
        self.swapchain = value;
        self
    }
    ///Sets the raw value of [`Self::timeout`]
    pub fn set_timeout(&mut self, value: u64) -> &mut Self {
        self.timeout = value;
        self
    }
    ///Sets the raw value of [`Self::semaphore`]
    pub fn set_semaphore(&mut self, value: crate::vulkan1_0::Semaphore) -> &mut Self {
        self.semaphore = value;
        self
    }
    ///Sets the raw value of [`Self::fence`]
    pub fn set_fence(&mut self, value: crate::vulkan1_0::Fence) -> &mut Self {
        self.fence = value;
        self
    }
    ///Sets the raw value of [`Self::device_mask`]
    pub fn set_device_mask(&mut self, value: u32) -> &mut Self {
        self.device_mask = value;
        self
    }
}
///[VkDeviceGroupPresentInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupPresentInfoKHR.html) - Mode and mask controlling which physical devices' images are presented
///# C Specifications
///If the [`p_next`] chain of [`PresentInfoKHR`] includes a
///[`DeviceGroupPresentInfoKHR`] structure, then that structure includes an
///array of device masks and a device group present mode.The [`DeviceGroupPresentInfoKHR`]
/// structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkDeviceGroupPresentInfoKHR {
///    VkStructureType                        sType;
///    const void*                            pNext;
///    uint32_t                               swapchainCount;
///    const uint32_t*                        pDeviceMasks;
///    VkDeviceGroupPresentModeFlagBitsKHR    mode;
///} VkDeviceGroupPresentInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`swapchain_count`] is zero or the number of elements in [`device_masks`].
/// - [`device_masks`] is a pointer to an array of device masks, one for each element of
///   [`PresentInfoKHR`]::pSwapchains.
/// - [`mode`] is a [`DeviceGroupPresentModeFlagBitsKHR`] value specifying the device group present
///   mode that will be used for this present.
///# Description
///If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`, then each
///element of [`device_masks`] selects which instance of the swapchain image
///is presented.
///Each element of [`device_masks`] **must**  have exactly one bit set, and the
///corresponding physical device  **must**  have a presentation engine as reported
///by [`DeviceGroupPresentCapabilitiesKHR`].If [`mode`] is
/// `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR`, then
///each element of [`device_masks`] selects which instance of the swapchain
///image is presented.
///Each element of [`device_masks`] **must**  have exactly one bit set, and some
///physical device in the logical device  **must**  include that bit in its
///[`DeviceGroupPresentCapabilitiesKHR::present_mask`].If [`mode`] is
/// `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR`, then each
///element of [`device_masks`] selects which instances of the swapchain image
///are component-wise summed and the sum of those images is presented.
///If the sum in any component is outside the representable range, the value of
///that component is undefined.
///Each element of [`device_masks`] **must**  have a value for which all set bits
///are set in one of the elements of
///[`DeviceGroupPresentCapabilitiesKHR::present_mask`].If [`mode`] is
///`VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`, then each
///element of [`device_masks`] selects which instance(s) of the swapchain
///images are presented.
///For each bit set in each element of [`device_masks`], the corresponding
///physical device  **must**  have a presentation engine as reported by
///[`DeviceGroupPresentCapabilitiesKHR`].If [`DeviceGroupPresentInfoKHR`] is not provided or
/// [`swapchain_count`]
///is zero then the masks are considered to be `1`.
///If [`DeviceGroupPresentInfoKHR`] is not provided, [`mode`] is
///considered to be `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
///## Valid Usage
/// - [`swapchain_count`] **must**  equal `0` or [`PresentInfoKHR`]::[`swapchain_count`]
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`, then each element of
///   [`device_masks`] **must**  have exactly one bit set, and the corresponding element of
///   [`DeviceGroupPresentCapabilitiesKHR::present_mask`] **must**  be non-zero
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR`, then each element of
///   [`device_masks`] **must**  have exactly one bit set, and some physical device in the logical
///   device  **must**  include that bit in its [`DeviceGroupPresentCapabilitiesKHR::present_mask`]
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR`, then each element of
///   [`device_masks`] **must**  have a value for which all set bits are set in one of the elements
///   of [`DeviceGroupPresentCapabilitiesKHR::present_mask`]
/// - If [`mode`] is `VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR`, then for each bit
///   set in each element of [`device_masks`], the corresponding element of
///   [`DeviceGroupPresentCapabilitiesKHR::present_mask`] **must**  be non-zero
/// - The value of each element of [`device_masks`] **must**  be equal to the device mask passed in
///   [`AcquireNextImageInfoKHR::device_mask`] when the image index was last acquired
/// - [`mode`] **must**  have exactly one bit set, and that bit  **must**  have been included in
///   [`DeviceGroupSwapchainCreateInfoKHR::modes`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR`
/// - If [`swapchain_count`] is not `0`, [`device_masks`] **must**  be a valid pointer to an array
///   of [`swapchain_count`]`uint32_t` values
/// - [`mode`] **must**  be a valid [`DeviceGroupPresentModeFlagBitsKHR`] value
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentModeFlagBitsKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupPresentInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupPresentInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`swapchain_count`] is zero or the number of elements in
    ///[`device_masks`].
    pub swapchain_count: u32,
    ///[`device_masks`] is a pointer to an array of device masks, one for
    ///each element of [`PresentInfoKHR`]::pSwapchains.
    pub device_masks: *const u32,
    ///[`mode`] is a [`DeviceGroupPresentModeFlagBitsKHR`] value
    ///specifying the device group present mode that will be used for this
    ///present.
    pub mode: DeviceGroupPresentModeFlagBitsKHR,
}
impl<'lt> Default for DeviceGroupPresentInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            swapchain_count: 0,
            device_masks: std::ptr::null(),
            mode: Default::default(),
        }
    }
}
impl<'lt> DeviceGroupPresentInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::device_masks`]
    pub fn device_masks_raw(&self) -> *const u32 {
        self.device_masks
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::device_masks`]
    pub fn set_device_masks_raw(&mut self, value: *const u32) -> &mut Self {
        self.device_masks = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::swapchain_count`]
    pub fn swapchain_count(&self) -> u32 {
        self.swapchain_count
    }
    ///Gets the value of [`Self::device_masks`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn device_masks(&self) -> &[u32] {
        std::slice::from_raw_parts(self.device_masks, self.swapchain_count as usize)
    }
    ///Gets the value of [`Self::mode`]
    pub fn mode(&self) -> DeviceGroupPresentModeFlagBitsKHR {
        self.mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::swapchain_count`]
    pub fn swapchain_count_mut(&mut self) -> &mut u32 {
        &mut self.swapchain_count
    }
    ///Gets a mutable reference to the value of [`Self::mode`]
    pub fn mode_mut(&mut self) -> &mut DeviceGroupPresentModeFlagBitsKHR {
        &mut self.mode
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::swapchain_count`]
    pub fn set_swapchain_count(&mut self, value: u32) -> &mut Self {
        self.swapchain_count = value;
        self
    }
    ///Sets the raw value of [`Self::device_masks`]
    pub fn set_device_masks(&mut self, value: &'lt [u32]) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.device_masks = value.as_ptr();
        self.swapchain_count = len_;
        self
    }
    ///Sets the raw value of [`Self::mode`]
    pub fn set_mode(
        &mut self,
        value: crate::extensions::khr_device_group::DeviceGroupPresentModeFlagBitsKHR,
    ) -> &mut Self {
        self.mode = value;
        self
    }
}
///[VkDeviceGroupSwapchainCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceGroupSwapchainCreateInfoKHR.html) - Structure specifying parameters of a newly created swapchain object
///# C Specifications
///If the [`p_next`] chain of [`SwapchainCreateInfoKHR`] includes a
///[`DeviceGroupSwapchainCreateInfoKHR`] structure, then that structure
///includes a set of device group present modes that the swapchain  **can**  be used
///with.The [`DeviceGroupSwapchainCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_VERSION_1_1 with VK_KHR_swapchain, VK_KHR_device_group with VK_KHR_swapchain
///typedef struct VkDeviceGroupSwapchainCreateInfoKHR {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    VkDeviceGroupPresentModeFlagsKHR    modes;
///} VkDeviceGroupSwapchainCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`modes`] is a bitfield of modes that the swapchain  **can**  be used with.
///# Description
///If this structure is not present, [`modes`] is considered to be
///`VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR`.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR`
/// - [`modes`] **must**  be a valid combination of [`DeviceGroupPresentModeFlagBitsKHR`] values
/// - [`modes`] **must**  not be `0`
///# Related
/// - [`VK_KHR_device_group`]
/// - [`VK_KHR_swapchain`]
/// - [`crate::vulkan1_1`]
/// - [`DeviceGroupPresentModeFlagsKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceGroupSwapchainCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct DeviceGroupSwapchainCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`modes`] is a bitfield of modes that the swapchain  **can**  be used with.
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl<'lt> Default for DeviceGroupSwapchainCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            modes: Default::default(),
        }
    }
}
impl<'lt> DeviceGroupSwapchainCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::modes`]
    pub fn modes(&self) -> DeviceGroupPresentModeFlagsKHR {
        self.modes
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::modes`]
    pub fn modes_mut(&mut self) -> &mut DeviceGroupPresentModeFlagsKHR {
        &mut self.modes
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::modes`]
    pub fn set_modes(
        &mut self,
        value: crate::extensions::khr_device_group::DeviceGroupPresentModeFlagsKHR,
    ) -> &mut Self {
        self.modes = value;
        self
    }
}
