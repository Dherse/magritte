//!# [VK_EXT_conditional_rendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_conditional_rendering.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/VK_EXT_conditional_rendering.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, DeviceSize, StructureType},
};
use std::ffi::CStr;
///# [VkConditionalRenderingBeginInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingBeginInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/VkConditionalRenderingBeginInfoEXT.md")]
#[doc(alias = "VkConditionalRenderingBeginInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    buffer: Buffer,
    offset: DeviceSize,
    flags: ConditionalRenderingFlagsEXT,
}
///# [VkCommandBufferInheritanceConditionalRenderingInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceConditionalRenderingInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/VkCommandBufferInheritanceConditionalRenderingInfoEXT.md")]
#[doc(alias = "VkCommandBufferInheritanceConditionalRenderingInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "conditionalRenderingEnable")]
    conditional_rendering_enable: Bool32,
}
///# [VkPhysicalDeviceConditionalRenderingFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceConditionalRenderingFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/VkPhysicalDeviceConditionalRenderingFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceConditionalRenderingFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "conditionalRendering")]
    conditional_rendering: Bool32,
    #[doc(alias = "inheritedConditionalRendering")]
    inherited_conditional_rendering: Bool32,
}
///# [VkConditionalRenderingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/VkConditionalRenderingFlagBitsEXT.md")]
#[doc(alias = "VkConditionalRenderingFlagsEXT")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConditionalRenderingFlagsEXT(u32);
impl ConditionalRenderingFlagsEXT {
    #[doc(alias = "VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT")]
    pub const INVERTED: Self = Self(1);
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
        {
            all |= Self::INVERTED;
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
impl const std::ops::BitOr for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ConditionalRenderingFlagsEXT {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ConditionalRenderingFlagsEXT {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ConditionalRenderingFlagsEXT> for ConditionalRenderingFlagsEXT {
    fn extend<T: IntoIterator<Item = ConditionalRenderingFlagsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ConditionalRenderingFlagsEXT> for ConditionalRenderingFlagsEXT {
    fn from_iter<T: IntoIterator<Item = ConditionalRenderingFlagsEXT>>(iterator: T) -> ConditionalRenderingFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<ConditionalRenderingFlagsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ConditionalRenderingFlagsEXT {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn from(bit: ConditionalRenderingFlagBitsEXT) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn extend<T: IntoIterator<Item = ConditionalRenderingFlagBitsEXT>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn from_iter<T: IntoIterator<Item = ConditionalRenderingFlagBitsEXT>>(iterator: T) -> ConditionalRenderingFlagsEXT {
        let mut out = Self::empty();
        <Self as Extend<ConditionalRenderingFlagBitsEXT>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ConditionalRenderingFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ConditionalRenderingFlagsEXT);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ConditionalRenderingFlagsEXT::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(ConditionalRenderingFlagsEXT::INVERTED) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(INVERTED))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ConditionalRenderingFlagsEXT))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION")]
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME")]
pub const EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_conditional_rendering");
///# [VkConditionalRenderingFlagBitsEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkConditionalRenderingFlagBitsEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/VkConditionalRenderingFlagBitsEXT.md")]
#[doc(alias = "VkConditionalRenderingFlagBitsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ConditionalRenderingFlagBitsEXT(u32);
impl ConditionalRenderingFlagBitsEXT {
    #[doc(alias = "VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT")]
    pub const INVERTED: Self = Self(1);
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
            x if x == Self::INVERTED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [vkCmdBeginConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginConditionalRenderingEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/vkCmdBeginConditionalRenderingEXT.md")]
#[doc(alias = "vkCmdBeginConditionalRenderingEXT")]
pub type FNCmdBeginConditionalRenderingExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
);
///# [vkCmdEndConditionalRenderingEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndConditionalRenderingEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_conditional_rendering/vkCmdEndConditionalRenderingEXT.md")]
#[doc(alias = "vkCmdEndConditionalRenderingEXT")]
pub type FNCmdEndConditionalRenderingExt = unsafe extern "system" fn(command_buffer: CommandBuffer);
