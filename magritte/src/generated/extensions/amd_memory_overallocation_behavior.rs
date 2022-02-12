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
//! - Extending [`DeviceCreateInfo`]:
//! - [`DeviceMemoryOverallocationCreateInfoAMD`]
//!# New enums
//! - [`MemoryOverallocationBehaviorAMD`]
//!# New constants
//! - [`AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME`]
//! - [`AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD`
//!# Version History
//! - Revision 1, 2018-09-19 (Martin Dinkov)
//! - Initial draft.
//!# Other info
//! * 2018-09-19
//! * No known IP claims.
//!*
//! - Martin Dinkov, AMD
//! - Matthaeus Chajdas, AMD
//! - Daniel Rakos, AMD
//! - Jon Campbell, AMD
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
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
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
/// - [`MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT`] lets the
///implementation decide if overallocation is allowed.
/// - [`MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED`] specifies
///overallocation is allowed if platform permits.
/// - [`MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED`] specifies the
///application is not allowed to allocate device memory beyond the heap
///sizes reported by [`PhysicalDeviceMemoryProperties`].
///Allocations that are not explicitly made by the application within the
///scope of the Vulkan instance are not accounted for.
///# Related
/// - [`VK_AMD_memory_overallocation_behavior`]
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
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct MemoryOverallocationBehaviorAMD(i32);
impl const Default for MemoryOverallocationBehaviorAMD {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for MemoryOverallocationBehaviorAMD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("MemoryOverallocationBehaviorAMD")
            .field(match *self {
                Self::MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT => &"MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT",
                Self::MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED => &"MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED",
                Self::MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED => &"MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED",
                other => unreachable!("invalid value for `MemoryOverallocationBehaviorAMD`: {:?}", other),
            })
            .finish()
    }
}
impl MemoryOverallocationBehaviorAMD {
    ///[`MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT`] lets the
    ///implementation decide if overallocation is allowed.
    pub const MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT: Self = Self(0);
    ///[`MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED`] specifies
    ///overallocation is allowed if platform permits.
    pub const MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED: Self = Self(1);
    ///[`MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED`] specifies the
    ///application is not allowed to allocate device memory beyond the heap
    ///sizes reported by [`PhysicalDeviceMemoryProperties`].
    ///Allocations that are not explicitly made by the application within the
    ///scope of the Vulkan instance are not accounted for.
    pub const MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED: Self = Self(2);
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
}
