//!# [VK_AMD_memory_overallocation_behavior](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_memory_overallocation_behavior.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_memory_overallocation_behavior/VK_AMD_memory_overallocation_behavior.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::ffi::CStr;
///# [VkDeviceMemoryOverallocationCreateInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDeviceMemoryOverallocationCreateInfoAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_memory_overallocation_behavior/VkDeviceMemoryOverallocationCreateInfoAMD.md")]
#[doc(alias = "VkDeviceMemoryOverallocationCreateInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "overallocationBehavior")]
    overallocation_behavior: MemoryOverallocationBehaviorAMD,
}
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: &'static CStr =
    cstr!("VK_AMD_memory_overallocation_behavior");
///# [VkMemoryOverallocationBehaviorAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryOverallocationBehaviorAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_memory_overallocation_behavior/VkMemoryOverallocationBehaviorAMD.md")]
#[doc(alias = "VkMemoryOverallocationBehaviorAMD")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct MemoryOverallocationBehaviorAMD(i32);
impl MemoryOverallocationBehaviorAMD {
    #[doc(alias = "VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD")]
    pub const DEFAULT: Self = Self(0);
    #[doc(alias = "VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD")]
    pub const ALLOWED: Self = Self(1);
    #[doc(alias = "VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD")]
    pub const DISALLOWED: Self = Self(2);
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
            x if x == Self::DEFAULT.bits() => Some(Self(x)),
            x if x == Self::ALLOWED.bits() => Some(Self(x)),
            x if x == Self::DISALLOWED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
