//![VK_AMD_shader_core_properties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties2.html) - device extension
//!# Description
//!This extension exposes additional shader core properties for a target
//!physical device through the `[`khr_get_physical_device_properties2`]`
//!extension.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`amd_shader_core_properties`]`
//!# Contacts
//! - Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_core_properties2]
//!   @anteru%0A<<Here describe the issue or question you have about the
//!   VK_AMD_shader_core_properties2 extension>>)
//!# New structures
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceShaderCoreProperties2AMD`]
//!# New enums
//! - [`ShaderCorePropertiesFlagBitsAMD`]
//!# New bitmasks
//! - [`ShaderCorePropertiesFlagsAMD`]
//!# New constants
//! - [`AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME`]
//! - [`AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`
//!# Version History
//! - Revision 1, 2019-07-26 (Matthaeus G. Chajdas)  - Initial draft.
//!# Other info
//! * 2019-07-26
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Tobias Hector, AMD
//!# Related
//! - [`PhysicalDeviceShaderCoreProperties2AMD`]
//! - [`ShaderCorePropertiesFlagBitsAMD`]
//! - [`ShaderCorePropertiesFlagsAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseOutStructure, StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_shader_core_properties2");
///[VkShaderCorePropertiesFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html) - Bitmask specifying shader core properties
///# C Specifications
///Bits for this type  **may**  be defined by future extensions, or new versions of
///the `[`amd_shader_core_properties2`]` extension.
///Possible values of the `flags` member of
///[`ShaderCorePropertiesFlagsAMD`] are:
///```c
///// Provided by VK_AMD_shader_core_properties2
///typedef enum VkShaderCorePropertiesFlagBitsAMD {
///} VkShaderCorePropertiesFlagBitsAMD;
///```
/// # Related
/// - [`amd_shader_core_properties2`]
/// - [`PhysicalDeviceShaderCoreProperties2AMD`]
/// - [`ShaderCorePropertiesFlagsAMD`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderCorePropertiesFlagBitsAMD")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ShaderCorePropertiesFlagBitsAMD(u32);
impl const Default for ShaderCorePropertiesFlagBitsAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl ShaderCorePropertiesFlagBitsAMD {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for ShaderCorePropertiesFlagBitsAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ShaderCorePropertiesFlagBitsAMD);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ShaderCorePropertiesFlagBitsAMD::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ShaderCorePropertiesFlagBitsAMD))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkShaderCorePropertiesFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderCorePropertiesFlagBitsAMD.html) - Bitmask specifying shader core properties
///# C Specifications
///Bits for this type  **may**  be defined by future extensions, or new versions of
///the `[`amd_shader_core_properties2`]` extension.
///Possible values of the `flags` member of
///[`ShaderCorePropertiesFlagsAMD`] are:
///```c
///// Provided by VK_AMD_shader_core_properties2
///typedef enum VkShaderCorePropertiesFlagBitsAMD {
///} VkShaderCorePropertiesFlagBitsAMD;
///```
/// # Related
/// - [`amd_shader_core_properties2`]
/// - [`PhysicalDeviceShaderCoreProperties2AMD`]
/// - [`ShaderCorePropertiesFlagsAMD`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkShaderCorePropertiesFlagsAMD")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ShaderCorePropertiesFlagsAMD(u32);
impl const Default for ShaderCorePropertiesFlagsAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    fn from(from: ShaderCorePropertiesFlagBitsAMD) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl ShaderCorePropertiesFlagsAMD {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
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
impl Extend<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    fn extend<T: IntoIterator<Item = ShaderCorePropertiesFlagBitsAMD>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ShaderCorePropertiesFlagBitsAMD>>::from(i));
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
///[VkPhysicalDeviceShaderCoreProperties2AMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html) - Structure describing shader core properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceShaderCoreProperties2AMD`] structure is defined as:
///```c
///// Provided by VK_AMD_shader_core_properties2
///typedef struct VkPhysicalDeviceShaderCoreProperties2AMD {
///    VkStructureType                   sType;
///    void*                             pNext;
///    VkShaderCorePropertiesFlagsAMD    shaderCoreFeatures;
///    uint32_t                          activeComputeUnitCount;
///} VkPhysicalDeviceShaderCoreProperties2AMD;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`shader_core_features`] is a bitmask of [`ShaderCorePropertiesFlagBitsAMD`] indicating the
///   set of features supported by the shader core.
/// - [`active_compute_unit_count`] is an unsigned integer value indicating the number of compute
///   units that have been enabled.
/// # Description
/// If the [`PhysicalDeviceShaderCoreProperties2AMD`] structure is included in the [`p_next`] chain
/// of the
/// [`PhysicalDeviceProperties2`] structure passed to
/// [`get_physical_device_properties2`], it is filled in with each
/// corresponding implementation-dependent property.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`
/// # Related
/// - [`amd_shader_core_properties2`]
/// - [`ShaderCorePropertiesFlagsAMD`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceShaderCoreProperties2AMD")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`shader_core_features`] is a bitmask of
    ///[`ShaderCorePropertiesFlagBitsAMD`] indicating the set of features
    ///supported by the shader core.
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    ///[`active_compute_unit_count`] is an
    ///unsigned integer value indicating the number of compute units that have
    ///been enabled.
    pub active_compute_unit_count: u32,
}
impl<'lt> Default for PhysicalDeviceShaderCoreProperties2AMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES2_AMD,
            p_next: std::ptr::null_mut(),
            shader_core_features: Default::default(),
            active_compute_unit_count: 0,
        }
    }
}
impl<'lt> PhysicalDeviceShaderCoreProperties2AMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::shader_core_features`]
    pub fn shader_core_features(&self) -> ShaderCorePropertiesFlagsAMD {
        self.shader_core_features
    }
    ///Gets the value of [`Self::active_compute_unit_count`]
    pub fn active_compute_unit_count(&self) -> u32 {
        self.active_compute_unit_count
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
    ///Gets a mutable reference to the value of [`Self::shader_core_features`]
    pub fn shader_core_features_mut(&mut self) -> &mut ShaderCorePropertiesFlagsAMD {
        &mut self.shader_core_features
    }
    ///Gets a mutable reference to the value of [`Self::active_compute_unit_count`]
    pub fn active_compute_unit_count_mut(&mut self) -> &mut u32 {
        &mut self.active_compute_unit_count
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::shader_core_features`]
    pub fn set_shader_core_features(
        mut self,
        value: crate::extensions::amd_shader_core_properties2::ShaderCorePropertiesFlagsAMD,
    ) -> Self {
        self.shader_core_features = value;
        self
    }
    ///Sets the value of [`Self::active_compute_unit_count`]
    pub fn set_active_compute_unit_count(mut self, value: u32) -> Self {
        self.active_compute_unit_count = value;
        self
    }
}
