pub use crate::common::extensions::khr_multiview::{KHR_MULTIVIEW_EXTENSION_NAME, KHR_MULTIVIEW_SPEC_VERSION};
use crate::vulkan1_1::{
    PhysicalDeviceMultiviewFeatures, PhysicalDeviceMultiviewProperties, RenderPassMultiviewCreateInfo,
};
#[doc(alias = "VkPhysicalDeviceMultiviewFeaturesKHR")]
pub type PhysicalDeviceMultiviewFeaturesKHR = PhysicalDeviceMultiviewFeatures;
#[doc(alias = "VkPhysicalDeviceMultiviewPropertiesKHR")]
pub type PhysicalDeviceMultiviewPropertiesKHR = PhysicalDeviceMultiviewProperties;
#[doc(alias = "VkRenderPassMultiviewCreateInfoKHR")]
pub type RenderPassMultiviewCreateInfoKHR = RenderPassMultiviewCreateInfo;
