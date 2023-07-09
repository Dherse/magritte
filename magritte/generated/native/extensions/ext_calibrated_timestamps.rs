use crate::native::vulkan1_0::{BaseInStructure, Device, PhysicalDevice, StructureType, VulkanResultCodes};
#[doc(alias = "VkCalibratedTimestampInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CalibratedTimestampInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "timeDomain")]
    pub time_domain: TimeDomainEXT,
}
impl Default for CalibratedTimestampInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::CalibratedTimestampInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            time_domain: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_calibrated_timestamps::{
    TimeDomainEXT, EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME, EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
pub type FNGetPhysicalDeviceCalibrateableTimeDomainsExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_time_domain_count: *mut u32,
    p_time_domains: *mut TimeDomainEXT,
) -> VulkanResultCodes;
#[doc(alias = "vkGetCalibratedTimestampsEXT")]
pub type FNGetCalibratedTimestampsExt = unsafe extern "system" fn(
    device: Device,
    timestamp_count: u32,
    p_timestamp_infos: *const CalibratedTimestampInfoEXT,
    p_timestamps: *mut u64,
    p_max_deviation: *mut u64,
) -> VulkanResultCodes;
