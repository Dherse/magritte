use crate::native::vulkan1_0::{BaseOutStructure, Bool32, DeviceSize, StructureType};
#[doc(alias = "VkPhysicalDeviceRobustness2FeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2FeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "robustBufferAccess2")]
    pub robust_buffer_access2: Bool32,
    #[doc(alias = "robustImageAccess2")]
    pub robust_image_access2: Bool32,
    #[doc(alias = "nullDescriptor")]
    pub null_descriptor: Bool32,
}
impl Default for PhysicalDeviceRobustness2FeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRobustness2FeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            robust_buffer_access2: unsafe { std::mem::zeroed() },
            robust_image_access2: unsafe { std::mem::zeroed() },
            null_descriptor: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceRobustness2PropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRobustness2PropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "robustStorageBufferAccessSizeAlignment")]
    pub robust_storage_buffer_access_size_alignment: DeviceSize,
    #[doc(alias = "robustUniformBufferAccessSizeAlignment")]
    pub robust_uniform_buffer_access_size_alignment: DeviceSize,
}
impl Default for PhysicalDeviceRobustness2PropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceRobustness2PropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            robust_storage_buffer_access_size_alignment: unsafe { std::mem::zeroed() },
            robust_uniform_buffer_access_size_alignment: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_robustness2::{EXT_ROBUSTNESS_2_EXTENSION_NAME, EXT_ROBUSTNESS_2_SPEC_VERSION};
