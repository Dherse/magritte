use crate::native::vulkan1_0::{
    BaseOutStructure, Bool32, PhysicalDevice, ShaderStageFlags, StructureType, VulkanResultCodes,
};
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "cooperativeMatrix")]
    pub cooperative_matrix: Bool32,
    #[doc(alias = "cooperativeMatrixRobustBufferAccess")]
    pub cooperative_matrix_robust_buffer_access: Bool32,
}
impl Default for PhysicalDeviceCooperativeMatrixFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceCooperativeMatrixFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            cooperative_matrix: unsafe { std::mem::zeroed() },
            cooperative_matrix_robust_buffer_access: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceCooperativeMatrixPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCooperativeMatrixPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "cooperativeMatrixSupportedStages")]
    pub cooperative_matrix_supported_stages: ShaderStageFlags,
}
impl Default for PhysicalDeviceCooperativeMatrixPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceCooperativeMatrixPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            cooperative_matrix_supported_stages: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCooperativeMatrixPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CooperativeMatrixPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "MSize")]
    pub m_size: u32,
    #[doc(alias = "NSize")]
    pub n_size: u32,
    #[doc(alias = "KSize")]
    pub k_size: u32,
    #[doc(alias = "AType")]
    pub a_type: ComponentTypeNV,
    #[doc(alias = "BType")]
    pub b_type: ComponentTypeNV,
    #[doc(alias = "CType")]
    pub c_type: ComponentTypeNV,
    #[doc(alias = "DType")]
    pub d_type: ComponentTypeNV,
    pub scope: ScopeNV,
}
impl Default for CooperativeMatrixPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::CooperativeMatrixPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            m_size: unsafe { std::mem::zeroed() },
            n_size: unsafe { std::mem::zeroed() },
            k_size: unsafe { std::mem::zeroed() },
            a_type: unsafe { std::mem::zeroed() },
            b_type: unsafe { std::mem::zeroed() },
            c_type: unsafe { std::mem::zeroed() },
            d_type: unsafe { std::mem::zeroed() },
            scope: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_cooperative_matrix::{
    ComponentTypeNV, ScopeNV, NV_COOPERATIVE_MATRIX_EXTENSION_NAME, NV_COOPERATIVE_MATRIX_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV")]
pub type FNGetPhysicalDeviceCooperativeMatrixPropertiesNv = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeMatrixPropertiesNV,
) -> VulkanResultCodes;
