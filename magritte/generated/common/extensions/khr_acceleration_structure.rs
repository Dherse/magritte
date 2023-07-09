use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkAccelerationStructureBuildRangeInfoKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureBuildRangeInfoKHR {
    #[doc(alias = "primitiveCount")]
    pub primitive_count: u32,
    #[doc(alias = "primitiveOffset")]
    pub primitive_offset: u32,
    #[doc(alias = "firstVertex")]
    pub first_vertex: u32,
    #[doc(alias = "transformOffset")]
    pub transform_offset: u32,
}
#[doc(alias = "VkAabbPositionsKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AabbPositionsKHR {
    #[doc(alias = "minX")]
    pub min_x: f32,
    #[doc(alias = "minY")]
    pub min_y: f32,
    #[doc(alias = "minZ")]
    pub min_z: f32,
    #[doc(alias = "maxX")]
    pub max_x: f32,
    #[doc(alias = "maxY")]
    pub max_y: f32,
    #[doc(alias = "maxZ")]
    pub max_z: f32,
}
#[doc(alias = "VkTransformMatrixKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TransformMatrixKHR {
    pub matrix: [f32; 3 as usize],
}
#[doc(alias = "VkAccelerationStructureInstanceKHR")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    pub transform: TransformMatrixKHR,
    #[doc(alias = "instanceCustomIndex")]
    pub instance_custom_index: u32,
    pub mask: u32,
    #[doc(alias = "instanceShaderBindingTableRecordOffset")]
    pub instance_shader_binding_table_record_offset: u32,
    pub flags: GeometryInstanceFlagsKHR,
    #[doc(alias = "accelerationStructureReference")]
    pub acceleration_structure_reference: u64,
}
#[doc(alias = "VkGeometryFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryFlagsKHR(u32);
impl GeometryFlagsKHR {
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const OPAQUE_NV: Self = Self::OPAQUE;
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
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
            all |= Self::OPAQUE;
        }
        {
            all |= Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::OPAQUE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::NO_DUPLICATE_ANY_HIT_INVOCATION_NV;
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
impl std::ops::BitOr for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for GeometryFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for GeometryFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for GeometryFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for GeometryFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<GeometryFlagsKHR> for GeometryFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<GeometryFlagsKHR> for GeometryFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryFlagsKHR>>(iterator: T) -> GeometryFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for GeometryFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn from(bit: GeometryFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryFlagBitsKHR>>(iterator: T) -> GeometryFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for GeometryFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(GeometryFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == GeometryFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(GeometryFlagsKHR::OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE))?;
                    }
                    if self.0.contains(GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NO_DUPLICATE_ANY_HIT_INVOCATION))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryFlagsKHR::OPAQUE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(OPAQUE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryFlagsKHR::NO_DUPLICATE_ANY_HIT_INVOCATION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(NO_DUPLICATE_ANY_HIT_INVOCATION_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(GeometryFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryFlagsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkGeometryInstanceFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeometryInstanceFlagsKHR(u32);
impl GeometryInstanceFlagsKHR {
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR")]
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR")]
    pub const TRIANGLE_FLIP_FACING: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR")]
    pub const FORCE_OPAQUE: Self = Self(4);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR")]
    pub const FORCE_NO_OPAQUE: Self = Self(8);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FLIP_FACING;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE;
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
            all |= Self::TRIANGLE_FACING_CULL_DISABLE;
        }
        {
            all |= Self::TRIANGLE_FLIP_FACING;
        }
        {
            all |= Self::FORCE_OPAQUE;
        }
        {
            all |= Self::FORCE_NO_OPAQUE;
        }
        {
            all |= Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::TRIANGLE_CULL_DISABLE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::TRIANGLE_FRONT_COUNTERCLOCKWISE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::FORCE_OPAQUE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::FORCE_NO_OPAQUE_NV;
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
impl std::ops::BitOr for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<GeometryInstanceFlagsKHR> for GeometryInstanceFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryInstanceFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<GeometryInstanceFlagsKHR> for GeometryInstanceFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryInstanceFlagsKHR>>(iterator: T) -> GeometryInstanceFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryInstanceFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for GeometryInstanceFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn from(bit: GeometryInstanceFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryInstanceFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryInstanceFlagBitsKHR>>(iterator: T) -> GeometryInstanceFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryInstanceFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for GeometryInstanceFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(GeometryInstanceFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == GeometryInstanceFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FACING_CULL_DISABLE))?;
                    }
                    if self.0.contains(GeometryInstanceFlagsKHR::TRIANGLE_FLIP_FACING) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FLIP_FACING))?;
                    }
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_OPAQUE))?;
                    }
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_NO_OPAQUE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_NO_OPAQUE))?;
                    }
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::TRIANGLE_FRONT_COUNTERCLOCKWISE)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FRONT_COUNTERCLOCKWISE))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryInstanceFlagsKHR::TRIANGLE_CULL_DISABLE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_CULL_DISABLE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::TRIANGLE_FRONT_COUNTERCLOCKWISE_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(TRIANGLE_FRONT_COUNTERCLOCKWISE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_OPAQUE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_OPAQUE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(GeometryInstanceFlagsKHR::FORCE_NO_OPAQUE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(FORCE_NO_OPAQUE_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(GeometryInstanceFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryInstanceFlagsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryInstanceFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkBuildAccelerationStructureFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuildAccelerationStructureFlagsKHR(u32);
impl BuildAccelerationStructureFlagsKHR {
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR")]
    pub const ALLOW_UPDATE: Self = Self(1);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR")]
    pub const ALLOW_COMPACTION: Self = Self(2);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR")]
    pub const PREFER_FAST_TRACE: Self = Self(4);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR")]
    pub const PREFER_FAST_BUILD: Self = Self(8);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR")]
    pub const LOW_MEMORY: Self = Self(16);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(32);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY;
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
            all |= Self::ALLOW_UPDATE;
        }
        {
            all |= Self::ALLOW_COMPACTION;
        }
        {
            all |= Self::PREFER_FAST_TRACE;
        }
        {
            all |= Self::PREFER_FAST_BUILD;
        }
        {
            all |= Self::LOW_MEMORY;
        }
        #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
        {
            all |= Self::MOTION_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::ALLOW_UPDATE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::ALLOW_COMPACTION_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::PREFER_FAST_TRACE_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::PREFER_FAST_BUILD_NV;
        }
        #[cfg(feature = "VK_NV_ray_tracing")]
        {
            all |= Self::LOW_MEMORY_NV;
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
impl std::ops::BitOr for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<BuildAccelerationStructureFlagsKHR> for BuildAccelerationStructureFlagsKHR {
    fn extend<T: IntoIterator<Item = BuildAccelerationStructureFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<BuildAccelerationStructureFlagsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from_iter<T: IntoIterator<Item = BuildAccelerationStructureFlagsKHR>>(
        iterator: T,
    ) -> BuildAccelerationStructureFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<BuildAccelerationStructureFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for BuildAccelerationStructureFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from(bit: BuildAccelerationStructureFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn extend<T: IntoIterator<Item = BuildAccelerationStructureFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from_iter<T: IntoIterator<Item = BuildAccelerationStructureFlagBitsKHR>>(
        iterator: T,
    ) -> BuildAccelerationStructureFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<BuildAccelerationStructureFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for BuildAccelerationStructureFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(BuildAccelerationStructureFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == BuildAccelerationStructureFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_UPDATE))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_COMPACTION))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_TRACE))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_BUILD) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_BUILD))?;
                    }
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::LOW_MEMORY) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOW_MEMORY))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::MOTION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MOTION_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_UPDATE_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_UPDATE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::ALLOW_COMPACTION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(ALLOW_COMPACTION_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_TRACE_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::PREFER_FAST_BUILD_NV)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PREFER_FAST_BUILD_NV))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing")]
                    if self.0.contains(BuildAccelerationStructureFlagsKHR::LOW_MEMORY_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(LOW_MEMORY_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(BuildAccelerationStructureFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BuildAccelerationStructureFlagsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BuildAccelerationStructureFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccelerationStructureCreateFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AccelerationStructureCreateFlagsKHR(u32);
impl AccelerationStructureCreateFlagsKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(4);
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
            all |= Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
        }
        #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
        {
            all |= Self::MOTION_NV;
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
impl std::ops::BitOr for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl std::ops::BitXor for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl std::ops::BitAnd for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl std::ops::Sub for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Not for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<AccelerationStructureCreateFlagsKHR> for AccelerationStructureCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = AccelerationStructureCreateFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<AccelerationStructureCreateFlagsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AccelerationStructureCreateFlagsKHR>>(
        iterator: T,
    ) -> AccelerationStructureCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AccelerationStructureCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl Default for AccelerationStructureCreateFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl From<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from(bit: AccelerationStructureCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = AccelerationStructureCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AccelerationStructureCreateFlagBitsKHR>>(
        iterator: T,
    ) -> AccelerationStructureCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AccelerationStructureCreateFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for AccelerationStructureCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(AccelerationStructureCreateFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == AccelerationStructureCreateFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(AccelerationStructureCreateFlagsKHR::DEVICE_ADDRESS_CAPTURE_REPLAY)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(DEVICE_ADDRESS_CAPTURE_REPLAY))?;
                    }
                    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
                    if self.0.contains(AccelerationStructureCreateFlagsKHR::MOTION_NV) {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(MOTION_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(AccelerationStructureCreateFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureCreateFlagsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureCreateFlagsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_acceleration_structure");
#[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct GeometryInstanceFlagBitsKHR(u32);
impl Default for GeometryInstanceFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl GeometryInstanceFlagBitsKHR {
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR")]
    pub const TRIANGLE_FACING_CULL_DISABLE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR")]
    pub const TRIANGLE_FLIP_FACING: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR")]
    pub const FORCE_OPAQUE: Self = Self(4);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR")]
    pub const FORCE_NO_OPAQUE: Self = Self(8);
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE: Self = Self::TRIANGLE_FLIP_FACING;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE;
    #[doc(alias = "VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE;
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
            x if x == Self::TRIANGLE_FACING_CULL_DISABLE.bits() => Some(Self(x)),
            x if x == Self::TRIANGLE_FLIP_FACING.bits() => Some(Self(x)),
            x if x == Self::FORCE_OPAQUE.bits() => Some(Self(x)),
            x if x == Self::FORCE_NO_OPAQUE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryInstanceFlagBitsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryInstanceFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkGeometryFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct GeometryFlagBitsKHR(u32);
impl Default for GeometryFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl GeometryFlagBitsKHR {
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_KHR")]
    pub const OPAQUE: Self = Self(1);
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION: Self = Self(2);
    #[doc(alias = "VK_GEOMETRY_OPAQUE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const OPAQUE_NV: Self = Self::OPAQUE;
    #[doc(alias = "VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
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
            x if x == Self::OPAQUE.bits() => Some(Self(x)),
            x if x == Self::NO_DUPLICATE_ANY_HIT_INVOCATION.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryFlagBitsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct BuildAccelerationStructureFlagBitsKHR(u32);
impl Default for BuildAccelerationStructureFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl BuildAccelerationStructureFlagBitsKHR {
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR")]
    pub const ALLOW_UPDATE: Self = Self(1);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR")]
    pub const ALLOW_COMPACTION: Self = Self(2);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR")]
    pub const PREFER_FAST_TRACE: Self = Self(4);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR")]
    pub const PREFER_FAST_BUILD: Self = Self(8);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR")]
    pub const LOW_MEMORY: Self = Self(16);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(32);
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD;
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing")]
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY;
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
            x if x == Self::ALLOW_UPDATE.bits() => Some(Self(x)),
            x if x == Self::ALLOW_COMPACTION.bits() => Some(Self(x)),
            x if x == Self::PREFER_FAST_TRACE.bits() => Some(Self(x)),
            x if x == Self::PREFER_FAST_BUILD.bits() => Some(Self(x)),
            x if x == Self::LOW_MEMORY.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
            x if x == Self::MOTION_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BuildAccelerationStructureFlagBitsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BuildAccelerationStructureFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AccelerationStructureCreateFlagBitsKHR(u32);
impl Default for AccelerationStructureCreateFlagBitsKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AccelerationStructureCreateFlagBitsKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR")]
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1);
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV")]
    #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
    pub const MOTION_NV: Self = Self(4);
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
            x if x == Self::DEVICE_ADDRESS_CAPTURE_REPLAY.bits() => Some(Self(x)),
            #[cfg(feature = "VK_NV_ray_tracing_motion_blur")]
            x if x == Self::MOTION_NV.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureCreateFlagBitsKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureCreateFlagBitsKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkCopyAccelerationStructureModeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum CopyAccelerationStructureModeKHR {
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR")]
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV")]
    Clone = 0,
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR")]
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV")]
    Compact = 1,
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR")]
    Serialize = 2,
    #[doc(alias = "VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR")]
    Deserialize = 3,
}
impl Default for CopyAccelerationStructureModeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl CopyAccelerationStructureModeKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Clone.bits() => Some(Self::Clone),
            x if x == Self::Compact.bits() => Some(Self::Compact),
            x if x == Self::Serialize.bits() => Some(Self::Serialize),
            x if x == Self::Deserialize.bits() => Some(Self::Deserialize),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for CopyAccelerationStructureModeKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for CopyAccelerationStructureModeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkBuildAccelerationStructureModeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum BuildAccelerationStructureModeKHR {
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR")]
    Build = 0,
    #[doc(alias = "VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR")]
    Update = 1,
}
impl Default for BuildAccelerationStructureModeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl BuildAccelerationStructureModeKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Build.bits() => Some(Self::Build),
            x if x == Self::Update.bits() => Some(Self::Update),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for BuildAccelerationStructureModeKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for BuildAccelerationStructureModeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccelerationStructureTypeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum AccelerationStructureTypeKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR")]
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV")]
    TopLevel = 0,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR")]
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV")]
    BottomLevel = 1,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR")]
    Generic = 2,
}
impl Default for AccelerationStructureTypeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AccelerationStructureTypeKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::TopLevel.bits() => Some(Self::TopLevel),
            x if x == Self::BottomLevel.bits() => Some(Self::BottomLevel),
            x if x == Self::Generic.bits() => Some(Self::Generic),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureTypeKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureTypeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkGeometryTypeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum GeometryTypeKHR {
    #[doc(alias = "VK_GEOMETRY_TYPE_TRIANGLES_KHR")]
    #[doc(alias = "VK_GEOMETRY_TYPE_TRIANGLES_NV")]
    Triangles = 0,
    #[doc(alias = "VK_GEOMETRY_TYPE_AABBS_KHR")]
    #[doc(alias = "VK_GEOMETRY_TYPE_AABBS_NV")]
    Aabbs = 1,
    #[doc(alias = "VK_GEOMETRY_TYPE_INSTANCES_KHR")]
    Instances = 2,
}
impl Default for GeometryTypeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl GeometryTypeKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Triangles.bits() => Some(Self::Triangles),
            x if x == Self::Aabbs.bits() => Some(Self::Aabbs),
            x if x == Self::Instances.bits() => Some(Self::Instances),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for GeometryTypeKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for GeometryTypeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccelerationStructureBuildTypeKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum AccelerationStructureBuildTypeKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR")]
    Host = 0,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR")]
    Device = 1,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR")]
    HostOrDevice = 2,
}
impl Default for AccelerationStructureBuildTypeKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AccelerationStructureBuildTypeKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Host.bits() => Some(Self::Host),
            x if x == Self::Device.bits() => Some(Self::Device),
            x if x == Self::HostOrDevice.bits() => Some(Self::HostOrDevice),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureBuildTypeKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureBuildTypeKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkAccelerationStructureCompatibilityKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum AccelerationStructureCompatibilityKHR {
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR")]
    Compatible = 0,
    #[doc(alias = "VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR")]
    Incompatible = 1,
}
impl Default for AccelerationStructureCompatibilityKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl AccelerationStructureCompatibilityKHR {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::Compatible.bits() => Some(Self::Compatible),
            x if x == Self::Incompatible.bits() => Some(Self::Incompatible),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for AccelerationStructureCompatibilityKHR {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for AccelerationStructureCompatibilityKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
