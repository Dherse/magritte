//![VK_AMD_memory_overallocation_behavior](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_memory_overallocation_behavior.html) - device extension
//!# Description
//!This extension allows controlling whether explicit overallocation beyond the
//!device memory heap sizes (reported by
//![`PhysicalDeviceMemoryProperties`]) is allowed or not.
//!Overallocation may lead to performance loss and is not supported for all
//!platforms.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Martin Dinkov [mdinkov](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_memory_overallocation_behavior]
//!   @mdinkov%0A<<Here describe the issue or question you have about the
//!   VK_AMD_memory_overallocation_behavior extension>>)
//!# New structures
//! - Extending [`DeviceCreateInfo`]:  - [`DeviceMemoryOverallocationCreateInfoAMD`]
//!# New enums
//! - [`MemoryOverallocationBehaviorAMD`]
//!# New constants
//! - [`AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME`]
//! - [`AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION`]
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`
//!# Version history
//! - Revision 1, 2018-09-19 (Martin Dinkov)  - Initial draft.
//!# Other information
//! * 2018-09-19
//! * No known IP claims.
//! * - Martin Dinkov, AMD  - Matthaeus Chajdas, AMD  - Daniel Rakos, AMD  - Jon Campbell, AMD
//!# Related
//! - [`DeviceMemoryOverallocationCreateInfoAMD`]
//! - [`MemoryOverallocationBehaviorAMD`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: &'static CStr =
    crate::cstr!("VK_AMD_memory_overallocation_behavior");
///[VkMemoryOverallocationBehaviorAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html) - Specify memory overallocation behavior
///# C Specifications
///Possible values for
///[`DeviceMemoryOverallocationCreateInfoAMD::overallocation_behavior`]
///include:
///```c
///// Provided by VK_AMD_memory_overallocation_behavior
///typedef enum VkMemoryOverallocationBehaviorAMD {
///    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD = 0,
///    VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD = 1,
///    VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD = 2,
///} VkMemoryOverallocationBehaviorAMD;
///```
///# Description
/// - [`DEFAULT`] lets the implementation decide if overallocation is allowed.
/// - [`ALLOWED`] specifies overallocation is allowed if platform permits.
/// - [`DISALLOWED`] specifies the application is not allowed to allocate device memory beyond the
///   heap sizes reported by [`PhysicalDeviceMemoryProperties`]. Allocations that are not explicitly
///   made by the application within the scope of the Vulkan instance are not accounted for.
///# Related
/// - [`amd_memory_overallocation_behavior`]
/// - [`DeviceMemoryOverallocationCreateInfoAMD`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkMemoryOverallocationBehaviorAMD")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct MemoryOverallocationBehaviorAMD(i32);
impl const Default for MemoryOverallocationBehaviorAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl MemoryOverallocationBehaviorAMD {
    ///[`DEFAULT`] lets the
    ///implementation decide if overallocation is allowed.
    pub const DEFAULT: Self = Self(0);
    ///[`ALLOWED`] specifies
    ///overallocation is allowed if platform permits.
    pub const ALLOWED: Self = Self(1);
    ///[`DISALLOWED`] specifies the
    ///application is not allowed to allocate device memory beyond the heap
    ///sizes reported by [`PhysicalDeviceMemoryProperties`].
    ///Allocations that are not explicitly made by the application within the
    ///scope of the Vulkan instance are not accounted for.
    pub const DISALLOWED: Self = Self(2);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for MemoryOverallocationBehaviorAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(MemoryOverallocationBehaviorAMD);
        impl std::fmt::Debug for Flags {
            #[allow(unused_assignments, unused_mut, unused_variables)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == MemoryOverallocationBehaviorAMD::empty() {
                    f.write_str("empty")?;
                } else {
                    match self.0 {
                        MemoryOverallocationBehaviorAMD::DEFAULT => f.write_str("DEFAULT")?,
                        MemoryOverallocationBehaviorAMD::ALLOWED => f.write_str("ALLOWED")?,
                        MemoryOverallocationBehaviorAMD::DISALLOWED => f.write_str("DISALLOWED")?,
                        _ => f.write_str("invalid")?,
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(MemoryOverallocationBehaviorAMD))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkDeviceMemoryOverallocationCreateInfoAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html) - Specify memory overallocation behavior for a Vulkan device
///# C Specifications
///To specify whether device memory allocation is allowed beyond the size
///reported by [`PhysicalDeviceMemoryProperties`], add a
///[`DeviceMemoryOverallocationCreateInfoAMD`] structure to the [`p_next`]
///chain of the [`DeviceCreateInfo`] structure.
///If this structure is not specified, it is as if the
///`VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD` value is used.
///```c
///// Provided by VK_AMD_memory_overallocation_behavior
///typedef struct VkDeviceMemoryOverallocationCreateInfoAMD {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    VkMemoryOverallocationBehaviorAMD    overallocationBehavior;
///} VkDeviceMemoryOverallocationCreateInfoAMD;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`overallocation_behavior`] is the desired overallocation behavior.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`
/// - [`overallocation_behavior`] **must**  be a valid [`MemoryOverallocationBehaviorAMD`] value
///# Related
/// - [`amd_memory_overallocation_behavior`]
/// - [`MemoryOverallocationBehaviorAMD`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceMemoryOverallocationCreateInfoAMD")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`overallocation_behavior`] is the desired overallocation behavior.
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
}
impl<'lt> Default for DeviceMemoryOverallocationCreateInfoAMD<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
            p_next: std::ptr::null(),
            overallocation_behavior: Default::default(),
        }
    }
}
impl<'lt> DeviceMemoryOverallocationCreateInfoAMD<'lt> {
    ///Creates a static version of this structure
    pub fn make_static(mut self) -> DeviceMemoryOverallocationCreateInfoAMD<'static> {
        unsafe {
            self.p_next = std::ptr::null_mut() as _;
            std::mem::transmute(self)
        }
    }
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    ///Gets the value of [`Self::overallocation_behavior`]
    pub fn overallocation_behavior(&self) -> MemoryOverallocationBehaviorAMD {
        self.overallocation_behavior
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::overallocation_behavior`]
    pub fn overallocation_behavior_mut(&mut self) -> &mut MemoryOverallocationBehaviorAMD {
        &mut self.overallocation_behavior
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::overallocation_behavior`]
    pub fn set_overallocation_behavior(
        &mut self,
        value: crate::extensions::amd_memory_overallocation_behavior::MemoryOverallocationBehaviorAMD,
    ) -> &mut Self {
        self.overallocation_behavior = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::overallocation_behavior`]
    pub fn with_overallocation_behavior(
        mut self,
        value: crate::extensions::amd_memory_overallocation_behavior::MemoryOverallocationBehaviorAMD,
    ) -> Self {
        self.overallocation_behavior = value;
        self
    }
}
