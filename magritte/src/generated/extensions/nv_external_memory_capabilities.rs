//![VK_NV_external_memory_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_external_memory_capabilities.html) - instance extension
//!# Description
//!Applications may wish to import memory from the Direct 3D API, or export
//!memory to other Vulkan instances.
//!This extension provides a set of capability queries that allow applications
//!determine what types of win32 memory handles an implementation supports for
//!a given set of use cases.
//!# Revision
//!1
//!# Dependencies
//! - *Deprecated* by `[`VK_KHR_external_memory_capabilities`]` extension  - Which in turn was *promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_external_memory_capabilities]
//!   @cubanismo%0A<<Here describe the issue or question you have about the
//!   VK_NV_external_memory_capabilities extension>>)
//!# New functions & commands
//! - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
//!# New structures
//! - [`ExternalImageFormatPropertiesNV`]
//!# New enums
//! - [`ExternalMemoryFeatureFlagBitsNV`]
//! - [`ExternalMemoryHandleTypeFlagBitsNV`]
//!# New bitmasks
//! - [`ExternalMemoryFeatureFlagsNV`]
//! - [`ExternalMemoryHandleTypeFlagsNV`]
//!# New constants
//! - [`NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME`]
//! - [`NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION`]
//!# Known issues & F.A.Q
//!1) Why do so many external memory capabilities need to be queried on a
//!per-memory-handle-type basis? **RESOLVED** : This is because some handle types are based on
//! OS-native objects
//!that have far more limited capabilities than the very generic Vulkan memory
//!objects.
//!Not all memory handle types can name memory objects that support 3D images,
//!for example.
//!Some handle types cannot even support the deferred image and memory binding
//!behavior of Vulkan and require specifying the image when allocating or
//!importing the memory object.2) Does the [`ExternalImageFormatPropertiesNV`] struct need to
//! include a
//!list of memory type bits that support the given handle type? **RESOLVED** : No.
//!The memory types that do not support the handle types will simply be
//!filtered out of the results returned by [`GetImageMemoryRequirements`]
//!when a set of handle types was specified at image creation time.3) Should the non-opaque handle
//! types be moved to their own extension? **RESOLVED** : Perhaps.
//!However, defining the handle type bits does very little and does not require
//!any platform-specific types on its own, and it is easier to maintain the
//!bitmask values in a single extension for now.
//!Presumably more handle types could be added by separate extensions though,
//!and it would be midly weird to have some platform-specific ones defined in
//!the core spec and some in extensions
//!# Version History
//! - Revision 1, 2016-08-19 (James Jones)  - Initial version
//!# Other info
//! * 2016-08-19
//! * No known IP claims.
//! * - Interacts with Vulkan 1.1.  - Interacts with `[`VK_KHR_dedicated_allocation`]`.  - Interacts
//!   with `[`VK_NV_dedicated_allocation`]`.
//! * - James Jones, NVIDIA
//!# Related
//! - [`ExternalImageFormatPropertiesNV`]
//! - [`ExternalMemoryFeatureFlagBitsNV`]
//! - [`ExternalMemoryFeatureFlagsNV`]
//! - [`ExternalMemoryHandleTypeFlagBitsNV`]
//! - [`ExternalMemoryHandleTypeFlagsNV`]
//! - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::ImageFormatProperties;
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_NV_external_memory_capabilities");
///[VkExternalMemoryHandleTypeFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) - Bitmask specifying external memory handle types
///# C Specifications
///Possible values of [`ImportMemoryWin32HandleInfoNV::handle_type`],
///specifying the type of an external memory handle, are:
///```c
///// Provided by VK_NV_external_memory_capabilities
///typedef enum VkExternalMemoryHandleTypeFlagBitsNV {
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = 0x00000001,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = 0x00000002,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = 0x00000004,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = 0x00000008,
///} VkExternalMemoryHandleTypeFlagBitsNV;
///```
///# Description
/// - [`ExternalMemoryHandleTypeOpaqueWin32KmtNv`] specifies a handle to memory returned by
///   [`GetMemoryWin32HandleNV`].
/// - [`ExternalMemoryHandleTypeOpaqueWin32Nv`] specifies a handle to memory returned by
///   [`GetMemoryWin32HandleNV`], or one duplicated from such a handle using `DuplicateHandle()`.
/// - [`ExternalMemoryHandleTypeD3D11ImageNv`] specifies a valid NT handle to memory returned by
///   `IDXGIResource1::CreateSharedHandle`, or a handle duplicated from such a handle using
///   `DuplicateHandle()`.
/// - [`ExternalMemoryHandleTypeD3D11ImageKmtNv`] specifies a handle to memory returned by
///   `IDXGIResource::GetSharedHandle()`.
///# Related
/// - [`VK_NV_external_memory_capabilities`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryHandleTypeFlagBitsNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalMemoryHandleTypeFlagBitsNV {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalMemoryHandleTypeOpaqueWin32Nv`] specifies a
    ///handle to memory returned by [`GetMemoryWin32HandleNV`], or one
    ///duplicated from such a handle using `DuplicateHandle()`.
    ExternalMemoryHandleTypeOpaqueWin32Nv = 1,
    ///[`ExternalMemoryHandleTypeOpaqueWin32KmtNv`] specifies a
    ///handle to memory returned by [`GetMemoryWin32HandleNV`].
    ExternalMemoryHandleTypeOpaqueWin32KmtNv = 2,
    ///[`ExternalMemoryHandleTypeD3D11ImageNv`] specifies a
    ///valid NT handle to memory returned by
    ///`IDXGIResource1::CreateSharedHandle`, or a handle duplicated from such a
    ///handle using `DuplicateHandle()`.
    ExternalMemoryHandleTypeD3D11ImageNv = 4,
    ///[`ExternalMemoryHandleTypeD3D11ImageKmtNv`] specifies a
    ///handle to memory returned by `IDXGIResource::GetSharedHandle()`.
    ExternalMemoryHandleTypeD3D11ImageKmtNv = 8,
}
impl const Default for ExternalMemoryHandleTypeFlagBitsNV {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalMemoryHandleTypeFlagBitsNV {
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
///[VkExternalMemoryFeatureFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) - Bitmask specifying external memory features
///# C Specifications
///Bits which  **can**  be set in
///[`ExternalImageFormatPropertiesNV::external_memory_features`],
///indicating properties of the external memory handle type, are:
///```c
///// Provided by VK_NV_external_memory_capabilities
///typedef enum VkExternalMemoryFeatureFlagBitsNV {
///    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = 0x00000001,
///    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = 0x00000002,
///    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = 0x00000004,
///} VkExternalMemoryFeatureFlagBitsNV;
///```
///# Description
/// - [`ExternalMemoryFeatureDedicatedOnlyNv`] specifies that external memory of the specified type
///   **must**  be created as a dedicated allocation when used in the manner specified.
/// - [`ExternalMemoryFeatureExportableNv`] specifies that the implementation supports exporting
///   handles of the specified type.
/// - [`ExternalMemoryFeatureImportableNv`] specifies that the implementation supports importing
///   handles of the specified type.
///# Related
/// - [`VK_NV_external_memory_capabilities`]
/// - [`ExternalImageFormatPropertiesNV`]
/// - [`ExternalMemoryFeatureFlagsNV`]
/// - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryFeatureFlagBitsNV")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum ExternalMemoryFeatureFlagBitsNV {
    #[doc(hidden)]
    Empty = 0,
    ///[`ExternalMemoryFeatureDedicatedOnlyNv`] specifies that
    ///external memory of the specified type  **must**  be created as a dedicated
    ///allocation when used in the manner specified.
    ExternalMemoryFeatureDedicatedOnlyNv = 1,
    ///[`ExternalMemoryFeatureExportableNv`] specifies that the
    ///implementation supports exporting handles of the specified type.
    ExternalMemoryFeatureExportableNv = 2,
    ///[`ExternalMemoryFeatureImportableNv`] specifies that the
    ///implementation supports importing handles of the specified type.
    ExternalMemoryFeatureImportableNv = 4,
}
impl const Default for ExternalMemoryFeatureFlagBitsNV {
    fn default() -> Self {
        Self::Empty
    }
}
impl ExternalMemoryFeatureFlagBitsNV {
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
///[VkExternalMemoryHandleTypeFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryHandleTypeFlagBitsNV.html) - Bitmask specifying external memory handle types
///# C Specifications
///Possible values of [`ImportMemoryWin32HandleInfoNV::handle_type`],
///specifying the type of an external memory handle, are:
///```c
///// Provided by VK_NV_external_memory_capabilities
///typedef enum VkExternalMemoryHandleTypeFlagBitsNV {
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = 0x00000001,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = 0x00000002,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = 0x00000004,
///    VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = 0x00000008,
///} VkExternalMemoryHandleTypeFlagBitsNV;
///```
///# Description
/// - [`ExternalMemoryHandleTypeOpaqueWin32KmtNv`] specifies a handle to memory returned by
///   [`GetMemoryWin32HandleNV`].
/// - [`ExternalMemoryHandleTypeOpaqueWin32Nv`] specifies a handle to memory returned by
///   [`GetMemoryWin32HandleNV`], or one duplicated from such a handle using `DuplicateHandle()`.
/// - [`ExternalMemoryHandleTypeD3D11ImageNv`] specifies a valid NT handle to memory returned by
///   `IDXGIResource1::CreateSharedHandle`, or a handle duplicated from such a handle using
///   `DuplicateHandle()`.
/// - [`ExternalMemoryHandleTypeD3D11ImageKmtNv`] specifies a handle to memory returned by
///   `IDXGIResource::GetSharedHandle()`.
///# Related
/// - [`VK_NV_external_memory_capabilities`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryHandleTypeFlagsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalMemoryHandleTypeFlagsNV(u32);
impl const Default for ExternalMemoryHandleTypeFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn from(from: ExternalMemoryHandleTypeFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalMemoryHandleTypeFlagsNV {
    ///[`ExternalMemoryHandleTypeOpaqueWin32Nv`] specifies a
    ///handle to memory returned by [`GetMemoryWin32HandleNV`], or one
    ///duplicated from such a handle using `DuplicateHandle()`.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_NV: Self = Self(1);
    ///[`ExternalMemoryHandleTypeOpaqueWin32KmtNv`] specifies a
    ///handle to memory returned by [`GetMemoryWin32HandleNV`].
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT_NV: Self = Self(2);
    ///[`ExternalMemoryHandleTypeD3D11ImageNv`] specifies a
    ///valid NT handle to memory returned by
    ///`IDXGIResource1::CreateSharedHandle`, or a handle duplicated from such a
    ///handle using `DuplicateHandle()`.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_NV: Self = Self(4);
    ///[`ExternalMemoryHandleTypeD3D11ImageKmtNv`] specifies a
    ///handle to memory returned by `IDXGIResource::GetSharedHandle()`.
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_KMT_NV: Self = Self(8);
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
        {
            all |= Self::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_NV;
        }
        {
            all |= Self::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT_NV;
        }
        {
            all |= Self::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_NV;
        }
        {
            all |= Self::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_KMT_NV;
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
impl const std::ops::BitOr for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalMemoryHandleTypeFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryHandleTypeFlagsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalMemoryHandleTypeFlagBitsNV>>::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlagsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagsNV>>(
        iterator: T,
    ) -> ExternalMemoryHandleTypeFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryHandleTypeFlagBitsNV>>(
        iterator: T,
    ) -> ExternalMemoryHandleTypeFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryHandleTypeFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryHandleTypeFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryHandleTypeFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryHandleTypeFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_NV))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN_32_KMT_NV))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_NV))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_KMT_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_HANDLE_TYPE_D_3_D_11_IMAGE_KMT_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryHandleTypeFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkExternalMemoryFeatureFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalMemoryFeatureFlagBitsNV.html) - Bitmask specifying external memory features
///# C Specifications
///Bits which  **can**  be set in
///[`ExternalImageFormatPropertiesNV::external_memory_features`],
///indicating properties of the external memory handle type, are:
///```c
///// Provided by VK_NV_external_memory_capabilities
///typedef enum VkExternalMemoryFeatureFlagBitsNV {
///    VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = 0x00000001,
///    VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = 0x00000002,
///    VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = 0x00000004,
///} VkExternalMemoryFeatureFlagBitsNV;
///```
///# Description
/// - [`ExternalMemoryFeatureDedicatedOnlyNv`] specifies that external memory of the specified type
///   **must**  be created as a dedicated allocation when used in the manner specified.
/// - [`ExternalMemoryFeatureExportableNv`] specifies that the implementation supports exporting
///   handles of the specified type.
/// - [`ExternalMemoryFeatureImportableNv`] specifies that the implementation supports importing
///   handles of the specified type.
///# Related
/// - [`VK_NV_external_memory_capabilities`]
/// - [`ExternalImageFormatPropertiesNV`]
/// - [`ExternalMemoryFeatureFlagsNV`]
/// - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalMemoryFeatureFlagsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ExternalMemoryFeatureFlagsNV(u32);
impl const Default for ExternalMemoryFeatureFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl From<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn from(from: ExternalMemoryFeatureFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl ExternalMemoryFeatureFlagsNV {
    ///[`ExternalMemoryFeatureDedicatedOnlyNv`] specifies that
    ///external memory of the specified type  **must**  be created as a dedicated
    ///allocation when used in the manner specified.
    pub const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_NV: Self = Self(1);
    ///[`ExternalMemoryFeatureExportableNv`] specifies that the
    ///implementation supports exporting handles of the specified type.
    pub const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_NV: Self = Self(2);
    ///[`ExternalMemoryFeatureImportableNv`] specifies that the
    ///implementation supports importing handles of the specified type.
    pub const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_NV: Self = Self(4);
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
        {
            all |= Self::EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_NV;
        }
        {
            all |= Self::EXTERNAL_MEMORY_FEATURE_EXPORTABLE_NV;
        }
        {
            all |= Self::EXTERNAL_MEMORY_FEATURE_IMPORTABLE_NV;
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
impl const std::ops::BitOr for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for ExternalMemoryFeatureFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<ExternalMemoryFeatureFlagsNV> for ExternalMemoryFeatureFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn extend<T: IntoIterator<Item = ExternalMemoryFeatureFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<ExternalMemoryFeatureFlagBitsNV>>::from(i));
        }
    }
}
impl FromIterator<ExternalMemoryFeatureFlagsNV> for ExternalMemoryFeatureFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlagsNV>>(iterator: T) -> ExternalMemoryFeatureFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn from_iter<T: IntoIterator<Item = ExternalMemoryFeatureFlagBitsNV>>(iterator: T) -> ExternalMemoryFeatureFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<ExternalMemoryFeatureFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for ExternalMemoryFeatureFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(ExternalMemoryFeatureFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == ExternalMemoryFeatureFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(ExternalMemoryFeatureFlagsNV::EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_NV))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryFeatureFlagsNV::EXTERNAL_MEMORY_FEATURE_EXPORTABLE_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_FEATURE_EXPORTABLE_NV))?;
                    }
                    if self
                        .0
                        .contains(ExternalMemoryFeatureFlagsNV::EXTERNAL_MEMORY_FEATURE_IMPORTABLE_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(EXTERNAL_MEMORY_FEATURE_IMPORTABLE_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(ExternalMemoryFeatureFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkExternalImageFormatPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkExternalImageFormatPropertiesNV.html) - Structure specifying external image format properties
///# C Specifications
///The [`ExternalImageFormatPropertiesNV`] structure is defined as:
///```c
///// Provided by VK_NV_external_memory_capabilities
///typedef struct VkExternalImageFormatPropertiesNV {
///    VkImageFormatProperties              imageFormatProperties;
///    VkExternalMemoryFeatureFlagsNV       externalMemoryFeatures;
///    VkExternalMemoryHandleTypeFlagsNV    exportFromImportedHandleTypes;
///    VkExternalMemoryHandleTypeFlagsNV    compatibleHandleTypes;
///} VkExternalImageFormatPropertiesNV;
///```
///# Members
/// - [`image_format_properties`] will be filled in as when calling
///   [`GetPhysicalDeviceImageFormatProperties`], but the values returned  **may**  vary depending
///   on the external handle type requested.
/// - [`external_memory_features`] is a bitmask of [`ExternalMemoryFeatureFlagBitsNV`], indicating
///   properties of the external memory handle type
///   ([`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType`) being queried, or
///   0 if the external memory handle type is 0.
/// - [`export_from_imported_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`]
///   containing a bit set for every external handle type that  **may**  be used to create memory
///   from which the handles of the type specified in
///   [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType` **can**  be
///   exported, or 0 if the external memory handle type is 0.
/// - [`compatible_handle_types`] is a bitmask of [`ExternalMemoryHandleTypeFlagBitsNV`] containing
///   a bit set for every external handle type that  **may**  be specified simultaneously with the
///   handle type specified by
///   [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType` when calling
///   [`AllocateMemory`], or 0 if the external memory handle type is 0. [`compatible_handle_types`]
///   will always contain [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]`::externalHandleType`
///# Related
/// - [`VK_NV_external_memory_capabilities`]
/// - [`ExternalMemoryFeatureFlagsNV`]
/// - [`ExternalMemoryHandleTypeFlagsNV`]
/// - [`ImageFormatProperties`]
/// - [`GetPhysicalDeviceExternalImageFormatPropertiesNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkExternalImageFormatPropertiesNV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ExternalImageFormatPropertiesNV {
    ///[`image_format_properties`] will be filled in as when calling
    ///[`GetPhysicalDeviceImageFormatProperties`], but the values returned
    /// **may**  vary depending on the external handle type requested.
    pub image_format_properties: ImageFormatProperties,
    ///[`external_memory_features`] is a bitmask of
    ///[`ExternalMemoryFeatureFlagBitsNV`], indicating properties of the
    ///external memory handle type
    ///([`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType`)
    ///being queried, or 0 if the external memory handle type is 0.
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    ///[`export_from_imported_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] containing a bit set for
    ///every external handle type that  **may**  be used to create memory from which
    ///the handles of the type specified in
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType` **can**  be exported, or 0 if the external memory handle type is 0.
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    ///[`compatible_handle_types`] is a bitmask of
    ///[`ExternalMemoryHandleTypeFlagBitsNV`] containing a bit set for
    ///every external handle type that  **may**  be specified simultaneously with
    ///the handle type specified by
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType`
    ///when calling [`AllocateMemory`], or 0 if the external memory handle
    ///type is 0.
    ///[`compatible_handle_types`] will always contain
    ///[`GetPhysicalDeviceExternalImageFormatPropertiesNV`]::`externalHandleType`
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl Default for ExternalImageFormatPropertiesNV {
    fn default() -> Self {
        Self {
            image_format_properties: Default::default(),
            external_memory_features: Default::default(),
            export_from_imported_handle_types: Default::default(),
            compatible_handle_types: Default::default(),
        }
    }
}
impl ExternalImageFormatPropertiesNV {
    ///Gets the value of [`Self::image_format_properties`]
    pub fn image_format_properties(&self) -> ImageFormatProperties {
        self.image_format_properties
    }
    ///Gets the value of [`Self::external_memory_features`]
    pub fn external_memory_features(&self) -> ExternalMemoryFeatureFlagsNV {
        self.external_memory_features
    }
    ///Gets the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.export_from_imported_handle_types
    }
    ///Gets the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types(&self) -> ExternalMemoryHandleTypeFlagsNV {
        self.compatible_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::image_format_properties`]
    pub fn image_format_properties_mut(&mut self) -> &mut ImageFormatProperties {
        &mut self.image_format_properties
    }
    ///Gets a mutable reference to the value of [`Self::external_memory_features`]
    pub fn external_memory_features_mut(&mut self) -> &mut ExternalMemoryFeatureFlagsNV {
        &mut self.external_memory_features
    }
    ///Gets a mutable reference to the value of [`Self::export_from_imported_handle_types`]
    pub fn export_from_imported_handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.export_from_imported_handle_types
    }
    ///Gets a mutable reference to the value of [`Self::compatible_handle_types`]
    pub fn compatible_handle_types_mut(&mut self) -> &mut ExternalMemoryHandleTypeFlagsNV {
        &mut self.compatible_handle_types
    }
    ///Sets the raw value of [`Self::image_format_properties`]
    pub fn set_image_format_properties(&mut self, value: crate::vulkan1_0::ImageFormatProperties) -> &mut Self {
        self.image_format_properties = value;
        self
    }
    ///Sets the raw value of [`Self::external_memory_features`]
    pub fn set_external_memory_features(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryFeatureFlagsNV,
    ) -> &mut Self {
        self.external_memory_features = value;
        self
    }
    ///Sets the raw value of [`Self::export_from_imported_handle_types`]
    pub fn set_export_from_imported_handle_types(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.export_from_imported_handle_types = value;
        self
    }
    ///Sets the raw value of [`Self::compatible_handle_types`]
    pub fn set_compatible_handle_types(
        &mut self,
        value: crate::extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    ) -> &mut Self {
        self.compatible_handle_types = value;
        self
    }
}
