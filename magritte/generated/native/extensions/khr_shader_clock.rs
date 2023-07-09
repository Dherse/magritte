use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderClockFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderClockFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSubgroupClock")]
    pub shader_subgroup_clock: Bool32,
    #[doc(alias = "shaderDeviceClock")]
    pub shader_device_clock: Bool32,
}
impl Default for PhysicalDeviceShaderClockFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderClockFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            shader_subgroup_clock: unsafe { std::mem::zeroed() },
            shader_device_clock: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_shader_clock::{KHR_SHADER_CLOCK_EXTENSION_NAME, KHR_SHADER_CLOCK_SPEC_VERSION};
