//![VK_NV_device_diagnostics_config](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostics_config.html) - device extension
//!# Description
//!Applications using Nvidia Nsight<sup>™</sup> Aftermath SDK for Vulkan to integrate
//!device crash dumps into their error reporting mechanisms,  **may**  use this
//!extension to configure options related to device crash dump creation.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Kedarnath Thangudu [kthangudu](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_device_diagnostics_config]
//!   @kthangudu%0A<<Here describe the issue or question you have about the
//!   VK_NV_device_diagnostics_config extension>>)
//!# New structures
//! - Extending [`DeviceCreateInfo`]:  - [`DeviceDiagnosticsConfigCreateInfoNV`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceDiagnosticsConfigFeaturesNV`]
//!# New enums
//! - [`DeviceDiagnosticsConfigFlagBitsNV`]
//!# New bitmasks
//! - [`DeviceDiagnosticsConfigFlagsNV`]
//!# New constants
//! - [`NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME`]
//! - [`NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`
//!   - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV`
//!# Version History
//! - Revision 1, 2019-11-21 (Kedarnath Thangudu)  - Internal revisions
//!# Other info
//! * 2019-12-15
//! * - Kedarnath Thangudu, NVIDIA  - Thomas Klein, NVIDIA
//!# Related
//! - [`DeviceDiagnosticsConfigCreateInfoNV`]
//! - [`DeviceDiagnosticsConfigFlagBitsNV`]
//! - [`DeviceDiagnosticsConfigFlagsNV`]
//! - [`PhysicalDeviceDiagnosticsConfigFeaturesNV`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_NV_device_diagnostics_config");
///[VkDeviceDiagnosticsConfigFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) - Bitmask specifying diagnostics flags
///# C Specifications
///Bits which  **can**  be set in
///[`DeviceDiagnosticsConfigCreateInfoNV::flags`] include:
///```c
///// Provided by VK_NV_device_diagnostics_config
///typedef enum VkDeviceDiagnosticsConfigFlagBitsNV {
///    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = 0x00000001,
///    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = 0x00000002,
///    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = 0x00000004,
///} VkDeviceDiagnosticsConfigFlagBitsNV;
///```
/// # Description
/// - [`ENABLE_SHADER_DEBUG_INFO`] enables the generation of debug information for shaders.
/// - [`ENABLE_RESOURCE_TRACKING`] enables driver side tracking of resources (images, buffers, etc.)
///   used to augment the device fault information.
/// - [`ENABLE_AUTOMATIC_CHECKPOINTS`] enables automatic insertion of [diagnostic checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-diagnostic-checkpoints)
///   for draw calls, dispatches, trace rays, and copies. The CPU call stack at the time of the
///   command will be associated as the marker data for the automatically inserted checkpoints.
/// # Related
/// - [`nv_device_diagnostics_config`]
/// - [`DeviceDiagnosticsConfigFlagsNV`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceDiagnosticsConfigFlagBitsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigFlagBitsNV(u32);
impl const Default for DeviceDiagnosticsConfigFlagBitsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl DeviceDiagnosticsConfigFlagBitsNV {
    ///[`ENABLE_SHADER_DEBUG_INFO`]
    ///enables the generation of debug information for shaders.
    pub const ENABLE_SHADER_DEBUG_INFO: Self = Self(1);
    ///[`ENABLE_RESOURCE_TRACKING`]
    ///enables driver side tracking of resources (images, buffers, etc.) used
    ///to augment the device fault information.
    pub const ENABLE_RESOURCE_TRACKING: Self = Self(2);
    ///[`ENABLE_AUTOMATIC_CHECKPOINTS`]
    ///enables automatic insertion of [diagnostic checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-diagnostic-checkpoints) for draw calls, dispatches,
    ///trace rays,
    ///and copies.
    ///The CPU call stack at the time of the command will be associated as the
    ///marker data for the automatically inserted checkpoints.
    pub const ENABLE_AUTOMATIC_CHECKPOINTS: Self = Self(4);
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
impl std::fmt::Debug for DeviceDiagnosticsConfigFlagBitsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DeviceDiagnosticsConfigFlagBitsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DeviceDiagnosticsConfigFlagBitsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        DeviceDiagnosticsConfigFlagBitsNV::ENABLE_SHADER_DEBUG_INFO => {
                            f.write_str("ENABLE_SHADER_DEBUG_INFO")?
                        },
                        DeviceDiagnosticsConfigFlagBitsNV::ENABLE_RESOURCE_TRACKING => {
                            f.write_str("ENABLE_RESOURCE_TRACKING")?
                        },
                        DeviceDiagnosticsConfigFlagBitsNV::ENABLE_AUTOMATIC_CHECKPOINTS => {
                            f.write_str("ENABLE_AUTOMATIC_CHECKPOINTS")?
                        },
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DeviceDiagnosticsConfigFlagBitsNV))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDeviceDiagnosticsConfigFlagBitsNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigFlagBitsNV.html) - Bitmask specifying diagnostics flags
///# C Specifications
///Bits which  **can**  be set in
///[`DeviceDiagnosticsConfigCreateInfoNV::flags`] include:
///```c
///// Provided by VK_NV_device_diagnostics_config
///typedef enum VkDeviceDiagnosticsConfigFlagBitsNV {
///    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = 0x00000001,
///    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = 0x00000002,
///    VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = 0x00000004,
///} VkDeviceDiagnosticsConfigFlagBitsNV;
///```
/// # Description
/// - [`ENABLE_SHADER_DEBUG_INFO`] enables the generation of debug information for shaders.
/// - [`ENABLE_RESOURCE_TRACKING`] enables driver side tracking of resources (images, buffers, etc.)
///   used to augment the device fault information.
/// - [`ENABLE_AUTOMATIC_CHECKPOINTS`] enables automatic insertion of [diagnostic checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-diagnostic-checkpoints)
///   for draw calls, dispatches, trace rays, and copies. The CPU call stack at the time of the
///   command will be associated as the marker data for the automatically inserted checkpoints.
/// # Related
/// - [`nv_device_diagnostics_config`]
/// - [`DeviceDiagnosticsConfigFlagsNV`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceDiagnosticsConfigFlagsNV")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct DeviceDiagnosticsConfigFlagsNV(u32);
impl const Default for DeviceDiagnosticsConfigFlagsNV {
    fn default() -> Self {
        Self(0)
    }
}
impl From<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn from(from: DeviceDiagnosticsConfigFlagBitsNV) -> Self {
        unsafe { Self::from_bits_unchecked(from.bits()) }
    }
}
impl DeviceDiagnosticsConfigFlagsNV {
    ///[`ENABLE_SHADER_DEBUG_INFO`]
    ///enables the generation of debug information for shaders.
    pub const ENABLE_SHADER_DEBUG_INFO: Self = Self(1);
    ///[`ENABLE_RESOURCE_TRACKING`]
    ///enables driver side tracking of resources (images, buffers, etc.) used
    ///to augment the device fault information.
    pub const ENABLE_RESOURCE_TRACKING: Self = Self(2);
    ///[`ENABLE_AUTOMATIC_CHECKPOINTS`]
    ///enables automatic insertion of [diagnostic checkpoints](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#device-diagnostic-checkpoints) for draw calls, dispatches,
    ///trace rays,
    ///and copies.
    ///The CPU call stack at the time of the command will be associated as the
    ///marker data for the automatically inserted checkpoints.
    pub const ENABLE_AUTOMATIC_CHECKPOINTS: Self = Self(4);
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
            all |= Self::ENABLE_SHADER_DEBUG_INFO;
        }
        {
            all |= Self::ENABLE_RESOURCE_TRACKING;
        }
        {
            all |= Self::ENABLE_AUTOMATIC_CHECKPOINTS;
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
impl const std::ops::BitOr for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl const std::ops::BitOrAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl const std::ops::BitXorAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl const std::ops::BitAndAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl const std::ops::SubAssign for DeviceDiagnosticsConfigFlagsNV {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<DeviceDiagnosticsConfigFlagsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn extend<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn extend<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagBitsNV>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<DeviceDiagnosticsConfigFlagBitsNV>>::from(i));
        }
    }
}
impl FromIterator<DeviceDiagnosticsConfigFlagsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn from_iter<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagsNV>>(
        iterator: T,
    ) -> DeviceDiagnosticsConfigFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<DeviceDiagnosticsConfigFlagsNV>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn from_iter<T: IntoIterator<Item = DeviceDiagnosticsConfigFlagBitsNV>>(
        iterator: T,
    ) -> DeviceDiagnosticsConfigFlagsNV {
        let mut out = Self::empty();
        <Self as Extend<DeviceDiagnosticsConfigFlagBitsNV>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for DeviceDiagnosticsConfigFlagsNV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(DeviceDiagnosticsConfigFlagsNV);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == DeviceDiagnosticsConfigFlagsNV::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(DeviceDiagnosticsConfigFlagsNV::ENABLE_SHADER_DEBUG_INFO)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ENABLE_SHADER_DEBUG_INFO))?;
                    }
                    if self
                        .0
                        .contains(DeviceDiagnosticsConfigFlagsNV::ENABLE_RESOURCE_TRACKING)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ENABLE_RESOURCE_TRACKING))?;
                    }
                    if self
                        .0
                        .contains(DeviceDiagnosticsConfigFlagsNV::ENABLE_AUTOMATIC_CHECKPOINTS)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ENABLE_AUTOMATIC_CHECKPOINTS))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(DeviceDiagnosticsConfigFlagsNV))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkPhysicalDeviceDiagnosticsConfigFeaturesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDiagnosticsConfigFeaturesNV.html) - Structure describing the device-generated diagnostic configuration features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceDiagnosticsConfigFeaturesNV`] structure is defined
///as:
///```c
///// Provided by VK_NV_device_diagnostics_config
///typedef struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           diagnosticsConfig;
///} VkPhysicalDeviceDiagnosticsConfigFeaturesNV;
///```
/// # Members
/// This structure describes the following feature:
/// # Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`diagnostics_config`] indicates whether the implementation supports the ability to configure
///   diagnostic tools.
/// If the [`PhysicalDeviceDiagnosticsConfigFeaturesNV`] structure is included in the [`p_next`]
/// chain of the
/// [`PhysicalDeviceFeatures2`] structure passed to
/// [`get_physical_device_features2`], it is filled in to indicate whether each
/// corresponding feature is supported.
/// [`PhysicalDeviceDiagnosticsConfigFeaturesNV`] **can**  also be used in the [`p_next`] chain of
/// [`DeviceCreateInfo`] to selectively enable these features.
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV`
/// # Related
/// - [`nv_device_diagnostics_config`]
/// - [`Bool32`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceDiagnosticsConfigFeaturesNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`diagnostics_config`] indicates whether
    ///the implementation supports the ability to configure diagnostic tools.
    pub diagnostics_config: Bool32,
}
impl<'lt> Default for PhysicalDeviceDiagnosticsConfigFeaturesNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV,
            p_next: std::ptr::null_mut(),
            diagnostics_config: 0,
        }
    }
}
impl<'lt> PhysicalDeviceDiagnosticsConfigFeaturesNV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::diagnostics_config`]
    pub fn diagnostics_config_raw(&self) -> Bool32 {
        self.diagnostics_config
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::diagnostics_config`]
    pub fn set_diagnostics_config_raw(mut self, value: Bool32) -> Self {
        self.diagnostics_config = value;
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
    ///Gets the value of [`Self::diagnostics_config`]
    pub fn diagnostics_config(&self) -> bool {
        unsafe { std::mem::transmute(self.diagnostics_config as u8) }
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
    ///Gets a mutable reference to the value of [`Self::diagnostics_config`]
    pub fn diagnostics_config_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.diagnostics_config as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.diagnostics_config as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
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
    ///Sets the value of [`Self::diagnostics_config`]
    pub fn set_diagnostics_config(mut self, value: bool) -> Self {
        self.diagnostics_config = value as u8 as u32;
        self
    }
}
///[VkDeviceDiagnosticsConfigCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceDiagnosticsConfigCreateInfoNV.html) - Specify diagnostics config for a Vulkan device
///# C Specifications
///When using the Nsight<sup>™</sup> Aftermath SDK, to configure how device crash
///dumps are created, add a [`DeviceDiagnosticsConfigCreateInfoNV`]
///structure to the [`p_next`] chain of the [`DeviceCreateInfo`]
///structure.
///```c
///// Provided by VK_NV_device_diagnostics_config
///typedef struct VkDeviceDiagnosticsConfigCreateInfoNV {
///    VkStructureType                     sType;
///    const void*                         pNext;
///    VkDeviceDiagnosticsConfigFlagsNV    flags;
///} VkDeviceDiagnosticsConfigCreateInfoNV;
///```
/// # Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is a bitmask of [`DeviceDiagnosticsConfigFlagBitsNV`] specifying addtional
///   parameters for configuring diagnostic tools.
/// # Description
/// ## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV`
/// - [`flags`] **must**  be a valid combination of [`DeviceDiagnosticsConfigFlagBitsNV`] values
/// # Related
/// - [`nv_device_diagnostics_config`]
/// - [`DeviceDiagnosticsConfigFlagsNV`]
/// - [`StructureType`]
///
/// # Notes and documentation
/// For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
/// This documentation is generated from the Vulkan specification and documentation.
/// The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
/// This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceDiagnosticsConfigCreateInfoNV")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is a bitmask of [`DeviceDiagnosticsConfigFlagBitsNV`]
    ///specifying addtional parameters for configuring diagnostic tools.
    pub flags: DeviceDiagnosticsConfigFlagsNV,
}
impl<'lt> Default for DeviceDiagnosticsConfigCreateInfoNV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV,
            p_next: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
impl<'lt> DeviceDiagnosticsConfigCreateInfoNV<'lt> {
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
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> DeviceDiagnosticsConfigFlagsNV {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut DeviceDiagnosticsConfigFlagsNV {
        &mut self.flags
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
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        mut self,
        value: crate::extensions::nv_device_diagnostics_config::DeviceDiagnosticsConfigFlagsNV,
    ) -> Self {
        self.flags = value;
        self
    }
}
