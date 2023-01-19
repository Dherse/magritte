//!# [VK_EXT_pipeline_creation_cache_control](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_creation_cache_control.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_pipeline_creation_cache_control/VK_EXT_pipeline_creation_cache_control.md")]
use crate::{cstr, vulkan1_3::PhysicalDevicePipelineCreationCacheControlFeatures};
use std::ffi::CStr;
///See [`PhysicalDevicePipelineCreationCacheControlFeatures`]
#[doc(alias = "VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT")]
pub type PhysicalDevicePipelineCreationCacheControlFeaturesEXT = PhysicalDevicePipelineCreationCacheControlFeatures;
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME: &'static CStr =
    cstr!("VK_EXT_pipeline_creation_cache_control");
///# [VkPipelineCacheCreateFlagBits](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineCacheCreateFlagBits.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_pipeline_creation_cache_control/VkPipelineCacheCreateFlagBits.md")]
#[doc(alias = "VkPipelineCacheCreateFlagBits")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PipelineCacheCreateFlagBits(u32);
impl PipelineCacheCreateFlagBits {
    #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT")]
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1);
    #[doc(alias = "VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT")]
    pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
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
            x if x == Self::EXTERNALLY_SYNCHRONIZED.bits() => Some(Self(x)),
            #[cfg(feature = "VK_GOOGLE_extension_196")]
            x if x == Self::RESERVED1_EXT.bits() => Some(Self(x)),
            #[cfg(feature = "VK_KHR_extension_299")]
            x if x == Self::RESERVED2_KHR.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
}
