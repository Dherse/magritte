use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, PhysicalDevice, ShaderStageFlags, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "cooperativeMatrix")]
    cooperative_matrix: Bool32,
    #[doc(alias = "cooperativeMatrixRobustBufferAccess")]
    cooperative_matrix_robust_buffer_access: Bool32,
}
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "cooperativeMatrixSupportedStages")]
    cooperative_matrix_supported_stages: ShaderStageFlags,
}
#[doc(alias = "VkCooperativeMatrixPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CooperativeMatrixPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "MSize")]
    m_size: u32,
    #[doc(alias = "NSize")]
    n_size: u32,
    #[doc(alias = "KSize")]
    k_size: u32,
    #[doc(alias = "AType")]
    a_type: ComponentTypeNV,
    #[doc(alias = "BType")]
    b_type: ComponentTypeNV,
    #[doc(alias = "CType")]
    c_type: ComponentTypeNV,
    #[doc(alias = "DType")]
    d_type: ComponentTypeNV,
    scope: ScopeNV,
}
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION")]
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME")]
pub const NV_COOPERATIVE_MATRIX_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_cooperative_matrix");
#[doc(alias = "VkScopeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ScopeNV(i32);
impl ScopeNV {
    #[doc(alias = "VK_SCOPE_DEVICE_NV")]
    pub const DEVICE: Self = Self(1);
    #[doc(alias = "VK_SCOPE_WORKGROUP_NV")]
    pub const WORKGROUP: Self = Self(2);
    #[doc(alias = "VK_SCOPE_SUBGROUP_NV")]
    pub const SUBGROUP: Self = Self(3);
    #[doc(alias = "VK_SCOPE_QUEUE_FAMILY_NV")]
    pub const QUEUE_FAMILY: Self = Self(5);
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
            x if x == Self::DEVICE.bits() => Some(Self(x)),
            x if x == Self::WORKGROUP.bits() => Some(Self(x)),
            x if x == Self::SUBGROUP.bits() => Some(Self(x)),
            x if x == Self::QUEUE_FAMILY.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkComponentTypeNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ComponentTypeNV(i32);
impl ComponentTypeNV {
    #[doc(alias = "VK_COMPONENT_TYPE_FLOAT16_NV")]
    pub const FLOAT16: Self = Self(0);
    #[doc(alias = "VK_COMPONENT_TYPE_FLOAT32_NV")]
    pub const FLOAT32: Self = Self(1);
    #[doc(alias = "VK_COMPONENT_TYPE_FLOAT64_NV")]
    pub const FLOAT64: Self = Self(2);
    #[doc(alias = "VK_COMPONENT_TYPE_SINT8_NV")]
    pub const SINT8: Self = Self(3);
    #[doc(alias = "VK_COMPONENT_TYPE_SINT16_NV")]
    pub const SINT16: Self = Self(4);
    #[doc(alias = "VK_COMPONENT_TYPE_SINT32_NV")]
    pub const SINT32: Self = Self(5);
    #[doc(alias = "VK_COMPONENT_TYPE_SINT64_NV")]
    pub const SINT64: Self = Self(6);
    #[doc(alias = "VK_COMPONENT_TYPE_UINT8_NV")]
    pub const UINT8: Self = Self(7);
    #[doc(alias = "VK_COMPONENT_TYPE_UINT16_NV")]
    pub const UINT16: Self = Self(8);
    #[doc(alias = "VK_COMPONENT_TYPE_UINT32_NV")]
    pub const UINT32: Self = Self(9);
    #[doc(alias = "VK_COMPONENT_TYPE_UINT64_NV")]
    pub const UINT64: Self = Self(10);
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
            x if x == Self::FLOAT16.bits() => Some(Self(x)),
            x if x == Self::FLOAT32.bits() => Some(Self(x)),
            x if x == Self::FLOAT64.bits() => Some(Self(x)),
            x if x == Self::SINT8.bits() => Some(Self(x)),
            x if x == Self::SINT16.bits() => Some(Self(x)),
            x if x == Self::SINT32.bits() => Some(Self(x)),
            x if x == Self::SINT64.bits() => Some(Self(x)),
            x if x == Self::UINT8.bits() => Some(Self(x)),
            x if x == Self::UINT16.bits() => Some(Self(x)),
            x if x == Self::UINT32.bits() => Some(Self(x)),
            x if x == Self::UINT64.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
pub type FNGetPhysicalDeviceCooperativeMatrixPropertiesNv = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeMatrixPropertiesNV,
) -> VulkanResultCodes;
