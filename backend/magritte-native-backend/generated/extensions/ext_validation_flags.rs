use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkValidationFlagsEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ValidationFlagsEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "disabledValidationCheckCount")]
    disabled_validation_check_count: u32,
    #[doc(alias = "pDisabledValidationChecks")]
    disabled_validation_checks: *const ValidationCheckEXT,
}
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_SPEC_VERSION")]
pub const EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME")]
pub const EXT_VALIDATION_FLAGS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_validation_flags");
#[doc(alias = "VkValidationCheckEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ValidationCheckEXT(i32);
impl ValidationCheckEXT {
    #[doc(alias = "VK_VALIDATION_CHECK_ALL_EXT")]
    pub const ALL: Self = Self(0);
    #[doc(alias = "VK_VALIDATION_CHECK_SHADERS_EXT")]
    pub const SHADERS: Self = Self(1);
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
            x if x == Self::ALL.bits() => Some(Self(x)),
            x if x == Self::SHADERS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
