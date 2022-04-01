//![VK_EXT_display_surface_counter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_display_surface_counter.html) - instance extension
//!# Description
//!This extension defines a vertical blanking period counter associated with
//!display surfaces.
//!It provides a mechanism to query support for such a counter from a
//![`SurfaceKHR`] object.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_display`]`
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_display_surface_counter]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_EXT_display_surface_counter extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceSurfaceCapabilities2EXT`]
//!# New structures
//! - [`SurfaceCapabilities2EXT`]
//!# New enums
//! - [`SurfaceCounterFlagBitsEXT`]
//!# New bitmasks
//! - [`SurfaceCounterFlagsEXT`]
//!# New constants
//! - [`EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME`]
//! - [`EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT`  -
//!   `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`
//!# Version History
//! - Revision 1, 2016-12-13 (James Jones)  - Initial draft
//!# Other info
//! * 2016-12-13
//! * No known IP claims.
//! * - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone, NVIDIA  - Pierre-Loup
//!   Griffais, Valve  - Daniel Vetter, Intel
//!# Related
//! - [`SurfaceCapabilities2EXT`]
//! - [`SurfaceCounterFlagBitsEXT`]
//! - [`SurfaceCounterFlagsEXT`]
//! - [`GetPhysicalDeviceSurfaceCapabilities2EXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    extensions::{
        khr_display::SurfaceTransformFlagsKHR,
        khr_surface::{CompositeAlphaFlagsKHR, SurfaceTransformFlagBitsKHR},
    },
    vulkan1_0::{BaseOutStructure, Extent2D, ImageUsageFlags, StructureType},
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
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION")]
pub const EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME")]
pub const EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_display_surface_counter");
///[VkSurfaceCounterFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) - Surface-relative counter types
///# C Specifications
///Bits which  **can**  be set in
///[`SurfaceCapabilities2EXT::supported_surface_counters`], indicating
///supported surface counter types, are:
///```c
///// Provided by VK_EXT_display_surface_counter
///typedef enum VkSurfaceCounterFlagBitsEXT {
///    VK_SURFACE_COUNTER_VBLANK_BIT_EXT = 0x00000001,
///    VK_SURFACE_COUNTER_VBLANK_EXT = VK_SURFACE_COUNTER_VBLANK_BIT_EXT,
///} VkSurfaceCounterFlagBitsEXT;
///```
///# Description
/// - [`SurfaceCounterVblankExt`] specifies a counter incrementing once every time a vertical
///   blanking period occurs on the display associated with the surface.
///# Related
/// - [`VK_EXT_display_surface_counter`]
/// - [`SurfaceCounterFlagsEXT`]
/// - [`GetSwapchainCounterEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceCounterFlagBitsEXT")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum SurfaceCounterFlagBitsEXT {
    #[doc(hidden)]
    Empty = 0,
    ///[`SurfaceCounterVblankExt`] specifies a counter incrementing
    ///once every time a vertical blanking period occurs on the display
    ///associated with the surface.
    SurfaceCounterVblankExt = 1,
}
impl const Default for SurfaceCounterFlagBitsEXT {
    fn default() -> Self {
        Self::Empty
    }
}
impl SurfaceCounterFlagBitsEXT {
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
///[VkSurfaceCounterFlagBitsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCounterFlagBitsEXT.html) - Surface-relative counter types
///# C Specifications
///Bits which  **can**  be set in
///[`SurfaceCapabilities2EXT::supported_surface_counters`], indicating
///supported surface counter types, are:
///```c
///// Provided by VK_EXT_display_surface_counter
///typedef enum VkSurfaceCounterFlagBitsEXT {
///    VK_SURFACE_COUNTER_VBLANK_BIT_EXT = 0x00000001,
///    VK_SURFACE_COUNTER_VBLANK_EXT = VK_SURFACE_COUNTER_VBLANK_BIT_EXT,
///} VkSurfaceCounterFlagBitsEXT;
///```
///# Description
/// - [`SurfaceCounterVblankExt`] specifies a counter incrementing once every time a vertical
///   blanking period occurs on the display associated with the surface.
///# Related
/// - [`VK_EXT_display_surface_counter`]
/// - [`SurfaceCounterFlagsEXT`]
/// - [`GetSwapchainCounterEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceCounterFlagsEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct SurfaceCounterFlagsEXT(u32);
impl const Default for SurfaceCounterFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl From<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn from(from: SurfaceCounterFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl SurfaceCounterFlagsEXT {
    ///[`SurfaceCounterVblankExt`] specifies a counter incrementing
    ///once every time a vertical blanking period occurs on the display
    ///associated with the surface.
    pub const SURFACE_COUNTER_VBLANK_EXT: Self = Self(1);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::SURFACE_COUNTER_VBLANK_EXT
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
impl const std::ops::BitOr for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for SurfaceCounterFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for SurfaceCounterFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<SurfaceCounterFlagsEXT> for SurfaceCounterFlagsEXT {
    fn extend<T: IntoIterator<Item = SurfaceCounterFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn extend<T: IntoIterator<Item = SurfaceCounterFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<SurfaceCounterFlagBitsEXT>>::from(i));
        }
    }
}
impl FromIterator<SurfaceCounterFlagsEXT> for SurfaceCounterFlagsEXT {
    fn from_iter<T: IntoIterator<Item = SurfaceCounterFlagsEXT>>(iterator: T) -> SurfaceCounterFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<SurfaceCounterFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn from_iter<T: IntoIterator<Item = SurfaceCounterFlagBitsEXT>>(iterator: T) -> SurfaceCounterFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<SurfaceCounterFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for SurfaceCounterFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(SurfaceCounterFlagsEXT);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == SurfaceCounterFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(SurfaceCounterFlagsEXT::SURFACE_COUNTER_VBLANK_EXT) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(SURFACE_COUNTER_VBLANK_EXT))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(SurfaceCounterFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkSurfaceCapabilities2EXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSurfaceCapabilities2EXT.html) - Structure describing capabilities of a surface
///# C Specifications
///The [`SurfaceCapabilities2EXT`] structure is defined as:
///```c
///// Provided by VK_EXT_display_surface_counter
///typedef struct VkSurfaceCapabilities2EXT {
///    VkStructureType                  sType;
///    void*                            pNext;
///    uint32_t                         minImageCount;
///    uint32_t                         maxImageCount;
///    VkExtent2D                       currentExtent;
///    VkExtent2D                       minImageExtent;
///    VkExtent2D                       maxImageExtent;
///    uint32_t                         maxImageArrayLayers;
///    VkSurfaceTransformFlagsKHR       supportedTransforms;
///    VkSurfaceTransformFlagBitsKHR    currentTransform;
///    VkCompositeAlphaFlagsKHR         supportedCompositeAlpha;
///    VkImageUsageFlags                supportedUsageFlags;
///    VkSurfaceCounterFlagsEXT         supportedSurfaceCounters;
///} VkSurfaceCapabilities2EXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`min_image_count`] is the minimum number of images the specified device supports for a
///   swapchain created for the surface, and will be at least one.
/// - [`max_image_count`] is the maximum number of images the specified device supports for a
///   swapchain created for the surface, and will be either 0, or greater than or equal to
///   [`min_image_count`]. A value of 0 means that there is no limit on the number of images, though
///   there  **may**  be limits related to the total amount of memory used by presentable images.
/// - [`current_extent`] is the current width and height of the surface, or the special value
///   (0xFFFFFFFF, 0xFFFFFFFF) indicating that the surface size will be determined by the extent of
///   a swapchain targeting the surface.
/// - [`min_image_extent`] contains the smallest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be less than or equal to
///   the corresponding `width` and `height` of [`current_extent`], unless [`current_extent`] has
///   the special value described above.
/// - [`max_image_extent`] contains the largest valid swapchain extent for the surface on the
///   specified device. The `width` and `height` of the extent will each be greater than or equal to
///   the corresponding `width` and `height` of [`min_image_extent`]. The `width` and `height` of
///   the extent will each be greater than or equal to the corresponding `width` and `height` of
///   [`current_extent`], unless [`current_extent`] has the special value described above.
/// - [`max_image_array_layers`] is the maximum number of layers presentable images  **can**  have
///   for a swapchain created for this device and surface, and will be at least one.
/// - [`supported_transforms`] is a bitmask of [`SurfaceTransformFlagBitsKHR`] indicating the
///   presentation transforms supported for the surface on the specified device. At least one bit
///   will be set.
/// - [`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value indicating the surface’s
///   current transform relative to the presentation engine’s natural orientation.
/// - [`supported_composite_alpha`] is a bitmask of [`CompositeAlphaFlagBitsKHR`], representing the
///   alpha compositing modes supported by the presentation engine for the surface on the specified
///   device, and at least one bit will be set. Opaque composition  **can**  be achieved in any
///   alpha compositing mode by either using an image format that has no alpha component, or by
///   ensuring that all pixels in the presentable images have an alpha value of 1.0.
/// - [`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`] representing the ways the
///   application  **can**  use the presentable images of a swapchain created with
///   [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`, `VK_PRESENT_MODE_MAILBOX_KHR`,
///   `VK_PRESENT_MODE_FIFO_KHR` or `VK_PRESENT_MODE_FIFO_RELAXED_KHR` for the surface on the
///   specified device. `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set.
///   Implementations  **may**  support additional usages.
/// - [`supported_surface_counters`] is a bitmask of [`SurfaceCounterFlagBitsEXT`] indicating the
///   supported surface counter types.
///# Description
///## Valid Usage
/// -  [`supported_surface_counters`] **must**  not include `VK_SURFACE_COUNTER_VBLANK_BIT_EXT` unless the surface queried is a [display surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#wsi-display-surfaces)
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_EXT_display_surface_counter`]
/// - [`CompositeAlphaFlagsKHR`]
/// - [`Extent2D`]
/// - [`ImageUsageFlags`]
/// - [`StructureType`]
/// - [`SurfaceCounterFlagsEXT`]
/// - [`SurfaceTransformFlagBitsKHR`]
/// - [`SurfaceTransformFlagsKHR`]
/// - [`GetPhysicalDeviceSurfaceCapabilities2EXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkSurfaceCapabilities2EXT")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct SurfaceCapabilities2EXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`min_image_count`] is the minimum number of images the specified device
    ///supports for a swapchain created for the surface, and will be at least
    ///one.
    pub min_image_count: u32,
    ///[`max_image_count`] is the maximum number of images the specified device
    ///supports for a swapchain created for the surface, and will be either 0,
    ///or greater than or equal to [`min_image_count`].
    ///A value of 0 means that there is no limit on the number of images,
    ///though there  **may**  be limits related to the total amount of memory used
    ///by presentable images.
    pub max_image_count: u32,
    ///[`current_extent`] is the current width and height of the surface, or
    ///the special value (0xFFFFFFFF, 0xFFFFFFFF) indicating that the
    ///surface size will be determined by the extent of a swapchain targeting
    ///the surface.
    pub current_extent: Extent2D,
    ///[`min_image_extent`] contains the smallest valid swapchain extent for
    ///the surface on the specified device.
    ///The `width` and `height` of the extent will each be less than or
    ///equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    pub min_image_extent: Extent2D,
    ///[`max_image_extent`] contains the largest valid swapchain extent for the
    ///surface on the specified device.
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`min_image_extent`].
    ///The `width` and `height` of the extent will each be greater than
    ///or equal to the corresponding `width` and `height` of
    ///[`current_extent`], unless [`current_extent`] has the special value
    ///described above.
    pub max_image_extent: Extent2D,
    ///[`max_image_array_layers`] is the maximum number of layers presentable
    ///images  **can**  have for a swapchain created for this device and surface,
    ///and will be at least one.
    pub max_image_array_layers: u32,
    ///[`supported_transforms`] is a bitmask of
    ///[`SurfaceTransformFlagBitsKHR`] indicating the presentation
    ///transforms supported for the surface on the specified device.
    ///At least one bit will be set.
    pub supported_transforms: SurfaceTransformFlagsKHR,
    ///[`current_transform`] is [`SurfaceTransformFlagBitsKHR`] value
    ///indicating the surface’s current transform relative to the presentation
    ///engine’s natural orientation.
    pub current_transform: SurfaceTransformFlagBitsKHR,
    ///[`supported_composite_alpha`] is a bitmask of
    ///[`CompositeAlphaFlagBitsKHR`], representing the alpha compositing
    ///modes supported by the presentation engine for the surface on the
    ///specified device, and at least one bit will be set.
    ///Opaque composition  **can**  be achieved in any alpha compositing mode by
    ///either using an image format that has no alpha component, or by ensuring
    ///that all pixels in the presentable images have an alpha value of 1.0.
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    ///[`supported_usage_flags`] is a bitmask of [`ImageUsageFlagBits`]
    ///representing the ways the application  **can**  use the presentable images of
    ///a swapchain created
    ///with [`PresentModeKHR`] set to `VK_PRESENT_MODE_IMMEDIATE_KHR`,
    ///`VK_PRESENT_MODE_MAILBOX_KHR`, `VK_PRESENT_MODE_FIFO_KHR` or
    ///`VK_PRESENT_MODE_FIFO_RELAXED_KHR`
    ///for the surface on the specified device.
    ///`VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` **must**  be included in the set.
    ///Implementations  **may**  support additional usages.
    pub supported_usage_flags: ImageUsageFlags,
    ///[`supported_surface_counters`] is a bitmask of
    ///[`SurfaceCounterFlagBitsEXT`] indicating the supported surface
    ///counter types.
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}
impl<'lt> Default for SurfaceCapabilities2EXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::SurfaceCapabilities2Ext,
            p_next: std::ptr::null_mut(),
            min_image_count: 0,
            max_image_count: 0,
            current_extent: Default::default(),
            min_image_extent: Default::default(),
            max_image_extent: Default::default(),
            max_image_array_layers: 0,
            supported_transforms: Default::default(),
            current_transform: Default::default(),
            supported_composite_alpha: Default::default(),
            supported_usage_flags: Default::default(),
            supported_surface_counters: Default::default(),
        }
    }
}
impl<'lt> SurfaceCapabilities2EXT<'lt> {
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
    ///Gets the value of [`Self::min_image_count`]
    pub fn min_image_count(&self) -> u32 {
        self.min_image_count
    }
    ///Gets the value of [`Self::max_image_count`]
    pub fn max_image_count(&self) -> u32 {
        self.max_image_count
    }
    ///Gets the value of [`Self::current_extent`]
    pub fn current_extent(&self) -> Extent2D {
        self.current_extent
    }
    ///Gets the value of [`Self::min_image_extent`]
    pub fn min_image_extent(&self) -> Extent2D {
        self.min_image_extent
    }
    ///Gets the value of [`Self::max_image_extent`]
    pub fn max_image_extent(&self) -> Extent2D {
        self.max_image_extent
    }
    ///Gets the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers(&self) -> u32 {
        self.max_image_array_layers
    }
    ///Gets the value of [`Self::supported_transforms`]
    pub fn supported_transforms(&self) -> SurfaceTransformFlagsKHR {
        self.supported_transforms
    }
    ///Gets the value of [`Self::current_transform`]
    pub fn current_transform(&self) -> SurfaceTransformFlagBitsKHR {
        self.current_transform
    }
    ///Gets the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha(&self) -> CompositeAlphaFlagsKHR {
        self.supported_composite_alpha
    }
    ///Gets the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags(&self) -> ImageUsageFlags {
        self.supported_usage_flags
    }
    ///Gets the value of [`Self::supported_surface_counters`]
    pub fn supported_surface_counters(&self) -> SurfaceCounterFlagsEXT {
        self.supported_surface_counters
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
    ///Gets a mutable reference to the value of [`Self::min_image_count`]
    pub fn min_image_count_mut(&mut self) -> &mut u32 {
        &mut self.min_image_count
    }
    ///Gets a mutable reference to the value of [`Self::max_image_count`]
    pub fn max_image_count_mut(&mut self) -> &mut u32 {
        &mut self.max_image_count
    }
    ///Gets a mutable reference to the value of [`Self::current_extent`]
    pub fn current_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.current_extent
    }
    ///Gets a mutable reference to the value of [`Self::min_image_extent`]
    pub fn min_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.min_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_extent`]
    pub fn max_image_extent_mut(&mut self) -> &mut Extent2D {
        &mut self.max_image_extent
    }
    ///Gets a mutable reference to the value of [`Self::max_image_array_layers`]
    pub fn max_image_array_layers_mut(&mut self) -> &mut u32 {
        &mut self.max_image_array_layers
    }
    ///Gets a mutable reference to the value of [`Self::supported_transforms`]
    pub fn supported_transforms_mut(&mut self) -> &mut SurfaceTransformFlagsKHR {
        &mut self.supported_transforms
    }
    ///Gets a mutable reference to the value of [`Self::current_transform`]
    pub fn current_transform_mut(&mut self) -> &mut SurfaceTransformFlagBitsKHR {
        &mut self.current_transform
    }
    ///Gets a mutable reference to the value of [`Self::supported_composite_alpha`]
    pub fn supported_composite_alpha_mut(&mut self) -> &mut CompositeAlphaFlagsKHR {
        &mut self.supported_composite_alpha
    }
    ///Gets a mutable reference to the value of [`Self::supported_usage_flags`]
    pub fn supported_usage_flags_mut(&mut self) -> &mut ImageUsageFlags {
        &mut self.supported_usage_flags
    }
    ///Gets a mutable reference to the value of [`Self::supported_surface_counters`]
    pub fn supported_surface_counters_mut(&mut self) -> &mut SurfaceCounterFlagsEXT {
        &mut self.supported_surface_counters
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
    ///Sets the raw value of [`Self::min_image_count`]
    pub fn set_min_image_count(&mut self, value: u32) -> &mut Self {
        self.min_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_count`]
    pub fn set_max_image_count(&mut self, value: u32) -> &mut Self {
        self.max_image_count = value;
        self
    }
    ///Sets the raw value of [`Self::current_extent`]
    pub fn set_current_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.current_extent = value;
        self
    }
    ///Sets the raw value of [`Self::min_image_extent`]
    pub fn set_min_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.min_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_extent`]
    pub fn set_max_image_extent(&mut self, value: crate::vulkan1_0::Extent2D) -> &mut Self {
        self.max_image_extent = value;
        self
    }
    ///Sets the raw value of [`Self::max_image_array_layers`]
    pub fn set_max_image_array_layers(&mut self, value: u32) -> &mut Self {
        self.max_image_array_layers = value;
        self
    }
    ///Sets the raw value of [`Self::supported_transforms`]
    pub fn set_supported_transforms(
        &mut self,
        value: crate::extensions::khr_display::SurfaceTransformFlagsKHR,
    ) -> &mut Self {
        self.supported_transforms = value;
        self
    }
    ///Sets the raw value of [`Self::current_transform`]
    pub fn set_current_transform(
        &mut self,
        value: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> &mut Self {
        self.current_transform = value;
        self
    }
    ///Sets the raw value of [`Self::supported_composite_alpha`]
    pub fn set_supported_composite_alpha(
        &mut self,
        value: crate::extensions::khr_surface::CompositeAlphaFlagsKHR,
    ) -> &mut Self {
        self.supported_composite_alpha = value;
        self
    }
    ///Sets the raw value of [`Self::supported_usage_flags`]
    pub fn set_supported_usage_flags(&mut self, value: crate::vulkan1_0::ImageUsageFlags) -> &mut Self {
        self.supported_usage_flags = value;
        self
    }
    ///Sets the raw value of [`Self::supported_surface_counters`]
    pub fn set_supported_surface_counters(
        &mut self,
        value: crate::extensions::ext_display_surface_counter::SurfaceCounterFlagsEXT,
    ) -> &mut Self {
        self.supported_surface_counters = value;
        self
    }
}
