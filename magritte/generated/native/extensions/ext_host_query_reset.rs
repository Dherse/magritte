use crate::native::vulkan1_2::{FNResetQueryPool, PhysicalDeviceHostQueryResetFeatures};
///See [`PhysicalDeviceHostQueryResetFeatures`]
#[doc(alias = "VkPhysicalDeviceHostQueryResetFeaturesEXT")]
pub type PhysicalDeviceHostQueryResetFeaturesEXT = PhysicalDeviceHostQueryResetFeatures;
pub use crate::common::extensions::ext_host_query_reset::{
    EXT_HOST_QUERY_RESET_EXTENSION_NAME, EXT_HOST_QUERY_RESET_SPEC_VERSION,
};
///See [`reset_query_pool`]
#[doc(alias = "vkResetQueryPoolEXT")]
pub type FNResetQueryPoolExt = FNResetQueryPool;
