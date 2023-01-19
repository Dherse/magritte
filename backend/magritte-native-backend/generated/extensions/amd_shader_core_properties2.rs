//!# [VK_AMD_shader_core_properties2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties2.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_shader_core_properties2/VK_AMD_shader_core_properties2.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceShaderCoreProperties2AMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_shader_core_properties2/VkPhysicalDeviceShaderCoreProperties2AMD.md")]
#[doc(alias = "VkPhysicalDeviceShaderCoreProperties2AMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderCoreFeatures")]
    shader_core_features: ShaderCorePropertiesFlagsAMD,
    #[doc(alias = "activeComputeUnitCount")]
    active_compute_unit_count: u32,
}
///# [VkShaderCorePropertiesFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_shader_core_properties2/VkShaderCorePropertiesFlagBitsAMD.md")]
#[doc(alias = "VkShaderCorePropertiesFlagsAMD")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShaderCorePropertiesFlagsAMD(u32);
impl ShaderCorePropertiesFlagsAMD {
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
impl const std::ops::BitOr for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ShaderCorePropertiesFlagsAMD {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ShaderCorePropertiesFlagsAMD {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ShaderCorePropertiesFlagsAMD {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ShaderCorePropertiesFlagsAMD {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ShaderCorePropertiesFlagsAMD> for ShaderCorePropertiesFlagsAMD {
    fn extend<T: IntoIterator<Item = ShaderCorePropertiesFlagsAMD>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<ShaderCorePropertiesFlagsAMD> for ShaderCorePropertiesFlagsAMD {
    fn from_iter<T: IntoIterator<Item = ShaderCorePropertiesFlagsAMD>>(iterator: T) -> ShaderCorePropertiesFlagsAMD {
        let mut out = Self::empty();
        <Self as Extend<ShaderCorePropertiesFlagsAMD>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for ShaderCorePropertiesFlagsAMD {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    fn from(bit: ShaderCorePropertiesFlagBitsAMD) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    fn extend<T: IntoIterator<Item = ShaderCorePropertiesFlagBitsAMD>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    fn from_iter<T: IntoIterator<Item = ShaderCorePropertiesFlagBitsAMD>>(iterator: T) -> ShaderCorePropertiesFlagsAMD {
        let mut out = Self::empty();
        <Self as Extend<ShaderCorePropertiesFlagBitsAMD>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ShaderCorePropertiesFlagsAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ShaderCorePropertiesFlagsAMD);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ShaderCorePropertiesFlagsAMD::empty() {
                    f.write_str("empty")?;
                } else {
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ShaderCorePropertiesFlagsAMD))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_shader_core_properties2");
///# [VkShaderCorePropertiesFlagBitsAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_shader_core_properties2/VkShaderCorePropertiesFlagBitsAMD.md")]
#[doc(alias = "VkShaderCorePropertiesFlagBitsAMD")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ShaderCorePropertiesFlagBitsAMD(u32);
impl ShaderCorePropertiesFlagBitsAMD {
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
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
