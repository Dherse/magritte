use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceDiagnosticsConfigFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "diagnosticsConfig")]
    pub diagnostics_config: Bool32,
}
impl Default for PhysicalDeviceDiagnosticsConfigFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceDiagnosticsConfigFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            diagnostics_config: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkDeviceDiagnosticsConfigCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceDiagnosticsConfigCreateInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: DeviceDiagnosticsConfigFlagsNV,
}
impl Default for DeviceDiagnosticsConfigCreateInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceDiagnosticsConfigCreateInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_device_diagnostics_config::{
    DeviceDiagnosticsConfigFlagBitsNV, DeviceDiagnosticsConfigFlagsNV, NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME,
    NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION,
};
