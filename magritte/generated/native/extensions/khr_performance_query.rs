//!# [VK_KHR_performance_query](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VK_KHR_performance_query.md")]
use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Device, PhysicalDevice, StructureType, VulkanResultCodes,
        MAX_DESCRIPTION_SIZE, UUID_SIZE,
    },
};
use std::ffi::CStr;
///# [VkPhysicalDevicePerformanceQueryFeaturesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryFeaturesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPhysicalDevicePerformanceQueryFeaturesKHR.md")]
#[doc(alias = "VkPhysicalDevicePerformanceQueryFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "performanceCounterQueryPools")]
    performance_counter_query_pools: Bool32,
    #[doc(alias = "performanceCounterMultipleQueryPools")]
    performance_counter_multiple_query_pools: Bool32,
}
///# [VkPhysicalDevicePerformanceQueryPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePerformanceQueryPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPhysicalDevicePerformanceQueryPropertiesKHR.md")]
#[doc(alias = "VkPhysicalDevicePerformanceQueryPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "allowCommandBufferQueryCopies")]
    allow_command_buffer_query_copies: Bool32,
}
///# [VkPerformanceCounterKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterKHR.md")]
#[doc(alias = "VkPerformanceCounterKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    unit: PerformanceCounterUnitKHR,
    scope: PerformanceCounterScopeKHR,
    storage: PerformanceCounterStorageKHR,
    uuid: [u8; UUID_SIZE as usize],
}
///# [VkPerformanceCounterDescriptionKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterDescriptionKHR.md")]
#[doc(alias = "VkPerformanceCounterDescriptionKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterDescriptionKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    flags: PerformanceCounterDescriptionFlagsKHR,
    name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    category: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
}
///# [VkQueryPoolPerformanceCreateInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkQueryPoolPerformanceCreateInfoKHR.md")]
#[doc(alias = "VkQueryPoolPerformanceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "queueFamilyIndex")]
    queue_family_index: u32,
    #[doc(alias = "counterIndexCount")]
    counter_index_count: u32,
    #[doc(alias = "pCounterIndices")]
    counter_indices: *const u32,
}
///# [VkAcquireProfilingLockInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkAcquireProfilingLockInfoKHR.md")]
#[doc(alias = "VkAcquireProfilingLockInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: AcquireProfilingLockFlagsKHR,
    timeout: u64,
}
///# [VkPerformanceQuerySubmitInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceQuerySubmitInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceQuerySubmitInfoKHR.md")]
#[doc(alias = "VkPerformanceQuerySubmitInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "counterPassIndex")]
    counter_pass_index: u32,
}
///# [VkPerformanceCounterResultKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterResultKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterResultKHR.md")]
#[doc(alias = "VkPerformanceCounterResultKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PerformanceCounterResultKHR {
    int32: i32,
    int64: i64,
    uint32: u32,
    uint64: u64,
    float32: f32,
    float64: f64,
}
///# [VkPerformanceCounterDescriptionFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterDescriptionFlagBitsKHR.md")]
#[doc(alias = "VkPerformanceCounterDescriptionFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerformanceCounterDescriptionFlagsKHR(u32);
impl PerformanceCounterDescriptionFlagsKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR")]
    pub const PERFORMANCE_IMPACTING: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR")]
    pub const CONCURRENTLY_IMPACTED: Self = Self(2);
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
            all |= Self::PERFORMANCE_IMPACTING;
        }
        {
            all |= Self::CONCURRENTLY_IMPACTED;
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
impl const std::ops::BitOr for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for PerformanceCounterDescriptionFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PerformanceCounterDescriptionFlagsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn extend<T: IntoIterator<Item = PerformanceCounterDescriptionFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<PerformanceCounterDescriptionFlagsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from_iter<T: IntoIterator<Item = PerformanceCounterDescriptionFlagsKHR>>(
        iterator: T,
    ) -> PerformanceCounterDescriptionFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<PerformanceCounterDescriptionFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for PerformanceCounterDescriptionFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from(bit: PerformanceCounterDescriptionFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn extend<T: IntoIterator<Item = PerformanceCounterDescriptionFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from_iter<T: IntoIterator<Item = PerformanceCounterDescriptionFlagBitsKHR>>(
        iterator: T,
    ) -> PerformanceCounterDescriptionFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<PerformanceCounterDescriptionFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PerformanceCounterDescriptionFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PerformanceCounterDescriptionFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PerformanceCounterDescriptionFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(PerformanceCounterDescriptionFlagsKHR::PERFORMANCE_IMPACTING)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(PERFORMANCE_IMPACTING))?;
                    }
                    if self
                        .0
                        .contains(PerformanceCounterDescriptionFlagsKHR::CONCURRENTLY_IMPACTED)
                    {
                        if !first {
                            f.write_str(" | ")?;
                        }
                        first = false;
                        f.write_str(stringify!(CONCURRENTLY_IMPACTED))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PerformanceCounterDescriptionFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///# [VkAcquireProfilingLockFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkAcquireProfilingLockFlagBitsKHR.md")]
#[doc(alias = "VkAcquireProfilingLockFlagsKHR")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcquireProfilingLockFlagsKHR(u32);
impl AcquireProfilingLockFlagsKHR {
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
impl const std::ops::BitOr for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for AcquireProfilingLockFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<AcquireProfilingLockFlagsKHR> for AcquireProfilingLockFlagsKHR {
    fn extend<T: IntoIterator<Item = AcquireProfilingLockFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl FromIterator<AcquireProfilingLockFlagsKHR> for AcquireProfilingLockFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AcquireProfilingLockFlagsKHR>>(iterator: T) -> AcquireProfilingLockFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AcquireProfilingLockFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl const Default for AcquireProfilingLockFlagsKHR {
    fn default() -> Self {
        Self::empty()
    }
}
impl const From<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn from(bit: AcquireProfilingLockFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(bit.bits()) }
    }
}
impl Extend<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn extend<T: IntoIterator<Item = AcquireProfilingLockFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, Self::from(i));
        }
    }
}
impl FromIterator<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AcquireProfilingLockFlagBitsKHR>>(iterator: T) -> AcquireProfilingLockFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AcquireProfilingLockFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for AcquireProfilingLockFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(AcquireProfilingLockFlagsKHR);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == AcquireProfilingLockFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(AcquireProfilingLockFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_performance_query");
///# [VkPerformanceCounterDescriptionFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterDescriptionFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterDescriptionFlagBitsKHR.md")]
#[doc(alias = "VkPerformanceCounterDescriptionFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceCounterDescriptionFlagBitsKHR(u32);
impl PerformanceCounterDescriptionFlagBitsKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR")]
    pub const PERFORMANCE_IMPACTING: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR")]
    pub const CONCURRENTLY_IMPACTED: Self = Self(2);
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
            x if x == Self::PERFORMANCE_IMPACTING.bits() => Some(Self(x)),
            x if x == Self::CONCURRENTLY_IMPACTED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
///# [VkAcquireProfilingLockFlagBitsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAcquireProfilingLockFlagBitsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkAcquireProfilingLockFlagBitsKHR.md")]
#[doc(alias = "VkAcquireProfilingLockFlagBitsKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct AcquireProfilingLockFlagBitsKHR(u32);
impl AcquireProfilingLockFlagBitsKHR {
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
///# [VkPerformanceCounterScopeKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterScopeKHR.md")]
#[doc(alias = "VkPerformanceCounterScopeKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceCounterScopeKHR(i32);
impl PerformanceCounterScopeKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR")]
    pub const COMMAND_BUFFER: Self = Self(0);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR")]
    pub const RENDER_PASS: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR")]
    pub const COMMAND: Self = Self(2);
    #[doc(alias = "VK_QUERY_SCOPE_COMMAND_BUFFER_KHR")]
    pub const QUERY_SCOPE_COMMAND_BUFFER: Self = Self::COMMAND_BUFFER;
    #[doc(alias = "VK_QUERY_SCOPE_RENDER_PASS_KHR")]
    pub const QUERY_SCOPE_RENDER_PASS: Self = Self::RENDER_PASS;
    #[doc(alias = "VK_QUERY_SCOPE_COMMAND_KHR")]
    pub const QUERY_SCOPE_COMMAND: Self = Self::COMMAND;
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::COMMAND_BUFFER.bits() => Some(Self(x)),
            x if x == Self::RENDER_PASS.bits() => Some(Self(x)),
            x if x == Self::COMMAND.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkPerformanceCounterUnitKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterUnitKHR.md")]
#[doc(alias = "VkPerformanceCounterUnitKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceCounterUnitKHR(i32);
impl PerformanceCounterUnitKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR")]
    pub const GENERIC: Self = Self(0);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR")]
    pub const PERCENTAGE: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR")]
    pub const NANOSECONDS: Self = Self(2);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR")]
    pub const BYTES: Self = Self(3);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR")]
    pub const BYTES_PER_SECOND: Self = Self(4);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR")]
    pub const KELVIN: Self = Self(5);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR")]
    pub const WATTS: Self = Self(6);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR")]
    pub const VOLTS: Self = Self(7);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR")]
    pub const AMPS: Self = Self(8);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR")]
    pub const HERTZ: Self = Self(9);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR")]
    pub const CYCLES: Self = Self(10);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::GENERIC.bits() => Some(Self(x)),
            x if x == Self::PERCENTAGE.bits() => Some(Self(x)),
            x if x == Self::NANOSECONDS.bits() => Some(Self(x)),
            x if x == Self::BYTES.bits() => Some(Self(x)),
            x if x == Self::BYTES_PER_SECOND.bits() => Some(Self(x)),
            x if x == Self::KELVIN.bits() => Some(Self(x)),
            x if x == Self::WATTS.bits() => Some(Self(x)),
            x if x == Self::VOLTS.bits() => Some(Self(x)),
            x if x == Self::AMPS.bits() => Some(Self(x)),
            x if x == Self::HERTZ.bits() => Some(Self(x)),
            x if x == Self::CYCLES.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkPerformanceCounterStorageKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/VkPerformanceCounterStorageKHR.md")]
#[doc(alias = "VkPerformanceCounterStorageKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceCounterStorageKHR(i32);
impl PerformanceCounterStorageKHR {
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR")]
    pub const INT32: Self = Self(0);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR")]
    pub const INT64: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR")]
    pub const UINT32: Self = Self(2);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR")]
    pub const UINT64: Self = Self(3);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR")]
    pub const FLOAT32: Self = Self(4);
    #[doc(alias = "VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR")]
    pub const FLOAT64: Self = Self(5);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::INT32.bits() => Some(Self(x)),
            x if x == Self::INT64.bits() => Some(Self(x)),
            x if x == Self::UINT32.bits() => Some(Self(x)),
            x if x == Self::UINT64.bits() => Some(Self(x)),
            x if x == Self::FLOAT32.bits() => Some(Self(x)),
            x if x == Self::FLOAT64.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.md")]
#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
pub type FNEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> VulkanResultCodes;
///# [vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.md")]
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
pub type FNGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
    p_num_passes: *mut u32,
);
///# [vkAcquireProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/vkAcquireProfilingLockKHR.md")]
#[doc(alias = "vkAcquireProfilingLockKHR")]
pub type FNAcquireProfilingLockKhr =
    unsafe extern "system" fn(device: Device, p_info: *const AcquireProfilingLockInfoKHR) -> VulkanResultCodes;
///# [vkReleaseProfilingLockKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_performance_query/vkReleaseProfilingLockKHR.md")]
#[doc(alias = "vkReleaseProfilingLockKHR")]
pub type FNReleaseProfilingLockKhr = unsafe extern "system" fn(device: Device);
