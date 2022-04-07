//![VK_AMD_pipeline_compiler_control](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_pipeline_compiler_control.html) - device extension
//!# Description
//!This extension introduces [`PipelineCompilerControlCreateInfoAMD`]
//!structure that can be chained to a pipeline’s creation information to
//!specify additional flags that affect pipeline compilation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_pipeline_compiler_control]
//!   @anteru%0A<<Here describe the issue or question you have about the
//!   VK_AMD_pipeline_compiler_control extension>>)
//!# New structures
//! - Extending [`GraphicsPipelineCreateInfo`], [`ComputePipelineCreateInfo`]:  -
//!   [`PipelineCompilerControlCreateInfoAMD`]
//!# New enums
//! - [`PipelineCompilerControlFlagBitsAMD`]
//!# New bitmasks
//! - [`PipelineCompilerControlFlagsAMD`]
//!# New constants
//! - [`AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME`]
//! - [`AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`
//!# Known issues & F.A.Q
//!None.
//!# Version History
//! - Revision 1, 2019-07-26 (Tobias Hector)  - Initial revision.
//!# Other info
//! * 2019-07-26
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Daniel Rakos, AMD  - Maciej Jesionowski, AMD  - Tobias Hector,
//!   AMD
//!# Related
//! - [`PipelineCompilerControlCreateInfoAMD`]
//! - [`PipelineCompilerControlFlagBitsAMD`]
//! - [`PipelineCompilerControlFlagsAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, StructureType};
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
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION")]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME")]
pub const AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_AMD_pipeline_compiler_control");
///[VkPipelineCompilerControlFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html) - Enum specifying available compilation control flags
///# C Specifications
///There are currently no available flags for this extension; flags will be
///added by future versions of this extension.
///```c
///// Provided by VK_AMD_pipeline_compiler_control
///typedef enum VkPipelineCompilerControlFlagBitsAMD {
///} VkPipelineCompilerControlFlagBitsAMD;
///```
/// # Related
/// - [`VK_AMD_pipeline_compiler_control`]
/// - [`PipelineCompilerControlFlagsAMD`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineCompilerControlFlagBitsAMD")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineCompilerControlFlagBitsAMD(u32);
impl const Default for PipelineCompilerControlFlagBitsAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl PipelineCompilerControlFlagBitsAMD {
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
///[VkPipelineCompilerControlFlagBitsAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlFlagBitsAMD.html) - Enum specifying available compilation control flags
///# C Specifications
///There are currently no available flags for this extension; flags will be
///added by future versions of this extension.
///```c
///// Provided by VK_AMD_pipeline_compiler_control
///typedef enum VkPipelineCompilerControlFlagBitsAMD {
///} VkPipelineCompilerControlFlagBitsAMD;
///```
/// # Related
/// - [`VK_AMD_pipeline_compiler_control`]
/// - [`PipelineCompilerControlFlagsAMD`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineCompilerControlFlagsAMD")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineCompilerControlFlagsAMD(u32);
impl const Default for PipelineCompilerControlFlagsAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl From<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    fn from(from: PipelineCompilerControlFlagBitsAMD) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl PipelineCompilerControlFlagsAMD {
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
impl const std::ops::BitOr for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for PipelineCompilerControlFlagsAMD {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<PipelineCompilerControlFlagsAMD> for PipelineCompilerControlFlagsAMD {
    fn extend<T: IntoIterator<Item = PipelineCompilerControlFlagsAMD>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    fn extend<T: IntoIterator<Item = PipelineCompilerControlFlagBitsAMD>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<PipelineCompilerControlFlagBitsAMD>>::from(i));
        }
    }
}
impl FromIterator<PipelineCompilerControlFlagsAMD> for PipelineCompilerControlFlagsAMD {
    fn from_iter<T: IntoIterator<Item = PipelineCompilerControlFlagsAMD>>(
        iterator: T,
    ) -> PipelineCompilerControlFlagsAMD {
        let mut out = Self::empty();
        <Self as Extend<PipelineCompilerControlFlagsAMD>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    fn from_iter<T: IntoIterator<Item = PipelineCompilerControlFlagBitsAMD>>(
        iterator: T,
    ) -> PipelineCompilerControlFlagsAMD {
        let mut out = Self::empty();
        <Self as Extend<PipelineCompilerControlFlagBitsAMD>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for PipelineCompilerControlFlagsAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(PipelineCompilerControlFlagsAMD);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == PipelineCompilerControlFlagsAMD::empty() {
                    f.write_str("empty")?;
                } else {
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(PipelineCompilerControlFlagsAMD))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkPipelineCompilerControlCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCompilerControlCreateInfoAMD.html) - Structure used to pass compilation control flags to a pipeline
///# C Specifications
///The compilation of a pipeline  **can**  be tuned by adding a
///[`PipelineCompilerControlCreateInfoAMD`] structure to the [`p_next`]
///chain of [`GraphicsPipelineCreateInfo`] or
///[`ComputePipelineCreateInfo`].
///```c
///// Provided by VK_AMD_pipeline_compiler_control
///typedef struct VkPipelineCompilerControlCreateInfoAMD {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkPipelineCompilerControlFlagsAMD    compilerControlFlags;
///} VkPipelineCompilerControlCreateInfoAMD;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`compiler_control_flags`] is a bitmask of [`PipelineCompilerControlFlagBitsAMD`] affecting
///   how the pipeline will be compiled.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD`
/// - [`compiler_control_flags`] **must**  be `0`
/// # Related
/// - [`VK_AMD_pipeline_compiler_control`]
/// - [`PipelineCompilerControlFlagsAMD`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineCompilerControlCreateInfoAMD")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PipelineCompilerControlCreateInfoAMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`compiler_control_flags`] is a bitmask of
    ///[`PipelineCompilerControlFlagBitsAMD`] affecting how the pipeline
    ///will be compiled.
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
impl<'lt> Default for PipelineCompilerControlCreateInfoAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            compiler_control_flags: Default::default(),
        }
    }
}
impl<'lt> PipelineCompilerControlCreateInfoAMD<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::compiler_control_flags`]
    pub fn compiler_control_flags(&self) -> PipelineCompilerControlFlagsAMD {
        self.compiler_control_flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::compiler_control_flags`]
    pub fn compiler_control_flags_mut(&mut self) -> &mut PipelineCompilerControlFlagsAMD {
        &mut self.compiler_control_flags
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::compiler_control_flags`]
    pub fn set_compiler_control_flags(
        mut self,
        value: crate::extensions::amd_pipeline_compiler_control::PipelineCompilerControlFlagsAMD,
    ) -> Self {
        self.compiler_control_flags = value;
        self
    }
}
